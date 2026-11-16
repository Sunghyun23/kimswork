# Groove Dice expansion backend

Standalone Python-standard-library `expansions.py` owns `.gdexp` packing, validation, local install/remove/list and public/personal runtime assembly. It is shared by the plugin's tool entry point and intended for the desktop Pack Builder. The desktop GUI has not yet been implemented.

```sh
python3 mpc-software/shared/groove-dice-expansion/expansions.py --help
```

No SDK dependency, device SSH, or runtime audio changes. Public packages include this standalone file, not a repository-relative wrapper. The dedicated format currently identifies Groove Dice implicitly via GDEXP001; explicit target product/UID metadata remains a future compatible extension.
