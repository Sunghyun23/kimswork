#!/usr/bin/env python3
import os,subprocess,sys
from pathlib import Path
P=Path(__file__).resolve().parents[1];ROOT=P.parents[2]
subprocess.run([sys.executable,str(P/'tools/generate_parameters.py')],check=True)
env=dict(os.environ)
env.setdefault('ZIG_GLOBAL_CACHE_DIR',str(P/'build/zig-global-cache'));env.setdefault('ZIG_LOCAL_CACHE_DIR',str(P/'build/zig-local-cache'))
subprocess.run([sys.executable,str(ROOT/'scripts/build_plugin.py'),'public/GrooveDice','--target',sys.argv[1]if len(sys.argv)>1 else'all'],env=env,check=True)
if (P/'build/kim - VST - Groove Dice').exists():subprocess.run([sys.executable,str(P/'tools/finalize_build.py')],check=True)
