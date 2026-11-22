#!/usr/bin/env python3
"""Check the pinned Sample SDK in a temporary copy; no hardware access."""
import json
import shutil
import subprocess
import tempfile
from pathlib import Path
from development_environment import ROOT, REPOSITORY, environment


def main():
    lock = json.loads((ROOT / 'third_party/sample-sdk.lock.json').read_text())
    dependency = next(x for x in lock['dependencies'] if x['id'] == 'mpc-sample-sdk')
    source = ROOT / dependency['path']
    commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=source, text=True).strip()
    dirty = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=no'], cwd=source, text=True).strip()
    if commit != dependency['commit'] or dirty:
        raise SystemExit('SDK mismatch or local modifications; run/review bootstrap_sample_sdk.py first')
    output = REPOSITORY / '.sdk-tools/mpc-sample/validation/pinned'
    output.mkdir(parents=True, exist_ok=True)
    checks = []
    with tempfile.TemporaryDirectory(prefix='sdk-check-', dir=output) as temporary:
        scratch = Path(temporary) / 'sdk'
        shutil.copytree(source, scratch, ignore=shutil.ignore_patterns('.git', 'target'))
        manifest = str(scratch / 'Cargo.toml')
        commands = [
            ('host-tests', ['cargo', 'test', '--manifest-path', manifest]),
            ('clippy', ['cargo', 'clippy', '--manifest-path', manifest, '--all-targets', '--', '-D', 'warnings']),
            ('arm64-check', ['cargo', 'check', '--manifest-path', manifest, '--target', 'aarch64-unknown-linux-gnu']),
        ]
        for name, command in commands:
            log = output / (name + '.log')
            with log.open('w') as stream:
                result = subprocess.run(command, env=environment(), cwd=scratch, stdout=stream, stderr=subprocess.STDOUT)
            checks.append({'check': name, 'returncode': result.returncode, 'log': log.name})
            print(f'{name}: {"PASS" if result.returncode == 0 else "FAIL"}', flush=True)
    report = {'sdk_commit': commit, 'checks': checks, 'device_tested': False}
    (output / 'results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Local evidence: {output}')
    return int(any(x['returncode'] for x in checks))


if __name__ == '__main__':
    raise SystemExit(main())
