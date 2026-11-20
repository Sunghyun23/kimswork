# Current decision: separate MAIN / PERFORM Dice (1.9.0)

MAIN changes the eligible main groove/controls and configured kit pool only, preserving stored performance slots and master FX. PERFORM changes only unlocked twelve performance slots, using the current groove to generate fill endings. Later MAIN Dice does not silently update those saved fills. Recorded/edited fills auto-lock; selected GENERATE is an explicit replacement.

Each screen has independent Undo. Playing bulk operations apply next bar, stopped operations immediately. PERFORM Undo cancels its queued generation and leaves MAIN history intact; MAIN Undo leaves performance notes/settings intact. The fill editor shares the PERFORM actions/history, including its explicit ROLL FILL. PERFORM UNLOCK ALL clears its slot locks only. No changes to footer placement, playback trigger notes, capture, Q-Link mappings, or finite bar-end timing. Append PerformUndo at917; all previous917declarations stay exact. Current device/release evidence is recorded in GrooveDice/design/device-1.9.0/.

The sections below describe earlier released behavior where MAIN and PERFORM shared global Dice; that coupling is superseded by the current decision above.

## 1.8.1 release — 2026-11-20

## 1.8.1 — consistent controls

Every page and the fill editor share **UNLOCK ALL → ROLL DICE → UNDO** in the same bottom positions as Donut Theory and Crate Press. MAIN’s sixteen padlocks move to the left of their knobs. Their transparent open/closed artwork matches the other instruments; 44×44 touch targets remain, with no knob overlap.

On SOUND, the footer remains available on both pages. The **SOUND ACTIONS** button opens a drawer with **ROLL SOUND / UNDO SOUND / AUDITION / SAVE KIT**; these remain separate from global Dice. Sample-row Preview buttons and physical audition stay directly accessible. The SOUND1/2 counter and arrows remain beside the common footer.

Global Dice’s scope is unchanged: it regenerates eligible MAIN values and unlocked performance phrases, honoring kit/role/value/slot protection. FX uses its separate ROLL FX action. Moving buttons does not broaden randomisation. Engine, all 917 parameter definitions, presets, generated phrase logic and Q-Link targets are unchanged from 1.8.0.

Older page layouts below are historical. User authorized stable publication after final actual-device lock QA.

## 1.8.1 — common footer candidate (2026-11-20)

Common footer on seven pages and fill editor. SOUND ACTIONS retains local Roll Sound/Undo Sound/Audition/Save Kit. Global Dice scope,917parameters and Q-Link targets unchanged. Shared coordinates and scopes: mpc-software/shared/ui/README.md. Older layouts below are historical. Public1.8.0unchanged.

# Performance, capture and MIDI — 1.7.1

Status: current1.8.0implementation, native/ARM/SDK and automated tests passed. Actual MPC XL remote checks and MIDI timestamps verify bar-end timing. The owner confirmed physical B1/B3 select ROLL A/C and end correctly; full eight-pad/physical-encoder/audio/CPU certification is not implied. Vendor kim/UID KpGy; first880declarations unchanged,917parameters. This supersedes1.7.0's held-roll/next-beat/duplicate-entry behavior and1.5.1's unconditional fill preservation.

## Workflow

MAIN retains its four Q-Link groups (16 controls). PERFORM is reached only by pressing the first native function key to cycle MAIN/PERFORM; press it again to return. The duplicate MAIN PERFORM button and separate performance context panel are removed. MAIN's bottom EDIT FILL opens the existing note editor with its eight Screen Q-Link controls. The other five native keys and their mappings stay unchanged.

PERFORM mirrors a4×4MPC pad bank, numbered bottom-up:

| Top to bottom | Left to right |
|---|---|
| Fourth row | B13 LOOP, B14 STOP, B15–B16 unused |
| Third row | B9–B12 Hat Roll A–D |
| Second row | B5–B8 Fill A–D |
| First row | B1–B4 Snare Roll A–D |

Bank names are physical-location hints, not MIDI bank messages. Plugins receive note numbers, not the physical bank name. The initial note assignments are48,50,52,53,55,57,59,60,62,64 for the ten occupied pads; these follow the test device's two-octave Bank B offset in its C-major layout. Physical B1=48 was confirmed by LEARN. The former1.7.0defaults began at52 and placed the first roll on physical B3. Complete old default layouts, including the B1-only diagnostic learn, migrate to the corrected map; custom learned layouts remain intact. Different MPC scale/root/custom pad layouts require LEARN PAD. LOOP/STOP learn buttons are on ROUTING. Duplicate performance assignments swap; drum-note/reserved-articulation collisions are rejected, and existing direct drum mappings take priority. The plugin never changes MPC Pad Perform settings. Existing A-bank drum assignments remain intact.

## Playing and editing

- Roll: press once to queue the selected1/2/4-beat phrase so it finishes exactly at a bar boundary, then stop automatically; releasing early does not shorten the phrase. It substitutes the existing snare while retaining other drum parts. Four generated roles begin as1/16,1/32,1/16T and accelerating.
- Fill: tap to queue the selected1/2/4-beat variation at the end of a bar, play it once, then return to the underlying loop. Generated fills retain the inherited main prefix for full-bar note editing/audition, but PERFORM plays only the variation tail. Direct recordings and legacy fills play their full stored length. New fills replace queued fills; they never stack. A playing snare roll may replace the snare part of a fill.
- Loop: toggle internal preview when the host is stopped; while following a running host it gates this instance at a bar boundary. STOP cancels this instance's performance, not MPC transport.
- Host count-in remains authoritative and cannot be bypassed by loop, fill or roll triggers. Normal direct drum monitoring is unaffected.
- SELECT TO EDIT makes pad touches select a slot without sounding. Selected Rate/Velocity/Humanize/Length use one four-knob Screen Q-Link group. Recorded/legacy fills retain their note data: Rate is inactive, Velocity scales playback, Humanize is non-destructive, and Length uses the existing1/2/4/8/16-beat control. New recordings/edits start at unity velocity and no additional humanization. Other groups are unassigned on the native PERFORM page. No duplicate MAIN performance panel remains.
- GENERATE explicitly regenerates the selected slot. LOCK protects against bulk/global generation; it does not prevent manual editing or explicit selected-slot generation. Lock status is shown on pads and in the selected-slot assignment line.
- REC FILL appears for a selected fill and records physical drum input. Use MAIN’s EDIT FILL to open the note editor; PERFORM focuses on performance and capture. Finishing a recording or editing a note locks that fill automatically.

## Bar-end timing

Both fills and rolls use the earliest bar ending that still accommodates their entire chosen length. One beat occupies the last beat; two beats occupy the last two beats. A full bar starts at the next available downbeat. If the relevant start has already passed, wait for the next valid bar ending instead of truncating. Triggered Length and end time are latched; a subsequent Length edit applies to the next trigger. Recorded multi-bar fills keep their full stored length and end on a bar boundary. Host loop boundaries retain continuous musical position. STOP cancels queued/playing phrases. Host count-in stays silent.

## Dice and persistence

ROLL DICE regenerates the eligible MAIN groove and unlocked fills/rolls together at the next bar while playing, immediately while stopped. Existing MAIN/kit/pattern/role protections remain. Fills inherit the current main phrase before their ending variation. Roll roles retain their rate/length settings and regenerate rests/accents; open slots vary Velocity and Humanize within Random Amount bounds, so the change is visible without losing the chosen rhythmic role. FX are preserved. Random Amount0 leaves all musical data unchanged. UNDO restores the complete prior groove and twelve performance sequences. Directly recorded/edited fills are protected automatically.

Old four fill banks are reused, never deleted. Loading a pre1.7project automatically locks every populated legacy fill, preventing a new Dice policy from destroying older recorded performances. New roll sequences are appended in a separately versioned extra-state trailer. Existing state layouts/UID and all782published parameter IDs/ranges remain unchanged. Triggers, recording commands and screen press gates are transient and save as idle. Fresh/resumed instances return to MAIN without startup messages.

## Capture and export

CAPTURE starts/stops an internal bounded take of the actual triggered drum events: generated groove, fill, snare roll and direct drums. Performance trigger keys are consumed and never exported as drum notes. Sample-preview auditions are excluded. The limit is32768note-ons; a full take stops safely and reports TAKE FULL. Tempo is fixed to the tempo at capture start; changing host tempo during the take is not represented by a tempo map in this version. Captures are temporary until explicitly exported; save the MIDI before closing the instance.

EXPORT MIDI freezes the take and writes a Standard MIDI File type0at960PPQ, with tempo/time-signature metadata, channel10note-on/off and recorded velocities. File allocation/sorting/writes occur on a dedicated worker, never on the audio thread. Success/failure is displayed. Files go to `Content/Captures/Groove-Dice-<timestamp>-<serial>.mid` inside the installed plugin folder; browse that drive in MPC and import the file into a track using Groove Dice. Disable automatic loop playback when playing the imported notes to avoid doubling. The exported note mapping follows the current drum pad notes; Ride/Crash/Bell replay articulations use reserved notes96/97/98.

`Content/Expansions` and `Content/Captures` are declared as user_data in every Open MPC portable release, preserving packs and captures separately. Public packages must contain neither personal packs nor captured takes. This is file export/import, not live MIDI output: the MPC host discards plugin MIDI output.

## Verification boundary

Build/simulator/allocation/UBSan tests do not prove physical pad input, physical Q-Link movement, hardware audio or MPC CPU load. Record each separately against the exact installed binary and direct captures before declaring device validation complete.


## 1.8.0 — musical phrases and hat rolls

The current implementation extends the earlier eight-slot layout. The new Bank B layout is bottom-up:

| Physical row (left to right) | Assignments | Default MIDI notes |
|---|---|---|
| Bottom B1–B4 | Snare Roll A–D |48,50,52,53|
| B5–B8 | Fill A–D |55,57,59,60|
| B9–B12 | Hat Roll A–D |62,64,65,67|
| Top B13–B16 | LOOP, STOP, unused, unused |69,71,—,—|

These labels describe the tested C-major bank layout, not automatic MPC bank detection. Existing custom learned assignments remain custom. On an older project only the known default LOOP/STOP pair62/64 moves to69/71. When a retained custom note conflicts with a new hat slot, that new slot shows LEARN PAD and remains unassigned; use LEARN PAD to select an unused physical note. Do not alter MPC pad/scale settings.

Each new hat slot has the same Rate, Velocity, Humanize and Length Screen Q-Link controls as the snare slots. Select To Edit selects without playing; physical MIDI still plays. All rolls are one-shot phrases ending at the earliest bar end that accommodates their full selected length. A late trigger defers to the next valid ending. A new snare/hat roll request replaces the previously queued roll; one roll slot is active at a time. A roll can overlay a fill, replacing only its matching snare or hat role. Hat rolls replace the automatic closed/open-hat, ride and bell lane while active; kick/snare remain. Existing hat choke groups apply. LOOP/STOP retain their behavior at their new default notes.

Random generation chooses complete rhythmic cells with rests, call/answer structure, consistent strong/weak subdivisions and an ending accent. The selected division and triplets remain explicit; ACCEL tightens the last half. Some straight snare phrases include one quiet grace hit before the final accent. Velocity variation is small and correlated with phrase accents; Humanize adds bounded timing displacement. Hat rolls use closed hats, preventing random open-hat clutter.

Generated fills retain the current main phrase before the transition and retain its kick motif and2/4snare anchors inside the transition. Ornaments avoid doubling protected snare anchors. Four fill profiles provide snare pickups, a snare-to-tom turn, percussion replies, and a mixed ending. These are rhythmic arrangements of the current kit, not random sample selection. Recorded/edited fills retain their actual notes and automatic locks.

Global Dice regenerates unlocked MAIN and all12performance slots; Amount0 remains inert. Locks, explicit selected GENERATE, Undo, recorded-fill protection, unchanged FX and MIDI export keep their existing meanings. Undo now includes all12sequences/settings. Existing880parameter declarations and GDPF0001trailer bytes are unchanged;37newparameters bring the total to917, with a separate GDHH0001suffix for four hat sequences. Old projects seed only the new hat slots.

Automated/device validation for this candidate is recorded separately in the plugin VALIDATION.md; this section does not assert fresh physical encoder/pad/audio certification.
