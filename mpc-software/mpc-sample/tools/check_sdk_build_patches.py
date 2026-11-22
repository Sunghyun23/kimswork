#!/usr/bin/env python3
"""Validate reviewed build-only SDK patches in an ephemeral source copy."""
import argparse,json,re,subprocess
from pathlib import Path
from development_environment import ROOT,REPOSITORY,environment
from patched_sdk import build_manifest

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--project',type=Path,required=True);a=p.parse_args()
    lock=json.loads((ROOT/'third_party/sample-sdk.lock.json').read_text());dep=next(x for x in lock['dependencies']if x['id']=='mpc-sample-sdk')
    output=REPOSITORY/'.sdk-tools/mpc-sample/validation/build-patches'/dep['build_patches'][0]['sha256'][:12];output.mkdir(parents=True,exist_ok=True)
    report={'upstream_commit':dep['commit'],'patches':dep['build_patches'],'canonical_sdk_modified':False,'checks':{}}
    env=environment();env['CARGO_TARGET_DIR']=str(REPOSITORY/'.sdk-tools/mpc-sample/validation/patched-target')
    with build_manifest(a.project.resolve())as(_,sdk):
        for name,args in [('format',['fmt','--check']),('tests',['test']),('clippy',['clippy','--all-targets','--','-D','warnings']),('arm64',['check','--target','aarch64-unknown-linux-gnu'])]:
            result=subprocess.run(['cargo',args[0],'--manifest-path',str(sdk/'Cargo.toml'),*args[1:]],env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
            (output/(name+'.log')).write_text(result.stdout);report['checks'][name]=result.returncode
            if name=='tests':report['tests_passed']=sum(map(int,re.findall(r'test result: ok\. (\d+) passed',result.stdout)))
            print(name,result.returncode,flush=True)
            if result.returncode:print(result.stdout[-3000:]);return result.returncode
    (output/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2));return 0
if __name__=='__main__':raise SystemExit(main())
