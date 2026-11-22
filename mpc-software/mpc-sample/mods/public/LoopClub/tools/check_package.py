#!/usr/bin/env python3
"""Check a candidate ZIP and exercise its launcher in a fully redirected host fixture."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import zipfile

PROJECT = Path(__file__).resolve().parents[1]
SALT = b'f7bed1c9-7e90-4731-9194-c996666b4331'

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--archive',type=Path);args=parser.parse_args()
    archive = args.archive or PROJECT/'dist/loop-club-0.1.0-sample-development.zip'
    results=[]
    with zipfile.ZipFile(archive) as z:
        for info in z.infolist():
            path=Path(info.filename)
            assert not path.is_absolute() and '..' not in path.parts
            assert info.file_size < 20*1024*1024
        check=z.read('autorun/autorun.check').decode()
        assert check==hashlib.sha256(z.read('autorun/autorun.json')+z.read('autorun/autorun.sh')+SALT).hexdigest()
        metadata=json.loads(z.read('autorun/compatibility.json'))
        assert metadata['library_sha256']==hashlib.sha256(z.read('autorun/loop-club.so')).hexdigest()
        assert metadata['device_installed'] is False
        results.extend(['manifest/checksum integrity','development status retained'])
        for nested in (False,True):
            for match,library in ((False,True),(True,True),(True,False)):
                with tempfile.TemporaryDirectory(prefix='loopclub-launch-') as tmp:
                    root=Path(tmp)
                    card=root/'card with spaces'
                    own=card/'autorun'
                    if nested: own=own/'mods/Loop Club 0.1.0'
                    own.mkdir(parents=True)
                    for entry in z.infolist():
                        if entry.is_dir(): continue
                        rel=Path(entry.filename).relative_to('autorun')
                        target=own/rel;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(z.read(entry))
                    if not library:(own/'loop-club.so').write_bytes(b'tampered')
                    run=root/'run';run.mkdir()
                    binary=root/'stock-app';binary.write_bytes(b'fixture firmware'*500)
                    digest=hashlib.sha256(binary.read_bytes()[:4096]).hexdigest()
                    mounts=root/'mounts'
                    mounts.write_text('/dev/fixture '+str(card).replace(' ','\\040')+' vfat rw 0 0\n')
                    helpers=root/'bin';helpers.mkdir()
                    (helpers/'sha256sum').write_text('#!/usr/bin/env python3\nimport sys,hashlib\nb=open(sys.argv[1],"rb").read() if len(sys.argv)>1 else sys.stdin.buffer.read()\nprint(hashlib.sha256(b).hexdigest()+"  fixture")\n')
                    (helpers/'systemctl').write_text('#!/bin/sh\nprintf "%s\\n" "$*" >> "$SYSTEMCTL_LOG"\n')
                    for helper in helpers.iterdir():helper.chmod(0o755)
                    script=(own/'autorun.sh').read_text()
                    script=script.replace('/usr/bin/MPC Sample',str(binary)).replace('/proc/mounts',str(mounts)).replace('/run/',str(run)+'/')
                    script=script.replace(metadata['expected_app_first_4KiB_sha256'],digest if match else '0'*64)
                    (own/'autorun.sh').write_text(script)
                    if metadata.get('qa_agent_sha256'):
                        agent=z.read('autorun/loop-club-qa-agent')
                        assert hashlib.sha256(agent).hexdigest()==metadata['qa_agent_sha256']
                        (helpers/'systemd-run').write_text('#!/bin/sh\nprintf "%s\\n" "$*" >> "$AGENT_CALLS"\n')
                        (helpers/'systemd-run').chmod(0o755)
                    env=os.environ.copy();env['PATH']=str(helpers)+os.pathsep+env['PATH'];env['SYSTEMCTL_LOG']=str(root/'calls');env['AGENT_CALLS']=str(root/'agent-calls')
                    subprocess.run(['sh',str(own/'autorun.sh')],env=env,check=True,capture_output=True,text=True)
                    if match and library:
                        calls=(root/'calls').read_text().splitlines()
                        assert calls[:2]==['daemon-reload','restart ac50']
                        if metadata.get('usb_qa_enabled',False):
                            assert len(calls)==(4 if metadata.get('qa_agent_sha256') else 3) and calls[2].startswith('show ac50.service --property=MainPID')
                            if metadata.get('qa_agent_sha256'):
                                assert calls[3]=='stop loopclub-qa-agent.service'
                                assert '--unit=loopclub-qa-agent --collect ' in (root/'agent-calls').read_text()
                        else:assert len(calls)==2
                        assert (run/'loop-club.so').read_bytes()==z.read('autorun/loop-club.so')
                        unit=(run/'systemd/system/ac50.service.d/loop-club.conf').read_text()
                        assert f'Environment="LOOP_CLUB_DATA_DIR={card}/LoopClub/Projects"' in unit.splitlines()
                        assert f'Environment="LOOP_CLUB_CARD_DIR={card}"' in unit.splitlines()
                        assert ('Environment="LOOP_CLUB_QA=1"' in unit.splitlines())==metadata.get('usb_qa_enabled',False)
                    else:
                        assert not (root/'calls').exists()
                        assert not (run/'loop-club.so').exists()
                    results.append(f'{"SampleBoot relocated" if nested else "SD root"} launcher: firmware={match} library={library}')
    print(json.dumps({'scope':'host fixture; no device commands executed','checks':results,'passed':len(results)},indent=2))

if __name__=='__main__': main()
