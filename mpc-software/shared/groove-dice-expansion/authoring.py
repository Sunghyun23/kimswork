"""Folder-based Groove Dice bank authoring. Standard-library only; no source mutation."""
from __future__ import annotations
import array, copy, hashlib, io, json, math, os, random, re, struct, tempfile, wave, zlib
from pathlib import Path

ROLES = ('Kick', 'Snare', 'Closed Hat', 'Open Hat', 'Clap', 'Tom', 'Cymbal', 'Perc')
ALIASES = ({'kick','kicks','bd','bassdrum'}, {'snare','snares','sd'},
 {'closedhat','closedhats','closedhihat','ch','chh','hihat','hihats','hat','hats'},
 {'openhat','openhats','openhihat','oh','ohh'}, {'clap','claps'}, {'tom','toms'},
 {'cymbal','cymbals','ride','rides','crash','crashes'}, {'perc','percs','percussion','shaker','rim','rimshot','conga','bongo'})
MAX_FILES=5000

def token(value): return re.sub('[^a-z0-9]', '', value.lower())
def digest(data): return hashlib.sha256(data).hexdigest()
def safe_file(root, name):
    root=Path(root).resolve(); p=root/name
    if not isinstance(name,str) or Path(name).is_absolute() or '..' in Path(name).parts: raise ValueError('Source must be a relative path inside the project folder')
    if p.is_symlink() or root not in p.resolve().parents or not p.is_file(): raise ValueError('Source is missing or outside the selected folder: '+str(name))
    return p

def role_for(path):
    # Nearest folder wins; filename prefixes are only a fallback.
    for part in reversed(path.parts[:-1]):
        for i, names in enumerate(ALIASES):
            if token(part) in names: return i
    stem=token(path.stem)
    matches=[i for i,names in enumerate(ALIASES) if any(stem.startswith(n) for n in names if len(n)>2)]
    return matches[0] if len(matches)==1 else None

def wave_info(data):
    with wave.open(io.BytesIO(data),'rb') as w:
        info={'channels':w.getnchannels(),'width':w.getsampwidth(),'rate':w.getframerate(),'frames':w.getnframes()}
        if w.getcomptype()!='NONE' or info['channels'] not in (1,2) or info['width'] not in (1,2,3,4): raise ValueError('Use mono/stereo PCM WAV (8/16/24/32-bit integer)')
        if not 8000<=info['rate']<=192000 or not 1<=info['frames']<=1920000: raise ValueError('WAV rate/frame count exceeds MPC limits')
        if len(w.readframes(info['frames'])) != info['frames']*info['width']*info['channels']: raise ValueError('Truncated WAV payload')
    return info

def midi_info(data):
    # Same finite PPQ/note limits as the runtime parser; retain original file bytes.
    if len(data)<14 or data[:4]!=b'MThd': raise ValueError('Invalid MIDI header')
    head,fmt,tracks,ppq=struct.unpack_from('>IHHH',data,4)
    if head<6 or fmt not in (0,1) or not 0<tracks<=256 or not 0<ppq<32768: raise ValueError('Use format 0/1 PPQ MIDI')
    p=8+head;notes=[];end_tick=0
    for _ in range(tracks):
        if p+8>len(data) or data[p:p+4]!=b'MTrk': raise ValueError('Missing MIDI track')
        size=struct.unpack_from('>I',data,p+4)[0];p+=8;end=p+size
        if end>len(data): raise ValueError('Truncated MIDI track')
        tick=0;running=0
        def vlq():
            nonlocal p
            value=0
            for _ in range(4):
                if p>=end: raise ValueError('Truncated MIDI event')
                x=data[p];p+=1;value=(value<<7)|(x&127)
                if x<128:return value
            raise ValueError('Invalid MIDI delta')
        while p<end:
            tick+=vlq()
            if p>=end:raise ValueError('Missing MIDI status')
            status=data[p]
            if status>=128:p+=1
            else:status=running
            if status==255:
                if p>=end:raise ValueError('Truncated MIDI meta event')
                p+=1;n=vlq();p+=n
            elif status in (240,247):n=vlq();p+=n;running=0
            else:
                if not 128<=status<240:raise ValueError('Invalid MIDI running status')
                running=status;n=1 if status&240 in (192,208) else 2
                if p+n>end or any(v>127 for v in data[p:p+n]):raise ValueError('Invalid MIDI event')
                if status&240==144 and data[p+1]:notes.append((round(tick*960/ppq),data[p],data[p+1]))
                p+=n
            if p>end:raise ValueError('Truncated MIDI event data')
            if len(notes)>65536:raise ValueError('MIDI note limit exceeded')
        end_tick=max(end_tick,tick)
    if p!=len(data) or not notes or math.ceil(end_tick/ppq/4)*3840>960*1024:raise ValueError('Empty, oversized or trailing-data MIDI')
    return {'notes':len(notes),'ppq':ppq,'note_values':sorted({n[1]for n in notes})}

def scan(root):
    root=Path(root).resolve()
    if not root.is_dir():raise ValueError('Choose a source folder')
    result={'root':str(root),'files':[],'issues':[]}
    for p in sorted(root.rglob('*')):
        if p.suffix.lower() not in ('.wav','.mid','.midi'):continue
        if len(result['files'])>=MAX_FILES:raise ValueError('At most 5000 source files per pack')
        rel=p.relative_to(root).as_posix()
        try:
            source=safe_file(root,rel)
            if source.stat().st_size>32*1024**2:raise ValueError('File exceeds 32 MB runtime limit')
            data=source.read_bytes()
            if p.suffix.lower()=='.wav':
                info=wave_info(data);role=role_for(Path(rel));kind=0
                if role is None:result['issues'].append(rel+': choose an instrument role')
            else:
                info=midi_info(data);role=0
                kind=4 if any(token(v) in ('groove','grooves','timing','templates') for v in p.parts[:-1]) else 3
            result['files'].append(dict(path=rel,sha=digest(data),role=role,kind=kind,**info))
        except (ValueError,OSError,wave.Error,EOFError,struct.error) as ex:result['issues'].append(rel+': '+str(ex))
    if not result['files']:raise ValueError('No supported WAV or MIDI files found')
    return result

def new_project(root):
    s=scan(root)
    return dict(schema=1,id='user-'+os.urandom(6).hex(),name=Path(root).name[:40] or 'My Pack',creator='kim',version='1.0.0',private=True,rights='Personal use only; no redistribution clearance declared.',allow_silence=False,midi_mode='auto',source_root=s['root'],files=s['files'],issues=s['issues'],kits=[])

def generate(project,count=8,seed=1):
    if not 1<=count<=512:raise ValueError('Choose 1–512 kits')
    pools=[[f['path'] for f in project['files'] if f['kind']==0 and f['role']==r] for r in range(8)]
    missing=[ROLES[r]for r in range(8)if not pools[r]]
    if missing and not project.get('allow_silence'):raise ValueError('Missing '+', '.join(missing)+'. Map files or explicitly allow silent missing roles.')
    rng=random.Random(seed);old=project.get('kits',[]);kits=[]
    # Named Kits/<name>/<role> folders are honored before filling from role pools.
    named={}
    for f in project['files']:
        parts=Path(f['path']).parts
        if f['kind']==0 and f['role'] is not None and len(parts)>3 and token(parts[0])=='kits':named.setdefault(parts[1],{}).setdefault(f['role'],[]).append(f['path'])
    names=list(named)
    for i in range(count):
        previous=old[i] if i<len(old) else None
        if previous and previous.get('locked'):kits.append(copy.deepcopy(previous));continue
        source=named[names[i]] if i<len(names) else {}
        name=names[i] if i<len(names) else ['Velvet Room','Warm Circuit','Wood & Wire','Pocket Lantern','Honey Dust','Soft Concrete','After Hours','Basement Gold'][i%8]+(f' {i//8+1}' if i>=8 else '')
        paths=[];locks=previous.get('locks',[False]*8) if previous else [False]*8
        for r in range(8):
            choices=source.get(r,pools[r]);v=previous['samples'][r] if previous and locks[r] else rng.choice(choices) if choices else None
            paths.append(v)
        kits.append({'name':previous['name'] if previous else name,'samples':paths,'locks':locks,'locked':False,'levels':previous.get('levels',[0]*8) if previous else [0]*8})
    project['kits']=kits;return kits

def save_project(project,path):
    p=Path(path);data=copy.deepcopy(project);data['source_root']=os.path.relpath(project['source_root'],p.resolve().parent)
    text=json.dumps(data,indent=2)+'\n';tmp=p.with_suffix('.partial');tmp.write_text(text,encoding='utf-8');tmp.replace(p)

def load_project(path):
    p=Path(path);d=json.loads(p.read_text(encoding='utf-8'))
    if d.get('schema')!=1:raise ValueError('Unsupported project format')
    d['source_root']=str((p.resolve().parent/d['source_root']).resolve())
    if not Path(d['source_root']).is_dir():raise ValueError('Source folder is missing; restore it beside the project')
    validate_project(d,verify_files=True,require_kits=False);return d

def validate_project(p,verify_files=False,require_kits=True):
    if not re.fullmatch('[a-z0-9][a-z0-9_-]{0,62}',p.get('id','')):raise ValueError('Pack ID: 1–63 lower-case letters, digits, - or _')
    if not isinstance(p.get('name'),str) or not 0<len(p['name'].encode())<=63 or any(c in p['name']for c in '\t\r\n/\\'):raise ValueError('Pack name: 1–63 UTF-8 bytes, no slashes or line breaks')
    if not re.fullmatch(r'\d+\.\d+\.\d+',p.get('version','')):raise ValueError('Use a version such as 1.0.0')
    if not p.get('rights','').strip():raise ValueError('Describe the rights for this pack')
    if not isinstance(p.get('private'),bool):raise ValueError('Choose Personal or Redistributable')
    if not (1 if require_kits else 0)<=len(p.get('kits',[]))<=512:raise ValueError('Generate at least one kit before exporting')
    files=p.get('files',[])
    if not 1<=len(files)<=MAX_FILES:raise ValueError('Invalid file count')
    paths={f['path']:f for f in files}
    if len(paths)!=len(files):raise ValueError('Duplicate source paths')
    for f in files:
        if f['kind']not in (0,3,4) or not isinstance(f['role'],int) or not 0<=f['role']<8:raise ValueError('Map every valid WAV to an instrument before exporting')
        source=safe_file(p['source_root'],f['path'])
        if verify_files:
            b=source.read_bytes()
            if digest(b)!=f['sha']:raise ValueError('Source changed; rescan and review kits: '+f['path'])
            if f['kind']==0:wave_info(b)
            else:midi_info(b);map_midi(b,p.get('midi_mode','auto'))
    seen=set()
    for k in p['kits']:
        name=k.get('name','')
        if not isinstance(name,str) or not 0<len(name.encode())<=63 or '\0'in name or any(c in name for c in '\t\r\n'):raise ValueError('Kit names must contain 1–63 UTF-8 bytes')
        if name in seen:raise ValueError('Kit names must be unique')
        seen.add(name)
        if len(k['samples'])!=8 or len(k.get('levels',[]))!=8:raise ValueError('A kit must have eight parts')
        for r,path in enumerate(k['samples']):
            if path is None:
                if not p.get('allow_silence'):raise ValueError('Missing kit part; enable silent missing roles explicitly')
            elif path not in paths or paths[path]['kind']!=0 or paths[path]['role']!=r:raise ValueError('Kit sample role no longer matches: '+str(path))
            if not -48<=float(k['levels'][r])<=6:raise ValueError('Kit level out of range')

def silence(role):
    b=io.BytesIO()
    with wave.open(b,'wb')as w:w.setparams((1,2,44100,0,'NONE',''));w.writeframes(b'\0\0'*(882+role*44))
    return b.getvalue()
def fnv(b):
    n=2166136261
    for v in b:n=((n^v)*16777619)&0xffffffff
    return n

def write_bank(project,path):
    validate_project(project,True);out=Path(path);root=Path(project['source_root']);records=[];lookup={};silent={}
    for f in project['files']:
        source=safe_file(root,f['path']);e=dict(f);e['source']=source
        # A fixed-width runtime name never contains a partial UTF-8 code point.
        name='User Samples/Library/'+f['path']
        if len(name.encode())>250:name='User Samples/Library/'+Path(name).stem[:40]+'-'+f['sha'][:12]+Path(name).suffix
        if not name.startswith('User Samples/Library/'):name='User Samples/Library/'+name
        e['name']=name;lookup[f['path']]=e;records.append(e)
    for role in range(8):
        if not any(f['kind']==0 and f['role']==role for f in records) or any(k['samples'][role]is None for k in project['kits']):
            data=silence(role);e=dict(data=data,sha=digest(data),name=f'User Samples/Library/Generated Silence/{ROLES[role]}.wav',kind=0,role=role,**wave_info(data));records.append(e);silent[role]=e
    if len(records)>50000:raise ValueError('Runtime directory limit exceeded')
    def entry(role,key):return lookup[key] if key else silent[role]
    def settings(r,k=None):return [float(k['levels'][r])if k else 0.,0.,[650.,400.,180.,1100.,350.,650.,1800.,500.][r],0.,1. if r in (2,3) else 0.]
    # Small compatible prefix: silence only. Actual kit sounds are loaded from the indexed library.
    prefix=[];payload=bytearray()
    for r in range(8):
        data=silence(r);info=wave_info(data)
        for _ in range(8):prefix.append(struct.pack('<64s8I',digest(data).encode(),r,44100,1,info['frames'],len(payload),len(data),fnv(data),0));payload+=data
    out.parent.mkdir(parents=True,exist_ok=True)
    with out.open('wb')as f:
        f.write(struct.pack('<8s6I',b'GDBANK01',1,64,32,96,6176,len(payload)));f.write(b''.join(prefix));f.write(payload)
        start=f.tell();f.write(struct.pack('<8s6I',b'GDLIB002',2,len(records),384,0,0,0));f.write(b'\0'*(384*len(records)));directory=[]
        for e in records:
            data=e['data']if 'data'in e else e['source'].read_bytes()
            if e['kind']>=3:data=map_midi(data,project.get('midi_mode','auto'));e['sha']=digest(data)
            offset=f.tell();f.write(data)
            directory.append(struct.pack('<64s256sQ10I16x',e['sha'].encode(),e['name'].encode(),offset,len(data),zlib.crc32(data),e['kind'],e['role'],e.get('rate',0),e.get('channels',0),e.get('frames',0),0,0,0))
        pos=f.tell();f.seek(start+32);f.write(b''.join(directory));f.seek(pos)
        choices=[]
        for group,role in enumerate((0,1,2,7)):
            pool=[e for e in records if e['kind']==0 and e['role']==role];other=[e for e in records if e['kind']==0 and e['role']==3]
            for n,e in enumerate(pool):
                second=other[n%len(other)] if group==2 else None
                record=struct.pack('<I',group)+e['sha'].encode()+(second['sha'].encode()if second else b'\0'*64)+struct.pack('<10f',*(settings(role)+settings(3 if second else role)))+b'\0'*24
                assert len(record)==196;choices.append(record+struct.pack('<I',zlib.crc32(record)))
        if len(choices)>5000:raise ValueError('Too many role choices; maximum 5000')
        poolstart=f.tell();f.write(struct.pack('<8s2I',b'GDSND004',len(choices),200));f.write(b''.join(choices));f.write(struct.pack('<8sQ',b'GDSEND04',poolstart))
        kitstart=f.tell();f.write(struct.pack('<8s2I',b'GDKITS03',len(project['kits']),800))
        for k in project['kits']:
            ids=[entry(r,key)['sha'] for r,key in enumerate(k['samples'])]
            record=struct.pack('<64s',k['name'].encode())+b''.join(v.encode()for v in ids)+b''.join(struct.pack('<5f',*settings(r,k))for r in range(8))+struct.pack('<8f',0,0,0,0,0,.25,0,.7)+b'\0'*28
            assert len(record)==796;f.write(record+struct.pack('<I',zlib.crc32(record)))
        f.write(struct.pack('<8sQ',b'GDKEND03',kitstart));f.write(struct.pack('<8sQ',b'GDLEND02',start))
        if f.tell()>4*1024**3:raise ValueError('Bank exceeds 4 GB')
    return {'wav':sum(e['kind']==0 for e in records),'midi':sum(e['kind']>=3 for e in records),'kits':len(project['kits']),'bytes':out.stat().st_size}

def export(project,out):
    import expansions
    out=Path(out)
    if out.exists():raise ValueError('Output already exists; choose a new filename')
    out.parent.mkdir(parents=True,exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='gd-author-')as temp:
        bank=Path(temp)/'bank.gdbank';summary=write_bank(project,bank)
        expansions.pack(bank,out,project['id'],project['name'],project['private'],version=project['version'],rights=project['rights'],creator=project['creator'])
    return summary

def decode(data,max_seconds=4):
    info=wave_info(data)
    with wave.open(io.BytesIO(data),'rb')as w:b=w.readframes(min(info['frames'],int(max_seconds*info['rate'])))
    width=info['width'];ch=info['channels'];vals=[]
    for i in range(0,len(b),width*ch):
        total=0.
        for c in range(ch):
            v=int.from_bytes(b[i+c*width:i+(c+1)*width],'little',signed=width!=1)
            total+=(v-128)/128 if width==1 else v/float(1<<(8*width-1))
        vals.append(total/ch)
    return vals,info['rate']

def render_preview(project,kit_index=0,bpm=88,swing=57):
    bpm=max(40,min(180,bpm));swing=max(50,min(75,swing));rate=44100;beat=60/bpm;length=beat*8;mix=[0.]*int((length+2)*rate);kit=project['kits'][kit_index]
    for r,path in enumerate(kit['samples']):
        if path is None:continue
        data,sr=decode(safe_file(project['source_root'],path).read_bytes());gain=10**(kit['levels'][r]/20)*.6
        times=([0,1.5,2.5,4,5.5,6.75]if r==0 else [1,3,5,7]if r==1 else [k*.5+(swing-50)/100 if k%2 else k*.5 for k in range(16)]if r==2 else [3.5,7.5]if r==3 else [6.5]if r>=4 else [])
        for t in times:
            start=round(t*beat*rate);end=min(len(mix)-start,int(len(data)*rate/sr))
            for n in range(end):
                ix=n*sr/rate;lo=int(ix);a=data[min(lo,len(data)-1)];b=data[min(lo+1,len(data)-1)];mix[start+n]+=(a+(b-a)*(ix-lo))*gain
    peak=max(max(map(abs,mix)),1.);raw=array.array('h',(round(max(-1,min(1,v/peak*.85))*32767)for v in mix))
    if os.sys.byteorder!='little':raw.byteswap()
    out=io.BytesIO()
    with wave.open(out,'wb')as w:w.setparams((1,2,rate,0,'NONE',''));w.writeframes(raw.tobytes())
    return out.getvalue()


GM_MAP={35:0,36:0,38:1,40:1,42:2,44:2,46:3,39:4,41:5,43:5,45:5,47:5,48:5,50:5,49:6,51:6,52:6,53:6,55:6,57:6,59:6}
GM_MAP.update({n:7 for n in range(35,82)if n not in GM_MAP})
def map_midi(data,mode='auto'):
    info=midi_info(data);notes=info['note_values']
    if mode not in ('auto','native','gm'):raise ValueError('Unknown MIDI note mapping')
    if mode=='native' or mode=='auto' and all(60<=n<=67 for n in notes):
        if not all(60<=n<=67 for n in notes):raise ValueError('Native Groove Dice MIDI uses notes 60–67')
        return data
    if not all(n in GM_MAP for n in notes):raise ValueError('MIDI notes outside GM drum range 35–81; remap in a MIDI editor before import')
    result=bytearray(data);p=8+struct.unpack_from('>I',data,4)[0]
    while p<len(data):
        end=p+8+struct.unpack_from('>I',data,p+4)[0];p+=8;running=0
        def vlq():
            nonlocal p
            value=0
            while True:
                v=data[p];p+=1;value=(value<<7)|(v&127)
                if v<128:return value
        while p<end:
            vlq();status=data[p]
            if status>=128:p+=1
            else:status=running
            if status==255:p+=1;n=vlq();p+=n
            elif status in (240,247):n=vlq();p+=n;running=0
            else:
                running=status;n=1 if status&240 in (192,208)else 2
                if status&240 in (128,144,160):result[p]=60+GM_MAP.get(data[p],7)
                p+=n
    return bytes(result)
