"""Common actionable footer geometry on every reachable non-modal plugin page."""
from pathlib import Path
import json,argparse
R=Path(__file__).resolve().parents[1];policy=json.loads((R/'shared/ui/action-footer.json').read_text());want={a['label']:(a['x'],policy['y'],a['width'],policy['height'])for a in policy['actions']};count=0
ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('projects',nargs='*');args=ap.parse_args()
for project in (args.projects or ['GrooveDice','CratePress']):
 if project not in ['DonutTheory','GrooveDice','CratePress']:ap.error('Unknown project '+project)
 pd=json.loads((R/f'plugins/public/{project}/skins/TUI.json').read_text())['pageData'];defs={e['key']:e['value']for e in pd['componentDefinitions']['localComponentDefinitions']}
 if project=='GrooveDice':pd['tabs'].append({'tabName':'FILL_EDITOR','componentDefinition':{**defs['GDFill'],'hideQLinkBounds':False},'qlinkBoundsData':[]})
 for tab in pd['tabs']:
  found={key:[]for key in want}
  def walk(node,x=0,y=0,stack=()):
   if node.get('hideQLinkBounds'):return
   for w in node.get('componentsData',[]):
    xx,yy,ww,hh=map(int,w['bounds']['bounds'].split());c=w['componentData'];typ=c['type'];name=c.get('name','');d=defs.get(typ);caption=name if typ!='Label'and name in want else None
    if d:
     texts=[v['componentData'].get('data',{}).get('text')for v in d.get('componentsData',[])if v['componentData']['type']=='Label'];caption=next((v for v in texts if v in want),caption)
    if caption in want:found[caption].append((x+xx,y+yy,ww,hh))
    if d and typ not in stack:walk(d,x+xx,y+yy,stack+(typ,))
  walk(tab['componentDefinition'])
  for label,bounds in want.items():assert found[label]and set(found[label])=={bounds},(project,tab['tabName'],label,found[label],bounds)
  for raw in tab['qlinkBoundsData']:
   x,y,w,h=map(int,raw.split());assert not w or not h or y+h<=policy['y'],(project,tab['tabName'],'guide covers footer',raw)
  count+=1
print('PASS',count,'plugin pages: common UNLOCK ALL / ROLL DICE / UNDO coordinates, no Q-Link guides over footer')
