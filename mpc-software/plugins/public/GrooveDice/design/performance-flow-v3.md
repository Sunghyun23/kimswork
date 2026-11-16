# Groove Dice — performance flow v3

2026-11-15. User requested a redesign following usability analysis and frequent-use Q-Link assignments. The user approved this v3 design on2026-11-15 and requested implementation. Concept images remain artwork references; working native UI and actual device captures are tracked in README.md and VALIDATION.md. Generated with built-in image_gen; complete prompts saved beside images. Earlier variants preserved.

## MAIN

`main-concept-v3-performance.png`: kit → pattern/randomize → play → adjust groove → trigger fill → save. FOLLOW MPC/position/read-only BPM, preview while host stopped, SAVE, scope/UNDO/KEEP and kit/pattern locks. Preview hands over to host playback without duplicate playback; state designs still pending.

| Q-Link | Proposed assignment |
|---|---|
| A | Swing / Snare Late / Hat Shift / Random Nudge |
| B | Density / Ghost / Velocity Spread / Random Amount |
| C | Fill Select / Fill Density / Fill Every / Variation |
| D | Drive / Dust / Room / Duck |

Snare Late is a deterministic snare offset; Hat Shift is a signed deterministic hat offset. Random Nudge sets maximum timing jitter; Random Amount controls generation mutation strength within selected scope, preserving locks and drum/choke rules. Velocity Spread is controlled velocity variation, Ghost the ghost-hit amount, Density overall hit density. Fill Every is Off or a specified bar interval. Variation changes pattern variation depth; exact mapping/ranges remain proposed. Duck is internal ducking, not validated external-track sidechain. Part volumes remain in DRUMS rather than deleted.

Compact fill strip shows selected fill/status/TRIGGER FILL/EDIT-REC. Trigger queues once, then returns to base pattern; exact immediate/next-beat/bar policy to define. No bulky recorder setup on MAIN. Kit/pattern LOCK excludes items from generation; KEEP commits current generated result and freezes runtime randomization when relevant, separate from persistent SAVE. User-facing final semantics still need review.

## Fill editor

`fill-editor-concept-v1.png`: EDIT / REC from MAIN opens editor in one action, BACK returns. Four fill slots, finite recording length/mode/count-in, touch event grid, audition/record/cancel/undo/save and large performance pads. State shown is READY; count-in/recording/completion/queued transitions not yet drawn. Count-in and recording should not start merely on page entry. Incoming pad notes are recorded internally; generated base-pattern events must not be recursively recorded.

| Q-Link | Proposed assignment |
|---|---|
| A | Fill Select / Length / Record Mode / Count-in |
| B | Selected-note Nudge / Velocity / Gate / Chance |

C/D remain unassigned on this screen; do not leave MAIN Q-Link actions or guide bounds active. Selected-note controls disable when no note selected. Gate applies to gated playback only; one-shot/choke behavior must remain explicit. Sketch grid labels/selected-step alignment are illustrative and must be made exact in implementation. Physical pads can expose more voices than the four shown quick pads.

SDK feasibility of arbitrary subpage transitions, drawable/editable note grid, count-in recording and file-save UI is not proven by generated artwork. Resolve against actual fixed SDK before promising implementation. Touch geometry, text contrast at1280×800 and actual host-owned regions require real UI validation. Do not describe these as MPC screenshots.
