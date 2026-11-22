#!/usr/bin/env python3
"""Build ARM64 through the pinned SDK; no device access or installation."""
import argparse
import json
from pathlib import Path
import subprocess
import sys

PROJECT = Path(__file__).resolve().parents[1]
SAMPLE = PROJECT.parents[2]
DOMAIN = SAMPLE.parent
sys.path.insert(0, str(SAMPLE / 'tools'))
from development_environment import environment
from patched_sdk import build_manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--qa-session',help='New isolated QA session folder ID; ASCII letters/digits/hyphen only')
    parser.add_argument('--qa',action='store_true')
    parser.add_argument('--pack',action='store_true')
    parser.add_argument('--firmware-prefix-sha256')
    args = parser.parse_args()
    if args.pack and (not args.firmware_prefix_sha256 or len(args.firmware_prefix_sha256)!=64 or any(c not in '0123456789abcdef' for c in args.firmware_prefix_sha256)):
        parser.error('--pack requires a lowercase 64-hex firmware prefix hash')
    lock = json.loads((DOMAIN / 'third_party/sample-sdk.lock.json').read_text())
    sdk = DOMAIN / 'third_party/mpc-sample-sdk'
    expected = next(d['commit'] for d in lock['dependencies'] if d['id'] == 'mpc-sample-sdk')
    actual = subprocess.check_output(['git', '-C', str(sdk), 'rev-parse', 'HEAD'], text=True).strip()
    dirty = subprocess.check_output(['git', '-C', str(sdk), 'status', '--porcelain', '--untracked-files=no'], text=True)
    if actual != expected or dirty:
        parser.error('Pinned SDK commit/cleanliness check failed')
    if args.qa_session and (not args.qa or len(args.qa_session)>48 or not all(c.isascii() and (c.isalnum() or c=='-')for c in args.qa_session)):parser.error('Invalid isolated QA session ID')
    env = environment()
    patch_lock=json.loads((PROJECT.parents[3]/'third_party/sample-sdk.lock.json').read_text())
    env['LOOP_CLUB_SDK_PATCH']=next(d for d in patch_lock['dependencies']if d['id']=='mpc-sample-sdk')['build_patches'][0]['sha256']
    env.pop('LOOP_CLUB_QA_SESSION',None)
    if args.qa_session:env['LOOP_CLUB_QA_SESSION']=args.qa_session
    if args.qa:env['LOOP_CLUB_BUILD_ID']='qa5'
    else:env.pop('LOOP_CLUB_BUILD_ID',None)
    env['CARGO_PROFILE_RELEASE_STRIP'] = 'none'
    env['CARGO_TARGET_DIR'] = str(PROJECT / 'build/target')
    extra = ['--pack','--firmware-sha256',args.firmware_prefix_sha256,'--display-name','Loop Club'] if args.pack else []
    with build_manifest(PROJECT) as (manifest,_):
        return subprocess.run(['sh', str(sdk / 'build-mod.sh'), '--manifest-path', str(manifest), '--package', 'loop-club', '--name', 'loop-club', '--require-symbol', 'g_screen', '--require-symbol', 'snd_pcm_writei', '--output-dir', str(PROJECT / 'build/device')] + extra, cwd=PROJECT, env=env).returncode

if __name__ == '__main__':
    raise SystemExit(main())
