# Groove Dice — MAIN concept v1

## MAIN concept v2 — fill selection and recording

User requested visible fill selection and recording on MAIN. `main-concept-v2-fill.png` is the revised built-in image_gen edit; source v1 remains preserved. Proposal, not approved runtime implementation. Full edit prompt preserved in `main-concept-v2-fill-prompt.txt`.

Preserves16 knobs and four performance pads. Replaces overview timeline with a dedicated two-row FILL recorder: Fill A–D selection, length (1 BAR), recording mode (REPLACE), count-in (1 BAR), REC FILL, PLAY FILL and state display. Proposed flow: select slot/length/mode → arm REC FILL → host-synchronized count-in → record incoming physical or screen pad notes into internal fill pattern → finish after chosen length and return to main pattern. Recording should show count-in/recording/completion clearly; image depicts READY only. REPLACE overwrites after explicit record arming; mode may also offer overdub. PLAY FILL auditions/triggers selected fill once and returns to base pattern. Saving stores actual notes/velocity/timing with parent pattern; does not record generated MIDI into MPC sequencer. State designs, arming cancellation, undo and actual touch validation remain pending.

MAIN16 Q-Link proposals remain unchanged; fill recording/selection Q-Link mapping on detail page still to design. Artwork is a16:10 concept, not evidence of SDK layout/touch functionality. Small bank buttons and dense right-side return text need actual1280×800 readability/touch checks in implementation.

2026-11-15 / built-in image_gen / ui-mockup. Proposal only, not approved and not an implemented MPC screenshot. Image is 16:10, targeting 1280×800 logical MPC display. Prompt preserved in main-concept-v1-prompt.txt.

Espresso header, cream panels, olive value displays, amber activity, worn dice and record sleeve illustration. No Donut Theory asset replacement. Native host header is illustrative and remains host-owned in implementation.

Proposed MAIN Q-Link groups: A Pocket/Swing/Random Nudge/Feel; B Kick/Snare/Hi-Hat/Perc levels; C Density/Ghost/Fill/Evolve; D Drive/Dust/Room/Duck. Exactly 16 knobs. Units, semantic distinction between Pocket/Feel/Evolve, and final mappings still need review. Duck means internal ducking proposal, not proof of external-track sidechain support.

Kit/pattern selectors with locks and broad arrows; Roll Dice with scope/Undo/Freeze; momentary Fill; three-lane rhythm overview; four performance pads with shared hat choke tags. Other percussion remains available through physical pads/details; this is not a four-sound engine limit.

Six proposed tabs: MAIN, GROOVE (per-note Nudge), DRUMS, KITS (including choke configuration), FX, ROUTING. Detail-page designs remain pending. Step strip is an overview, not yet a validated finger-sized note editor. Final SDK coordinates, touch targets, fonts and actual host region need implementation and real-device validation. Artwork is not runtime widget art.

## Approved runtime restoration — 2026-11-16

Current authority is approved main-concept-v3-performance.png + fill-editor-concept-v1.png. The0.1 flat-circle/flat-bar implementation did not preserve the approved look; user explicitly requested correction.0.2 restores espresso chassis, rounded ivory double-border cards, dimensional cocoa bakelite knobs, olive inset readouts and tactile action/pad materials. Runtime is1280x628; knob frames64px, group cards138px. Host header/native6function keys remain host-owned. Earlier proposal texts above are historical and do not supersede current mapping in README/AGENTS. build/qa/main-runtime-composition-v2.png is an offline composition, never labelled as a device screenshot.
