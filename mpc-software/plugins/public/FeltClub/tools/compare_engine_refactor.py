#!/usr/bin/env python3
"""Compare native pre/post-refactor float output, without quantisation or FX."""
from pathlib import Path
import argparse,tempfile,subprocess,json,hashlib
import numpy as np
p=Path(__file__).resolve().parents[1];sdk=p.parents[2]/'third_party/mpc-plugin-builder-sdk'
a=argparse.ArgumentParser();a.add_argument('--before',type=Path,required=True);a.add_argument('--after',type=Path,required=True);a.add_argument('--out',type=Path,required=True);a.add_argument('--changed-model',action='append',default=[]);args=a.parse_args()
s=(p/'tools/render_demo.cpp').read_text()
start=s.index('static void wav(');end=s.index('\nint main(',start)
s=s[:start]+'''static void wav(const std::string&file,const std::vector<float>&x){std::ofstream f(file,std::ios::binary);f.write(reinterpret_cast<const char*>(x.data()),x.size()*sizeof(float));}'''+s[end:]
s=s.replace('assert(argc==4);','assert(argc==5);const int sampleRate=std::stoi(argv[4]);').replace('48000','sampleRate').replace('-dry.wav','-dry.f32')
s=s.replace('#include "../src/Parameters.h"',f'#include "{p}/src/Parameters.h"')
report={'scope':'same dry native VST notes, three velocities/registers, chords; float output, not listening certification','before_sha256':hashlib.sha256(args.before.read_bytes()).hexdigest(),'after_sha256':hashlib.sha256(args.after.read_bytes()).hexdigest(),'cases':[]}
with tempfile.TemporaryDirectory(prefix='felt-refactor-audio-') as td:
 td=Path(td);unit=td/'snapshot.cpp';unit.write_text(s);exe=td/'snapshot'
 subprocess.run(['clang++','-std=c++17','-O2','-I'+str(sdk/'framework'),str(unit),'-o',str(exe)],check=True)
 for rate in [44100,48000,96000]:
  before=td/'before';after=td/'after'
  for lib,folder,tag in [(args.before,before,'before'),(args.after,after,'after')]:
   subprocess.run([str(exe),str(lib.resolve()),str(folder),tag,str(rate)],check=True,stdout=subprocess.DEVNULL)
  for model in ['tine','wurli','upright','felt','clav','muted','thumb']:
   old=before/f'before-{model}-dry.f32';new=after/f'after-{model}-dry.f32';x=np.fromfile(old,dtype=np.float32).astype(float);y=np.fromfile(new,dtype=np.float32).astype(float);assert x.shape==y.shape and np.isfinite(y).all()
   delta=y-x;peak=float(abs(delta).max());relative=float(20*np.log10(max(1e-30,np.linalg.norm(delta))/max(1e-30,np.linalg.norm(x))))
   row={'model':model,'rate':rate,'samples':len(x),'exact_samples':int(np.sum(x==y)),'max_abs_float_difference':peak,'relative_rms_difference_db':relative,'intentionally_changed':model in args.changed_model,'pass':(peak>0 if model in args.changed_model else peak<2e-6 and relative< -90)}
   report['cases'].append(row);old.unlink();new.unlink()
  print(f'{rate}: float comparisons complete',flush=True)
args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_text(json.dumps(report,indent=2)+'\n')
assert all(c['pass'] for c in report['cases']),report
print('PASS21model/rate comparisons; preserved models max2e-6/relative RMS below-90dB; declared changed models differ')
