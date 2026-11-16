# Groove Dice 1.2.2 — one-bar loop and live rules

The main instrument plays one repeating bar. Roll Dice replaces the active loop in the current audio block without restarting transport or replaying elapsed beats. Pattern slots remain three alternate loops; changing the active loop does not overwrite the other two. There are no automatically alternating four-bar responses.

Main playback uses the first bar of selected MIDI and legacy patterns. Existing longer saved notes are retained when loading; generating/editing a loop commits a one-bar result. Dotted hat divisions restart at the one-bar boundary. `+ FILL` and `+ ROLL` in Hat Rhythm are small ornaments inside that same repeating bar; they are separate from recorded Fill A–D.

## Live controls

Division, its fixed ornament and rule-style edits update the affected part at the next process block. Other parts, sample assignments, song position and recorded fills are preserved. The seeds remain fixed until Dice/groove randomization changes them, so editing hats does not silently reroll the kick. Existing timing/FX/mix controls remain live. A past beat is not retriggered; the next eligible event uses the new settings. MAIN displays `LOOP 1/1` and shows `FILL x/y` while a fill is playing.

Density/velocity rules and the selected style can limit what a control changes. LOCK holds that part's notes. Pattern Lock protects against Dice regeneration; explicit editing remains available. KIT FIXED holds sample choices/layers/mix during Roll Dice; it does not freeze manual sound selection or the explicit FX scope.

## Rules

Each MAIN B Q-Link selects eight choices for its part. Common first four: LOCK, BASIC (core rhythm), VARIED (bounded decoration), BUSY (denser decoration). Ghost controls affect the extra decoration in VARIED/BUSY; the explicitly named GHOST modes select fixed ghost placements. Part-specific choices:

| Part | Additional choices | Result |
|---|---|---|
| Kick | DOWNBEAT | One kick on beat 1. |
| Kick | OFFBEATS | Beat 1 plus offbeat placements, excluding beats 2 and 4. |
| Kick | SYNCOPATED | A syncopated five-hit motif, excluding beats 2 and 4. |
| Kick | DOUBLE HIT | Paired kick attacks within a bounded motif. |
| Snare | SOFT | Quiet backbeats on beats 2 and 4. |
| Snare | GHOST BEFORE | Backbeats plus a quiet pickup before beat 2. |
| Snare | GHOST AFTER | Backbeats plus a quiet response after beat 4. |
| Snare | DOUBLE GHOST | Backbeats plus two quiet pickups. |
| Hat | SPARSE | Every other event from the selected division. |
| Hat | OFFBEATS | The selected division offset by half a step. |
| Hat | ACCENTED | Strong main beats with quieter intervening hats. |
| Hat | OPEN HAT | An open-hat ending with the standard shared choke group. |
| Perc | SPARSE | A single quiet percussion hit. |
| Perc | OFFBEATS | Multiple percussion voices on offbeats. |
| Perc | ANSWER SNARE | Percussion responses after the backbeats. |
| Perc | PERC LOOP | A small loop spread across the four percussion voices. |

Short LCD captions abbreviate long names; the touch drawer shows full choices. Division remains independent of the Hat style. Rule styles govern rhythm; sound layers and their locks remain in SOUND.

## Fills

A–D are separate recorded MIDI banks. They retain velocities, timing, gates and their chosen lengths (1 beat,2 beats,1 bar,2 bars,4 bars). Only physical/touch performance inputs enter fill recording; automatic MAIN output is not recorded back into it.

- Press FILL: the selected slot starts at the next bar, plays once, then the same main loop resumes at the continuing song position.
- Fill Every: Off/4/8/16 bars. A fill ends at the interval boundary, starting early enough to play its complete chosen length. A two-bar fill every four bars therefore occupies bars3–4; bars1–2 play MAIN.
- MAIN and recorded fill events are mutually exclusive within the fill window. Main rule density budgets do not drop recorded fill hits. Fill Density and timing/performance settings still apply.
- Main-pattern Dice leaves A–D alone. Kit + Fill / All explicitly randomize the selected fill, except during active fill recording. UNDO restores the preceding Dice result.
- REC FILL retains Replace/Overdub, count-in and automatic recording completion. Cancel ends queued/playing/recording fills. Existing fill slots are not shortened to one bar.

## Compatibility and verification

Four style parameters are appended at indices521–524. The first521 parameter IDs and order remain; original four-choice rule automation ranges are preserved. Old521-parameter chunks derive new styles from the old rule values. The GDLY0001 layer trailer remains the same size and excludes rule styles; sound-kit load and native sound presets do not overwrite rules.

Automated regressions cover stable repeated scheduled events, live mid-bar Division and Q-Link changes, unaffected-part preservation, immediate Dice/Undo,32style selections,521-parameter state migration, current byte-identical state round-trip, kit/rule independence and manual/automatic fills including full-length two-bar playback. Actual device verification status is recorded in the project VALIDATION.md rather than inferred from these tests.

## Physical test incident

The user reported MPC shutdown on a physical Kick RULE rotation in1.2.0. Automated one-bar tests alone do not establish hardware stability.1.2.1 removes a reproduced host-notification storm; its physical retest status is recorded in VALIDATION.md. No public release was issued for the unstable build.
