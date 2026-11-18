# Extended RULES — 1.4.0

Each MAIN B part now offers16playable styles, with no LOCK value. The first seven keep their earlier musical meaning; nine new rules per part add36patterns. The padlocks remain separate. Use the existing Q-Link or tap the value for the two-page left list. MAIN reading order, four Q-Links and guide bounds stay unchanged. The footer shows PAGE1/2 and PAGE2/2. Selecting a style updates the live one-bar pattern without pressing Dice; Dice can choose the new styles while honoring locks and Amount bounds.

## Kick

BASIC, VARIED, BUSY, DOWNBEAT, OFFBEATS, SYNCOPATE, DOUBLE HIT, ONE THREE, ANTICIPATE, LATE ANSWER, PICKUP, THREE HITS, FOUR HITS, SYNC PAIR, STAGGER, END DOUBLE.

## Snare

BASIC, VARIED, BUSY, SOFT, PRE GHOST, POST GHOST, 2 GHOSTS, SECOND UP, FIRST UP, LOW ACCENT, POST BOTH, EARLY PAIR, LATE PAIR, TRIPLET, DRAG IN, BAR TURN.

## Hat

BASIC, VARIED, BUSY, SPARSE, OFFBEATS, ACCENTED, OPEN HAT, FIRST HALF, LAST HALF, ON BEATS, UPBEATS, TWO ON ONE, ONE ON TWO, SOFT TAIL, CRESCENDO, SKIP TWO.

## Perc

BASIC, VARIED, BUSY, SPARSE, OFFBEATS, ANSWER, PERC LOOP, CLAP REPLY, TOM TURN, SHAKER, CALL REPLY, OFFBEAT TOM, CLAP PAIR, LATIN THREE, TOM CLAP, END PICKUP.

## Musical behavior

Kick variants use distinct bounded sixteenth-note motifs, keep the downbeat and avoid the snare's exact2/4beats. Snare variants always keep both backbeats, changing main accents and adding quiet pre/post, triplet, drag or bar-ending ghosts. DRAG IN alone permits close32nd ghost spacing; caps and duplicate/backbeat protection remain. Hats keep the separate Hat Rhythm division, shaping halves, beat placement, omissions or crescendo/tail accents. Explicit FILL/ROLL ornaments remain controlled by Hat Rhythm. Perc uses role7percussion, role5toms and role4claps in distinct call/reply motifs; sound kit/choke routing is unchanged.

The repeating MAIN bar remains one bar. Recorded fills and other parts are preserved by a live part edit. Rules do not modify direct pad/MIDI playing. Existing Hat/Open/Ride/Crash and count-in behavior remain.

## Compatibility and checks

All672previous declarations/normalized ranges remain exact. Four16-choice controls append at672–675(total676); old4/8/7-choice aliases keep their ranges. Old state seeds the appended selectors; newer state retains them. Old aliases remain available for automation; legacy LOCK migration still becomes BASIC plus a closed padlock. Saved tracks may cache the previous seven-choice map; reset to defaults or compare a new instance to reach all16through Q-Link. Full-range controls, Dice and Undo share the same canonical styles. Legacy fallback values are readback aliases, not extra current styles.

Native/ARM32/ARM64 and SDK validation pass; full test/UBSan/bridge/skin/host suite is required. Targeted tests compare36distinct raw and36distinct actual scheduled new patterns, with variable32/512blocks, backbeats, kick exclusions, other-part/fill preservation and old672/full-state migration. Actual-device outcomes follow separately. Owner installs must preserve all four existing expansion banks and registry; never publish personal packs. Public stable remains1.3.4 until an authorized new release.

## Verification

1.4.0 is installed on the MPC XL internalNVMe. Direct remote UI checks cover the two-page list, selection, live knob adjustment and Dice, with actual captures under plugins/public/GrooveDice/design/device-1.4.0. Automated checks cover36distinct new scheduled motifs, all240hat-style/division combinations, legacy state migration and real VST full-range/fine-step/reentrant notification behavior. Full regression, native/ARM builds and SDK checks pass. Owner expansions are preserved. Physical encoder rotation and hardware audio/CPU are not newly certified.
