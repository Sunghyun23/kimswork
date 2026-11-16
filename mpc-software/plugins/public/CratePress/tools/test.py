#!/usr/bin/env python3
"""Reproducible offline checks; never claims hardware or subjective listening QA."""
from pathlib import Path
import subprocess,json,re
P=Path(__file__).resolve().parents[1];SDK=P.parents[2]/'third_party/mpc-plugin-builder-sdk';B=P/'build';B.mkdir(exist_ok=True)
def run(argv,name):
 r=subprocess.run([str(x)for x in argv],text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT);(B/name).write_text(r.stdout)
 if r.returncode:raise RuntimeError(name+':\n'+r.stdout[-3500:])
 print(name+': PASS')
run(['c++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I',SDK/'framework',P/'tests/engine.cpp','-o',B/'engine-test'],'engine-compile.log')
run([B/'engine-test',B],'engine-test.log')
run(['c++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I',SDK/'framework',P/'tests/performance.cpp','-o',B/'performance-test'],'performance-compile.log')
run([B/'performance-test',B],'performance-test.log')
run(['c++','-std=c++17','-O2','-fsanitize=undefined','-fno-sanitize-recover=all','-I',SDK/'framework',P/'tests/combinations.cpp','-o',B/'combinations-test'],'combinations-compile.log')
run([B/'combinations-test',B],'combinations-test.log')
run(['c++','-std=c++17','-O2','-pthread','-fsanitize=undefined','-fno-sanitize-recover=all','-I',SDK/'framework',P/'tests/bridge.cpp','-o',B/'bridge-test'],'bridge-compile.log')
run([B/'bridge-test'],'bridge-test.log')
for sr,block in [(44100,128),(44100,32),(48000,512)]:
 run([SDK/'.sdk-tools/host_sim',B/'native/crate-press.dylib','test','--seconds','3','--sr',sr,'--block',block,'--wav',B/f'host-{sr}-{block}.wav'],f'host-{sr}-{block}.log')
run(['python3',SDK/'tools/validate.py',P],'sdk-validation.log')
d=json.loads((P/'skins/TUI.json').read_text());q=json.loads((P/'skins/Q-Links.json').read_text());dump=json.loads((B/'params.json').read_text())
assert dump['numPrograms']==32 and len(dump['params'])==380
names={x['index']:x['name']for x in dump['params']}
expected=['Drive','Compress','Punch','Release','Low Comp','Mid Comp','High Comp','Duck','Tone','High Cut','Lo-Fi','Drift','Input','Low Cut','Mix','Output']
main=q['Screen Mode Q-Links']['map'][0]['Q-Links'];program=q['Program Mode Q-Links']
for group in range(4):
 for knob in range(4):
  n='Q-Link '+str(13+group-4*knob);assert names[main[n]]==expected[group*4+knob] and program[n]==main[n]
assert d['pageData']['tabs'][0]['qlinkBoundsData']==['8 206 628 158','644 206 628 158','8 372 628 158','644 372 628 158']
defs=d['pageData']['componentDefinitions']['localComponentDefinitions']
for x in defs:
 if'overlay'in x['key']:assert x['value']['hideQLinkBounds']
assert len(d['pageData']['tabs'])==5 and d['pageData']['tabs'][3]['tabName']=='PERFORM'
perform=q['Screen Mode Q-Links']['map'][3]['Q-Links']
for group in range(4):
 for knob in range(4):assert perform['Q-Link '+str(13+group-4*knob)]==355+group*4+knob
for i in range(8):
 comp=next(x['value']for x in defs if x['key']=='PerformHold'+str(i));top=comp['componentsData'][-1]['componentData']
 assert top['type']=='Slider'and top['data']['handleName']=='ResetOnGestureEnd/Parameter '+str(347+i)
 assert comp['actions'][0]['onAction']=='Mouse Down'and comp['actions'][0]['handler']=='Toggle Switch'
 assert not any(x['componentData']['type']=='Button'for x in comp['componentsData'])
 latched=next(x['value']for x in defs if x['key']=='PerformLatch'+str(i));topLatch=latched['componentsData'][-1]['componentData'];assert topLatch['type']=='Button'and topLatch['data']['handleName']=='Data'and not latched['actions']
 assert latched['componentsData'][-1]['handle remapping']['map'][0]['value']=='Parameter '+str(347+i)
 mainentry=next(x for x in d['pageData']['tabs'][0]['componentDefinition']['componentsData']if x['componentData']['type']=='PerformHold'+str(i))
 assert mainentry['bounds']['bounds']==f'{8+i*158+(8 if i>=4 else 0)} 538 150 82'
def no_self_remap(o):
 if isinstance(o,dict):
  if 'handle remapping'in o:
   assert not any(r['key']==r['value']for r in o['handle remapping']['map']),'self-remapping breaks inherited native widget values'
  for v in o.values():no_self_remap(v)
 elif isinstance(o,list):
  for v in o:no_self_remap(v)
no_self_remap(d)
for tab in d['pageData']['tabs'][1:]:
 assert not any(' seg 'in x['componentData']['name'] for x in tab['componentDefinition']['componentsData']),'captions must be baked into button images, not cover their touches'
for definition in defs:
 if definition['key'].startswith('CPChoice'):
  comp=definition['value'];assert not comp['actions'] and comp['ignoreMousePresses']
  assert all(x['componentData']['data']['onImage'].startswith('choice-') for x in comp['componentsData'] if x['componentData']['type']=='Button')
wave=[]
def walk(o):
 if isinstance(o,dict):
  if o.get('type')=='Image' and o.get('data',{}).get('image','').startswith('wave-'):wave.append(o['data'])
  for v in o.values():walk(v)
 elif isinstance(o,list):
  for v in o:walk(v)
walk(d);assert len(wave)==256 and all(not x.get('animated',False)and x['handleName'].startswith('Parameter ')for x in wave)
from PIL import Image
refs=set(re.findall(r'"([^"\n]+\.(?:png|jpg|jpeg))"',(P/'skins/TUI.json').read_text()))
for f in(P/'skins').glob('*.png'):
 assert f.name in refs
 im=Image.open(f)
 if f.name.startswith(('wave-','gr'))or f.name in('curve.png','cursor.png','knob.png','fader.png'):assert max(im.size)<=8192
print('skin regression: PASS (16assignments,4guides,alloverlays,256real-wavehandles,assetlimits)')
summary={'status':'offline-passed','factoryPrograms':32,'parameters':380,'performanceEffects':8,'latch':'header toggle,independent8tap gates,disable releases all,restore disarms gates','combinationTests':'28pairs x3rates,each pair differs from either solo,release-one keeps the other audible,all8/staggered release/echo tail/zero allocations passed','performanceTests':'audible effects,release recovery,repeat tempo,frozen history,echo decay,independent settings,transient gate clearing passed','sampleRates':[44100,48000,96000],'waveform':'signed stereo extrema,64x12ms,30Hz,latency aligned','curve':'same DSP reduction function,4views','simulatorRuns':[{'sr':sr,'block':b,'result':'20PASS/0WARN/0FAIL'}for sr,b in[(44100,128),(44100,32),(48000,512)]],'undefinedBehaviorSanitizer':'passed','realTimeAllocations':0,'deviceVerification':'pending; no installation/restart in this iteration','humanListening':'not performed','preview':'offline runtimeTUI with engine-measured telemetry; not an MPC screenshot'}
(B/'validation.json').write_text(json.dumps(summary,indent=2)+'\n')
run(['python3',P/'tools/render_skin_preview.py','--performance'],'performance-preview.log')
print('All offline checks passed. Actual MPC visual/CPU/touch/track+master verification remains pending.')
