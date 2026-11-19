#!/usr/bin/env python3
"""Produce a public-safe plugin ZIP and a separate personal runtime folder.
SDK install text is reused without modifying the shared SDK. No network publishing.
"""
from pathlib import Path
import argparse,importlib.util,json,zipfile,sys
P=Path(__file__).resolve().parents[1];sys.path.insert(0,str(P/'tools'));import expansions
SDK=P.parents[2]/'third_party/mpc-plugin-builder-sdk'
spec=importlib.util.spec_from_file_location('mpc_sdk_package',SDK/'tools/package.py');sdk=importlib.util.module_from_spec(spec);spec.loader.exec_module(sdk)
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--personal',action='store_true',help='Also assemble local private expansions');args=ap.parse_args()
 base=P/'build/kim - VST - Groove Dice';meta=json.loads((base/'plugin.json').read_text());version=meta['version'];root=P/'build/profiles'/(version+'-monorepo');root.mkdir(parents=True,exist_ok=True)
 public=root/'public'/base.name;personal=root/'personal'/base.name
 for path in [public,personal]:path.parent.mkdir(parents=True,exist_ok=True)
 expansions.assemble(base,public,None,False)
 actual=set()
 if args.personal:
  extras=P.parents[2]/'expansions/groove-dice/private/splat-to-clat/work/personal'
  expansions.assemble(base,personal,extras,True)
  expected={f.parent.name for f in(extras/'Content/Expansions').glob('*/manifest.json')}
  actual={f.parent.name for f in(personal/'Content/Expansions').glob('*/manifest.json')}
  assert expected==actual and 'splat-to-clat-private'in actual, 'Personal profile omitted installed expansions'
 dist=P/'build/dist';dist.mkdir(exist_ok=True);target=dist/f'groove-dice-{version}-mpc.zip';tmp=target.with_suffix('.partial')
 with zipfile.ZipFile(tmp,'w',zipfile.ZIP_DEFLATED,compresslevel=6,allowZip64=True)as z:
  for f in sorted(public.rglob('*')):
   if f.is_file():z.write(f,str(f.relative_to(public.parent)))
  z.write(P/'LICENSE','LICENSE');z.write(P/'PACK_BUILDER.md','PACK_BUILDER.md');z.writestr('INSTALL.txt',sdk.install_text(meta,base.name));z.write(P/'RELEASE_README.md','README.md');z.write(P.parents[2]/'docs/plugins/groove-dice/manual.md','MANUAL.md');z.write(P/'VALIDATION.md','VERIFICATION.md');z.write(P.parents[2]/'shared/groove-dice-expansion/expansions.py','Expansion Manager/expansions.py')
  z.writestr('Expansion Manager/README.txt','Groove Dice .gdexp installer (Python3; no engine rebuild required).\nInstall: python3 expansions.py install PACK.gdexp --content "/path/to/kim - VST - Groove Dice/Content"\nList: python3 expansions.py list --content CONTENT\nRemove: python3 expansions.py remove PACK-ID --content CONTENT\nReopen the plugin after changing packs. Core cannot be removed through this tool.\nOnly install packs you have rights to use. Private packs require --allow-private and are excluded from default public assembly.\n')
 tmp.replace(target)
 # Audit actual ZIP, not just staging manifests.
 with zipfile.ZipFile(target)as z:
  bank_paths=[n for n in z.namelist()if n.endswith('.gdbank')];assert bank_paths==[base.name+'/Content/Banks/default.gdbank'];assert not any('splat' in n.lower()for n in z.namelist());assert z.read(base.name+'/Content/Banks/default.gdbank')==(P/'Content/Banks/default.gdbank').read_bytes()
 report=dict(version=version,public_zip=str(target.resolve()),public_zip_bytes=target.stat().st_size,public_banks=bank_paths,private_payloads_in_public=0,personal_folder=str(personal.resolve()) if args.personal else None,bank_payloads_are_separate=True,personal_expansions=sorted(actual))
 (dist/f'groove-dice-{version}-profiles.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
if __name__=='__main__':main()
