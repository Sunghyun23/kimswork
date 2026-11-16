# Groove Dice expansion backend

Standalone Python-standard-library `expansions.py` owns `.gdexp` packing, validation, local install/remove/list and public/personal runtime assembly. It is shared by the plugin's tool entry point and used by the standalone desktop Pack Builder. authoring.py validates source folders and emits runtime-compatible banks.

```sh
python3 mpc-software/shared/groove-dice-expansion/expansions.py --help
```

No SDK dependency, device SSH, or runtime audio changes. Public packages include this standalone file, not a repository-relative wrapper. The dedicated format currently identifies Groove Dice implicitly via GDEXP001; new manifests include target=groove-dice and plugin_uid=KpGy; old manifests without those fields remain compatible.
