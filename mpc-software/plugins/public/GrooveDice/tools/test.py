#!/usr/bin/env python3
import os,subprocess,sys
from pathlib import Path
P=Path(__file__).resolve().parents[1];SDK=P.parents[2]/'third_party/mpc-plugin-builder-sdk';env=dict(os.environ,MPC_PLUGIN_FOLDER=str(P));with_private='--with-private'in sys.argv
for flag,name in [([], 'engine_test'),(['-fsanitize=undefined'],'engine_test_ubsan')]:
 subprocess.run(['clang++','-std=c++17','-O2',*flag,'-I'+str(SDK/'framework'),str(P/'tests/engine_test.cpp'),'-o',str(P/'build'/name)],check=True)
 subprocess.run([str(P/'build'/name)],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-I'+str(SDK/'framework'),str(P/'tests/bank_test.cpp'),'-o',str(P/'build/bank_test')],check=True)
subprocess.run([str(P/'build/bank_test'),str(P/'Content/Banks/default.gdbank')],check=True)
if with_private:
 private_env=dict(env,MPC_PLUGIN_FOLDER=str(P.parents[2]/'expansions/groove-dice/private/splat-to-clat/work/legacy-reference'),GD_VERIFY_BANK=str(P.parents[2]/'expansions/groove-dice/private/splat-to-clat/work/legacy-reference/Content/Banks/default.gdbank'))
 subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-I'+str(SDK/'framework'),str(P/'tests/library_test.cpp'),'-o',str(P/'build/library_test')],check=True)
 subprocess.run([str(P/'build/library_test')],env=private_env,check=True)
 subprocess.run([sys.executable,str(P.parents[2]/'expansions/groove-dice/private/splat-to-clat/tools/verify_library.py')],env=private_env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-I'+str(SDK/'framework'),str(P/'tests/factory_test.cpp'),'-o',str(P/'build/factory_test')],check=True)
subprocess.run([str(P/'build/factory_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-I'+str(SDK/'framework'),str(P/'tests/groove_test.cpp'),'-o',str(P/'build/groove_test')],check=True)
subprocess.run([str(P/'build/groove_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-I'+str(SDK/'framework'),str(P/'tests/rules_test.cpp'),'-o',str(P/'build/rules_test')],check=True)
subprocess.run([str(P/'build/rules_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-I'+str(SDK/'framework'),str(P/'tests/usability_test.cpp'),'-o',str(P/'build/usability_test')],check=True)
subprocess.run([str(P/'build/usability_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-I'+str(SDK/'framework'),str(P/'tests/musical_test.cpp'),'-o',str(P/'build/musical_test')],check=True)
subprocess.run([str(P/'build/musical_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-I'+str(SDK/'framework'),str(P/'tests/hat_rhythm_test.cpp'),'-o',str(P/'build/hat_rhythm_test')],check=True)
subprocess.run([str(P/'build/hat_rhythm_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-I'+str(SDK/'framework'),str(P/'tests/original_bank_test.cpp'),'-o',str(P/'build/original_bank_test')],check=True)
subprocess.run([str(P/'build/original_bank_test')],env=env,check=True)
subprocess.run([sys.executable,str(P/'tests/skin_test.py')],check=True)
subprocess.run([sys.executable,str(P/'tests/expansion_manager_test.py')],check=True)
env=dict(os.environ,MPC_PLUGIN_FOLDER=str(P))
subprocess.run([str(SDK/'.sdk-tools/host_sim'),str(P/'build/native/groove-dice.dylib'),'test','--notes','24,26,28','--seconds','3','--wav',str(P/'build/test.wav')],env=env,check=True)
