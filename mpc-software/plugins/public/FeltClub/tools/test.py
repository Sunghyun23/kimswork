#!/usr/bin/env python3
from pathlib import Path
import subprocess,sys,os
p=Path(__file__).resolve().parents[1];sdk=p.parents[2]/'third_party/mpc-plugin-builder-sdk';env=dict(os.environ,MPC_PLUGIN_FOLDER=str(p))
for name in ['cpu_optimization_test','engine_test','factory_bank_test','texture_noise_test','vst_test','physical_test','engine_isolation_test','wurli_field_test','upright_radiation_test']:
 out=p/'build'/name
 subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I'+str(sdk/'framework'),str(p/'tests'/f'{name}.cpp'),'-o',str(out)],check=True)
 subprocess.run([str(out),*([str(p/'build/native/felt-club.dylib')]if name=='vst_test'else[])],check=True,env=env)
subprocess.run([sys.executable,str(p/'tools/calibration/tine_diagnostics.py'),'--self-test'],check=True)
subprocess.run([sys.executable,str(p/'tests/skin_test.py')],check=True)
subprocess.run([sys.executable,str(p/'tests/engine_dependencies_test.py')],check=True)
subprocess.run([sys.executable,str(p/'tests/wurli_touch_test.py')],check=True)
subprocess.run([str(sdk/'.sdk-tools/host_sim'),str(p/'build/native/felt-club.dylib'),'test','--notes','60,64,67','--seconds','3','--wav',str(p/'build/host-test.wav')],check=True,env=env)
