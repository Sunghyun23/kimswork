#!/usr/bin/env python3
"""Copy optional banks to an owner-identified Mac-mounted Sample SD. Dry-run default."""
import argparse,hashlib,json,os,plistlib,re,shutil,struct,subprocess
from pathlib import Path

def sha(path):
    h=hashlib.sha256()
    with path.open('rb')as f:
        for b in iter(lambda:f.read(8*1024*1024),b''):h.update(b)
    return h.hexdigest()
def inspect(path):
    with path.open('rb')as f:
        b=f.read(16)
        if len(b)!=16 or b[:8]!=b'LCBANK01':raise ValueError('Not a Loop Club bank')
        n=struct.unpack_from('<Q',b,8)[0]
        if n>16*1024*1024:raise ValueError('Metadata too large')
        m=json.loads(f.read(n))
    if not re.fullmatch('[a-zA-Z0-9_-]{1,63}',m.get('id',''))or not re.fullmatch(r'\d+\.\d+\.\d+',m.get('version',''))or not isinstance(m.get('redistributable'),bool):raise ValueError('Invalid bank identity')
    if n+16+m['payload_bytes']!=path.stat().st_size:raise ValueError('Bank length mismatch')
    return m

def install(bank,folder,allow_private=False,apply=False):
    m=inspect(bank)
    if not m['redistributable']and not allow_private:raise ValueError('Private bank requires --allow-private')
    target=folder/(m['id']+'-v'+m['version']+'.lcbank');checksum=sha(bank)
    if target.exists():
        if target.is_symlink()or sha(target)!=checksum:raise ValueError('Different bank already exists; nothing overwritten')
        return {'target':str(target),'status':'already present, SHA256 verified','sha256':checksum}
    if apply:
        folder.mkdir(parents=True,exist_ok=True)
        with target.open('xb')as dst:
            try:
                with bank.open('rb')as src:shutil.copyfileobj(src,dst,8*1024*1024)
                dst.flush();os.fsync(dst.fileno())
            except BaseException:
                dst.close();target.unlink();raise
        if sha(target)!=checksum:raise RuntimeError('Bank read-back mismatch: '+str(target))
    return {'target':str(target),'status':'copied and SHA256 verified'if apply else'planned','sha256':checksum,'private':not m['redistributable']}

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--card',type=Path,required=True);p.add_argument('--bank',type=Path,action='append',required=True);p.add_argument('--apply',action='store_true');p.add_argument('--allow-private',action='store_true');a=p.parse_args();card=a.card.resolve()
    info=plistlib.loads(subprocess.check_output(['diskutil','info','-plist',str(card)]))
    if not card.is_mount()or info.get('MountPoint')!=str(card)or info.get('Internal')is not False or info.get('FilesystemType','').lower()not in ('exfat','msdos'):p.error('Expected identified external FAT/exFAT card root')
    folder=card/'LoopClub/Expansions'
    for parent in [card/'LoopClub',folder]:
        if parent.is_symlink():p.error('Refusing a bank folder symlink')
    # Validate every source/destination before writing any bank.
    for b in a.bank:install(b,folder,a.allow_private,False)
    result=[install(b,folder,a.allow_private,a.apply)for b in a.bank]
    print(json.dumps({'card':str(card),'apply':a.apply,'restart':False,'banks':result},indent=2))
if __name__=='__main__':main()
