# Sound selection, audition and Dice source range

Release 1.3.4. Public package contains only Original Workshop; owner-only expansions remain separate.

## SOUND

Choose KICK, SNARE, HI-HAT or PERC. Tap a sample name to open the left sample list; dragging the name is no longer required. Previous/next arrows and the sample Q-Link use the same catalogue and selected source. In the list, SCREEN A1 selects the sample; other SOUND relays are inactive. PREVIEW auditions the selected sample; DONE closes the list. Pack arrows filter the list. The modal suppresses background Q-Link guides and remains closable after changing tabs.

All sound rows use Sample / Level / Tune / Shift on SOUND1/2, and Start / Decay / Low Cut / High Cut on SOUND2/2. The 48px previous/next arrows change every row together; the counter shows the current page. The same SCREEN group stays with its row. HI-HAT has two rows (C/D inactive); CYMBALS has three (D inactive). REV is available on both pages; page2 adds independent polarity Ø. Decay shortens the tail smoothly; no End parameter was added.

Each sample row has its own PREVIEW. It plays only that layer, not the entire kick/snare stack, and it never records a fill note. Row mute/solo and MAIN part trim are bypassed for audition; sound level/tuning/filter/shift, master and FX still apply. Audition goes to Main so it can be heard even with Separate Out. Preview voices do not choke performance voices. A newly selected asynchronous sample must be ready before audition; it does not audition the previous cached sound. An empty extra layer has no sound to preview.

## CYMBALS

SOUND > CYMBALS opens Ride, Crash and Bell. A/B/C each follow their sound row on both SOUND pages; D is inactive. ON/SOLO/LOCK are independent per cymbal. Sample list, arrows, Q-Link and PREVIEW work here too.

AUTO / CURRENT KIT chooses a named family from the same pack as the kit's Cymbal source. BUILT-IN chooses the independently synthesized original fallback. Manual source choices include the matching named family and unclassified Cymbal sources, excluding sources known to belong to a different family. Missing chosen assets show MISSING SOUND and stay silent; they are not silently replaced. State and user kits keep sample SHA identities so reinstalling a pack reconnects the source.

Ride and Bell follow Hat Mix; Crash follows Perc Mix. All three share the Cymbal role's output routing, pan and choke assignment. They are independent articulations, not extra MIDI note roles or extra output buses. Unlocked sound Dice returns them to AUTO; their LOCK keeps a manual source. KIT FIXED preserves all sound selections.

## Dice range

MAIN has USE SOUNDS FROM:

- THIS PACK uses the current pack filter. If the filter reads ALL EXPANSIONS, its pool includes all installed packs; choose a named pack with the adjacent arrows to restrict it.
- ALL PACKS includes core and all installed expansions, including private owner-only packs on that device.
- SELECT KITS opens a checkbox list. Select up to 64 specific kits across packs. The set is stored by stable kit ID. CLEAR empties it; DONE closes it. An empty or unavailable set does not borrow sounds from outside the selection.

Whole Kit chooses one kit from the range. Mix Parts chooses Kick, Snare, the linked closed/open Hat pair and the four-role Perc ensemble from eligible kits. Active unlocked extra kick/snare layers also stay inside the selected range. Part/layer locks and KIT FIXED are respected. Scope still decides whether rhythm, groove, fills or FX change alongside sounds. Directly recorded fills change only under a fill-changing scope.

## Fill editing

The fill note grid starts with Snare, Tom, Perc and Clap. PERC / DRUMS switches to Kick, Closed Hat, Open Hat and Cymbal. These are two views of the same recorded data, not separate fills. All eight roles can be selected or created in the grid. Bar selection and selected-note timing/velocity/gate/chance Q-Links remain available.

## Verification boundary

Native/ARM builds, SDK validation, UBSan, allocation guards, saved-state identity tests, one-bar/record/count-in fixtures and UI bounds/guide checks are automated. Direct MPC XL remote clicks verified text-center sample and kit selection, page navigation, REV/polarity and selected-kit checkboxes. Published captures are device frames. Physical encoder feel, hardware audio and worst-case load remain outside this remote UI verification.

## Visible Dice controls and individual locks (1.3.2)

Each MAIN knob has a 44×44 padlock target. Cream/open means Dice may change that value; orange/closed means Dice preserves the exact parameter value. Manual touch/Q-Link editing, preset selection and Undo are still available. Locks are stored in the MPC project and are not themselves randomised or reverted by Undo.

Roll Dice now changes eligible visible MAIN values rather than hiding new timing offsets behind unchanged readouts. All 16 controls participate independently of the extra-change choice:

- A Timing: Swing 50–66%, Snare Late −2–28ms, Hat Shift −12–8ms and Random Nudge 0–6ms. Amount narrows these absolute ranges instead of allowing a random walk. Hidden legacy offsets are cleared only for timing columns that are rolled.
- B Rules: choose musical styles without randomly choosing LOCK. The old Hat OPEN HAT alias is not randomly selected, so it cannot override a locked independent Open Hat control. A legacy RULE=LOCK remains protected.
- C Drum Mix: Kick −2–0dB, Snare −4–1dB, Hat −8–−2dB, Perc −12–−4dB. Amount narrows the ranges. Kit Fixed also protects these trims.
- D Hats/Cymbals: weighted divisions, dotted/fill/roll endings and sparse Open Hat/Ride/Crash choices. Discrete random selections may happen to repeat their current choice. Ride still replaces automatic hats, and crashes remain structural accents.

Pattern Lock additionally protects B/D and pattern regeneration. Existing per-part rhythm LOCK prevents random style changes for that part; it also protects associated hat/cymbal choices and shared timing changes that would move the locked rhythm. Individual locks alone protect a parameter, not the underlying note sequence: use Pattern Lock when the entire pattern must stay fixed.

The former Scope selector is displayed as Extra: None / Pattern / Groove / Fill / All / FX. It chooses supplementary operations; it no longer implies that visible unlocked MAIN controls remain unchanged. Kit sound selection is separately controlled by Kit Fixed and the selected sound pool. The No extras option still rolls eligible MAIN values and unlocked kit sounds. Amount zero performs no randomisation. Directly recorded fills are preserved unless the selected extra choice includes Fill.

Undo restores all 16 front values, sources, note data, seeds and legacy timing offsets, while retaining the current padlock configuration. Existing 623 parameter declarations stay byte-for-byte unchanged; 16 non-automatable, persistent Dice Lock parameters append at indices 623–638. Sound-bank format and expansion contents are unchanged.

## RULES without a LOCK value (1.3.3)

The four B controls now offer seven playable styles each. LOCK is absent from the touch lists, full knob travel, SCREEN Q-Links and default Program/Track Q-Link assignments. Randomisation protection is configured with the adjacent padlock, rather than an extra style value. Short ghost/answer names fit the value displays.

Previously saved LOCK states appear as BASIC with the corresponding padlock closed. Their existing frozen rhythm is preserved for compatibility. Opening the padlock releases that legacy hold into BASIC; manually selecting another style also replaces it. New selectors cannot create a legacy LOCK value. The existing eight-state parameter IDs remain internally for old project/automation compatibility; four seven-state selectors append at 639–642. The old 0–638 definitions are unchanged, and old SCREEN selector positions are converted to the seven-state range when loading an older project.
