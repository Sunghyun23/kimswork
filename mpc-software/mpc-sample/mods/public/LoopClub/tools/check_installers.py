#!/usr/bin/env python3
"""Host fixtures only: no SD mounts, diskutil calls or device services are used."""
import contextlib,hashlib,importlib.util,io,json,plistlib,sys,tempfile,unittest,zipfile
from pathlib import Path
from unittest.mock import patch
ROOT=Path(__file__).resolve().parent

def load(name):
    spec=importlib.util.spec_from_file_location(name,ROOT/(name+'.py'));m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

class Installers(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='lc-install-fixture-');self.root=Path(self.tmp.name).resolve();self.card=self.root/'card';self.card.mkdir();self.repo=self.root/'repo';self.project=self.repo/'project';self.project.mkdir(parents=True)
        self.tool=load('install_card');self.tool.PROJECT=self.project;self.tool.REPO=self.repo
        boot=self.repo/'.sdk-tools/mpc-sample/downloads/SampleBoot-1.0.2.zip';boot.parent.mkdir(parents=True)
        with zipfile.ZipFile(boot,'w')as z:
            for n,data in [('autorun.sh','fixture launcher'),('sampleboot/sampleboot.conf','fixture config')]:z.writestr('SampleBoot-1.0.2/autorun/'+n,data)
        self.tool.BOOT_SHA=hashlib.sha256(boot.read_bytes()).hexdigest()
        own=self.project/'dist/loop-club-0.1.0-sample-development.zip';own.parent.mkdir()
        with zipfile.ZipFile(own,'w')as z:z.writestr('autorun/autorun.sh','fixture Loop Club');z.writestr('autorun/loop-club.so',b'fixture binary')
        own.with_suffix('.zip.sha256').write_text(hashlib.sha256(own.read_bytes()).hexdigest())
        old=self.card/'autorun';old.mkdir()
        self.original={name:(b'SampQOL fixture'if name=='autorun.json'else ('original '+name).encode())for name in ['autorun.sh','autorun.json','autorun.check','mpcqol.so','qol-run.sh','qol-snap.sh']}
        self.original['._qol-snap.sh']=b'AppleDouble fixture bytes'
        for n,data in self.original.items():(old/n).write_bytes(data)
        (self.card/'mpcqol.conf').write_text('owner config');(self.card/'project.dat').write_bytes(b'owner project')
    def tearDown(self):self.tmp.cleanup()
    def invoke(self,*args):
        info=plistlib.dumps({'MountPoint':str(self.card),'Internal':False,'FilesystemType':'exfat','VolumeName':'fixture'})
        out=io.StringIO()
        with patch.object(Path,'is_mount',return_value=True),patch.object(self.tool.subprocess,'check_output',return_value=info),patch.object(sys,'argv',['install_card.py','--card',str(self.card),*args]),contextlib.redirect_stdout(out),contextlib.redirect_stderr(io.StringIO()):self.tool.main()
        return json.loads(out.getvalue())
    def test_foreign_boot_is_refused_without_explicit_migration(self):
        with self.assertRaises(SystemExit):self.invoke('--apply')
        for n,data in self.original.items():self.assertEqual((self.card/'autorun'/n).read_bytes(),data)
    def test_dry_run_changes_nothing(self):
        before=sorted(str(p.relative_to(self.card))for p in self.card.rglob('*'));report=self.invoke('--migrate-sampqol');self.assertTrue(report['migrate_sampqol']);self.assertEqual(before,sorted(str(p.relative_to(self.card))for p in self.card.rglob('*')))
    def test_migration_retains_every_original_and_owner_data(self):
        report=self.invoke('--migrate-sampqol','--apply');backup=Path(report['sampqol_backup'])
        for n,data in self.original.items():
            self.assertEqual((backup/'autorun-original'/n).read_bytes(),data)
            with zipfile.ZipFile(backup/'autorun-original.zip')as z:self.assertEqual(z.read(n),data)
            if not n.startswith('._'):self.assertEqual((self.card/'autorun/mods/SampQOL OS'/n).read_bytes(),data)
        self.assertEqual((self.card/'mpcqol.conf').read_text(),'owner config');self.assertEqual((self.card/'project.dat').read_bytes(),b'owner project');self.assertTrue((self.card/'autorun/mods/Loop Club 0.1.0/loop-club.so').is_file())
        with self.assertRaises(SystemExit):self.invoke('--apply')
    def test_copy_does_not_rewrite_appledouble_metadata(self):
        def refuse(*args,**kwargs):raise AssertionError('copy2 metadata replication is forbidden for boot migration')
        with patch.object(self.tool.shutil,'copy2',refuse):self.invoke('--migrate-sampqol','--apply')
        backup=next((self.card/'.LoopClubBackups').iterdir())
        with zipfile.ZipFile(backup/'autorun-original.zip')as z:self.assertEqual(z.read('._qol-snap.sh'),self.original['._qol-snap.sh'])
    def test_new_qa_slot_keeps_existing_installation(self):
        self.invoke('--migrate-sampqol','--apply')
        normal=self.card/'autorun/mods/Loop Club 0.1.0/loop-club.so';original=normal.read_bytes()
        self.invoke('--folder-name','Loop Club USB QA','--apply')
        self.assertEqual(normal.read_bytes(),original);self.assertEqual((self.card/'autorun/mods/Loop Club USB QA/loop-club.so').read_bytes(),original)
    def test_failed_activation_restores_original_boot(self):
        rename=Path.rename
        def fail(src,dst):
            if src.name.startswith('.loopclub-autorun-'):raise OSError('fixture activation failure')
            return rename(src,dst)
        with patch.object(Path,'rename',fail),self.assertRaises(OSError):self.invoke('--migrate-sampqol','--apply')
        for n,data in self.original.items():self.assertEqual((self.card/'autorun'/n).read_bytes(),data)
    def test_bank_copy_private_guard_readback_and_no_overwrite(self):
        tool=load('install_banks');source=self.root/'source.lcbank';meta={'id':'private-fixture','version':'1.0.0','redistributable':False,'payload_bytes':8};data=json.dumps(meta).encode();source.write_bytes(b'LCBANK01'+len(data).to_bytes(8,'little')+data+b'GDBANK01');dest=self.card/'LoopClub/Expansions'
        with self.assertRaises(ValueError):tool.install(source,dest,False,True)
        self.assertFalse(dest.exists());result=tool.install(source,dest,True,True);self.assertEqual(Path(result['target']).read_bytes(),source.read_bytes());self.assertIn('already present',tool.install(source,dest,True,True)['status']);Path(result['target']).write_bytes(b'owner changed data')
        with self.assertRaises(ValueError):tool.install(source,dest,True,True)

if __name__=='__main__':unittest.main()
