#!/usr/bin/env python3
"""Replace active boot with direct Loop Club; preserve original autorun exactly."""
import argparse,hashlib,json,plistlib,subprocess,tempfile,uuid,zipfile
from datetime import datetime,timezone
from pathlib import Path
from install_card import extract_under,copy_bytes
import shutil

def hashes(root):
    result={}
    for p in root.rglob('*'):
        if p.is_symlink():raise ValueError('Boot symlinks require manual review')
        if p.is_file():result[str(p.relative_to(root))]=hashlib.sha256(p.read_bytes()).hexdigest()
    return result

def install(card,archive,apply):
    expected=archive.with_suffix('.zip.sha256').read_text().split()[0]
    if hashlib.sha256(archive.read_bytes()).hexdigest()!=expected:raise ValueError('Package SHA mismatch')
    with zipfile.ZipFile(archive) as z:
        meta=json.loads(z.read('autorun/compatibility.json'))
        manifest=json.loads(z.read('autorun/autorun.json'))
        if meta.get('boot_mode')!='direct' or manifest['value0']['scripts'][0]['askQuestion'] is not False:raise ValueError('Expected direct-boot package')
    active=card/'autorun'
    if not active.is_dir() or active.is_symlink():raise ValueError('Expected existing autorun to preserve')
    before=hashes(active)
    report={'mode':'direct Loop Club; no SampleBoot menu','apply':apply,'archive_sha256':expected,'old_files_to_preserve':len(before),'projects_samples_banks':'untouched','restart':False}
    if not apply:return report
    stamp=datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')+'-'+uuid.uuid4().hex[:8]
    backup=card/'.LoopClubBackups'/stamp;backup.mkdir(parents=True)
    incoming=card/('.loopclub-direct-'+stamp)
    with tempfile.TemporaryDirectory(prefix='loopclub-direct-') as temp:
        prepared=Path(temp)/'autorun'
        extract_under(archive,Path('autorun'),prepared)
        wanted=hashes(prepared)
        shutil.copytree(prepared,incoming,copy_function=copy_bytes)
        actual=hashes(incoming)
        if any(actual.get(name)!=sha for name,sha in wanted.items()):raise RuntimeError('New boot read-back mismatch before activation')
        extras=set(actual)-set(wanted)
        if any(not Path(name).name.startswith('._') for name in extras):raise RuntimeError('Unexpected non-metadata file in new boot')
        # exFAT on macOS creates AppleDouble companions when permissions are set.
        # Runtime payload must match exactly; record those new metadata bytes too.
        wanted=actual
    # ZIP keeps every original byte, including macOS AppleDouble companions.
    with zipfile.ZipFile(backup/'autorun-original.zip','w',zipfile.ZIP_DEFLATED) as z:
        for rel in before:z.write(active/rel,rel)
    with zipfile.ZipFile(backup/'autorun-original.zip') as z:
        for rel,sha in before.items():
            if hashlib.sha256(z.read(rel)).hexdigest()!=sha:raise RuntimeError('Original ZIP verification failed')
    (backup/'manifest.json').write_text(json.dumps({'original_sha256':before,'new_sha256':wanted,'restore':'Move current autorun aside, rename autorun-original back to card root'},indent=2)+'\n')
    active.rename(backup/'autorun-original')
    try:
        if hashes(backup/'autorun-original')!=before:raise RuntimeError('Moved original changed')
        incoming.rename(active)
        if hashes(active)!=wanted:raise RuntimeError('Activated boot verification failed')
    except BaseException:
        if active.exists():active.rename(card/('.loopclub-direct-failed-'+stamp))
        (backup/'autorun-original').rename(active)
        raise
    report.update(backup=str(backup),new_files_sha256_verified=len(wanted),old_files_sha256_preserved=len(before),installed=True)
    return report

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--card',type=Path,required=True);p.add_argument('--volume-uuid',required=True);p.add_argument('--archive',type=Path,required=True);p.add_argument('--apply',action='store_true');a=p.parse_args()
    card=a.card.resolve()
    if not card.is_mount() or card==Path('/'):p.error('Expected mounted external card')
    info=plistlib.loads(subprocess.check_output(['diskutil','info','-plist',str(card)]))
    if info.get('Internal') is not False or info.get('VolumeUUID')!=a.volume_uuid or info.get('FilesystemType','').lower() not in ['exfat','msdos']:p.error('Card identity/type mismatch')
    print(json.dumps(install(card,a.archive,a.apply),indent=2))
if __name__=='__main__':main()
