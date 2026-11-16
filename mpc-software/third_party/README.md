# Shared SDK

Run `python3 mpc-software/scripts/bootstrap_sdk.py` from the repository root. The repository URL and full commit are in `sdk.lock.json`. The checkout is ignored; SDK source is not modified or duplicated per plugin. Its upstream MIT license remains in the installed checkout.

SDK compiler bootstrap installed `ziglang 0.17.1` during the initial build. For reproducible setup after installing the SDK:

```sh
python3 -m venv mpc-software/third_party/mpc-plugin-builder-sdk/.sdk-tools/zigenv
mpc-software/third_party/mpc-plugin-builder-sdk/.sdk-tools/zigenv/bin/python -m pip install ziglang==0.17.1
```

The initial build uses Apple clang for quick native builds and Zig for ARM builds. Python 3.13.7, Apple clang 16 and Pillow 11.3.0 were used. UI asset generation/preview requires Pillow. These are developer dependencies, not MPC runtime dependencies.
