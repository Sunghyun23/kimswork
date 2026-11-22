#!/usr/bin/env python3
"""Inspect/prepare a Mac-mounted SD. Dry-run by default; never replaces existing files."""
import argparse
import hashlib
import json
from pathlib import Path
import plistlib
import shutil
import subprocess
import tempfile
import uuid
import re
from datetime import datetime,timezone
import zipfile

PROJECT=Path(__file__).resolve().parents[1]
REPO=PROJECT.parents[4]
BOOT_SHA='61ea76816035496a969690c514a05fa7ba10dc0e8a447ec7233f59bc05c17c9f'

def copy_bytes(source,target):
    # copy2/xattrs can rewrite explicit AppleDouble sidecars on exFAT. Preserve
    # every file byte-for-byte and set only the executable/permission bits.
    shutil.copyfile(source,target)
    Path(target).chmod(Path(source).stat().st_mode & 0o777)
    return str(target)

def extract_under(archive,prefix,dest):
    with zipfile.ZipFile(archive)as z:
        for entry in z.infolist():
            path=Path(entry.filename)
            if '..' in path.parts or path.is_absolute(): raise ValueError('Unsafe ZIP path')
            try:relative=path.relative_to(prefix)
            except ValueError:continue
            if entry.is_dir():continue
            if (entry.external_attr>>16)&0o170000==0o120000:raise ValueError('ZIP symlinks unsupported')
            target=dest/relative;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(z.read(entry))
            if relative.suffix=='.sh':target.chmod(0o755)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--card',type=Path,required=True,help='Exact mounted removable SD root')
    parser.add_argument('--archive',type=Path,help='Explicit verified candidate archive; default is the normal development ZIP')
    parser.add_argument('--folder-name',default='Loop Club 0.1.0',help='New unique menu folder, never overwrites existing versions')
    parser.add_argument('--migrate-sampqol',action='store_true',help='Preserve/verify the existing SampQOL boot files and add them to SampleBoot alongside Loop Club')
    parser.add_argument('--apply',action='store_true',help='Create a new installation only; existing targets are refused')
    args=parser.parse_args();card=args.card.resolve()
    if not card.is_mount() or str(card) in ('/','/Volumes'):parser.error('Specify a mounted SD root')
    info=plistlib.loads(subprocess.check_output(['diskutil','info','-plist',str(card)]))
    if info.get('MountPoint')!=str(card) or info.get('Internal') is not False:parser.error('Expected an external removable card, not internal storage')
    if info.get('FilesystemType','').lower() not in ('msdos','exfat'):parser.error('Expected FAT/exFAT SD filesystem')
    archive=args.archive or PROJECT/'dist/loop-club-0.1.0-sample-development.zip'
    if not re.fullmatch('[A-Za-z0-9 _.-]{1,63}',args.folder_name)or args.folder_name.startswith('.'):parser.error('Invalid new menu folder name')
    boot=REPO/'.sdk-tools/mpc-sample/downloads/SampleBoot-1.0.2.zip'
    expected=archive.with_suffix('.zip.sha256').read_text().split()[0]
    if hashlib.sha256(archive.read_bytes()).hexdigest()!=expected:parser.error('Loop Club package checksum mismatch')
    autorun=card/'autorun';exists=autorun.exists()
    if autorun.is_symlink():parser.error('Refusing an autorun symlink')
    sampleboot=(autorun/'sampleboot/sampleboot.conf').is_file()
    migrating=exists and not sampleboot
    if migrating:
        if not args.migrate_sampqol:parser.error('Existing autorun is not SampleBoot; inspected SampQOL migration requires --migrate-sampqol. Nothing changed.')
        required=['autorun.sh','autorun.json','autorun.check','mpcqol.so','qol-run.sh','qol-snap.sh']
        if not all((autorun/name).is_file()for name in required)or 'SampQOL' not in (autorun/'autorun.json').read_text():parser.error('Existing autorun is not recognized SampQOL; migration refused')
        if any(p.is_symlink()for p in autorun.rglob('*')):parser.error('Existing boot symlinks need manual inspection')
    target=autorun/'mods'/args.folder_name
    if target.exists():parser.error('Version folder already exists. Nothing overwritten.')
    if (not exists or migrating) and hashlib.sha256(boot.read_bytes()).hexdigest()!=BOOT_SHA:parser.error('Pinned SampleBoot checksum mismatch')
    report={'card':str(card),'volume':info.get('VolumeName'),'action':'add new version folder to existing SampleBoot' if exists else 'create new SampleBoot autorun with Loop Club','target':str(target),'preserve':'all existing autorun files, samples and projects','apply':args.apply,'restart':False,'hardware_verified':False}
    report['migrate_sampqol']=migrating
    if migrating:report['action']='back up SampQOL and preserve it as a SampleBoot menu entry alongside Loop Club'
    if args.apply:
        with tempfile.TemporaryDirectory(prefix='loopclub-install-')as tmp:
            stage=Path(tmp)
            if not exists or migrating:extract_under(boot,Path('SampleBoot-1.0.2/autorun'),stage/'autorun')
            own=stage/'loop-club';extract_under(archive,Path('autorun'),own)
            if migrating:
                stamp=datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')+'-'+uuid.uuid4().hex[:8]
                backup=card/'.LoopClubBackups'/stamp;backup.mkdir(parents=True,exist_ok=False)
                originals={str(p.relative_to(autorun)):hashlib.sha256(p.read_bytes()).hexdigest()for p in autorun.rglob('*')if p.is_file()}
                # macOS normalizes AppleDouble (._*) bytes on exFAT. Archive every
                # original byte, including sidecars, and retain the original tree
                # by rename. Only runtime files belong in the active mod clone.
                with zipfile.ZipFile(backup/'autorun-original.zip','x',zipfile.ZIP_STORED)as z:
                    for rel in originals:z.writestr(rel,(autorun/rel).read_bytes())
                shutil.copytree(autorun,stage/'autorun/mods/SampQOL OS',copy_function=copy_bytes,ignore=shutil.ignore_patterns('._*'))
                if (card/'mpcqol.conf').is_file():copy_bytes(card/'mpcqol.conf',backup/'mpcqol.conf')
                with zipfile.ZipFile(backup/'autorun-original.zip')as z:
                    for rel,sha in originals.items():
                        if hashlib.sha256(z.read(rel)).hexdigest()!=sha:raise RuntimeError('SampQOL archive mismatch: '+rel)
                        if not any(part.startswith('._')for part in Path(rel).parts) and hashlib.sha256((stage/'autorun/mods/SampQOL OS'/rel).read_bytes()).hexdigest()!=sha:raise RuntimeError('SampQOL runtime mismatch: '+rel)
                shutil.copytree(own,stage/'autorun/mods'/args.folder_name,copy_function=copy_bytes)
                incoming=card/('.loopclub-autorun-'+stamp);shutil.copytree(stage/'autorun',incoming,copy_function=copy_bytes)
                for path in (stage/'autorun').rglob('*'):
                    if path.is_file() and hashlib.sha256(path.read_bytes()).digest()!=hashlib.sha256((incoming/path.relative_to(stage/'autorun')).read_bytes()).digest():raise RuntimeError('Staged boot read-back mismatch')
                (backup/'manifest.json').write_text(json.dumps({'scope':'boot files/config only; not a full card backup','original_files':originals,'restore':'Move active autorun aside, restore autorun-original to card root'},indent=2)+'\n')
                autorun.rename(backup/'autorun-original')
                try:
                    for rel,sha in originals.items():
                        if hashlib.sha256((backup/'autorun-original'/rel).read_bytes()).hexdigest()!=sha:raise RuntimeError('Moved original mismatch: '+rel)
                    incoming.rename(autorun)
                except BaseException:
                    (backup/'autorun-original').rename(autorun);raise
                report['sampqol_backup']=str(backup);report['sampqol_preserved_sha256_files']=len(originals);report['appledouble_policy']='all original sidecar bytes retained in verified ZIP and moved original; active clone contains runtime files only'
            elif exists:
                # Only a brand-new destination is copied. Do not merge or replace.
                shutil.copytree(own,target)
            else:
                shutil.copytree(own,stage/'autorun/mods'/args.folder_name,copy_function=copy_bytes)
                shutil.copytree(stage/'autorun',autorun)
        # Complete read-back of every newly installed Loop Club file.
        with zipfile.ZipFile(archive)as z:
            for entry in z.infolist():
                if entry.is_dir():continue
                rel=Path(entry.filename).relative_to('autorun')
                if hashlib.sha256((target/rel).read_bytes()).digest()!=hashlib.sha256(z.read(entry)).digest():raise RuntimeError(f'Read-back mismatch: {rel}')
        report['read_back']='all new Loop Club files SHA-256 exact'
    print(json.dumps(report,indent=2))

if __name__=='__main__':main()
