#!/usr/bin/env python3
"""USB runtime updates/restart; explicit --persist updates only verified own SD boot, with backup. No firmware or owner-project writes."""
import argparse,hashlib,json,time,urllib.request,struct
from pathlib import Path
BASE='http://192.168.155.1:8091'
def request(path,body=None,extra=None):
    headers={'X-Loop-Club-Management':'qa4'}
    headers.update(extra or {})
    req=urllib.request.Request(BASE+path,data=body,headers=headers)
    with urllib.request.urlopen(req,timeout=35) as response:
        return json.loads(response.read(256*1024))
def own_screen_ready(raw):
    """The native loading overlay is not an interactive Loop Club page."""
    if len(raw)<54 or raw[:2]!=b'BM':return False
    offset=struct.unpack_from('<I',raw,10)[0]
    width,height=struct.unpack_from('<ii',raw,18)
    bits=struct.unpack_from('<H',raw,28)[0]
    if width!=320 or abs(height)!=240 or bits not in (24,32):return False
    stride=((width*bits+31)//32)*4
    row=60 if height<0 else 239-60
    point=offset+row*stride+bits//8
    return raw[point:point+3]==bytes((0x20,0x26,0x14))

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--persist',action='store_true',help='Persist verified current runtime only to recognized direct Loop Club autorun, preserving entire old boot')
    p.add_argument('--stage',type=Path,help='Local ARM64 QA .so to stage in device RAM')
    p.add_argument('--restart',action='store_true',help='Apply staged library and restart only ac50; failed candidate restores previous runtime')
    p.add_argument('--discard-session',action='store_true',help='Explicit authorization to discard this volatile QA app session')
    a=p.parse_args()
    if a.restart and not a.discard_session:p.error('--restart requires --discard-session; save owner work first')
    expected=None
    if a.stage:
        body=a.stage.read_bytes()
        if len(body)>8*1024*1024 or len(body)<64 or body[:7]!=b'\x7fELF\x02\x01\x01' or body[16:20]!=b'\x03\x00\xb7\x00':p.error('Expected ARM64 ELF shared library up to8MiB')
        expected=hashlib.sha256(body).hexdigest()
        result=request('/stage',body,{'X-Content-SHA256':expected,'Content-Type':'application/octet-stream'})
        if result.get('staged_sha256')!=expected:raise RuntimeError('Device SHA read-back mismatch')
        print(json.dumps(result,indent=2),flush=True)
    if a.restart:
        print(json.dumps(request('/restart',json.dumps({'discard_session':True}).encode(),{'Content-Type':'application/json'}),indent=2),flush=True)
        time.sleep(1)
        status=request('/status.json');result=status.get('last_result') or {}
        print(json.dumps(status,indent=2))
        if result.get('result')!='restarted':raise RuntimeError(result.get('error','No restart completion record'))
        if expected is not None and result.get('runtime_sha256')!=expected:raise RuntimeError('Restarted runtime differs from candidate')
        deadline=time.monotonic()+60
        while True:
            try:
                with urllib.request.urlopen('http://192.168.155.1:8090/status.json',timeout=2)as response:app=json.loads(response.read(65536))
                if app.get('pid')==result.get('app_pid') and app.get('paint_count',0)>0 and len(app.get('words',[]))>1 and app['words'][1]>=8000:
                    with urllib.request.urlopen('http://192.168.155.1:8090/screen.bmp',timeout=2)as response:
                        if own_screen_ready(response.read(512*1024)):break
            except (OSError,ValueError):pass
            if time.monotonic()>deadline:raise RuntimeError('App HTTP returned but engine/display did not become ready')
            time.sleep(.2)
        print(json.dumps({'engine_and_display_ready':True,'pid':app['pid'],'output_rate':app['words'][1]}))
    else:print(json.dumps(request('/status.json'),indent=2))
    if a.persist:
        with urllib.request.urlopen('http://192.168.155.1:8090/boot-storage.json',timeout=10)as response:boot=json.loads(response.read(65536))
        if boot.get('product')!='Loop Club' or boot.get('boot_mode')!='direct':raise RuntimeError('Unrecognized boot target')
        if expected is not None and boot.get('runtime_sha256')!=expected:raise RuntimeError('Staged code has not been restarted into current runtime')
        payload=json.dumps({'previous':boot['previous_library_sha256'],'runtime':boot['runtime_sha256']}).encode()
        req=urllib.request.Request('http://192.168.155.1:8090/persist',data=payload,headers={'Content-Type':'application/json','X-Loop-Club-Management':'persist-v1'})
        with urllib.request.urlopen(req,timeout=60)as response:report=json.loads(response.read(65536))
        if report.get('persisted')is not True or report.get('runtime_sha256')!=boot['runtime_sha256']:raise RuntimeError('Persistence read-back failed')
        print(json.dumps(report,indent=2))
if __name__=='__main__':main()
