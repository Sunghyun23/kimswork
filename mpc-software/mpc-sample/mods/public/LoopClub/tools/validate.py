#!/usr/bin/env python3
"""Local candidate validation. No SD/device install or reboot."""
import ast
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
from datetime import datetime,timezone

PROJECT=Path(__file__).resolve().parents[1]

def main():
    logs=PROJECT/'build/validation';logs.mkdir(parents=True,exist_ok=True)
    commands=[('format',['tools/run.py','fmt','--check']),('clippy',['tools/run.py','clippy','--all-targets','--','-D','warnings']),('tests',['tools/run.py','test','--','--test-threads=1']),('preview',['tools/run.py','run','--example','preview']),('render',['tools/render_preview.py','build/previews']),('benchmark',['tools/run.py','run','--release','--example','bench']),('package',['tools/package.py']),('launcher',['tools/check_package.py']),('installers',['tools/check_installers.py'])]
    report={'time_utc':datetime.now(timezone.utc).isoformat(),'product':'Loop Club 0.1.0','scope':'local host and ARM64 cross-build; not hardware validation','checks':{},'device_installed':False,'sd_changed':False,'physical_audio_controls_cpu_boot_verified':False}
    for name,args in commands:
        result=subprocess.run([sys.executable,*args],cwd=PROJECT,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
        (logs/(name+'.log')).write_text(result.stdout)
        print(f'{name}: {"PASS" if result.returncode==0 else "FAIL"}',flush=True)
        if result.returncode:
            print(result.stdout[-6000:]);return result.returncode
        report['checks'][name]='PASS'
        if name=='tests':report['native_tests_passed']=sum(int(n)for n in re.findall(r'test result: ok\. (\d+) passed',result.stdout))
        if name=='benchmark':report['native_benchmark']=json.loads(next(line for line in result.stdout.splitlines()if line.startswith('{"results"')))
        if name=='launcher':report['launcher_host_fixture']=json.loads(result.stdout)
    for source in (PROJECT/'tools').glob('*.py'):ast.parse(source.read_text(),filename=str(source))
    archive=PROJECT/'dist/loop-club-0.1.0-sample-development.zip';report['package_sha256']=hashlib.sha256(archive.read_bytes()).hexdigest()
    report['source_hashes']={str(p.relative_to(PROJECT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in sorted([*(PROJECT/'src').glob('*.rs'),*(PROJECT/'tests').glob('*.rs'),*(PROJECT/'examples').glob('*.rs'),*(PROJECT/'tools').glob('*.py'),PROJECT/'Cargo.toml',PROJECT/'Cargo.lock'])}
    (PROJECT/'docs/validation.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:report[k]for k in ('native_tests_passed','package_sha256','device_installed','sd_changed')},indent=2))
    return 0

if __name__=='__main__':raise SystemExit(main())
