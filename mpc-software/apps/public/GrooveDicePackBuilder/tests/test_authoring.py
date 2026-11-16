"""Real format roundtrip, hostile inputs and deterministic authoring tests."""
import copy,importlib.util,io,json,os,shutil,struct,sys,tempfile,unittest,wave,zipfile
from pathlib import Path
APP=Path(__file__).resolve().parents[1];M=APP.parents[2];sys.path.insert(0,str(M/'shared/groove-dice-expansion'))
import authoring as a,expansions as e

def wav(n=1200):
 b=io.BytesIO()
 with wave.open(b,'wb')as w:w.setparams((1,2,44100,0,'NONE',''));w.writeframes(b''.join(struct.pack('<h',int(8000*((i%80)/40-1)))for i in range(n)))
 return b.getvalue()
def midi(note=60):
 track=bytes([0,0x99,note,100,120,0x89,note,0,0,255,47,0]);return b'MThd'+struct.pack('>IHHH',6,0,1,960)+b'MTrk'+struct.pack('>I',len(track))+track

def fixture(root):
 for r,name in enumerate(a.ROLES):
  for v in range(2):
   p=root/name.replace(' ','')/f'{name.replace(" ","")}_{v+1}.wav';p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(wav(1200+r*100+v))
 p=root/'MIDI/Beats/Backbeat.mid';p.parent.mkdir(parents=True);p.write_bytes(midi(36));p=root/'MIDI/Grooves/Swing.mid';p.parent.mkdir(parents=True);p.write_bytes(midi(62))

class AuthoringTests(unittest.TestCase):
 def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.root=Path(self.tmp.name)/'Source';self.root.mkdir();fixture(self.root);self.p=a.new_project(self.root);a.generate(self.p,8,42)
 def tearDown(self):self.tmp.cleanup()
 def test_roundtrip(self):
  p=self.p;self.assertEqual(len(p['files']),18);self.assertEqual(len(p['kits']),8)
  path=Path(self.tmp.name)/'session.gdproject';a.save_project(p,path);loaded=a.load_project(path);self.assertEqual(loaded,p)
  p['kits'][0]['locks'][0]=True;old=p['kits'][0]['samples'][0];p['kits'][1]['locked']=True;keep=copy.deepcopy(p['kits'][1]);a.generate(p,8,88);self.assertEqual(p['kits'][0]['samples'][0],old);self.assertEqual(p['kits'][1],keep)
  out=Path(self.tmp.name)/'kit.gdexp';report=a.export(p,out);self.assertEqual(report['kits'],8)
  with zipfile.ZipFile(out)as z:
   m=json.loads(z.read('manifest.json'));self.assertEqual(m['target'],'groove-dice');self.assertEqual(m['plugin_uid'],'KpGy');self.assertIn('Personal use',m['rights']);self.assertFalse(m['redistributable']);self.assertEqual(e.sha(self.root/'Kick/Kick_1.wav'),next(f['sha']for f in p['files']if f['path']=='Kick/Kick_1.wav'))
  content=Path(self.tmp.name)/'Content';e.install(out,content,True);self.assertIn(p['id'],(content/'Expansions/registry.tsv').read_text());e.remove(p['id'],content)
  self.assertGreater(len(a.render_preview(p)),100000)
 def test_incomplete_and_changed(self):
  self.p['files']=[f for f in self.p['files']if f['role']!=7]
  with self.assertRaises(ValueError):a.generate(self.p)
  self.p['allow_silence']=True;a.generate(self.p);a.write_bank(self.p,Path(self.tmp.name)/'partial.gdbank')
  f=self.p['files'][0];(self.root/f['path']).write_bytes(b'changed')
  with self.assertRaisesRegex(ValueError,'changed'):a.export(self.p,Path(self.tmp.name)/'bad.gdexp')
 def test_mapping_and_guards(self):
  self.assertEqual(a.midi_info(a.map_midi(midi(36)))['note_values'],[60]);self.assertEqual(a.map_midi(midi(62)),midi(62));self.assertEqual(a.midi_info(a.map_midi(midi(62),'gm'))['note_values'],[67])
  with self.assertRaises(ValueError):a.map_midi(midi(20))
  with self.assertRaises(ValueError):a.safe_file(self.root,'../outside.wav')
  outside=Path(self.tmp.name)/'outside.wav';outside.write_bytes(wav())
  if os.name!='nt':
   (self.root/'Kick/link.wav').symlink_to(outside)
   with self.assertRaises(ValueError):a.safe_file(self.root,'Kick/link.wav')
  self.p['kits'][0]['samples'][0]=self.p['kits'][0]['samples'][1]
  with self.assertRaisesRegex(ValueError,'role'):a.validate_project(self.p)
  with self.assertRaises(ValueError):e.validate(dict(format='GDEXP001',id='wrong',target='other',plugin_uid='KpGy'))
 def test_empty_project_save_and_named_kits(self):
  self.p['kits']=[];path=Path(self.tmp.name)/'empty.gdproject';a.save_project(self.p,path);self.assertEqual(a.load_project(path)['kits'],[])
  target=self.root/'Kits/Named Session/Kick/Kick.wav';target.parent.mkdir(parents=True);target.write_bytes(wav(999));p=a.new_project(self.root);a.generate(p,2);self.assertEqual(p['kits'][0]['name'],'Named Session');self.assertIn('Named Session',p['kits'][0]['samples'][0])

if __name__=='__main__':unittest.main()
