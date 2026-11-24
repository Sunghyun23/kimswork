#!/usr/bin/env python3
"""Create a local guarded SDK/SampleBoot candidate. Does not install or restart."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import zipfile

PROJECT = Path(__file__).resolve().parents[1]
DOMAIN = PROJECT.parents[3]
SDK = DOMAIN / 'third_party/mpc-sample-sdk'
SALT = b'f7bed1c9-7e90-4731-9194-c996666b4331'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--public',action='store_true',help='Create the original-content public preview; never includes QA tools or banks')
    parser.add_argument('--direct',action='store_true',help='Direct Loop Club autorun without SampleBoot or launch question')
    parser.add_argument('--qa',action='store_true',help='Separate USB-only screen/input QA package; existing normal package is retained')
    parser.add_argument('--firmware-prefix-sha256', help='Optional independently measured first-4KiB hash; defaults to explicitly labelled pinned upstream reference')
    args = parser.parse_args()
    version=json.loads((PROJECT/"plugin.json").read_text())["version"]
    if not re.fullmatch(r"\d+\.\d+\.\d+",version):parser.error("Invalid product version")
    if args.public and args.qa:parser.error("Public packages cannot include the USB QA agent")
    reference = re.search(r'--firmware-sha256 ([a-f0-9]{64})', (SDK/'docs/22-build-and-install.md').read_text()).group(1)
    expected = args.firmware_prefix_sha256 or reference
    if not re.fullmatch('[a-f0-9]{64}', expected):
        parser.error('Expected a lowercase 64-hex hash')
    subprocess.run([sys.executable, str(PROJECT/'tools/build.py'), '--pack', '--firmware-prefix-sha256', expected]+(['--qa']if args.qa else[]), check=True)
    stage = PROJECT/'build/device/pkg/autorun'
    script = (stage/'autorun.sh').read_text()
    agent_hash = None
    if args.qa:
        sys.path.insert(0, str(PROJECT.parents[2]/'tools'))
        from development_environment import environment
        from patched_sdk import build_manifest
        agent_env = environment()
        agent_env['CARGO_TARGET_DIR'] = str(PROJECT/'build/target')
        agent_env['CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER'] = str(SDK/'scripts/zig-cc-aarch64-linux-gnu')
        with build_manifest(PROJECT) as (manifest,_):
            subprocess.run(['cargo','build','--release','--manifest-path',str(manifest),'--target','aarch64-unknown-linux-gnu','--bin','loop-club-qa-agent'], env=agent_env, check=True)
        agent = PROJECT/'build/target/aarch64-unknown-linux-gnu/release/loop-club-qa-agent'
        shutil.copyfile(agent,stage/'loop-club-qa-agent')
        agent_hash=hashlib.sha256(agent.read_bytes()).hexdigest()
    # Keep SDK hash validation and startup. Add card-relative persistent data,
    # atomic runtime replacement and an integrity guard before any preload.
    lib_hash = hashlib.sha256((stage/'loop-club.so').read_bytes()).hexdigest()
    prefix = '''
# Resolve from this script, including when relocated by SampleBoot.
CARD=$(awk -v d="$DIR/" '{ m=$2; gsub(/\\\\040/, " ", m); gsub(/\\\\011/, "\\t", m); if (m != "/" && index(d,m "/")==1 && length(m)>n) { n=length(m); c=m } } END { print c }' /proc/mounts)
[ -n "$CARD" ] || exit 0
DATA="$CARD/LoopClub/Projects"
mkdir -p "$DATA" || exit 0
ACTUAL=$(dd if="$STOCK" bs=4096 count=1 2>/dev/null | sha256sum | cut -d" " -f1)
printf '%s\\n' "$ACTUAL" > "$CARD/LoopClub/observed-app-prefix-sha256.txt"
'''
    script = script.replace('FW="'+expected+'"', 'FW="'+expected+'"\n'+prefix)
    line = 'cp "$DIR/loop-club.so" /run/loop-club.so || exit 0'
    integrity = '[ "$(sha256sum "$DIR/loop-club.so" | cut -d" " -f1)" = "'+lib_hash+'" ] || exit 0\n'
    script = script.replace(line, integrity+'cp "$DIR/loop-club.so" /run/.loop-club.so.new || exit 0\nmv /run/.loop-club.so.new /run/loop-club.so || exit 0')
    env = '''# Escape systemd quoted Environment syntax, including percent specifiers.
ESCAPED_DATA=$(printf '%s' "$DATA" | sed 's/\\\\/\\\\\\\\/g; s/"/\\\\"/g; s/%/%%/g')
printf 'Environment="LOOP_CLUB_DATA_DIR=%s"\\n' "$ESCAPED_DATA" >> /run/systemd/system/ac50.service.d/loop-club.conf
'''
    env += '''ESCAPED_CARD=$(printf '%s' "$CARD" | sed 's/\\\\/\\\\\\\\/g; s/"/\\\\"/g; s/%/%%/g')
printf 'Environment="LOOP_CLUB_CARD_DIR=%s"\\n' "$ESCAPED_CARD" >> /run/systemd/system/ac50.service.d/loop-club.conf
'''
    if args.qa:env += """printf '%s\\n' 'Environment="LOOP_CLUB_QA=1"' >> /run/systemd/system/ac50.service.d/loop-club.conf
"""
    script = script.replace('systemctl daemon-reload', env+'systemctl daemon-reload')
    if args.qa:
        script += '''\nmkdir -p "$CARD/LoopClub/QA-qa5"
systemctl show ac50.service --property=MainPID --property=ActiveState --property=SubState --property=PrivateNetwork --property=PrivateDevices --property=RestrictAddressFamilies --property=IPAddressDeny --property=IPAddressAllow > "$CARD/LoopClub/QA-qa5/service.txt" 2>&1
'''
    if args.qa:
        script += '\n[ "$(sha256sum "$DIR/loop-club-qa-agent" | cut -d" " -f1)" = "'+agent_hash+'" ] || exit 0\n'
        script += 'cp "$DIR/loop-club-qa-agent" /run/.loop-club-qa-agent-bin.new || exit 0\nchmod 755 /run/.loop-club-qa-agent-bin.new\nmv /run/.loop-club-qa-agent-bin.new /run/loop-club-qa-agent-bin || exit 0\nsystemctl stop loopclub-qa-agent.service >/dev/null 2>&1 || true\nsystemd-run --unit=loopclub-qa-agent --collect /run/loop-club-qa-agent-bin --usb-only\n'
    (stage/'autorun.sh').write_text(script)
    subprocess.run(['sh','-n',str(stage/'autorun.sh')],check=True)
    data = json.loads((stage/'autorun.json').read_text())
    data['value0']['scripts'][0]['askQuestion'] = not args.direct
    data['value0']['scripts'][0]['message'] = 'Start Loop Club? The app will restart. Save your work first.'
    (stage/'autorun.json').write_text(json.dumps(data,indent=2)+'\n')
    (stage/'autorun.check').write_text(hashlib.sha256((stage/'autorun.json').read_bytes()+(stage/'autorun.sh').read_bytes()+SALT).hexdigest())
    (stage/'sampleboot.txt').write_text('Loop Club USB QA5\n'if args.qa else f'Loop Club {version} - development\n')
    shutil.copy2(PROJECT/'LICENSE',stage/'LICENSE')
    (stage/'SDK-LICENSE').write_text((SDK/'LICENSE').read_text()+'\n\n'+(PROJECT/'src/roxmltree-license.txt').read_text())
    metadata = {'product':'Loop Club','version':version,'status':'local development candidate; hardware unverified','firmware':'1.3.0.12','expected_app_first_4KiB_sha256':expected,'hash_source':'caller supplied; measurement not independently verified by packager' if args.firmware_prefix_sha256 else 'pinned SDK upstream reference; NOT measured on connected device','library_sha256':lib_hash,'device_installed':False,'screen_capture':'none; host previews only','persistent_data':'<card>/LoopClub/Projects/ (Session.lcproj, Projects/*.lcproj and active-project.txt)'}
    metadata['boot_mode']='direct' if args.direct else 'sampleboot-compatible'
    metadata['qa_agent_sha256']=agent_hash
    metadata['usb_qa_enabled']=args.qa
    metadata['qa_variant']='qa5'if args.qa else None
    metadata['usb_qa_address']='http://192.168.155.1:8090'if args.qa else None
    (stage/'compatibility.json').write_text(json.dumps(metadata,indent=2)+'\n')
    (stage/'INSTALL.txt').write_text((PROJECT/'docs/public-install.txt').read_text())
    dest=PROJECT/(f'dist/loop-club-{version}-sample-usb-qa.zip'if args.qa else f'dist/loop-club-{version}-sample-development.zip')
    if args.direct:dest=dest.with_name(dest.stem+'-direct.zip')
    if args.public:
        dest=PROJECT/f'dist/loop-club-{version}-mpc-sample.zip'
        if (stage/'loop-club-qa-agent').exists():raise RuntimeError('QA agent in public payload')
        metadata.update(status='public preview; actual MPC Sample 1.3.0.12 remote UI and USB audio tested; physical controls and power-cycle acceptance pending',screen_capture='actual active DRM; see public verification notes',usb_qa_enabled=False,qa_variant=None,usb_qa_address=None)
        (stage/'compatibility.json').write_text(json.dumps(metadata,indent=2)+'\n')
        (stage/'sampleboot.txt').write_text(f'Loop Club {version} Preview\n')
        (stage/'INSTALL.txt').write_text((PROJECT/'docs/public-install.txt').read_text())
    dest.parent.mkdir(exist_ok=True)
    with zipfile.ZipFile(dest,'w',zipfile.ZIP_DEFLATED)as archive:
        for file in sorted(stage.rglob('*')):
            if file.is_file(): archive.write(file,Path('autorun')/file.relative_to(stage))
    sha=hashlib.sha256(dest.read_bytes()).hexdigest()
    dest.with_suffix('.zip.sha256').write_text(f'{sha}  {dest.name}\n')
    print(json.dumps({'package':str(dest),'sha256':sha,'firmware_hash_source':metadata['hash_source'],'device_changed':False},indent=2))

if __name__=='__main__':
    main()
