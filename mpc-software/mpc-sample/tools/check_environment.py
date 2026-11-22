#!/usr/bin/env python3
"""Report local Sample development prerequisites without installing anything."""
import json
import re
import shutil
import subprocess
from development_environment import environment
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    lock = json.loads((ROOT / 'third_party/sample-sdk.lock.json').read_text())
    required = tuple(map(int, lock['rust_minimum'].split('.')))
    env = environment()
    tools = {name: shutil.which(name, path=env['PATH']) for name in ['rustc', 'cargo', 'rustup', 'uv', 'zig', 'python3', 'zip']}
    # The SDK also supports Homebrew LLVM at this documented location.
    for name in ['llvm-nm', 'llvm-objcopy', 'llvm-objdump']:
        fallback = Path('/opt/homebrew/opt/llvm/bin') / name
        tools[name] = shutil.which(name, path=env['PATH']) or (str(fallback) if fallback.is_file() else None)
    version = None
    rust_ok = False
    if tools['rustc']:
        result = subprocess.run([tools['rustc'], '--version'], capture_output=True, text=True, timeout=10, env=env)
        version = result.stdout.strip()
        match = re.search(r'rustc (\d+)\.(\d+)', version)
        rust_ok = bool(match and tuple(map(int, match.groups())) >= required)
    missing = [name for name, path in tools.items() if not path]
    ready = rust_ok and not missing
    print(json.dumps({'rust': version, 'rust_minimum': lock['rust_minimum'], 'rust_supported': rust_ok, 'tools': tools, 'missing': missing, 'local_build_prerequisites_ready': ready, 'device_validation': 'not performed'}, indent=2))
    return 0 if ready else 1


if __name__ == '__main__':
    raise SystemExit(main())
