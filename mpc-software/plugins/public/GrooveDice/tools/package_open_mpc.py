#!/usr/bin/env python3
"""Package the public SDK ZIP with explicit persistent expansion ownership.
No device access. Private/installed runtime folders are never accepted as inputs.
"""
import argparse,hashlib,json,subprocess,sys,zipfile
from pathlib import Path
P=Path(__file__).resolve().parents[1]
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--tools',required=True,type=Path,help='External open-mpc/mpc-vst-plugins checkout');a=ap.parse_args()
 version=json.loads((P/'plugin.json').read_text())['version'];src=P/f'build/dist/groove-dice-{version}-mpc.zip';out=P/'build/open-mpc'/version;out.mkdir(parents=True,exist_ok=True)
 with zipfile.ZipFile(src)as z:
  base='kim - VST - Groove Dice/';banks=[n for n in z.namelist()if n.endswith('.gdbank')];assert banks==[base+'Content/Banks/default.gdbank'],'public core only'
  assert not any(n.endswith('.gdexp')or '/Content/Captures/'in n for n in z.namelist()),'personal captures/expansions excluded'
  assert z.read(base+'Content/Expansions/registry.tsv').strip()==b'','public registry must be empty'
  assert not any('/Content/QA/'in n for n in z.namelist()),'QA isolation marker must not ship'
  core=z.read(base+'Content/Banks/default.gdbank');assert hashlib.sha256(core).hexdigest()==json.loads(z.read(base+'Content/Banks/core.manifest.json'))['sha256']
  assert core==(P/'Content/Banks/default.gdbank').read_bytes()
 tool=a.tools/'tools'
 subprocess.run([sys.executable,str(tool/'sdk_release.py'),str(src),'--version',version,'--id','groove-dice','--repo','Sunghyun23/kimswork','--license','MIT','--asset-name','groove-dice','--user-data','Content/Expansions','--user-data','Content/Captures','--out',str(out)],check=True)
 report={'version':version,'sourceZip':src.name,'sourceSha256':hashlib.sha256(src.read_bytes()).hexdigest(),'user_data':['Content/Expansions','Content/Captures'],'packages':[]}
 for cpu in ('armv7','aarch64'):
  f=out/f'groove-dice-{version}-mpc-{cpu}.zip'
  with zipfile.ZipFile(f)as z:
   manifests=[n for n in z.namelist()if n.endswith('/mpc-plugin.json')];assert manifests
   for name in manifests:assert json.loads(z.read(name))['user_data']==['Content/Expansions','Content/Captures'],('persistent expansion declaration missing',name)
   install=z.read(next(n for n in z.namelist()if n.endswith('/install.sh'))).decode();assert 'USER_DATA="Content/Expansions Content/Captures"'in install or "USER_DATA='Content/Expansions Content/Captures'"in install
  subprocess.run([sys.executable,str(tool/'catalog_check.py'),str(f),'--catalog','--require-portable','--expect-id','groove-dice','--expect-repo','Sunghyun23/kimswork'],check=True)
  sha=hashlib.sha256(f.read_bytes()).hexdigest();f.with_suffix('.zip.sha256').write_text(sha+'  '+f.name+'\n');report['packages'].append({'file':f.name,'sha256':sha,'bytes':f.stat().st_size})
 (out/'verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
if __name__=='__main__':main()
