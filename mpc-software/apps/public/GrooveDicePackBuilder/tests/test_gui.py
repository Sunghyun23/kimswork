"""Exercise actual Qt controls and the common authoring paths offscreen."""
import os,sys,tempfile,copy,json
from pathlib import Path
os.environ.setdefault('QT_QPA_PLATFORM','offscreen')
P=Path(__file__).resolve().parents[1];sys.path.insert(0,str(P/'src'));sys.path.insert(0,str(P/'tests'))
from test_authoring import fixture
from main import QApplication,Window,STYLE,authoring
app=QApplication([]);app.setStyle('Fusion');app.setStyleSheet(STYLE)
with tempfile.TemporaryDirectory()as temp:
 root=Path(temp)/'My Sessions';root.mkdir();fixture(root);p=authoring.new_project(root);w=Window();w.set_project(p);w.show();app.processEvents();w.generate_kits();assert w.kits.count()==8
 w.parts[0][0].setCurrentIndex(2);assert w.project['kits'][0]['samples'][0].endswith('_2.wav')
 w.parts[0][1].setChecked(True);assert w.project['kits'][0]['locks'][0]
 w.parts[0][2].setValue(-3);assert w.project['kits'][0]['levels'][0]==-3
 w.kitlock.setChecked(True);assert w.project['kits'][0]['locked'];saved=copy.deepcopy(w.project['kits'][0]);w.generate_kits();assert w.project['kits'][0]==saved
 w.project_file=str(Path(temp)/'test.gdproject');w.save();assert not w.dirty and Path(w.project_file).exists();restored=authoring.load_project(w.project_file);assert restored['kits']==w.project['kits']
 w.metadata();out=Path(temp)/'test.gdexp';authoring.export(w.project,out);assert out.exists();print('PASS Qt role selection, part lock/gain, kit lock, regeneration, save/reopen and export')
 if os.environ.get('GD_SCREENSHOT'):w.grab().save(os.environ['GD_SCREENSHOT'])
 w.dirty=False;w.close();app.processEvents()
