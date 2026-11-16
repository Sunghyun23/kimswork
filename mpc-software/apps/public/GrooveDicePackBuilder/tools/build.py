#!/usr/bin/env python3
"""Build a native standalone app; invoke independently on macOS and Windows."""
from pathlib import Path
import hashlib,json,os,platform,shutil,subprocess,sys,zipfile
P=Path(__file__).resolve().parents[1];M=P.parents[2];SHARED=M/'shared/groove-dice-expansion';NAME='Groove Dice Pack Builder';VERSION='1.0.0'
def main():
 if sys.platform not in ('darwin','win32'):raise SystemExit('Build on macOS or Windows')
 build=P/'build';dist=P/'dist';build.mkdir(exist_ok=True);dist.mkdir(exist_ok=True)
 command=[sys.executable,'-m','PyInstaller','--noconfirm','--clean','--windowed','--onedir','--name',NAME,'--distpath',str(dist),'--workpath',str(build/'pyinstaller'),'--specpath',str(build),'--paths',str(SHARED),'--add-data',str(SHARED)+os.pathsep+'shared/groove-dice-expansion','--add-data',str(P/'design/main-concept-v1.png')+os.pathsep+'design']
 if sys.platform=='darwin':command+=['--osx-bundle-identifier','app.kimswork.groovedicepackbuilder']
 command+=[str(P/'src/main.py')];subprocess.run(command,check=True)
 runtime=dist/(NAME+'.app') if sys.platform=='darwin'else dist/NAME
 executable=runtime/'Contents/MacOS'/NAME if sys.platform=='darwin'else runtime/(NAME+'.exe')
 # Include dynamically linked third-party license notices in the bundle.
 notices=runtime/'Contents/Resources/THIRD_PARTY' if sys.platform=='darwin'else runtime/'THIRD_PARTY';notices.mkdir(parents=True,exist_ok=True)
 import PySide6
 site=Path(PySide6.__file__).parent.parent
 for package in ['PySide6','PySide6_Essentials','PySide6_Addons','shiboken6']:
  for info in [d for d in site.glob('*.dist-info') if d.name.lower().startswith(package.lower()+'-')]:
   for f in info.rglob('*'):
    if f.is_file()and any(k in f.name.lower()for k in ['license','copying','notice']):dest=notices/package/f.relative_to(info);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(f,dest)
 shutil.copytree(P/'licenses',notices/'licenses',dirs_exist_ok=True)
 shutil.copy2(P/'THIRD_PARTY.md',notices/'README.md')
 if len(list(notices.rglob('*LICENSE*')))+len(list(notices.rglob('*license*')))==0:raise RuntimeError('Third-party license texts missing from bundle')
 # Library replacements modify macOS signatures. Ad-hoc sign after all bundle changes.
 if sys.platform=='darwin':subprocess.run(['codesign','--force','--deep','--sign','-',str(runtime)],check=True)
 env=dict(os.environ,QT_QPA_PLATFORM='offscreen');subprocess.run([str(executable),'--smoke-test'],env=env,timeout=60,check=True)
 target=dist/f'groove-dice-pack-builder-{VERSION}-{platform.system().lower()}-{platform.machine().lower()}.zip'
 if sys.platform=='darwin':subprocess.run(['ditto','-c','-k','--sequesterRsrc','--keepParent',str(runtime),str(target)],check=True)
 else:
  with zipfile.ZipFile(target,'w',zipfile.ZIP_DEFLATED,allowZip64=True)as z:
   for f in sorted(runtime.rglob('*')):
    if f.is_file():z.write(f,f.relative_to(runtime.parent))
 h=hashlib.sha256(target.read_bytes()).hexdigest();target.with_suffix('.zip.sha256').write_text(h+'  '+target.name+'\n')
 (dist/'build-report.json').write_text(json.dumps(dict(version=VERSION,platform=platform.platform(),machine=platform.machine(),python=platform.python_version(),bundle=str(runtime),asset=target.name,sha256=h,smoke='PASS',signed='ad-hoc only; not notarized'if sys.platform=='darwin'else'not code-signed'),indent=2)+'\n');print(target)
if __name__=='__main__':main()
