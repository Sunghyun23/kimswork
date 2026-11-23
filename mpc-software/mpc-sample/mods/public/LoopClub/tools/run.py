#!/usr/bin/env python3
"""Run Cargo using the isolated MPC Sample toolchain; never contacts a device."""
from pathlib import Path
import subprocess
import sys

PROJECT = Path(__file__).resolve().parents[1]
SAMPLE = PROJECT.parents[2]
sys.path.insert(0, str(SAMPLE / "tools"))
from development_environment import environment
from patched_sdk import build_manifest

def main():
    env = environment()
    # Ephemeral SDK staging changes crate identities; incremental caches cannot be reused reliably.
    # Keep the default bounded; an explicit caller override is still respected.
    env.setdefault("CARGO_INCREMENTAL", "0")
    import json
    patch_lock=json.loads((PROJECT.parents[3]/'third_party/sample-sdk.lock.json').read_text())
    env['LOOP_CLUB_SDK_PATCH']=next(d for d in patch_lock['dependencies']if d['id']=='mpc-sample-sdk')['build_patches'][0]['sha256']
    env["CARGO_TARGET_DIR"] = str(PROJECT / "build/target")
    args = sys.argv[1:] or ["test", "--", "--test-threads=1"]
    with build_manifest(PROJECT) as (manifest,_):
        return subprocess.run(["cargo",args[0],"--manifest-path",str(manifest),*args[1:]],cwd=PROJECT,env=env).returncode

if __name__ == "__main__":
    raise SystemExit(main())
