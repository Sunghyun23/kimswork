# Groove Dice 1.2.2

A repeatable one-bar drum groove, immediate rule editing and four-layer kick/snare sound design for MPC standalone.

- MAIN repeats one bar. Roll Dice replaces the current groove immediately, at the continuing transport position. Hat Rhythm and RULE changes update the affected part without another roll; other parts and fixed kits stay intact.
- Eight RULE choices per part. Keep the backbeat, change kick placement, introduce ghosts and choose supporting hat/percussion behavior. LOCK preserves the selected part.
- MAIN C now controls Kick / Snare / Hat / Perc mix. SOUND offers four kick/snare layers with Sample / Level / Tune / Shift on each SCREEN Q-Link group. ON, SOLO, LOCK, source/filter/decay/polarity details, audition and eight user kits are included.
- Sample names stay visible; 44px left/right buttons step sounds. Vertical drag and the first Q-Link of each row also select samples. Empty extra layers show + ADD SOUND.
- ROOM is now a stereo diffused room reverb, replacing the old single feedback echo. Existing wet presets intentionally sound different.
- Fixed excessive host notifications while turning RULE Q-Links. Repeated ticks within the same choice no longer rebuild the pattern or notify unrelated layer controls.
- FOLLOW recognizes measured MPC XL record pre-roll behavior, keeping automatic drums silent during count-in. No extra setting is needed. Preview cannot bypass it; direct pads remain playable.
- Kit locking and effective random scope are explicit. Visible cumulative Dice/action counters are removed. Friendly kit/sample names retain stable source identities.
- Recorded fills A–D retain their independent lengths, up to four bars. Pattern rolls preserve recorded fills; explicit Fill/All scope can vary them. Automatic fills play their full chosen duration before returning to MAIN.

## Package and compatibility

Original Workshop only: 128 original synthesized one-shots, 96 kit combinations, two authored beats and 93 timing templates. No private/commercial sample or MIDI library is bundled. Optional .gdexp packs stay separate.

Requires MPC standalone OS 3.10.1+ with Enable Enhancements and External Plugin Support. Copy the whole kim - VST - Groove Dice folder into VST2 Plugins, preferably on internal storage; keep one installed copy across attached drives. Preserve work before restarting. Full manual and verification record are included.

The first 342 legacy parameter definitions remain unchanged; appended controls bring the total to 533. Older states initialize new layers off. Signed Shift requires a fixed reported 30ms latency, including direct pads. Hardware compensation/playing feel is not certified. Save a copy of important projects before updating; older engines cannot reproduce the new layer state. ROOM's wet sound changes intentionally.

## Verification

Native/ARM32/ARM64 builds and SDK validation pass with zero errors/warnings. Engine/UBSan, one-bar/live edits, host count-in fixtures, state/layer/expansion/skin, sample-arrow, reentrant VST bridge and host-simulator checks pass (19 host checks, zero warnings/failures). Room impulse tests cover 44.1/48/96kHz, decay, stereo, bus isolation and process-time allocation guards.

MPC XL / OS3.10.1.12: installed on internal NVMe with all 143 owner-profile files matching. The user confirmed count-in On/Off, Kick RULE stability and the final candidate. This is not certification of every physical control, output route or worst-case hardware load. A stopped recording intentionally started inside a bar may wait for a bar anchor/seek. No ASan pass is claimed because the local ASan runtime fails at initialization. See VERIFICATION.md for historical evidence and limits.
