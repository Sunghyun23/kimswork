"""Select the repository-local Sample toolchain without changing global defaults."""
import json
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REPOSITORY = ROOT.parent


def environment():
    env = os.environ.copy()
    local = REPOSITORY / '.sdk-tools/mpc-sample'
    lock = json.loads((ROOT / 'third_party/sample-sdk.lock.json').read_text())
    toolchain = local / 'rustup/toolchains' / (lock.get('rust_toolchain', '1.97.0') + '-aarch64-apple-darwin')
    paths = []
    if (toolchain / 'bin/rustc').is_file():
        paths.append(str(toolchain / 'bin'))
        env['RUSTUP_HOME'] = str(local / 'rustup')
        env['CARGO_HOME'] = str(local / 'cargo')
        llvm = toolchain / 'lib/rustlib/aarch64-apple-darwin/bin'
        if llvm.is_dir():
            paths.append(str(llvm))
    # Reuse the already installed official-SDK compiler executable, not its
    # headers, runtime or build rules. This leaves the existing checkout intact.
    zigs = sorted((ROOT / 'third_party/mpc-plugin-builder-sdk/.sdk-tools/zigenv/lib').glob('python*/site-packages/ziglang/zig'))
    if zigs:
        env.setdefault('ZIG', str(zigs[-1]))
        paths.append(str(zigs[-1].parent))
    env['PATH'] = os.pathsep.join(paths + [env.get('PATH', '')])
    env['CARGO_TARGET_DIR'] = str(local / 'target')
    return env
