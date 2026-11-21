#!/usr/bin/env python3
from pathlib import Path
import subprocess,sys,os,shutil,re
P=Path(__file__).resolve().parents[1];SDK=P.parents[2]/'third_party/mpc-plugin-builder-sdk';target=sys.argv[1]if len(sys.argv)>1 else'all';assert target in ('native','all')
subprocess.run([sys.executable,str(P/'tools/generate_parameters.py')],check=True)
env=dict(os.environ,ZIG_GLOBAL_CACHE_DIR=str(P/'build/zig-global-cache'),ZIG_LOCAL_CACHE_DIR=str(P/'build/zig-local-cache'))
subprocess.run([sys.executable,str(SDK/'tools/build.py'),str(P),'--target','native','--toolchain','gcc'],env=env,check=True)
subprocess.run([sys.executable,str(P/'tools/build_skin.py')],check=True)
if target=='all':subprocess.run([sys.executable,str(SDK/'tools/build.py'),str(P),'--target','all'],env=env,check=True)
else:shutil.copytree(P/'skins',P/'build/kim - VST - Felt Club/Plugin Skins',dirs_exist_ok=True)
skin=P/'build/kim - VST - Felt Club/Plugin Skins';refs=set(re.findall(r'"([^"\n]+\.(?:png|jpg))"',(skin/'TUI.json').read_text()))
for f in skin.rglob('*'):
 if f.is_file()and f.suffix in ('.png','.jpg')and f.relative_to(skin).as_posix()not in refs:f.unlink()
if target=='all':subprocess.run([sys.executable,str(SDK/'tools/validate.py'),str(P)],check=True)
