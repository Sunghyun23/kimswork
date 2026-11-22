#!/usr/bin/env python3
"""Read actual device LCD/status and enqueue bounded Loop Club QA controls over USB."""
import argparse,hashlib,io,json,urllib.request
from datetime import datetime,timezone
from pathlib import Path
BASE='http://192.168.155.1:8090'
def request(path,data=None):
    body=None if data is None else json.dumps(data).encode()
    req=urllib.request.Request(BASE+path,data=body,headers={'Content-Type':'application/json'}if body is not None else{})
    with urllib.request.urlopen(req,timeout=5)as r:return r.read(2*1024*1024)
def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--capture',type=Path,help='New PNG file; existing captures are never overwritten');p.add_argument('--input-json',help='Bounded QA controller JSON, not a shell command');a=p.parse_args()
    if a.input_json is not None:
        data=json.loads(a.input_json)
        if not isinstance(data,dict):p.error('Expected one input object')
        print(request('/input',data).decode())
    status=json.loads(request('/status.json'))
    if a.capture is not None:
        from PIL import Image
        raw=request('/screen.bmp')
        if not raw.startswith(b'BM'):raise ValueError('Device did not return framebuffer BMP')
        image=Image.open(io.BytesIO(raw));image.load()
        if image.size!=(320,240):raise ValueError('Unexpected device screen dimensions')
        output=io.BytesIO();image.save(output,format='PNG');a.capture.parent.mkdir(parents=True,exist_ok=True)
        with a.capture.open('xb')as f:f.write(output.getvalue())
        record={'source':'actual MPC Sample active DRM framebuffer; not host UI rendering','time_utc':datetime.now(timezone.utc).isoformat(),'bmp_sha256':hashlib.sha256(raw).hexdigest(),'png_sha256':hashlib.sha256(output.getvalue()).hexdigest(),'status':status}
        with a.capture.with_suffix('.json').open('x')as f:json.dump(record,f,indent=2)
        print(str(a.capture.resolve()))
    print(json.dumps({k:v for k,v in status.items()if k!='words'},indent=2))
if __name__=='__main__':main()
