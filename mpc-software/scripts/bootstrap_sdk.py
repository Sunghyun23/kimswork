#!/usr/bin/env python3
"""Install the shared, commit-pinned SDK. Python standard library only."""
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def run(*args, cwd=None):
    return subprocess.check_output(args, cwd=cwd, text=True).strip()


def main():
    lock = json.loads((ROOT / 'third_party/sdk.lock.json').read_text())
    target = ROOT / lock['path']
    if not target.exists():
        subprocess.run(['git', 'clone', '--no-checkout', lock['repository'], str(target)], check=True)
        subprocess.run(['git', 'checkout', '--detach', lock['commit']], cwd=target, check=True)
    else:
        actual = run('git', 'rev-parse', 'HEAD', cwd=target)
        if actual != lock['commit']:
            raise SystemExit(f'SDK mismatch: {actual}; expected {lock["commit"]}. Preserve local work before updating.')
        if run('git', 'status', '--porcelain', '--untracked-files=no', cwd=target):
            raise SystemExit('SDK has tracked local modifications; refusing to use a modified dependency.')
    print(f'SDK ready: {target} @ {lock["commit"]}')


if __name__ == '__main__':
    main()
