#!/usr/bin/env python3
"""Run simulator and deterministic instrument regressions after a build."""
import subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];p=ROOT/'plugins/public/DonutTheory';sdk=ROOT/'third_party/mpc-plugin-builder-sdk'
# Current developer baseline is macOS; the SDK supports other build hosts.
if sys.platform!='darwin':raise SystemExit('This test wrapper currently requires macOS; run SDK host_sim manually on other hosts.')
for name,flags in [('engine_test',[]),('engine_test_ubsan',['-fsanitize=undefined'])]:
 subprocess.run(['clang++','-std=c++17','-O2',*flags,'-I'+str(sdk/'framework'),str(p/'tests/engine_test.cpp'),'-o',str(p/'build'/name)],check=True)
 subprocess.run([str(p/'build'/name)],check=True)
for suffix,flags in [('',[]),('_ubsan',['-fsanitize=undefined'])]:
 binary=p/'build'/('analog_engine_test'+suffix)
 subprocess.run(['clang++','-std=c++17','-O2',*flags,'-I'+str(sdk/'framework'),str(p/'tests/analog_engine_test.cpp'),'-o',str(binary)],check=True)
 subprocess.run([str(binary)],check=True)
for suffix,flags in [('',[]),('_ubsan',['-fsanitize=undefined'])]:
 binary=p/'build'/('host_bridge_test'+suffix)
 subprocess.run(['clang++','-std=c++17','-O2',*flags,'-I'+str(sdk/'framework'),str(p/'tests/host_bridge_test.cpp'),'-o',str(binary)],check=True)
 subprocess.run([str(binary)],check=True)
subprocess.run([str(sdk/'.sdk-tools/host_sim'),str(p/'build/native/donut-theory.dylib'),'test','--notes','48,52,55','--seconds','5','--wav',str(p/'build/demo.wav')],check=True)
subprocess.run([sys.executable,str(p/'tests/skin_test.py')],check=True)
