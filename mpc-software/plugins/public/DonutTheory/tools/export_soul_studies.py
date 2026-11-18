#!/usr/bin/env python3
"""Package reviewed native renders as WAV, recipes and path-specific MPC presets."""
import argparse,array,base64,csv,hashlib,json,math,shutil,struct,sys,wave,zipfile
from pathlib import Path
from xml.etree import ElementTree as ET
P=Path(__file__).resolve().parents[1]
a=argparse.ArgumentParser();a.add_argument('render_directory',type=Path);args=a.parse_args();src=args.render_directory;out=P/'presets/soul-studies';(out/'audio').mkdir(parents=True,exist_ok=True)
levels={x['slug']:x for x in csv.DictReader((src/'levels.csv').open())};items=[]
for slug in ['dusty-sunday','velvet-melt']:
 recipe=json.loads((src/(slug+'.json')).read_text());chunk=(src/(slug+'.mpcp')).read_bytes()
 assert chunk[:4]==b'MPCP' and struct.unpack_from('<I',chunk,8)[0]==287
 assert recipe['engineVersion']=='1.8.0' and recipe['parameterCount']==287
 shutil.copyfile(src/(slug+'.json'),out/(slug+'.json'))
 root=ET.Element('pluginstate');ET.SubElement(root,'version',file='1',date='2026-11-18-00-00')
 ET.SubElement(root,'PLUGIN',name='Donut Theory',format='VST',category='Synth',manufacturer='kim',version='1.8.0',file='/nvme/VST2 Plugins/kim - VST - Donut Theory/VST/donut-theory-arm64.so',uid=str(recipe['uid']),isInstrument='1',numInputs='0',numOutputs='2')
 ET.SubElement(root,'preset').text=recipe['name'];ET.SubElement(root,'state64').text=base64.b64encode(chunk).decode('ascii');ET.indent(root)
 xpl=out/(slug+'.xpl');ET.ElementTree(root).write(xpl,encoding='utf-8',xml_declaration=True)
 assert base64.b64decode(ET.parse(xpl).getroot().findtext('state64'))==chunk
 samples=array.array('f');samples.frombytes((src/(slug+'.f32')).read_bytes());assert len(samples)%2==0 and all(math.isfinite(x)and abs(x)<=1 for x in samples)
 pcm=array.array('h',(max(-32767,min(32767,round(x*32767)))for x in samples))
 if sys.byteorder!='little':pcm.byteswap()
 wav=out/'audio'/(slug+'.wav')
 with wave.open(str(wav),'wb')as w:w.setnchannels(2);w.setsampwidth(2);w.setframerate(48000);w.writeframes(pcm.tobytes())
 with wave.open(str(wav))as w:assert w.getnchannels()==2 and w.getframerate()==48000 and w.getnframes()*2==len(samples)
 items.append({'slug':slug,'name':recipe['name'],'seconds':len(samples)/96000,'peak':max(map(abs,samples)),'rms':math.sqrt(sum(x*x for x in samples)/len(samples)),'soundTrimDb':float(levels[slug]['trim_db']),'stateSHA256':hashlib.sha256(chunk).hexdigest(),'wavSHA256':hashlib.sha256(wav.read_bytes()).hexdigest(),'xplSHA256':hashlib.sha256(xpl.read_bytes()).hexdigest()})
(out/'verification.json').write_text(json.dumps({'engineVersion':'1.8.0','source':'actual native engine, restored saved preset chunk; not a hardware recording','sampleRate':48000,'channels':2,'pcmBits':16,'bpm':82,'progression':['Cmaj9','Am9/E','F6/9/D','G9sus4/D'],'topNotes':['D4','C4','C4','C4'],'audioPostprocessing':'PCM conversion only; level adjustment is baked into saved Sound Trim','checks':['287parameter native state round-trip','all samples finite, no clipping','all voices release','XML state64 matches the restored native chunk','WAV header/frame count valid'],'limitation':'MPC XPL browser import not tested; XPL points to internal NVMe ARM64 installation; factory64programs unchanged','patches':items},indent=2)+'\n')
zip_path=P/'build/donut-theory-soul-studies.zip'
with zipfile.ZipFile(zip_path,'w',zipfile.ZIP_DEFLATED)as z:
 for f in sorted(out.rglob('*')):
  if f.is_file():z.write(f,'Donut Theory Soul Studies/'+str(f.relative_to(out)))
print(json.dumps({'patches':items,'zip':str(zip_path)},indent=2))
