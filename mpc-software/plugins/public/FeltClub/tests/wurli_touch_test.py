"""Black-box native audio test: dense velocities, not internal coefficients."""
from pathlib import Path
import subprocess,csv,math
p=Path(__file__).resolve().parents[1];sdk=p.parents[2]/'third_party/mpc-plugin-builder-sdk';exe=p/'build/audit_wurli_touch';out=p/'build/wurli-touch.csv'
subprocess.run(['clang++','-std=c++17','-O2','-I'+str(sdk/'framework'),str(p/'tools/audit_wurli_touch.cpp'),'-o',str(exe)],check=True)
subprocess.run([str(exe),str(p/'build/native/felt-club.dylib'),str(out)],check=True)
rows=list(csv.DictReader(out.open()));assert len(rows)==762
for note in [36,48,59,60,72,84]:
 a=[r for r in rows if int(r['note'])==note];assert [int(r['velocity']) for r in a]==list(range(1,128))
 assert all(math.isfinite(float(v)) for r in a for v in r.values())
 assert all(0<float(b['body_dbfs'])-float(c['body_dbfs'])<1 for c,b in zip(a,a[1:]))
 assert min(float(r['adjacent_cosine']) for r in a)>.99
print('PASS762native Wurli strikes: velocity1..127, increasing body level, <1dBsteps and smooth adjacent waveforms')
