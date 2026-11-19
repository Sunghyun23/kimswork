#!/usr/bin/env python3
"""Exercise actual generated installers against temporary files on Linux.
MPC_INSTALL_TEST=1 replaces device/service access; never run against an MPC.
"""
import argparse,hashlib,json,os,subprocess,sys,tempfile,zipfile
from pathlib import Path
NAME='kim - VST - Groove Dice'
def digest_tree(root):return {p.relative_to(root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest()for p in root.rglob('*')if p.is_file()}
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--old',type=Path,required=True);ap.add_argument('--new',type=Path,required=True);ap.add_argument('--core',type=Path,required=True);a=ap.parse_args();assert sys.platform=='linux','Run in isolated Linux; no device needed.'
 results=[]
 for cpu in ('armv7','aarch64'):
  with tempfile.TemporaryDirectory(prefix='gd-upgrade-')as t:
   work=Path(t);target=work/'device/VST2 Plugins';settings=work/'MPC.settings';settings.write_text('<PROPERTIES>\n<VALUE name="pluginList-arm"><KNOWNPLUGINS/></VALUE>\n</PROPERTIES>\n');env=dict(os.environ,MPC_INSTALL_TEST='1',MPC_SETTINGS=str(settings),MPC_LEGACY_ROOT=str(work/'legacy'),MPC_TEST_LOG=str(work/'services.log'))
   packages={}
   for label,folder in [('old',a.old),('new',a.new)]:
    matches=list(folder.glob(f'groove-dice-*-mpc-{cpu}.zip'));assert len(matches)==1,(label,matches)
    with zipfile.ZipFile(matches[0])as z:
     assert all(not n.startswith('/')and'..'not in n.split('/')for n in z.namelist());z.extractall(work/label)
    packages[label]=next((work/label).glob('*/install.sh')).parent
   def run(label,script='install.sh'):
    cmd=['/bin/sh',str(packages[label]/script),'-y','-t',str(target)];r=subprocess.run(cmd,env=env,cwd=packages[label],capture_output=True,text=True);assert r.returncode==0,(label,script,r.stdout,r.stderr)
   run('old');exp=target/NAME/'Content/Expansions';pack=exp/'qa-original-pack';pack.mkdir(parents=True)
   payload=a.core.read_bytes();(pack/'bank.gdbank').write_bytes(payload);(pack/'manifest.json').write_text(json.dumps({'format':'GDEXP001','id':'qa-original-pack','name':'QA Original Pack','version':'1.0.0','redistributable':True,'sha256':hashlib.sha256(payload).hexdigest(),'bytes':len(payload)}));(pack/'RIGHTS.txt').write_text('Original procedural fixture; no private samples.\n');(pack/'nested').mkdir();(pack/'nested/sentinel.txt').write_text('keep nested user files\n');(exp/'registry.tsv').write_text('qa-original-pack\tQA Original Pack\t1\n');expected=digest_tree(exp)
   run('old');assert not(pack/'bank.gdbank').exists(),'baseline must reproduce loss';run('old')
   for rel,sha in expected.items():
    dest=exp/rel;dest.parent.mkdir(parents=True,exist_ok=True)
    if rel.endswith('bank.gdbank'):dest.write_bytes(payload)
    elif rel=='registry.tsv':dest.write_text('qa-original-pack\tQA Original Pack\t1\n')
    elif rel.endswith('manifest.json'):dest.write_text(json.dumps({'format':'GDEXP001','id':'qa-original-pack','name':'QA Original Pack','version':'1.0.0','redistributable':True,'sha256':hashlib.sha256(payload).hexdigest(),'bytes':len(payload)}))
    elif rel.endswith('RIGHTS.txt'):dest.write_text('Original procedural fixture; no private samples.\n')
    else:dest.write_text('keep nested user files\n')
   assert digest_tree(exp)==expected
   for _ in range(2):run('new');assert digest_tree(exp)==expected,'new upgrade/reinstall must retain pack and registry byte-for-byte'
   assert json.loads((target/NAME/'plugin.json').read_text())['version']=='1.6.2'
   run('new','uninstall.sh');assert digest_tree(exp)==expected,'uninstall must retain declared user data';assert not(target/NAME/f'VST/groove-dice-{"arm32"if cpu=="armv7"else"arm64"}.so').exists()
   results.append({'arch':cpu,'old161LosesPack':True,'new162UpgradePreservesAllUserBytes':True,'repeatInstallPreserves':True,'uninstallPreservesPacks':True,'retainedFiles':len(expected)})
 print(json.dumps({'pass':True,'mode':'Linux isolated fake device; actual unmodified generated installers','results':results},indent=2))
if __name__=='__main__':main()
