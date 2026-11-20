#!/usr/bin/env python3
from pathlib import Path
import re
P=Path(__file__).resolve().parents[1];skin=P/'build/kim - VST - Groove Dice/Plugin Skins'
refs=set(re.findall(r'"([^"\n]+\.(?:png|jpg|jpeg))"',(skin/'TUI.json').read_text()))
removed=[]
for f in skin.rglob('*'):
 if f.is_file()and f.relative_to(skin).as_posix()not in refs|{'TUI.json','Q-Links.json'}:removed.append((str(f.relative_to(skin)),f.stat().st_size));f.unlink()
print('Removed',len(removed),'unreferenced runtime files;',sum(x[1]for x in removed),'bytes. Source assets retained.')

# A public build starts with an empty optional-pack registry. Owner installations
# use a separate verified staging folder and retain their existing registry.
registry=P/"build/kim - VST - Groove Dice/Content/Expansions/registry.tsv"
registry.parent.mkdir(parents=True,exist_ok=True);registry.write_text("")
