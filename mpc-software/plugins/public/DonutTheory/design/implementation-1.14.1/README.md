# Donut Theory1.14.1 local Sync verification

## Local1.14.1 — Sync encoder fix and dotted/triplet rates (2026-11-19)

Supersedes the five-step Sync behavior in the local1.14.0candidate. Public/installed version remains1.13.14; no device, Release or website update is part of this task.

Confirmed native reproduction:200relative+0.01inputs to the old Sync rate relay returned to cursor0/first division every block. The render-side readback had snapped the continuous Q-Link cursor to the center of its quantized choice, discarding all sub-step movement. The fix retains fractional cursor movement while it denotes the current choice; explicit division edits or a Sync-mode change rebind it to the appropriate source. Selected-LFO front relays share that cursor. Clockwise now increases frequency in both Hz and Sync; free Hz is retained when Sync is toggled.

LFO1, LFO2 and shared Tremolo/Auto Pan each provide15beat periods:1/16,1/8,1/4,1/2,1bar with straight/Dotted/Triplet variants. The single Rate knob traverses them from slow to fast; no extra Q-Link or front button is added. Compact LCD labels use D/T; the sidebar spells out Dotted/Triplet and pages8+7rows. Dotted is3/2and Triplet is2/3of the straight period.1bar retains the original four-quarter-note period.

First388parameter declarations stay exact. Append Lfo1SyncDivision388,MotionSyncDivision389,Lfo2SyncDivision390(391total). Existing five-choice Division/ModDivision/Lfo2Division IDs/ranges remain forward aliases; old states seed the matching straight periods, while new dotted/triplet states restore independently. The new choices share existing Rate/Sync Dice locks. Factory recipes retain their original periods; Dice can choose all15, and exact-normalized Undo restores them. Voice mode, harmony, MIDI, Delay and artwork/layout policies remain as1.14.0.

Validation includes slow relative0.001turns both directions for all3modules and selected-LFO control, all15periods at60/88/120/200BPM, PPQ phase checks, free-rate memory, state migration, locks/Undo and VST callback stability. Physical MPC encoders/rendering/audio/CPU are not certified by local tests. See design/implementation-1.14.1 for final logs/status. Groove Dice was separately audited; it does not use the same LFO Sync relay. Its existing MAIN rhythm selectors, dotted hat grids and free-Hz FX ModRate passed the targeted native/UBSan/VST audit without engine or parameter changes. Its current hat list has no Triplet option; this audit did not add one.

The old reproduction output was `200small forward knob movements: division=0, retained cursor=0`. Updated per-module tests traverse every new choice with getter-relative0.001inputs in both directions. Offline UI examples are code renders, not hardware captures. Final logs/hashes/status are recorded in verification.json.
