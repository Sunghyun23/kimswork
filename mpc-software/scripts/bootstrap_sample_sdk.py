#!/usr/bin/env python3
"""Prepare exact MPC Sample dependencies on this computer; never access a device."""
import argparse
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def git(*args, cwd=None):
    return subprocess.check_output(['git', *args], cwd=cwd, text=True).strip()


def prepare(dependency):
    target = ROOT / dependency['path']
    if not target.exists():
        subprocess.run(['git', 'clone', '--no-checkout', dependency['repository'], str(target)], check=True)
        subprocess.run(['git', 'checkout', '--detach', dependency['commit']], cwd=target, check=True)
    actual = git('rev-parse', 'HEAD', cwd=target)
    if actual != dependency['commit']:
        raise SystemExit(f'{dependency["id"]}: commit mismatch; preserve the existing checkout before updating')
    if git('status', '--porcelain', '--untracked-files=no', cwd=target):
        raise SystemExit(f'{dependency["id"]}: tracked modifications; refusing to use a modified dependency')
    if git('rev-parse', dependency['tag'] + '^{commit}', cwd=target) != actual:
        raise SystemExit(f'{dependency["id"]}: tag does not match the locked commit')
    if not (target / 'LICENSE').is_file():
        raise SystemExit(f'{dependency["id"]}: missing upstream license')
    print(f'{dependency["id"]} ready: {target} @ {actual}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--with-sampleboot', action='store_true', help='Also prepare the separate boot selector source')
    args = parser.parse_args()
    lock = json.loads((ROOT / 'third_party/sample-sdk.lock.json').read_text())
    for dependency in lock['dependencies']:
        if dependency['id'] == 'sampleboot' and not args.with_sampleboot:
            continue
        prepare(dependency)
    print('Local source preparation only: no installation, firmware change or SD access.')


if __name__ == '__main__':
    main()
