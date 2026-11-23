# Turntable and whole-project MIDI Swing — local0.11.0

Public downloads and the website remain0.9.3. This document describes the local development candidate; verification is recorded separately in validation-0.11.0.json.

## Audio Turntable

Load a WAV or record PCM into an Audio track first. SOUND B1 cycles INPUT → CHROMATIC → CHORD → CHOP → TURN. TURN keeps the approved320×240 layout: platter and tonearm, actual playhead/direction, Slip status, source waveform and bottom K1/K2/K3 cards. The platter marker follows the real source position.

| Control | Action |
|---|---|
| PAD1 held | Grab the record; the fader controls relative source position |
| Left fader, while grabbed | Forward/reverse scratch; a stationary fader settles to silence |
| Release PAD1 | Resume normal playback |
| K1 | Speed0.50–2.00x |
| K2 | Fader range½ /1 /2beats at current BPM |
| K3 | Track level |
| SHIFT+B3 | Slip On/Off |
| STOP | Stop playback and all scratch audition |

Slip On keeps an independent moving playhead and returns there on release with a short crossfade. Slip Off continues from the scratched position. Fader travel anchors at the current physical position, so grabbing does not jump the sample. If the physical fader has not sent a value yet, its first movement establishes the anchor. Other pads are unused in TURN; do not label them as cue functions. MIX and other pages retain normal fader/pickup behavior. A held PAD1 releases its original owner after navigation.

SHIFT+track pad starts recording/count-in from LOOP; SHIFT+the same pad or REC finishes at the bar. LOOP stays visible; its PAD1 and fader perform the scratch. Recording stores bounded playhead/control snapshots against the unchanged source PCM. REPLACE records a new gesture loop. OVERDUB replaces only actually grabbed regions and preserves untouched gestures; an untouched pass preserves the existing automation. Up to128grab regions and4096events fit one pass. Capacity refusal preserves the previously completed clip, including a full4096-event Overdub with a pad already held at capture start.

INPUT recording remains PCM; CHROMATIC/CHORD/CHOP remain MIDI notes. Existing sampler-note sequences cannot be mixed into TURN with Overdub: use Replace deliberately, or resample first. The saved performance retains its identity when changing the input target. Gate/One Shot controls describe sample notes in other modes; TURN instead uses Grab/release and STOP. Turntable does not apply Warp, Root or Tune, and is not a master effect. Speed/range/Slip are set before capture; level remains a mix control. The fader/grab gesture is recorded; parameter automation is not.

## MIDI Swing

PROJECT (SHIFT+DATA from LOOP) groups K1 BPM / K2 Count-in / **K3 Swing**. K3 selects50–75%;50% is straight. SHIFT+K3 selects an eighth-note or sixteenth-note grid. This controls Synth, Drum and Audio CHROMATIC/CHORD/CHOP sequences together. Recorded PCM, audio Warp, effect LFO clocks and Turntable automation are not swung.

Original event timestamps and velocities are preserved. A preallocated playback schedule applies the timing transform to note-on and note-off, keeps bar boundaries fixed and orders simultaneous releases before attacks. Already generated swung hats carry their timing basis; global Swing replaces that amount rather than adding a second delay. New generation uses straight source timing while global Swing is active. Manual recording keeps its physical note timing as source data; Swing affects subsequent loop playback. Recording must finish before editing global Swing. During playback a Swing edit releases sequenced voices and seeks the revised schedule without replaying already-passed notes; live held input owners remain independent.

Older projects open as **AS REC** until K3 is used, preserving their exact original timing. New projects also start AS REC.50% explicitly straightens the MIDI schedule. No quantization or destructive grid rewrite is implied; performance timing within subdivisions is proportionally mapped. The existing MIDI grid displays original source timing.

## Compatibility and limits

Project format9 adds Swing, per-event timing basis and Turntable control data. Formats1–8 load; older apps reject new format9 saves. Copy/history/save/recall retain the original PCM plus control data. Undo/Redo remains volatile and refers to the latest completed clip edit. Existing8MiB project-metadata and event-count limits still apply. No private banks or owner sessions are included in public packages.

Validation covers native correctness, allocation boundaries and real remote Sample UI/USB separately. Physical fader/pad acceptance, power-cycle recovery, analog input and worst-case16-deck CPU remain separate checks; build success alone does not certify them.

## Approved signal order

Sample PCM → Turntable scratch → track inserts/mixer/sends → master processing → output. Scratch is an Audio-source performance, not a post-master processor. Downstream reverb/delay tails can continue when the fader stops; physical STOP has the established engine-stop semantics. The same track pan/routing/sidechain and stereo USB pipeline applies.
