#!/usr/bin/env python3
"""Read the connected MPC Sample updater's firmware version; no device mutations."""
import argparse
import json
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--base-url', default='http://mpc-sample.local', help='Connected device updater URL')
    args = parser.parse_args()
    parts = urllib.parse.urlsplit(args.base_url)
    if parts.scheme not in ('http', 'https') or not parts.hostname or parts.username or parts.password or parts.query or parts.fragment or parts.path not in ('', '/'):
        parser.error('Use an http(s) device origin without credentials, a path or query')
    endpoint = urllib.parse.urlunsplit((parts.scheme, parts.netloc, '/version.json', '', ''))
    try:
        # GET only; never fetch upload/update endpoints or execute downloaded JavaScript.
        with urllib.request.urlopen(endpoint, timeout=8) as response:
            raw = response.read(4097)
        if len(raw) > 4096:
            raise ValueError('Unexpectedly large version response')
        data = json.loads(raw)
        version = data.get('version')
        if not isinstance(version, str) or not version:
            raise ValueError('Missing firmware version')
    except (OSError, ValueError, urllib.error.URLError) as error:
        print(f'Unable to read device version: {type(error).__name__}. Check USB and the device URL.', file=sys.stderr)
        return 1
    lock = json.loads((ROOT / 'third_party/sample-sdk.lock.json').read_text())
    match = version in lock['supported_firmware']
    print(json.dumps({'device': 'MPC Sample', 'version': version, 'full_version': data.get('fullVersion'), 'matches_documented_sdk_version': match, 'firmware_fingerprint_verified': False, 'device_changed': False}, indent=2))
    return 0 if match else 2


if __name__ == '__main__':
    raise SystemExit(main())
