#!/usr/bin/env python3
"""Compatibility entry point for the shared Groove Dice expansion manager."""
from pathlib import Path
import runpy
_backend = Path(__file__).resolve().parents[4] / 'shared/groove-dice-expansion/expansions.py'
globals().update(runpy.run_path(str(_backend), run_name='__main__' if __name__ == '__main__' else '_groove_dice_expansions'))
