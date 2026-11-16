"""Groove Dice Pack Builder desktop. Local files only; no network services."""
from __future__ import annotations
import argparse, copy, importlib.util, json, os, random, sys, tempfile, wave
from pathlib import Path

HERE=Path(__file__).resolve().parent
SHARED=Path(getattr(sys,'_MEIPASS',HERE.parents[3]))/'shared/groove-dice-expansion'
if not SHARED.exists():SHARED=HERE.parents[3]/'shared/groove-dice-expansion'
sys.path.insert(0,str(SHARED))
import authoring, expansions
from PySide6.QtCore import Qt, QThread, Signal, QUrl, QTimer, QRectF
from PySide6.QtGui import QColor, QFont, QPainter, QPen, QPixmap, QAction, QKeySequence
from PySide6.QtWidgets import (QApplication,QMainWindow,QWidget,QVBoxLayout,QHBoxLayout,QGridLayout,QLabel,QPushButton,QLineEdit,QComboBox,QSpinBox,QDoubleSpinBox,QCheckBox,QListWidget,QListWidgetItem,QFileDialog,QMessageBox,QDialog,QDialogButtonBox,QTableWidget,QHeaderView,QFormLayout,QFrame,QSplitter,QPlainTextEdit,QProgressBar,QScrollArea)
from PySide6.QtMultimedia import QMediaPlayer, QAudioOutput

VERSION='1.0.0'
STYLE='''
QMainWindow,QDialog {background:#29271f;color:#f6e5c5;}
QWidget {font-family:"Arial";font-size:13px;color:#f1dfbf;}
QFrame#panel {background:#333126;border:1px solid #71654c;border-radius:9px;}
QLabel#heading {font-size:17px;letter-spacing:1px;font-weight:700;color:#f8e7c9;padding:8px 2px;}
QLabel#title {font-family:"Georgia";font-size:29px;font-weight:700;color:#f9e8c8;}
QFrame#header {background:#402818;border-bottom:2px solid #aa7140;}
QPushButton {background:#443b2c;border:1px solid #a38a62;border-radius:6px;padding:9px 13px;font-weight:600;min-height:20px;}
QPushButton:hover {background:#665038;border-color:#edc587;}
QPushButton:pressed,QPushButton:checked {background:#a66223;border-color:#ffe0a1;}
QPushButton#primary {background:#b7631e;color:#fff1d6;border:1px solid #e4aa62;}
QPushButton#primary:hover {background:#d17b2a;}
QPushButton:disabled {color:#817660;background:#353228;border-color:#514b3b;}
QLineEdit,QComboBox,QSpinBox,QDoubleSpinBox,QPlainTextEdit {min-height:20px;background:#24251e;border:1px solid #8a7858;border-radius:5px;padding:7px;selection-background-color:#a96a2b;}
QComboBox QAbstractItemView {background:#f0dfbf;color:#30271c;selection-background-color:#b9762d;}
QListWidget {background:#26271f;border:0;border-radius:6px;padding:3px;}
QListWidget::item {background:#e5d0a6;color:#2e2519;margin:3px 1px;padding:12px;border-radius:5px;}
QListWidget::item:selected {background:#d99a45;color:#271e13;}
QFrame#card {background:#ead6af;border:1px solid #ae8c56;border-radius:7px;}
QFrame#card QLabel {color:#33291d;}
QFrame#card QPushButton {background:#655b36;color:#fff1d5;border:0;padding:4px;min-height:16px;}
QFrame#card QLabel#role {font-size:13px;font-weight:700;}
QTableWidget {background:#efdfc0;color:#30271d;gridline-color:#bfa777;}
QHeaderView::section {background:#403929;color:#f1dfbf;padding:8px;}
QProgressBar {background:#23241c;border:0;border-radius:3px;height:5px;}
QProgressBar::chunk {background:#c37c2a;}
QLabel#muted {color:#bcb095;font-size:12px;}
QCheckBox {spacing:6px;}
'''

def button(text,fn,parent=None,primary=False):
    b=QPushButton(text,parent);b.clicked.connect(fn)
    if primary:b.setObjectName('primary')
    return b

def panel(title):
    w=QFrame();w.setObjectName('panel');v=QVBoxLayout(w);v.setContentsMargins(16,12,16,14);v.setSpacing(10)
    h=QLabel(title);h.setObjectName('heading');v.addWidget(h)
    return w,v

class Task(QThread):
    done=Signal(object);failed=Signal(str)
    def __init__(self,fn,parent):super().__init__(parent);self.fn=fn
    def run(self):
        try:self.done.emit(self.fn())
        except Exception as ex:self.failed.emit(str(ex))

class Waveform(QWidget):
    def __init__(self):super().__init__();self.values=[];self.setMinimumHeight(90)
    def set_audio(self,data):
        vals,rate=authoring.decode(data);stride=max(1,len(vals)//450);self.values=[max(map(abs,vals[i:i+stride]))for i in range(0,len(vals),stride)];self.update()
    def paintEvent(self,event):
        p=QPainter(self);p.setRenderHint(QPainter.RenderHint.Antialiasing);p.fillRect(self.rect(),QColor('#ead6af'));mid=self.height()/2;p.setPen(QPen(QColor('#bd7330'),1))
        p.drawLine(12,int(mid),self.width()-12,int(mid))
        if self.values:
            maximum=max(max(self.values),.001)
            for i,v in enumerate(self.values):
                x=12+i*(self.width()-24)/len(self.values);h=v/maximum*(mid-10);p.drawLine(int(x),int(mid-h),int(x),int(mid+h))

class Cover(QWidget):
    def __init__(self):
        super().__init__();self.setMinimumHeight(150);self.setMaximumHeight(190)
        asset=Path(getattr(sys,'_MEIPASS',HERE.parent))/'design/main-concept-v1.png';self.pix=QPixmap(str(asset))
    def paintEvent(self,event):
        p=QPainter(self);s=min(self.width(),self.height());x=(self.width()-s)/2
        if not self.pix.isNull():p.drawPixmap(QRectF(x,0,s,s),self.pix,QRectF(55,145,200,200))

class Window(QMainWindow):
    def __init__(self):
        super().__init__();self.project=None;self.project_file=None;self.selected_role=0;self.task=None;self.dirty=False;self.parts=[];self.cards=[];self.preview_temp=tempfile.TemporaryDirectory(prefix='gd-audition-');self.audio=QAudioOutput(self);self.audio.setVolume(.7);self.player=QMediaPlayer(self);self.player.setAudioOutput(self.audio)
        self.setWindowTitle('Groove Dice Pack Builder');self.resize(1450,940);self.setMinimumSize(1160,820)
        center=QWidget();self.setCentralWidget(center);root=QVBoxLayout(center);root.setContentsMargins(0,0,0,0);root.setSpacing(0)
        header=QFrame();header.setObjectName('header');hv=QHBoxLayout(header);hv.setContentsMargins(22,18,22,18);title=QLabel('⚄  Groove Dice Pack Builder');title.setObjectName('title');hv.addWidget(title);hv.addStretch();self.project_title=QLabel('YOUR NEXT SESSION');hv.addWidget(self.project_title);hv.addWidget(QLabel('v'+VERSION));root.addWidget(header)
        body=QHBoxLayout();body.setContentsMargins(14,14,14,14);body.setSpacing(12);root.addLayout(body,1)
        left,lv=panel('PACK DETAILS');left.setMinimumWidth(225);lv.addWidget(Cover());form=QFormLayout();form.setRowWrapPolicy(QFormLayout.RowWrapPolicy.WrapAllRows)
        self.name=QLineEdit();self.creator=QLineEdit('kim');self.packid=QLineEdit();self.version=QLineEdit('1.0.0');self.distribution=QComboBox();self.distribution.addItems(['Personal / private','Redistributable'])
        for label,widget in [('Pack name',self.name),('Creator',self.creator),('Version',self.version),('Distribution',self.distribution)]:form.addRow(label,widget)
        self.midi_mode=QComboBox();self.midi_mode.addItems(['Auto: Native / GM drums','Groove Dice notes 60–67','General MIDI drums']);form.addRow('MIDI note mapping',self.midi_mode);lv.addLayout(form);advanced=QWidget();av=QVBoxLayout(advanced);av.setContentsMargins(0,0,0,0);av.addWidget(QLabel('Pack ID'));av.addWidget(self.packid);self.packid.setToolTip('Identifies this sound pack, not a user account. Automatically generated. Keep it unchanged for pack updates and saved songs.');advanced.hide();toggle=QPushButton('Advanced pack settings ▸');toggle.setCheckable(True);toggle.toggled.connect(advanced.setVisible);toggle.toggled.connect(lambda on:toggle.setText('Advanced pack settings ▾'if on else'Advanced pack settings ▸'));lv.addWidget(toggle);lv.addWidget(advanced);self.rights=QPlainTextEdit();self.rights.setPlaceholderText('Describe the rights for these files');self.rights.setFixedHeight(62);lv.addWidget(self.rights);self.silence=QCheckBox('Allow silent missing roles');self.silence.setToolTip('Missing instruments become generated silence; no sounds are borrowed from other packs.');lv.addWidget(self.silence);self.source_label=QLabel('Choose a folder containing WAV / MIDI');self.source_label.setWordWrap(True);self.source_label.setObjectName('muted');lv.addWidget(self.source_label);lv.addStretch();leftscroll=QScrollArea();leftscroll.setWidgetResizable(True);leftscroll.setFrameShape(QFrame.Shape.NoFrame);leftscroll.setFixedWidth(250);leftscroll.setWidget(left);body.addWidget(leftscroll)
        middle,mv=panel('INSTRUMENTS');self.source_title=QLabel('Open a folder to begin');self.source_title.setWordWrap(True);mv.addWidget(self.source_title)
        grid=QGridLayout();grid.setSpacing(9)
        for r,role in enumerate(authoring.ROLES):
            card=QFrame();card.setObjectName('card');cv=QVBoxLayout(card);cv.setContentsMargins(10,9,10,9);cv.setSpacing(4);h=QLabel(role.upper());h.setObjectName('role');cv.addWidget(h);waveview=Waveform();waveview.setMinimumHeight(45);waveview.setMaximumHeight(65);cv.addWidget(waveview);filelabel=QLabel('No files');filelabel.setWordWrap(True);cv.addWidget(filelabel);row=QHBoxLayout();row.addWidget(button('▶',lambda checked=False,r=r:self.audition_role(r)));select=button('0 files',lambda checked=False,r=r:self.choose_role(r));row.addWidget(select,1);cv.addLayout(row);grid.addWidget(card,r//4,r%4);self.cards.append((waveview,filelabel,select))
        mv.addLayout(grid);self.file_label=QLabel('Select a role to inspect its waveform');mv.addWidget(self.file_label);self.wave=Waveform();self.wave.setMinimumHeight(95);mv.addWidget(self.wave)
        self.issues=QLabel('No source folder selected');self.issues.setWordWrap(True);self.issues.setObjectName('muted');mv.addWidget(self.issues);mv.addStretch()
        audition=QHBoxLayout();audition.addWidget(button('▶  Audition kit',self.audition_kit));audition.addWidget(button('■',self.stop_audio));audition.addStretch();audition.addWidget(QLabel('BPM'));self.bpm=QSpinBox();self.bpm.setRange(40,180);self.bpm.setValue(88);audition.addWidget(self.bpm);audition.addWidget(QLabel('Swing'));self.swing=QSpinBox();self.swing.setRange(50,75);self.swing.setValue(57);self.swing.setSuffix('%');audition.addWidget(self.swing);mv.addLayout(audition);body.addWidget(middle,1)
        right,rv=panel('GENERATED KITS');right.setFixedWidth(330);self.kits=QListWidget();self.kits.currentRowChanged.connect(self.show_kit);self.kits.itemChanged.connect(self.rename_kit);rv.addWidget(self.kits,1)
        lockrow=QHBoxLayout();self.kitlock=QCheckBox('Lock this kit');self.kitlock.toggled.connect(self.lock_kit);lockrow.addWidget(self.kitlock);lockrow.addStretch();lockrow.addWidget(button('Rename',self.rename_current));rv.addLayout(lockrow)
        for r,role in enumerate(authoring.ROLES):
            row=QHBoxLayout();lab=QLabel(role);lab.setFixedWidth(70);row.addWidget(lab);combo=QComboBox();combo.setMinimumWidth(100);combo.currentIndexChanged.connect(lambda i,r=r:self.change_part(r));row.addWidget(combo,1);lock=QCheckBox();lock.setToolTip('Keep this part when regenerating');lock.toggled.connect(lambda b,r=r:self.lock_part(r,b));row.addWidget(lock);level=QDoubleSpinBox();level.setRange(-48,6);level.setDecimals(1);level.setSuffix(' dB');level.setFixedWidth(76);level.valueChanged.connect(lambda v,r=r:self.change_level(r,v));row.addWidget(level);rv.addLayout(row);self.parts.append((combo,lock,level))
        countrow=QHBoxLayout();countrow.addWidget(QLabel('Kits to create'));self.count=QSpinBox();self.count.setRange(1,512);self.count.setValue(8);countrow.addWidget(self.count);rv.addLayout(countrow);body.addWidget(right)
        footer=QFrame();footer.setObjectName('header');fv=QVBoxLayout(footer);fv.setContentsMargins(18,12,18,10);actions=QHBoxLayout();self.action_buttons=[]
        for text,fn,primary in [('Open Folder',self.open_folder,False),('Review Mapping',self.review_mapping,False),('Generate Kits',self.generate_kits,False),('Export Expansion',self.export_pack,True)]:
            b=button(text,fn,primary=primary);actions.addWidget(b);self.action_buttons.append(b)
        fv.addLayout(actions);last=QHBoxLayout();self.status=QLabel('Choose a folder. Your source files are never changed.');self.status.setWordWrap(True);last.addWidget(self.status,1);last.addWidget(button('Save Project',self.save));last.addWidget(button('Open Project',self.open_project));last.addWidget(button('Manage Packs',self.manage_packs));fv.addLayout(last);self.progress=QProgressBar();self.progress.setRange(0,1);self.progress.setValue(0);self.progress.setTextVisible(False);fv.addWidget(self.progress);root.addWidget(footer)
        self.player.errorOccurred.connect(lambda error,text:self.status.setText('Audio: '+text))
        self.player.playbackStateChanged.connect(lambda state:self.status.setText('Audition stopped'if state==QMediaPlayer.PlaybackState.StoppedState else'Audition playing')if not self.task else None)
        for widget in [self.name,self.creator,self.packid,self.version]:widget.textEdited.connect(self.mark_dirty)
        self.midi_mode.currentIndexChanged.connect(self.mark_dirty);self.rights.textChanged.connect(self.mark_dirty);self.distribution.currentIndexChanged.connect(self.mark_dirty);self.silence.toggled.connect(self.mark_dirty)
        save_action=QAction('Save Project',self);save_action.setShortcut(QKeySequence.StandardKey.Save);save_action.triggered.connect(self.save);self.addAction(save_action)
        self.refresh()

    def mark_dirty(self,*args):self.dirty=True
    def error(self,message):self.status.setText(message);QMessageBox.warning(self,'Please review',message)
    def metadata(self):
        if self.project:
            self.project.update(name=self.name.text().strip(),creator=self.creator.text().strip(),id=self.packid.text().strip(),version=self.version.text().strip(),private=self.distribution.currentIndex()==0,rights=self.rights.toPlainText().strip(),allow_silence=self.silence.isChecked(),midi_mode=['auto','native','gm'][self.midi_mode.currentIndex()])
    def run_task(self,fn,done,label):
        if self.task:return
        self.status.setText(label);self.progress.setRange(0,0);self.centralWidget().setEnabled(False);self.stop_audio();task=Task(fn,self);self.task=task
        def finish(result=None,error=None):
            self.centralWidget().setEnabled(True);self.progress.setRange(0,1);self.progress.setValue(1);self.task=None
            if error:self.error(error)
            else:done(result)
            # Cleanup is tied to thread completion below.
        task.done.connect(lambda v:finish(v));task.failed.connect(lambda e:finish(error=e));task.finished.connect(task.deleteLater);task.start()
    def discard_ok(self):
        if not self.dirty:return True
        return QMessageBox.question(self,'Unsaved project','Discard the unsaved project changes?',QMessageBox.StandardButton.Discard|QMessageBox.StandardButton.Cancel)==QMessageBox.StandardButton.Discard
    def open_folder(self):
        if not self.discard_ok():return
        folder=QFileDialog.getExistingDirectory(self,'Choose WAV / MIDI source folder')
        if folder:self.load_folder(folder)
    def load_folder(self,folder):
        self.run_task(lambda:authoring.new_project(folder),self.set_project,'Scanning and validating source files…')
    def set_project(self,p):
        self.project=p;self.project_file=None
        for widget,key in [(self.name,'name'),(self.creator,'creator'),(self.packid,'id'),(self.version,'version')]:widget.setText(p[key])
        self.midi_mode.setCurrentIndex(['auto','native','gm'].index(p.get('midi_mode','auto')));self.distribution.setCurrentIndex(0 if p['private']else 1);self.rights.setPlainText(p['rights']);self.silence.setChecked(p.get('allow_silence',False));self.dirty=True;self.refresh()
    def refresh(self):
        p=self.project;has=bool(p)
        for b in self.action_buttons[1:]:b.setEnabled(has)
        if not p:return
        self.source_title.setText('▣  '+Path(p['source_root']).name);self.source_label.setText(p['source_root']);self.project_title.setText(p['name']);unmapped=0
        for r,(view,lab,sel)in enumerate(self.cards):
            pool=[f for f in p['files']if f['kind']==0 and f['role']==r];sel.setText(f'{len(pool)} files');lab.setText(Path(pool[0]['path']).name if pool else 'Missing · silence only if allowed')
            if pool:
                try:view.set_audio(authoring.safe_file(p['source_root'],pool[0]['path']).read_bytes())
                except Exception:pass
        unmapped=sum(f['role']is None for f in p['files']);excluded=len(p.get('issues',[]))-unmapped;wav=sum(f['kind']==0 for f in p['files']);midi=len(p['files'])-wav
        self.issues.setText(f'{wav} WAV · {midi} MIDI · {unmapped} need mapping · {max(0,excluded)} excluded / invalid. Review Mapping shows details.')
        self.kits.blockSignals(True);current=self.kits.currentRow();self.kits.clear()
        for i,k in enumerate(p['kits']):
            item=QListWidgetItem(f'{i+1:02d}   {k["name"]}'+('  🔒'if k.get('locked')else''));self.kits.addItem(item)
        self.kits.setCurrentRow(max(0,min(current,len(p['kits'])-1)));self.kits.blockSignals(False);self.show_kit(self.kits.currentRow());self.status.setText('Ready · map files, generate kits, audition, then export')
    def choose_role(self,r):self.selected_role=r;self.review_mapping(role=r)
    def review_mapping(self,checked=False,role=None):
        if not self.project:return
        dlg=QDialog(self);dlg.setWindowTitle('Review source mapping');dlg.resize(880,650);layout=QVBoxLayout(dlg);layout.addWidget(QLabel('Folder roles take priority. Change any uncertain mapping before export.'))
        table=QTableWidget();table.setColumnCount(2);table.setHorizontalHeaderLabels(['Source file','Instrument / MIDI type']);files=self.project['files'];table.setRowCount(len(files));mapping=[]
        from PySide6.QtWidgets import QTableWidgetItem
        for row,f in enumerate(files):
            item=QTableWidgetItem(f['path']);item.setFlags(item.flags()&~Qt.ItemFlag.ItemIsEditable);table.setItem(row,0,item);combo=QComboBox()
            if f['kind']==0:combo.addItems(['Unmapped']+list(authoring.ROLES));combo.setCurrentIndex(0 if f['role']is None else f['role']+1)
            else:combo.addItems(['Beat MIDI','Groove MIDI']);combo.setCurrentIndex(f['kind']-3)
            table.setCellWidget(row,1,combo);mapping.append(combo)
        table.horizontalHeader().setSectionResizeMode(0,QHeaderView.ResizeMode.Stretch);table.setColumnWidth(1,200);layout.addWidget(table,1)
        issuebox=QPlainTextEdit('\n'.join(self.project.get('issues',[]))or'All scanned files are valid.');issuebox.setReadOnly(True);issuebox.setMaximumHeight(100);layout.addWidget(issuebox);layout.addWidget(QLabel('Excluded invalid files are not packaged. Rescan by reopening the folder after replacing files.'))
        boxes=QDialogButtonBox(QDialogButtonBox.StandardButton.Save|QDialogButtonBox.StandardButton.Cancel);boxes.accepted.connect(dlg.accept);boxes.rejected.connect(dlg.reject);layout.addWidget(boxes)
        if dlg.exec():
            changed=False
            for f,c in zip(files,mapping):
                if f['kind']==0:
                    new=c.currentIndex()-1;new=None if new<0 else new;changed|=new!=f['role'];f['role']=new
                else:f['kind']=c.currentIndex()+3
            if changed:self.project['kits']=[]
            self.dirty=True;self.refresh()
    def generate_kits(self):
        if not self.project:return
        self.metadata()
        try:authoring.generate(self.project,self.count.value(),random.randrange(2**32));self.dirty=True;self.refresh()
        except Exception as e:self.error(str(e))
    def current_kit(self):
        row=self.kits.currentRow()
        return self.project['kits'][row]if self.project and 0<=row<len(self.project['kits'])else None
    def show_kit(self,index):
        k=self.current_kit();self.kitlock.blockSignals(True);self.kitlock.setChecked(bool(k and k.get('locked')));self.kitlock.blockSignals(False)
        for r,(combo,lock,level)in enumerate(self.parts):
            for w in (combo,lock,level):w.blockSignals(True);w.setEnabled(bool(k))
            combo.clear();combo.addItem('Generated silence',None)
            if self.project:
                for f in self.project['files']:
                    if f['kind']==0 and f['role']==r:combo.addItem(Path(f['path']).name,f['path'])
            if k:combo.setCurrentIndex(max(0,combo.findData(k['samples'][r])));lock.setChecked(k['locks'][r]);level.setValue(k['levels'][r])
            for w in (combo,lock,level):w.blockSignals(False)
    def change_part(self,r):
        k=self.current_kit()
        if k:
            k['samples'][r]=self.parts[r][0].currentData();self.dirty=True;self.show_wave(k['samples'][r])
    def lock_part(self,r,value):
        k=self.current_kit()
        if k:k['locks'][r]=value;self.dirty=True
    def change_level(self,r,value):
        k=self.current_kit()
        if k:k['levels'][r]=value;self.dirty=True
    def lock_kit(self,value):
        k=self.current_kit()
        if k:k['locked']=value;self.dirty=True;self.refresh()
    def rename_current(self):
        from PySide6.QtWidgets import QInputDialog
        k=self.current_kit()
        if k:
            name,ok=QInputDialog.getText(self,'Rename kit','Kit name',text=k['name'])
            if ok and name.strip():k['name']=name.strip();self.dirty=True;self.refresh()
    def rename_kit(self,*args):pass
    def show_wave(self,path):
        if path:
            try:
                data=authoring.safe_file(self.project['source_root'],path).read_bytes();self.wave.set_audio(data);info=authoring.wave_info(data);self.file_label.setText(f'{Path(path).name}  ·  {info["rate"]/1000:g} kHz  ·  {info["width"]*8}-bit  ·  {info["frames"]/info["rate"]:.2f} s')
            except Exception as e:self.error(str(e))
    def play_bytes(self,data):
        self.player.stop();target=Path(self.preview_temp.name)/'audition.wav';target.write_bytes(data);self.player.setSource(QUrl.fromLocalFile(str(target)));self.player.play();self.status.setText('Audition playing')
    def stop_audio(self):self.player.stop()
    def audition_role(self,r):
        if not self.project:return
        k=self.current_kit();path=k['samples'][r]if k else next((f['path']for f in self.project['files']if f['kind']==0 and f['role']==r),None)
        if path:
            self.show_wave(path);self.play_bytes(authoring.safe_file(self.project['source_root'],path).read_bytes())
    def audition_kit(self):
        if not self.current_kit():self.error('Generate a kit first');return
        self.metadata();p=copy.deepcopy(self.project);row=self.kits.currentRow();bpm=self.bpm.value();swing=self.swing.value();self.run_task(lambda:authoring.render_preview(p,row,bpm,swing),self.play_bytes,'Rendering a two-bar kit audition…')
    def export_pack(self):
        if not self.project:return
        self.metadata()
        try:authoring.validate_project(self.project)
        except Exception as e:self.error(str(e));return
        if not self.project['private'] and QMessageBox.question(self,'Redistributable pack','Confirm that you have permission to redistribute every included WAV and MIDI file. This declaration is stored with the pack.')!=QMessageBox.StandardButton.Yes:return
        target,_=QFileDialog.getSaveFileName(self,'Export expansion',f'groove-dice-{self.project["id"]}-{self.project["version"]}.gdexp','Groove Dice expansion (*.gdexp)')
        if target:
            p=copy.deepcopy(self.project);self.run_task(lambda:authoring.export(p,target),lambda report:self.status.setText(f'Exported {Path(target).name} · {report["kits"]} kits · {report["wav"]} WAV · {report["midi"]} MIDI'),'Writing and verifying expansion…')
    def save(self):
        if not self.project:return
        self.metadata();target=self.project_file
        if not target:target,_=QFileDialog.getSaveFileName(self,'Save project',self.name.text()+'.gdproject','Pack Builder project (*.gdproject)')
        if target:
            try:authoring.save_project(self.project,target);self.project_file=target;self.dirty=False;self.status.setText('Project saved · source files remain in their original folder')
            except Exception as e:self.error(str(e))
    def open_project(self):
        if not self.discard_ok():return
        target,_=QFileDialog.getOpenFileName(self,'Open project','','Pack Builder project (*.gdproject)')
        if target:
            try:self.set_project(authoring.load_project(target));self.project_file=target;self.dirty=False
            except Exception as e:self.error(str(e))
    def manage_packs(self):
        content=QFileDialog.getExistingDirectory(self,'Choose the installed Groove Dice Content folder')
        if not content:return
        content=Path(content)
        if not (content/'Banks/core.id').is_file():self.error('Choose Content inside the installed Groove Dice plugin folder');return
        dlg=QDialog(self);dlg.setWindowTitle('Installed expansions');dlg.resize(600,430);layout=QVBoxLayout(dlg);layout.addWidget(QLabel(str(content)));listing=QListWidget();layout.addWidget(listing);layout.addWidget(QLabel('Reopen Groove Dice after installing or removing packs. Core is retained.'))
        def refresh():
            listing.clear()
            for path in sorted((content/'Expansions').glob('*/manifest.json')):
                m=expansions.validate(json.loads(path.read_text()));item=QListWidgetItem(m['name']+'  ·  '+m['version']+('  PRIVATE'if not m['redistributable']else''));item.setData(Qt.ItemDataRole.UserRole,m['id']);listing.addItem(item)
        def install():
            archive,_=QFileDialog.getOpenFileName(dlg,'Install expansion','','Groove Dice expansion (*.gdexp)')
            if not archive:return
            try:
                import zipfile
                with zipfile.ZipFile(archive)as z:m=expansions.validate(json.loads(z.read('manifest.json')))
                allow=False
                if not m['redistributable']:
                    allow=QMessageBox.question(dlg,'Personal-only pack','Install this private pack for your own use? Its redistribution is not cleared.')==QMessageBox.StandardButton.Yes
                    if not allow:return
                expansions.install(Path(archive),content,allow);refresh()
            except Exception as e:self.error(str(e))
        def remove():
            item=listing.currentItem()
            if item and QMessageBox.question(dlg,'Remove expansion','Remove '+item.text()+'? Saved songs may report missing sounds. The source .gdexp is kept.')==QMessageBox.StandardButton.Yes:
                try:expansions.remove(item.data(Qt.ItemDataRole.UserRole),content);refresh()
                except Exception as e:self.error(str(e))
        row=QHBoxLayout();row.addWidget(button('Install .gdexp',install));row.addWidget(button('Remove selected',remove));row.addWidget(button('Done',dlg.accept));layout.addLayout(row)
        try:refresh();dlg.exec()
        except Exception as e:self.error(str(e))
    def closeEvent(self,event):
        if self.task:event.ignore();return
        if not self.discard_ok():event.ignore();return
        self.player.stop();self.preview_temp.cleanup();event.accept()

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--project',type=Path);parser.add_argument('--smoke-test',action='store_true');args=parser.parse_args()
    app=QApplication(sys.argv[:1]);app.setApplicationName('Groove Dice Pack Builder');app.setOrganizationName('kimswork');app.setStyle('Fusion');app.setStyleSheet(STYLE);w=Window()
    if args.project:w.set_project(authoring.load_project(args.project));w.dirty=False
    w.show()
    if args.smoke_test:
        def finish():
            assert w.isVisible() and len(w.cards)==8 and len(w.parts)==8
            w.dirty=False;w.close();app.quit()
        QTimer.singleShot(350,finish)
    sys.exit(app.exec())
if __name__=='__main__':main()
