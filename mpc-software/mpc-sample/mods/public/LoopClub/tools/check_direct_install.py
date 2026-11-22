#!/usr/bin/env python3
"""Host-only direct-boot migration/preservation/rollback fixtures."""
import json,tempfile,unittest
from pathlib import Path
from unittest.mock import patch
import install_direct
ARCHIVE=Path(__file__).resolve().parents[1]/'dist/loop-club-0.1.0-sample-usb-qa-direct.zip'
class Migration(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory(prefix='loopclub-direct-test-');self.card=Path(self.temp.name)
        self.active=self.card/'autorun';(self.active/'mods/Old').mkdir(parents=True)
        (self.active/'mods/Old/old.so').write_bytes(b'old runtime')
        (self.active/'autorun.sh').write_bytes(b'old SampleBoot')
        (self.active/'._autorun.sh').write_bytes(b'original sidecar bytes')
        (self.card/'owner-project').write_bytes(b'owner project untouched')
        self.before=install_direct.hashes(self.active)
    def tearDown(self):self.temp.cleanup()
    def test_direct_boot_preserves_all_old_bytes_and_owner_project(self):
        report=install_direct.install(self.card,ARCHIVE,True)
        self.assertFalse((self.active/'mods').exists())
        self.assertFalse(json.loads((self.active/'autorun.json').read_text())['value0']['scripts'][0]['askQuestion'])
        self.assertEqual(install_direct.hashes(Path(report['backup'])/'autorun-original'),self.before)
        self.assertEqual((self.card/'owner-project').read_bytes(),b'owner project untouched')
    def test_failed_activation_restores_original_boot(self):
        rename=Path.rename
        def fail(source,target):
            if source.name.startswith('.loopclub-direct-'):raise OSError('injected activation failure')
            return rename(source,target)
        with patch.object(Path,'rename',fail),self.assertRaises(OSError):install_direct.install(self.card,ARCHIVE,True)
        self.assertEqual(install_direct.hashes(self.active),self.before)
    def test_new_appledouble_companions_are_recorded_not_confused_with_payload(self):
        copy=install_direct.shutil.copytree
        def with_metadata(source,target,**kwargs):
            result=copy(source,target,**kwargs)
            (Path(target)/'._autorun.sh').write_bytes(b'new macOS metadata')
            return result
        with patch.object(install_direct.shutil,'copytree',with_metadata):report=install_direct.install(self.card,ARCHIVE,True)
        self.assertEqual((self.active/'._autorun.sh').read_bytes(),b'new macOS metadata')
        self.assertEqual(install_direct.hashes(Path(report['backup'])/'autorun-original'),self.before)
    def test_dry_run_never_moves_boot(self):
        report=install_direct.install(self.card,ARCHIVE,False)
        self.assertFalse(report['apply']);self.assertEqual(install_direct.hashes(self.active),self.before)
        self.assertFalse((self.card/'.LoopClubBackups').exists())
if __name__=='__main__':unittest.main(verbosity=2)
