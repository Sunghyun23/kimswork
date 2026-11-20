# Common action footer

All three plugin skins use the native1280×628body. Footer y578, height44; UNLOCK ALL x16/w160, ROLL DICE x188/w240, UNDO x440/w128. Keep the native function-key bar below the plugin body. The shared geometry is in action-footer.json; tests/test_action_footers.py checks all19native pages plus the Groove Dice fill editor and verifies guides end above the footer.

Theme, artwork and Q-Link parameter assignments remain product-specific. Modals hide underlying guides and actions. Do not place Dice locks on controls outside the documented random scope.

Donut Theory: shared sound-only generation and exact Undo keep their existing policy. POLY combines sound/chord browsing into one row, retains70px chord pads and moves page arrows beside the common footer. CHORD reflows rows/utilities without removing controls. Main and CHORD sound browsers remain category-first.

Groove Dice: common actions keep the existing global MAIN/pattern/unlocked-performance-slot scope. FX remains separate ROLL FX. SOUND keeps separate ROLL SOUND/UNDO SOUND/AUDITION/SAVE KIT in SOUND ACTIONS; sample-row Preview and physical audition remain directly available. Sound1/2 arrows remain beside the common bar. PERFORM retains capture/export at its right. No new engine random scope is introduced by moving buttons.

Crate Press: Dice randomizes13MAIN character values: Drive, Compress, Punch, Release, Low/Mid/High Comp, Duck, Tone, High Cut, Lo-Fi, Drift, Low Cut. Each has one shared lock across views. Restrained generation ranges; Low Cut0–65Hz, High Cut4–18kHz. Preserve Input, Output, Mix, Bypass, Peak Guard, LATCH, performance gates and all performance setup/other settings. Undo restores exact pre-Dice normalized values; Unlock All clears protection only. Commands run on VST command writes, audio or non-render UI idle under a non-waiting atomic ownership guard, so empty/stopped audio tracks respond immediately. No render-thread callbacks or allocation. Factory selection/state restore clear pending actions/Undo, retaining saved locks. First380parameter declarations retained; new commands380–382, locks383–395;396total. Eight performance pads move upward50px without changing size or hold/LATCH behavior; LATCH/curve utilities share the footer's right side.

Donut1.15.2 and Groove1.8.1 remain local device candidates; their public versions stay1.15.0 and1.8.0. Crate0.2.6preview publication and site update are authorized separately on2026-11-20. No SD or SSD operations are authorized.
