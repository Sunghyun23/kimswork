# Groove Dice 1.2.3 local update — beat-position velocity

User-approved: preserve an intentional strong / weak / medium / weak hierarchy instead of broad random loudness. No new mode or UI control.

Generated hats use90/45/68/40at quarter-beat positions (16th subdivisions), repeating each beat. Quarter/eighth/dotted divisions sample this same metrical grid; intermediate32nds are supporting taps at most38. ACCENTED uses100/35/66/30. Fill/roll ornaments within MAIN use the same metrical hierarchy. Kick downbeats are105, eighth offbeats84and16th connections64. Snare backbeats remain100(or68in SOFT), with quiet ghost hits. Percussion stays supportive with position-based levels.

Vel Spread remains0–50in GROOVE, mapped to at most ±6MIDI velocity units and further capped by BASIC/VARIED/BUSY at2/4/6. LOCK adds no variation. A fixed seed gives identical repeated bars, and the strong>medium>weak ordering cannot invert. Zero Spread gives exact generated accents. MIDI input, saved/recorded note velocities, sample-layer ownership and existing parameter identities remain intact; recorded fills have no added Spread. Authored/imported MAIN velocities remain their base values rather than being rewritten into the generated accent pattern.

The local1.2.3engine updates generation for new Dice/live edits. Existing saved notes are not silently rewritten on project load; use a Pattern-scope Dice or change the unlocked part rule/division to generate its updated accents. Keep KIT FIXED to preserve sounds. The public1.2.2release and website stay unchanged until publication is requested.

Verification: all15hat divisions ×7unlocked styles,100kick generations, every Spread amount with rule caps and1000seed values, actual repeated-bar scheduler, authored fill velocities and allocation guards. Full regression/build results are recorded in project VALIDATION.md.
