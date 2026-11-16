# Original Workshop

Original Groove Dice core expansion: 128 procedurally synthesized WAV one-shots, 96 curated kit configurations, two original beats and 93 independently authored timing templates. No external audio/MIDI/model inputs. Pack ID `kim-original-workshop`, version `1.0.0`, target Groove Dice (`KpGy`).

Install this project’s offline authoring dependency with `python3 -m pip install -r mpc-software/expansions/groove-dice/public/original-workshop/requirements.txt`. NumPy is not an MPC runtime dependency.

From the repository root:

```sh
python3 mpc-software/expansions/groove-dice/public/original-workshop/tools/build.py
```

Generated sources, catalogues, manifest and bank go to ignored `work/`; packaged `.gdexp` files go to ignored `dist/`. The active redistributable runtime core and its hash/provenance are in the public plugin's `Content/Banks/` and `Content/Licenses/`. Generation does not overwrite the active core automatically. Compare manifests/hashes and verify before promoting a new bank.

The source generator is independent of the plugin build. Historical generator hash records identify the exact earlier source before this folder migration; they are not silently rewritten.
