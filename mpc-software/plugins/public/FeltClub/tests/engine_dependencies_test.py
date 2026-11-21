"""Compile each backend alone and reject dependencies on another backend."""
from pathlib import Path
import subprocess,tempfile,re
p=Path(__file__).resolve().parents[1];sdk=p.parents[2]/'third_party/mpc-plugin-builder-sdk'
names=['Tine','Wurli','Upright','Felt','Clav','Muted','Thumb']
for name in names:
 header=p/'src/engines'/name.lower()/(name+'Voice.h')
 with tempfile.TemporaryDirectory(prefix='felt-independent-') as td:
  unit=Path(td)/'independent.cpp';unit.write_text(f'#include "{header}"\nint main(){{felt::{name}Voice v;v.prepare(48000);}}\n')
  common=['clang++','-std=c++17','-I'+str(sdk/'framework'),str(unit)]
  subprocess.run(common+['-fsyntax-only','-Werror'],check=True)
  result=subprocess.run(common+['-MM'],check=True,capture_output=True,text=True).stdout
  referenced=set(re.findall(r'/src/engines/([a-z]+)/',result))
  assert referenced=={name.lower()},(name,referenced)
print('PASS7standalone backend compiles; no cross-engine header dependencies')
