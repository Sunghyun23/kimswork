#!/usr/bin/env python3
"""Groove Dice .gdexp packages. Separate payloads; explicit private/public assembly.
No device SSH or firmware operations. Install/remove target a local Content directory.
"""
from pathlib import Path
import argparse,hashlib,json,os,re,shutil,struct,tempfile,zipfile
ID=re.compile(r'^[a-z0-9][a-z0-9_-]{0,62}$')
def sha(path):
 h=hashlib.sha256()
 with Path(path).open('rb')as f:
  for b in iter(lambda:f.read(8*1024*1024),b''):h.update(b)
 return h.hexdigest()
def atomic_json(path,data):
 path=Path(path);tmp=path.with_suffix('.partial');tmp.write_text(json.dumps(data,indent=2)+'\n');tmp.replace(path)
def validate(m):
 if m.get('format')!='GDEXP001'or not ID.fullmatch(m.get('id','')):raise ValueError('Unsupported expansion manifest/id')
 if not isinstance(m.get('name'),str)or not 0<len(m['name'].encode('utf-8'))<=63 or any(c in m['name']for c in '\t\r\n/\\'):raise ValueError('Invalid expansion name')
 if m.get('target','groove-dice')!='groove-dice' or m.get('plugin_uid','KpGy')!='KpGy':raise ValueError('Expansion targets another product')
 if not re.fullmatch(r'\d+\.\d+\.\d+',m.get('version','1.0.0')):raise ValueError('Invalid pack version')
 if not isinstance(m.get('redistributable'),bool):raise ValueError('Explicit redistribution classification required')
 if not re.fullmatch('[0-9a-f]{64}',m.get('sha256','')):raise ValueError('Missing payload hash')
 if not 32<=m.get('bytes',0)<=4*1024**3:raise ValueError('Payload size exceeds limit')
 return m
def checkbank(path):
 with Path(path).open('rb')as f:
  if f.read(8)!=b'GDBANK01':raise ValueError('Invalid bank prefix')
  f.seek(-16,2)
  if f.read(8)!=b'GDLEND02':raise ValueError('Missing indexed library')
def pack(bank,out,id,name,private,*,version="1.0.0",rights=None,creator="kim"):
 if not ID.fullmatch(id):raise ValueError('Invalid id')
 checkbank(bank);m=validate(dict(format='GDEXP001',id=id,name=name,redistributable=not private,bytes=bank.stat().st_size,sha256=sha(bank),version=version,target='groove-dice',plugin_uid='KpGy',creator=creator,rights=rights or ('Restricted original vendor content. Personal use package only; no redistribution clearance.'if private else'Original procedural synthesis and independently authored MIDI for kim. No external audio or MIDI inputs.')))
 out.parent.mkdir(parents=True,exist_ok=True);tmp=out.with_suffix('.partial')
 with zipfile.ZipFile(tmp,'w',zipfile.ZIP_DEFLATED,compresslevel=1,allowZip64=True)as z:
  z.writestr('manifest.json',json.dumps(m,indent=2)+'\n');z.write(bank,'bank.gdbank');z.writestr('RIGHTS.txt',m['rights']+'\n')
 tmp.replace(out);print(f'Packed {name}: {out} ({"PRIVATE"if private else"original/redistributable"})')
def registry(content):
 root=content/'Expansions';root.mkdir(parents=True,exist_ok=True);rows=[];core=(content/'Banks/core.id').read_text().strip()if(content/'Banks/core.id').exists()else'legacy-core';ids={core}
 for d in sorted(root.iterdir()):
  if not d.is_dir()or d.name.startswith('.'):continue
  m=validate(json.loads((d/'manifest.json').read_text()))
  if m['id']!=d.name or m['id']in ids:raise ValueError('Duplicate/mismatched expansion identity')
  ids.add(m['id']);rows.append(f'{m["id"]}\t{m["name"]}\t'+('public'if m['redistributable']else'private'))
 tmp=root/'registry.partial';tmp.write_text('\n'.join(rows)+('\n'if rows else''));tmp.replace(root/'registry.tsv')
def install(archive,content,allow_private):
 content.mkdir(parents=True,exist_ok=True);root=content/'Expansions';root.mkdir(exist_ok=True)
 with zipfile.ZipFile(archive)as z:
  if set(z.namelist())!={'manifest.json','bank.gdbank','RIGHTS.txt'}or len(z.infolist())!=3:raise ValueError('Unexpected/duplicate archive members')
  if z.getinfo('manifest.json').file_size>16384 or z.getinfo('RIGHTS.txt').file_size>65536:raise ValueError('Oversized metadata')
  m=validate(json.loads(z.read('manifest.json')))
  if not m['redistributable']and not allow_private:raise ValueError('Private expansion requires --allow-private')
  core=(content/'Banks/core.id').read_text().strip()if(content/'Banks/core.id').exists()else'legacy-core'
  if m['id']==core:raise ValueError('This expansion is already bundled as core')
  if z.getinfo('bank.gdbank').file_size!=m['bytes']:raise ValueError('Payload size mismatch')
  dest=root/m['id']
  if dest.exists():raise ValueError('Expansion already installed; remove it first')
  with tempfile.TemporaryDirectory(prefix='.gdexp-',dir=root)as staging:
   stage=Path(staging)
   with z.open('bank.gdbank')as src,(stage/'bank.gdbank').open('wb')as dst:shutil.copyfileobj(src,dst,8*1024*1024)
   if sha(stage/'bank.gdbank')!=m['sha256']:raise ValueError('Payload hash mismatch')
   checkbank(stage/'bank.gdbank');atomic_json(stage/'manifest.json',m);(stage/'RIGHTS.txt').write_bytes(z.read('RIGHTS.txt'));stage.rename(dest)
 registry(content);print('Installed '+m['name'])
def remove(id,content):
 if not ID.fullmatch(id):raise ValueError('Invalid id')
 d=content/'Expansions'/id
 if not d.is_dir():raise ValueError('Expansion not installed')
 m=validate(json.loads((d/'manifest.json').read_text()))
 if m['id']!=id:raise ValueError('Manifest mismatch')
 shutil.rmtree(d);registry(content);print('Removed '+m['name']+'; saved projects referencing it will report missing samples')
def assemble(plugin,out,extras,personal):
 if extras is not None:
  if (extras/'Content/Expansions').is_dir():extras=extras/'Content'
  if not (extras/'Expansions').is_dir():raise ValueError('Expected an expansion Content directory or plugin root containing Content/Expansions')
 if not(plugin/'plugin.json').exists():raise ValueError('Expected a built plugin folder')
 if out.exists():raise ValueError('Output already exists; choose a new output folder')
 core=plugin/'Content/Banks/core.manifest.json'
 if not core.exists():raise ValueError('Core manifest missing')
 m=validate(json.loads(core.read_text()))
 if not m['redistributable']:raise ValueError('Core is not marked original/redistributable')
 if sha(plugin/'Content/Banks/default.gdbank')!=m['sha256']:raise ValueError('Core payload/manifest hash mismatch')
 for f in (plugin/'Content').rglob('*'):
  if f.is_symlink():raise ValueError('Content symlinks are not allowed in distribution assembly')
 with tempfile.TemporaryDirectory(prefix='gd-build-',dir=out.parent)as staging:
  stage=Path(staging)/plugin.name;shutil.copytree(plugin,stage,ignore=lambda path,names:['Expansions']if Path(path).name=='Content'else[])
  sources=[plugin/'Content/Expansions']+([extras/'Expansions']if extras else[]);seen=set()
  for root in sources:
   if not root.exists():continue
   for d in sorted(root.iterdir()):
    if not d.is_dir()or d.name.startswith('.'):continue
    m=validate(json.loads((d/'manifest.json').read_text()))
    if d.name!=m['id']or sha(d/'bank.gdbank')!=m['sha256']:raise ValueError('Expansion identity/payload hash mismatch')
    if not personal and not m['redistributable']:continue
    if m['id']in seen:raise ValueError('Duplicate expansion source')
    seen.add(m['id']);dest=stage/'Content/Expansions'/m['id'];dest.parent.mkdir(exist_ok=True);shutil.copytree(d,dest)
  registry(stage/'Content')
  if not personal:
   for f in (stage/'Content/Expansions').glob('*/manifest.json'):
    if not validate(json.loads(f.read_text()))['redistributable']:raise ValueError('Private payload in public profile')
  stage.rename(out)
 print(f'Assembled {"PERSONAL"if personal else"PUBLIC"} profile: {out}')
def main():
 ap=argparse.ArgumentParser();sub=ap.add_subparsers(dest='cmd',required=True)
 a=sub.add_parser('pack');a.add_argument('--bank',type=Path,required=True);a.add_argument('--out',type=Path,required=True);a.add_argument('--id',required=True);a.add_argument('--name',required=True);a.add_argument('--private',action='store_true')
 a=sub.add_parser('install');a.add_argument('archive',type=Path);a.add_argument('--content',type=Path,required=True);a.add_argument('--allow-private',action='store_true')
 a=sub.add_parser('remove');a.add_argument('id');a.add_argument('--content',type=Path,required=True)
 a=sub.add_parser('list');a.add_argument('--content',type=Path,required=True)
 a=sub.add_parser('assemble');a.add_argument('--plugin',type=Path,required=True);a.add_argument('--out',type=Path,required=True);a.add_argument('--expansions',type=Path);a.add_argument('--personal',action='store_true')
 args=ap.parse_args()
 try:
  if args.cmd=='pack':pack(args.bank,args.out,args.id,args.name,args.private)
  elif args.cmd=='install':install(args.archive,args.content,args.allow_private)
  elif args.cmd=='remove':remove(args.id,args.content)
  elif args.cmd=='list':registry(args.content);print((args.content/'Expansions/registry.tsv').read_text()or'No expansions installed')
  else:args.out.parent.mkdir(parents=True,exist_ok=True);assemble(args.plugin,args.out,args.expansions,args.personal)
 except (ValueError,KeyError,OSError,zipfile.BadZipFile)as e:ap.exit(1,str(e)+'\n')
if __name__=='__main__':main()
