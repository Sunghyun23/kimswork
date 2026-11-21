#!/usr/bin/env python3
"""Original uniform Euler–Bernoulli kalimba beam, offline NumPy only.

Design assumption: 32 elements, clamp at 0, bridge displacement fixed at
12/32, free end. Pluck at 31/32; displacement readout at the tip. Not a
fit to a measured instrument. See docs/plugins/felt-club/thumb-research.md.
"""
from pathlib import Path
import json
import numpy as np


def calculate():
    n, bridge = 32, 12
    length = 1 / n
    h = length
    stiffness = np.array([[12,6*h,-12,6*h], [6*h,4*h*h,-6*h,2*h*h],
                          [-12,-6*h,12,-6*h], [6*h,2*h*h,-6*h,4*h*h]]) / h**3
    mass = h/420 * np.array([[156,22*h,54,-13*h], [22*h,4*h*h,13*h,-3*h*h],
                            [54,13*h,156,-22*h], [-13*h,-3*h*h,-22*h,4*h*h]])
    K = np.zeros((2*n+2, 2*n+2))
    M = K.copy()
    for element in range(n):
        indices = np.arange(2*element, 2*element+4)
        K[np.ix_(indices,indices)] += stiffness
        M[np.ix_(indices,indices)] += mass
    free = [i for i in range(2*n+2) if i not in {0,1,2*bridge}]
    A, B = K[np.ix_(free,free)], M[np.ix_(free,free)]
    C = np.linalg.cholesky(B)
    Z = np.linalg.solve(C,A)
    Z = np.linalg.solve(C,Z.T).T
    eigenvalues, eigenvectors = np.linalg.eigh(Z)
    assert (eigenvalues > 0).all()
    modes = np.linalg.solve(C.T,eigenvectors[:,:6])
    shapes = np.zeros((2*n+2,6))
    shapes[free] = modes
    frequencies = np.sqrt(eigenvalues[:6])
    values = [frequencies/frequencies[0], shapes[2*(n-1)]/shapes[2*(n-1),0],
              shapes[-2]/shapes[-2,0]]
    assert all(np.isfinite(v).all() for v in values)
    return values


def header(values):
    lines = ['#pragma once', '#include <array>', 'namespace felt::thumb {',
             '// Euler–Bernoulli beam, clamped at node 0, supported at 12/32,',
             '// free at node 32; six mass-normalized modes. Original offline FEM.']
    for name, values in zip(('BeamRatios','PluckShape','TipShape'),values):
        lines.append('inline constexpr std::array<float,6> '+name+'{'+
                     ','.join(f'{v:.8f}f' for v in values)+'};')
    return '\n'.join(lines+['}', ''])


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--write', action='store_true')
    args = parser.parse_args()
    values = calculate()
    target = Path(__file__).resolve().parents[2]/'src/engines/thumb/ThumbBeam.h'
    text = header(values)
    if args.write:
        target.write_text(text)
    else:
        # Different LAPACK versions can affect last decimal places.
        import re
        existing = target.read_text()
        for name, expected in zip(('BeamRatios','PluckShape','TipShape'),values):
            found = re.search(name+r'\{([^}]+)\}',existing)
            assert found
            actual = [float(v.rstrip('f')) for v in found.group(1).split(',')]
            assert np.allclose(actual,expected,rtol=2e-6,atol=2e-7), name
    print(json.dumps(dict(zip(('frequency_ratios','pluck_shape','tip_shape'),
                             (v.tolist() for v in values))),indent=2))
