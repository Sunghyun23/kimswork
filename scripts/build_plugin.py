#!/usr/bin/env python3
"""Build one monorepo plugin; never publishes or installs to hardware."""
import argparse, re, subprocess, sys
from pathlib import Path
from bootstrap_sdk import main as ensure_sdk
ROOT=Path(__file__).resolve().parents[1]
ap=argparse.ArgumentParser();ap.add_argument('plugin',choices=['DonutTheory']);ap.add_argument('--target',choices=['native','all','desktop'],default='native');a=ap.parse_args()
ensure_sdk();sdk=ROOT/'third_party/mpc-plugin-builder-sdk';project=ROOT/'plugins'/a.plugin
subprocess.run([sys.executable,str(project/'tools/generate_catalog.py')],check=True)
subprocess.run([sys.executable,str(sdk/'tools/build.py'),str(project),'--target','native','--toolchain','gcc'],check=True)
subprocess.run([sys.executable,str(project/'tools/build_skin.py')],check=True)
if a.target!='native':
 subprocess.run([sys.executable,str(sdk/'tools/build.py'),str(project),'--target',a.target],check=True)
# The SDK copies every design source from assets/. Ship only images referenced by
# the generated skin; large rejected concepts must not enter the runtime bundle.
if a.plugin=='DonutTheory' and a.target=='all':
 skin=project/'build'/'kim - VST - Donut Theory'/'Plugin Skins'
 references=set(re.findall(r'"([^"\n]+\.(?:png|jpg|jpeg))"',(skin/'TUI.json').read_text()))
 unused=[f for f in skin.rglob('*') if f.is_file() and f.suffix.lower() in ('.png','.jpg','.jpeg') and f.relative_to(skin).as_posix() not in references]
 removed_bytes=sum(f.stat().st_size for f in unused)
 for f in unused:f.unlink()
 print(f'Runtime skin: omitted {len(unused)} unreferenced images ({removed_bytes/1024/1024:.1f} MiB); source assets retained.')
 subprocess.run([sys.executable,str(sdk/'tools/validate.py'),str(project)],check=True)
subprocess.run([sys.executable,str(project/'tools/render_skin_preview.py')],check=True)
