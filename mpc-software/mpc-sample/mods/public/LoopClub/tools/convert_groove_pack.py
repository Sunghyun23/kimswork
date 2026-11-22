#!/usr/bin/env python3
"""Convert an existing indexed Groove Dice bank to a standalone Loop Club bank.
No device access. Private sources require --allow-private and stay private.
The original bank payload is copied unchanged; all indexed WAV/MIDI records survive.
"""
import argparse,hashlib,json,os,struct,sys,zipfile,zlib
from pathlib import Path

def digest(path):
    h=hashlib.sha256()
    with path.open('rb')as f:
        for b in iter(lambda:f.read(8*1024*1024),b''):h.update(b)
    return h.hexdigest()
def read(f,pos,n):
    f.seek(pos);b=f.read(n)
    if len(b)!=n:raise ValueError('Truncated bank')
    return b

def convert(bank,archive,out,allow_private=False):
    bank,archive,out=map(Path,(bank,archive,out))
    if out.exists():raise ValueError('Destination already exists; never overwrite')
    with zipfile.ZipFile(archive)as z:
        if z.getinfo('manifest.json').file_size>16384:raise ValueError('Oversized manifest')
        m=json.loads(z.read('manifest.json'))
    if m.get('format')!='GDEXP001' or not isinstance(m.get('redistributable'),bool):raise ValueError('Invalid source manifest')
    if not m['redistributable']and not allow_private:raise ValueError('Private source requires --allow-private')
    if m['bytes']!=bank.stat().st_size or digest(bank)!=m['sha256']:raise ValueError('Source hash mismatch')
    total=bank.stat().st_size
    with bank.open('rb')as f:
        if read(f,0,8)!=b'GDBANK01':raise ValueError('Invalid bank signature')
        tail,start=struct.unpack('<8sQ',read(f,total-16,16))
        sig,version,count,width,*_=struct.unpack('<8s6I',read(f,start,32))
        if tail!=b'GDLEND02'or sig!=b'GDLIB002'or version!=2 or width!=384 or not 1<=count<=50000:raise ValueError('Invalid library directory')
        records=[];lookup={}
        for i in range(count):
            b=read(f,start+32+i*384,384)
            sha,name,offset,*nums=struct.unpack('<64s256sQ10I16x',b)
            size,crc,kind,role,rate,channels,frames,*_=nums
            if offset>total or size>total-offset or size>64*1024*1024:raise ValueError('Record bounds')
            sha=sha.rstrip(b'\0').decode('ascii');name=name.rstrip(b'\0').decode('utf-8')
            lookup.setdefault(sha,i)
            records.append(dict(id=sha,name=name,offset=offset,bytes=size,crc=crc,kind=kind,role=role,rate=rate,channels=channels,frames=frames))
        marker,kitstart=struct.unpack('<8sQ',read(f,total-32,16))
        if marker!=b'GDKEND03':raise ValueError('Source has no factory kit index')
        sig,n,width=struct.unpack('<8s2I',read(f,kitstart,16))
        if sig!=b'GDKITS03'or width!=800 or not 1<=n<=512:raise ValueError('Invalid kit index')
        kits=[]
        # Groove roles: kick/snare/closed hat/open hat/clap/rim/crash/perc.
        pads=[0,1,2,3,4,5,13,7]
        for i in range(n):
            b=read(f,kitstart+16+i*800,800)
            if zlib.crc32(b[:796])!=struct.unpack_from('<I',b,796)[0]:raise ValueError('Kit CRC mismatch')
            name=b[:64].split(b'\0',1)[0].decode('utf-8');sounds=[None]*16;levels=[0.7]*16;tunes=[0.]*16
            for role,pad in enumerate(pads):
                key=b[64+role*64:128+role*64].decode('ascii');index=lookup.get(key)
                if index is None or records[index]['kind']!=0:raise ValueError('Missing kit sample')
                sounds[pad]=index
                db,tune,*_=struct.unpack_from('<5f',b,576+role*20)
                levels[pad]=min(1.,10**(db/20)*0.7);tunes[pad]=tune
            kits.append(dict(name=name,sounds=sounds,levels=levels,tunes=tunes))
    meta=dict(format='LCBANK01',id=m['id'],name=m['name'],version=m.get('version','1.0.0'),redistributable=m['redistributable'],rights=m.get('rights',''),payload_bytes=total,source_sha256=m['sha256'],records=records,kits=kits)
    data=json.dumps(meta,ensure_ascii=False,separators=(',',':')).encode()
    if len(data)>16*1024*1024:raise ValueError('Metadata exceeds limit')
    out.parent.mkdir(parents=True,exist_ok=True)
    tmp=out.with_suffix(out.suffix+'.partial')
    try:
        with tmp.open('xb')as dst,bank.open('rb')as src:
            dst.write(b'LCBANK01'+struct.pack('<Q',len(data))+data)
            for block in iter(lambda:src.read(8*1024*1024),b''):dst.write(block)
            dst.flush();os.fsync(dst.fileno())
        # Exclusive final creation; a concurrent existing destination is preserved.
        os.link(tmp,out);tmp.unlink()
    finally:
        if tmp.exists():tmp.unlink()
    return dict(path=str(out),sha256=digest(out),records=len(records),wav=sum(r['kind']<3 for r in records),drum_wav=sum(r['kind']==0 for r in records),midi=sum(r['kind']>=3 for r in records),kits=len(kits),redistributable=m['redistributable'])
def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--bank',type=Path,required=True);p.add_argument('--source-archive',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--allow-private',action='store_true');a=p.parse_args()
    print(json.dumps(convert(a.bank,a.source_archive,a.out,a.allow_private),indent=2))
if __name__=='__main__':main()
