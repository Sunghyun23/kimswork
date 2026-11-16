# Groove Dice 1.2.2 — Sound layers

Released in 1.2.2. Vendor `kim`, UID `KpGy`. This supersedes MAIN C sample selection and the former DRUMS editor in this version. No new bank payload format or merged expansion is required.

## Workflow and controls

MAIN C is **DRUM MIX**, with Kick / Snare / Hat / Perc level trims, −48 to +6 dB. Hat controls Closed/Open together; Perc controls Clap/Tom/Cymbal/Perc together. These trims preserve the kit's internal balance and apply to separate outputs as well as the main mix. Kit browsing preserves these trims.

Open **SOUND** to select KICK / SNARE / HI-HAT / PERC. Kick and Snare each have four parallel layers. SCREEN Q-Link groups A–D match the four visible rows; each row is **Sample / Level / Tune / Shift**. Selecting a sample for an empty extra layer enables it. Existing kits start with only their original layer enabled. Levels −48..+6 dB, Tune ±12 semitones, Shift ±30 ms.

Each sample cell displays its current name. Tap **<** or **>** to step one sound in the selected pack filter, drag the name field vertically, or turn the first SCREEN Q-Link for that row. Extra layers include **+ ADD SOUND** (empty/OFF); choosing a sound enables the layer. The two arrows do not change the selected part or other layers. Hat groups C/D remain detail controls, with no sample arrows.

Hi-hat remains Closed/Open with its existing choke behavior: groups A/B edit Sample/Level/Tune/Shift, groups C/D edit each voice's Start/Low Cut/High Cut/Decay. Perc A–D edit Clap/Tom/Cymbal/Perc; no extra stacked voices are added to Hat/Perc.

Each layer/voice has ON, SOLO and LOCK. SOLO isolates within the selected part; LOCK protects its sample from ROLL SOUND and main kit randomization. Four layers belonging to one hit share velocity, output route and choke ownership, so they never choke one another. A later logical hit still applies the existing choke group.

Select a row and open **DETAILS** for Start, Low Cut, High Cut, Decay and polarity inversion. The first physical Q-Link group controls those four details; other groups do nothing while this modal is open. The modal hides background Q-Link bounds. Start trims the sample's beginning; Shift changes its onset. Program/Track remain fixed MAIN controls; SOUND context requires SCREEN mode.

Bottom actions: **ROLL SOUND** changes enabled, unlocked sample choices in the selected part and current pack filter, preserving rhythm, mix, tune and timing (MAIN KIT LOCK must be off); **UNDO** restores the previous roll snapshot; **AUDITION** plays the selected part without recording a physical pad event; **SAVE KIT** stores the current configuration in the selected USER KIT slot (1–8). Existing LOAD KIT on KITS restores it. This is project/plugin state, not a portable .gdexp authoring/export command.

## Timing cost

The engine reports a fixed **30 ms latency** at the current sample rate, for every role. A Shift of −30/0/+30 ms starts at 0/30/60 ms after the incoming event. This keeps relative negative offsets possible for live MIDI without trimming the sample or predicting a future pad hit. Direct pad monitoring therefore has this baseline delay; do not describe it as zero latency. MPC compensation and physical playing feel require separate hardware assessment. Filters, source offset and choke begin at actual voice onset.

## Compatibility and implementation

The first 342 parameter declarations are preserved byte-for-byte; 179 are appended (521 total). MAIN C's existing Q relay targets now address the new mix trims. Actual legacy sample selector parameters remain available, so old saved sample choices remain intact; old automation of MAIN C relays now follows the new mix mapping. SOUND relay/view parameters are not automation targets.

A fixed `GDLY0001` suffix extends the existing state: current and eight saved-kit layer/mix settings, stable sample identities and unresolved references. Old 342-parameter states retain notes and existing sounds, with new layers OFF and mix trims 0 dB. Missing expansion layers mute and retain their identity across resave/reinstallation. New layer states require the1.1 engine; downgrading those saved states to1.0 is not supported. Expansion payloads stay separate, with public/private packaging unchanged.

Async sample cache: 14 request lanes, 64 fixed cache slots and 64 fixed voices. File reads, allocation and catalogue construction are outside process(). Main/voice level changes are smoothed; mute/solo fade already sounding voices. Tune/Shift/source/filter/decay changes take effect at the next hit.
