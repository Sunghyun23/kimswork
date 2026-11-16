# Production artwork

Runtime assets are compiled into `skins/` by `tools/build_skin.py` + `tools/skin_art.py` at their exact native dimensions. Only referenced runtime images enter the install package; source art remains here. UI labels/hit surfaces/Q-Link bindings are native widgets, never a screenshot overlay.

All raster source art below was generated with the built-in `image_gen` tool. Exact0.2 prompts are in `imagegen-prompts-v020.json`. Source versions are retained, not silently overwritten.

| Source | Purpose | Saved prompt |
|---|---|---|
| background-branded-source.png |0.1 cream-body source / ivory material sample|background-branded-prompt.txt|
| background-branded-source-v2.png |0.2 espresso chassis preserving approved masthead/dice direction; compiled1280x628|background-v2-prompt.txt|
| bakelite-knob-source-v1.png |Dimensional cocoa bakelite body; mechanical native64px filmstrip with code-native ticks/pointer|bakelite-knob-prompt-v1.txt|
| dice-icon-source-v1.png |Two aged ivory dice; mechanical48x32 ROLL DICE icon|dice-icon-prompt-v1.txt|

Approved reference remains `../design/main-concept-v3-performance.png`. The0.1 flat circles/flat title bars were rejected by the user; do not reintroduce them. Rounded ivory cards, olive readouts, material buttons and the illustrated masthead belong together. Do not rely on MPC automatic image scaling. Button/filmstrip bitmap size must match the widget bounds.
