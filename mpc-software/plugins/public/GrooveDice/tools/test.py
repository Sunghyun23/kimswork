#!/usr/bin/env python3
import os,subprocess,sys
from pathlib import Path
P=Path(__file__).resolve().parents[1];SDK=P.parents[2]/'third_party/mpc-plugin-builder-sdk';env=dict(os.environ,MPC_PLUGIN_FOLDER=str(P));with_private='--with-private'in sys.argv
for suffix,flags in [('',[]),('_ubsan',['-fsanitize=undefined','-fno-sanitize-recover=all'])]:
 binary=P/'build'/('common_locks_test'+suffix)
 subprocess.run(['clang++','-std=c++17','-O2',*flags,'-I'+str(SDK/'framework'),str(P/'tests/common_locks_test.cpp'),'-o',str(binary)],check=True)
 subprocess.run([str(binary)],env=env,check=True)
for flag,name in [([], 'engine_test'),(['-fsanitize=undefined'],'engine_test_ubsan')]:
 subprocess.run(['clang++','-std=c++17','-O2',*flag,'-I'+str(SDK/'framework'),str(P/'tests/engine_test.cpp'),'-o',str(P/'build'/name)],check=True)
 subprocess.run([str(P/'build'/name)],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I'+str(SDK/'framework'),str(P/'tests/one_bar_loop_test.cpp'),'-o',str(P/'build/one_bar_loop_test')],check=True)
subprocess.run([str(P/'build/one_bar_loop_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I'+str(SDK/'framework'),str(P/'tests/host_countin_test.cpp'),'-o',str(P/'build/host_countin_test')],check=True)
subprocess.run([str(P/'build/host_countin_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I'+str(SDK/'framework'),str(P/'tests/live_rules_bridge_test.cpp'),'-o',str(P/'build/live_rules_bridge_test')],check=True)
subprocess.run([str(P/'build/live_rules_bridge_test')],env=env,check=True)
for name in ['dice_scope_test','performance_test','musical_performance_test','triplet_test','rate_knob_audit_test','room_reverb_test','sample_arrows_test','velocity_pattern_test','cymbal_rules_test','sample_browser_test','state_context_test','decay_controls_test','front_dice_test','rule_selector_test','reverse_test','sound_pages_test','extended_rules_test']:
 binary=P/'build'/name
 subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I'+str(SDK/'framework'),str(P/'tests'/f'{name}.cpp'),'-o',str(binary)],check=True)
 subprocess.run([str(binary)],env=env,check=True)
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
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I'+str(SDK/'framework'),str(P/'tests/layers_test.cpp'),'-o',str(P/'build/layers_test')],check=True)
subprocess.run([str(P/'build/layers_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I'+str(SDK/'framework'),str(P/'tests/layer_names_test.cpp'),'-o',str(P/'build/layer_names_test')],check=True)
subprocess.run([str(P/'build/layer_names_test')],env=env,check=True)
subprocess.run(['clang++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I'+str(SDK/'framework'),str(P/'tests/display_names_test.cpp'),'-o',str(P/'build/display_names_test')],check=True)
subprocess.run([str(P/'build/display_names_test')],env=env,check=True)
import json
assert json.loads((P/'parameters.json').read_text())[:342]==json.loads((P/'tests/legacy-parameters-1.0.0.json').read_text()),'legacy342 parameter identities/order changed'
assert json.loads((P/'parameters.json').read_text())[:533]==json.loads((P/'tests/legacy-parameters-1.2.2.json').read_text()),'legacy533 parameter identities/order changed'
assert json.loads((P/'parameters.json').read_text())[:536]==json.loads((P/'tests/legacy-parameters-1.3.0.json').read_text()),'legacy536 changed'
assert json.loads((P/'parameters.json').read_text())[:623]==json.loads((P/'tests/legacy-parameters-1.3.1.json').read_text()),'legacy623 changed'
assert json.loads((P/'parameters.json').read_text())[:639]==json.loads((P/'tests/legacy-parameters-1.3.2.json').read_text()),'legacy639 changed'
assert json.loads((P/'parameters.json').read_text())[:643]==json.loads((P/'tests/legacy-parameters-1.3.3.json').read_text()),'legacy643 changed'
subprocess.run([sys.executable,str(P/'tests/skin_test.py')],check=True)
subprocess.run([sys.executable,str(P/'tests/expansion_manager_test.py')],check=True)
env=dict(os.environ,MPC_PLUGIN_FOLDER=str(P))
subprocess.run([str(SDK/'.sdk-tools/host_sim'),str(P/'build/native/groove-dice.dylib'),'test','--notes','24,26,28','--seconds','3','--wav',str(P/'build/test.wav')],env=env,check=True)

assert json.loads((P/'parameters.json').read_text())[:779]==json.loads((P/'tests/legacy-parameters-1.5.1.json').read_text()),'legacy779 changed'

assert json.loads((P/'parameters.json').read_text())[:782]==json.loads((P/'tests/legacy-parameters-1.6.2.json').read_text()),'legacy782 changed'

assert json.loads((P/'parameters.json').read_text())[:880]==json.loads((P/'tests/legacy-880-parameters.json').read_text()),'legacy880changed'
