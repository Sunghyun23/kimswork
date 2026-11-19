#!/usr/bin/env python3
"""Build one monorepo plugin; never publishes or installs to hardware."""
import argparse, re, subprocess, sys
from pathlib import Path
from bootstrap_sdk import main as ensure_sdk
ROOT=Path(__file__).resolve().parents[1]
ap=argparse.ArgumentParser();ap.add_argument('plugin',help='public/Name or private/Name; bare Name resolves to public');ap.add_argument('--target',choices=['native','all','desktop'],default='native');a=ap.parse_args()
plugin_path=Path(a.plugin) if '/' in a.plugin else Path('public')/a.plugin
if len(plugin_path.parts)!=2 or plugin_path.parts[0] not in ('public','private') or plugin_path.parts[1] in ('.','..'):
 ap.error('Expected public/Name or private/Name')
project=ROOT/'plugins'/plugin_path
if not (project/'plugin.json').is_file(): ap.error(f'Plugin not found: {plugin_path}')
ensure_sdk();sdk=ROOT/'third_party/mpc-plugin-builder-sdk'
if (project/'tools/generate_presets.py').is_file():
 subprocess.run([sys.executable,str(project/'tools/generate_presets.py')],check=True)
if (project/'tools/generate_catalog.py').is_file():
 subprocess.run([sys.executable,str(project/'tools/generate_catalog.py')],check=True)
subprocess.run([sys.executable,str(sdk/'tools/build.py'),str(project),'--target','native','--toolchain','gcc'],check=True)
if (project/'tools/build_skin.py').is_file():
 subprocess.run([sys.executable,str(project/'tools/build_skin.py')],check=True)
if a.target!='native':
 subprocess.run([sys.executable,str(sdk/'tools/build.py'),str(project),'--target',a.target],check=True)
# The SDK copies every design source from assets/. Ship only images referenced by
# the generated skin; large rejected concepts must not enter the runtime bundle.
if project.name=='DonutTheory' and a.target=='all':
 skin=project/'build'/'kim - VST - Donut Theory'/'Plugin Skins'
 references=set(re.findall(r'"([^"\n]+\.(?:png|jpg|jpeg))"',(skin/'TUI.json').read_text()))
 unused=[f for f in skin.rglob('*') if f.is_file() and f.suffix.lower() in ('.png','.jpg','.jpeg') and f.relative_to(skin).as_posix() not in references]
 removed_bytes=sum(f.stat().st_size for f in unused)
 for f in unused:f.unlink()
 print(f'Runtime skin: omitted {len(unused)} unreferenced images ({removed_bytes/1024/1024:.1f} MiB); source assets retained.')
 subprocess.run([sys.executable,str(sdk/'tools/validate.py'),str(project)],check=True)
if (project/'tools/export_ui_values.cpp').is_file():
 preview_export=project/'build/export_ui_values'
 subprocess.run(['clang++','-std=c++17','-O2','-I'+str(sdk/'framework'),str(project/'tools/export_ui_values.cpp'),'-o',str(preview_export)],check=True)
 with (project/'build/preview-values.tsv').open('w') as values:subprocess.run([str(preview_export)],stdout=values,check=True)
if (project/'tools/render_skin_preview.py').is_file():
 subprocess.run([sys.executable,str(project/'tools/render_skin_preview.py')],check=True)
