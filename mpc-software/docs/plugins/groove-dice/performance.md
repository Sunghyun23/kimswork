# Performance, capture and MIDI — 1.7.0

Status: release implementation. Actual MPC XL remote touch and MIDI-export checks completed; physical pad/encoder tests and full host MIDI-file import remain outside the verified scope. Vendor kim / UID KpGy and the first782parameter declarations remain unchanged. This specification supersedes the1.5.1 policy that global Dice always preserves every fill.

## Workflow

MAIN retains its four Q-Link groups (16 controls). PERFORM is reachable directly from MAIN's bottom PERFORM button; the first native function key also cycles MAIN/PERFORM without consuming another physical function key. Existing FILL note editing and recording remain accessible. The other five native keys and their mappings stay unchanged.

PERFORM mirrors a4×4MPC pad bank, numbered bottom-up:

| Top to bottom | Left to right |
|---|---|
| Fourth row | B13–B16 reserved |
| Third row | B9 Loop on/off, B10 Stop, B11–B12 reserved |
| Second row | B5–B8 Fill A–D |
| First row | B1–B4 Snare Roll A–D |

Bank names are physical-location hints, not MIDI bank messages. Plugins receive note numbers, not the physical bank name. The initial note assignments are52,53,55,57,59,60,62,64,65,67 for the ten occupied pads; these follow the intended next-bank C-major mapping. Different MPC scale/root/custom pad layouts require LEARN PAD. LOOP/STOP learn buttons are on ROUTING. Duplicate performance assignments swap; drum-note/reserved-articulation collisions are rejected, and existing direct drum mappings take priority. The plugin never changes MPC Pad Perform settings. Existing A-bank drum assignments remain intact.

## Playing and editing

- Roll: press and hold, release to stop; substitutes the existing snare while retaining other drum parts. Four generated roles begin as1/16,1/32,1/16T and accelerating.
- Fill: tap to queue at the next beat, play the selected1/2/4-beat variation once, return to the underlying loop. Generated fills retain the inherited main prefix for full-bar note editing/audition, but PERFORM plays only the variation tail. Direct recordings and legacy fills play their full stored length. New fills replace queued fills; they never stack. A held snare roll may replace the snare part of a fill.
- Loop: toggle internal preview when the host is stopped; while following a running host it gates this instance at a bar boundary. STOP cancels this instance's performance, not MPC transport.
- Host count-in remains authoritative and cannot be bypassed by loop, fill or roll triggers. Normal direct drum monitoring is unaffected.
- SELECT TO EDIT makes pad touches select a slot without sounding. Selected Rate/Velocity/Humanize/Length use one four-knob Screen Q-Link group. Recorded/legacy fills retain their note data: Rate is inactive, Velocity scales playback, Humanize is non-destructive, and Length uses the existing1/2/4/8/16-beat control. New recordings/edits start at unity velocity and no additional humanization. Other groups are unassigned on the native PERFORM page. On MAIN's direct performance panel the MAIN relays temporarily target these four values and the remaining twelve are inactive.
- GENERATE explicitly regenerates the selected slot. LOCK protects against bulk/global generation; it does not prevent manual editing or explicit selected-slot generation. Lock status is shown on pads and in the selected-slot assignment line.
- REC FILL appears for a selected fill and records physical drum input. EDIT NOTES in MAIN’s direct PERFORM panel opens the existing note editor; the native PERFORM subtab focuses on performance and capture. Finishing a recording or editing a note locks that fill automatically.

## Dice and persistence

ROLL DICE regenerates the eligible MAIN groove and unlocked fills/rolls together at the next bar while playing, immediately while stopped. Existing MAIN/kit/pattern/role protections remain. Fills inherit the current main phrase before their ending variation. Roll roles retain their rate/length settings and regenerate rests/accents; open slots vary Velocity and Humanize within Random Amount bounds, so the change is visible without losing the chosen rhythmic role. FX are preserved. Random Amount0 leaves all musical data unchanged. UNDO restores the complete prior groove and eight performance sequences. Directly recorded/edited fills are protected automatically.

Old four fill banks are reused, never deleted. Loading a pre1.7project automatically locks every populated legacy fill, preventing a new Dice policy from destroying older recorded performances. New roll sequences are appended in a separately versioned extra-state trailer. Existing state layouts/UID and all782published parameter IDs/ranges remain unchanged. Triggers, recording commands and screen press gates are transient and save as idle. Fresh/resumed instances return to MAIN without startup messages.

## Capture and export

CAPTURE starts/stops an internal bounded take of the actual triggered drum events: generated groove, fill, snare roll and direct drums. Performance trigger keys are consumed and never exported as drum notes. Sample-preview auditions are excluded. The limit is32768note-ons; a full take stops safely and reports TAKE FULL. Tempo is fixed to the tempo at capture start; changing host tempo during the take is not represented by a tempo map in this version. Captures are temporary until explicitly exported; save the MIDI before closing the instance.

EXPORT MIDI freezes the take and writes a Standard MIDI File type0at960PPQ, with tempo/time-signature metadata, channel10note-on/off and recorded velocities. File allocation/sorting/writes occur on a dedicated worker, never on the audio thread. Success/failure is displayed. Files go to `Content/Captures/Groove-Dice-<timestamp>-<serial>.mid` inside the installed plugin folder; browse that drive in MPC and import the file into a track using Groove Dice. Disable automatic loop playback when playing the imported notes to avoid doubling. The exported note mapping follows the current drum pad notes; Ride/Crash/Bell replay articulations use reserved notes96/97/98.

`Content/Expansions` and `Content/Captures` are declared as user_data in every Open MPC portable release, preserving packs and captures separately. Public packages must contain neither personal packs nor captured takes. This is file export/import, not live MIDI output: the MPC host discards plugin MIDI output.

## Verification boundary

Build/simulator/allocation/UBSan tests do not prove physical pad input, physical Q-Link movement, hardware audio or MPC CPU load. Record each separately against the exact installed binary and direct captures before declaring device validation complete.
