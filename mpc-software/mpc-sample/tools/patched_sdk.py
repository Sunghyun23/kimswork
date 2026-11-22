"""Audited build-only SDK staging. Canonical pinned checkout is never modified."""
from contextlib import contextmanager
import hashlib,json,shutil,subprocess,tempfile
from pathlib import Path
from development_environment import ROOT

@contextmanager
def build_manifest(project):
    lock=json.loads((ROOT/'third_party/sample-sdk.lock.json').read_text())
    dep=next(d for d in lock['dependencies']if d['id']=='mpc-sample-sdk');sdk=ROOT/dep['path']
    commit=subprocess.check_output(['git','-C',str(sdk),'rev-parse','HEAD'],text=True).strip()
    dirty=subprocess.check_output(['git','-C',str(sdk),'status','--porcelain','--untracked-files=no'],text=True)
    if commit!=dep['commit']or dirty:raise RuntimeError('Canonical SDK identity/cleanliness mismatch')
    patches=dep.get('build_patches',[])
    if not patches:yield project/'Cargo.toml',sdk;return
    with tempfile.TemporaryDirectory(prefix='loopclub-sdk-build-')as temporary:
        root=Path(temporary);staged=root/'sdk';shutil.copytree(sdk,staged,ignore=shutil.ignore_patterns('.git','target'))
        for entry in patches:
            patch=ROOT/entry['path']
            if hashlib.sha256(patch.read_bytes()).hexdigest()!=entry['sha256']:raise RuntimeError('SDK patch checksum mismatch')
            subprocess.run(['patch','-s','-p1','-i',str(patch)],cwd=staged,check=True)
        own=root/'project';own.mkdir()
        for item in project.iterdir():
            if item.name in ['Cargo.toml','Cargo.lock','.git']:continue
            (own/item.name).symlink_to(item,target_is_directory=item.is_dir())
        if (project/'Cargo.lock').is_file():shutil.copyfile(project/'Cargo.lock',own/'Cargo.lock')
        manifest=(project/'Cargo.toml').read_text();needle='path = "../../../../third_party/mpc-sample-sdk"'
        if manifest.count(needle)!=1:raise RuntimeError('Unrecognized SDK dependency path')
        (own/'Cargo.toml').write_text(manifest.replace(needle,'path = '+json.dumps(str(staged))))
        yield own/'Cargo.toml',staged
    after=subprocess.check_output(['git','-C',str(sdk),'status','--porcelain','--untracked-files=no'],text=True)
    if after:raise RuntimeError('Canonical SDK changed during build')
