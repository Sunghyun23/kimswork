#!/usr/bin/env python3
"""Build the independently versioned Original Workshop expansion."""
from pathlib import Path
import runpy
runpy.run_path(str(Path(__file__).resolve().parents[4]/'expansions/groove-dice/public/original-workshop/tools/build.py'), run_name='__main__')
