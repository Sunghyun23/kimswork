## Current1.4.0 —16RULES per part

Existing seven styles stay in place; nine new choices per part add36distinct generated/scheduled patterns. The four full selectors append after672legacy declarations. [Specification and compatibility](../../../docs/plugins/groove-dice/rules.md). Release1.4.0 is approved. Final automatic/device evidence is recorded below and in design/device-1.4.0/verification.json.

# 1.3.0 local update — HATS / CYMBALS

MAIN D moves colour FX to the existing FX page and maps Hat Rhythm/Open Hat/Ride/Crash. First533parameter declarations are unchanged;536total. New rules have separate classified Ride/Crash/Bell sources, pinned cache lanes and original procedural fallbacks; they share the existing Cymbal output route. Ride/Bell use Hat Mix; Crash uses Perc Mix. No ninth bus/physical pad or manual auxiliary sample selector is added.

Targeted UBSan/allocation-guard tests pass for Open Hat modes/replacement, all Ride/Bell divisions, Crash start/4/8/16bar boundaries, full-length fill endings without note-data mutation,100Dice retaining selections, state roundtrip and Density0. Owner-pack tests pass on all30kits: all240base assignments plus Ride/Crash/Bell family resolution, separate cache/voice activity, real isolated cymbal-bus audio and no main-output leak. Full regression passes, including count-in fixtures with new rules enabled,4004fine MAIN D VST increments (all15/4/5/6choices), stable bounded notifications, engine/state/layer/expansion/skin checks and host simulator19passed/0warnings/0failures. Native/ARM32/ARM64 builds and SDK validation pass (0errors/0warnings). Additional final tests cover original533-parameter state migration, existing custom choke groups, missing-sample muting and actual Ride Hat Mix energy. The earlier transfer failed and both prior addresses became unreachable; the original cause is not established. After wired192.168.0.56was supplied, the official SDK update succeeded on internal NVMe:146files matched by size; independent SHA256 matched ARM32/ARM64, TUI/Q-Links and Cosmic/Workshop/Splat banks. Installed metadata1.3.0was read before restarting. SDK restart returned running again. The .56 connection then stopped responding; mpc-xl reconnected on .64 and its installed1.3.0metadata was re-read after restart. This is file/device confirmation, not a GUI/audio certificate. Payload1,583,263,466bytes /30.135seconds =52.54MB/s (~420.31Mbps), timed from local install-log birth/last write and including packaging/copy/verification overhead. Interface enu1u1reports speed=-1/full duplex; ethtool provides no nominal rate. Physical Q-Link rotations, screen capture and direct listening of1.3.0are not claimed by these tests. No public release/website update is part of this local change.

# 1.2.3 local velocity update (not publicly released)

Generated MAIN uses metrical strong/weak/medium/weak accents, including fresh-instance hats. Hats90/45/68/40; kick105/84/64; original snare backbeats/ghost distinction retained. Vel Spread is bounded to ±6with per-rule caps and cannot invert strong/medium/weak. Fifteen divisions and all seven unlocked hat styles are covered. Recorded fills/direct MIDI keep their played velocities; stored old project notes are not silently rewritten. See ../../../docs/plugins/groove-dice/velocity-patterns.md.

New accent/humanization tests pass with UBSan and allocation guards: every Spread amount and rule cap over1000seeds, repeated actual scheduler,100kick generations, authored fill preservation and fresh-instance defaults. Final full engine/UBSan/state/fill/layer/expansion/skin regressions pass. Native/ARM32/ARM64 builds pass and SDK validation reports zero errors/warnings. Host simulator:19passed,0warnings/failures. The owner profile was installed on MPC XL internal NVMe;146files matched, existing owner packs were retained, and the MPC app restarted. Dedicated physical playing/listening of this velocity update is not claimed. Owner-only expansion verification covers all30kits/240sample-role assignments, non-silent PCM decoding, filtered Dice and state roundtrip. Private payloads/import tools/reports remain Git-ignored; public release and website stay at1.2.2.

# 1.2.2 local candidate — sample selection and ROOM

The user confirmed the installed final candidate and authorized publication on 2026-11-17. The final owner profile was installed on MPC XL internal NVMe and the app restarted; 143 files matched. This user confirmation does not imply an exhaustive per-control listening/physical matrix.

SOUND sample cells now show the current sample name, 44×44 previous/next buttons, and a drag/Q-Link hint. Transparent filmstrip input replaces the native ValueSlider text that could obscure the custom name. Existing SCREEN groups retain Sample / Level / Tune / Shift. Eight non-automatable command parameters are appended after the 525 existing parameters (533 total); fixed layer-state storage still ends at the original layer boundary. Selecting an extra-layer sound enables it; selecting None clears it. Base selection resolves the legacy source identity before stepping, so the first arrow changes the actual sound. Sample changes clear missing-kit references.

ROOM now uses an original stereo diffused, damped four-line feedback-delay-network reverb instead of the former single 4096-sample feedback echo. Each of eight output buses owns independent room state. The existing ROOM parameter identity is unchanged. This deliberately changes the wet sound of saved presets. Tests at 44.1/48/96 kHz verify finite output, decaying diffuse tails, stereo decorrelation, reset silence, bus isolation and no process-time allocations. The user subsequently accepted the installed candidate; an independent exhaustive physical listening matrix was not recorded.

Visible action/Dice cumulative counters were removed: labels report READY, DICE APPLIED, KIT FIXED or the last action. Internal bookkeeping is retained. Full regression and host simulation pass (19 passed, zero warnings/failures); UBSan sample-arrow, layer/state and live-rule bridge checks pass. Final native/ARM32/ARM64 assembly and SDK validation pass (zero errors/warnings). Post-edit sample-arrow, layer/state, layer-name and real VST live-rule bridge regressions pass. The owner profile was installed on MPC XL internal NVMe; all 143 files matched on remote verification. Its private expansion remains local to the owner profile. App restart was requested through the official SDK CLI. No ASan pass is claimed.

# 1.2.1 candidate — physical Kick RULE shutdown investigation (2026-11-17 KST)

**1.2.0 is not hardware-verified/stable.** The user reported the MPC shut down while turning the physical Kick RULE Q-Link. The device was reachable again, but the previous boot journal was not persistent, core dumps were disabled and no pstore entry remained. The exact shutdown cause cannot be claimed from those logs.

A reproducible VST-side defect was found:20tiny movements within the same style emitted5040host automation notifications, including2720unrelated layer notifications. A semantic style change emitted252notifications including136layer notifications. Live pattern refresh had called the full audio/kit notification fanout, and discrete settings were rebuilt even without a new choice. Kit-row selection flags were also marked dirty on every block.

The candidate deduplicates style changes by integer choice, refreshes only necessary loop metadata and only dirties actual kit-row changes. Under the initial matched bridge test,20same-style movements now emit0notifications/0layer writes; a style change emits3notifications/0layer writes. The expanded test includes actual VST setters/getters, a separate playing audio thread, all4x8relay/direct style selections,4004small normalized inputs and reentrant host reads/rule-value echoes. Notifications remain bounded. Full regression and host_sim19PASS/0WARN/0FAIL pass. Native/ARM32/ARM64 and SDK validator pass (0errors/0warnings).

AddressSanitizer cannot run in this macOS environment: an empty standalone ASan executable fails during sanitizer initialization with the same sanitizer_malloc_mac.inc assertion as the instrumented test. No ASan memory-safety pass is claimed; UBSan and allocation guards pass.

The user physically retested the Kick RULE Q-Link and confirmed the device stayed running. Device trace records also showed completed style transitions with exactly three notifications per change. This verifies the reported input path on the candidate; it does not establish the exact original reboot cause. Temporary trace code and files were removed from the normal build.

# 1.2.0 one-bar/live edit development — 2026-11-16

User-approved: one repeating MAIN bar; immediate Dice and live Division/rule editing; separate recorded Fill A–D; more rule choices. Four appended style parameters give eight choices per part without changing legacy rule ranges. The fixed layer trailer excludes these parameters. See../../../docs/plugins/groove-dice/one-bar-loops.md.

Targeted tests pass for100one-bar generations and six identical repeated scheduled bars, mid-bar16th-note delivery, per-part Q-Link edits preserving other parts,32styles, factory/user kit independence, old521-parameter/current state restore, Dice/Undo and recorded1beat..4bar fills. A two-bar automatic fill plays in full and MAIN resumes. Native/ARM32/ARM64 builds and SDK validation passed (0errors/0warnings). Full engine/UBSan/state/layer/expansion/skin regressions and actual-host count-in fixtures passed; host simulator19PASS/0WARN/0FAIL. Device installation/retest follows when completed.

# 1.1.1 automatic positive-position record pre-roll — 2026-11-16

## Actual-device findings

MPC XL / OS3.10.1.12; the captured tempo/sample-position relation is98BPM and44100Hz,128-frame render blocks. Four captured one-bar record count-ins began at PPQ2.14585185,0.64481481,7.73125926 and8.22940741. All advanced for842blocks (about4beats) and then reset PPQ/sample position to0 at actual recording start. FOLLOW=On, PREVIEW=Off. The three original captured rolls each produced14automatic hits during count-in.

The controlled test with count-in Off began directly at PPQ0. Raw VST flags were32719 on entering either record mode and32718 thereafter, including both before and after the count-in seek; SMPTE position followed the same free-running PPQ and next-clock was0. No independent count-in flag was observed. This replaces the earlier unverified assumption that MPC necessarily uses negative PPQ.

## Implementation and bounded claims

HostRecordGate watches transport transitions without new user parameters. When recording starts from stopped at a positive position beyond the host's announced bar start, it quarantines automatic playback until the host seeks back to its real starting point or advances the musical bar anchor. Ordinary playback and punch-in on an already running transport bypass this guard. Negative-PPQ hosts remain supported. Preview cannot bypass an active guard and direct MIDI/pad monitoring remains playable.

This is an inference from the measured MPC transport sequence, not a standardized pre-roll signal. Deliberately starting a stopped recording strictly inside an existing bar can be indistinguishable from that sequence in the available transport data; automatic playback can then wait for the next bar/seek. Such off-bar direct starts and other firmware/models are not certified. Aligned nonzero-bar starts and punching into an already playing sequence are covered by automated tests.

## Regression evidence

- Three checked-in, audio-free device fixtures: two positive-PPQ count-in captures and the controlled count-in-Off capture.842count-in blocks are silent; each actual recording starts with an event in its first block. The old binary reproduced14hits; the new scheduler reproduces0.
- UBSan and allocation guard;32/128/512frames × Preview on/off; MIDI monitoring; count-in cancel/retry; Follow toggles; normal loops; aligned nonzero-bar start; punch-in; gate reset and leaving record.
- Native/ARM32/ARM64 build and SDK validator0errors/0warnings passed. Full regression (including engine/UBSan, actual-host fixtures, layers/state, music/rules, UI, expansions) passed; host simulator19PASS/0WARN/0FAIL. Final user device retest passed: the user explicitly confirmed both count-in On and Off work normally on MPC XL after the automatic fix.
- Installation on MPC XL internal NVMe passed140-file verification. Engine10101, manifest1.1.1 and content1.1.1.0 are synchronized; personal banks retained. The plugin app was restarted remotely.
- The normal1.1.1ARM64 binary was checked for absence of diagnostic writer strings; it does not run the diagnostic writer thread. After device confirmation the compile-time-only tracer source/hooks were also removed (no change to the already installed normal binary). Personal expansion payloads remain separate and unchanged.

# Reopened: host record count-in — 2026-11-16

The user reports sound during count-in again after the earlier positive test. The previous negative-PPQ regression passes but does not establish every real MPC count-in behavior. The earlier hardware-success entry below is historical, not a current resolution claim. A compile-time-only transport tracer is being used to capture playing/recording/PPQ/sample position and Follow/Preview/scheduled hits on the actual MPC XL. Default production builds contain no tracer, file I/O or extra thread from this diagnostic.

# Count-in, kit lock and internal installation — 2026-11-16

- Reproduced automatic events while host transport is playing at negative PPQ. The scheduler had wrapped those positions into previous pattern cycles. FOLLOW now suppresses automatic playback throughout negative-PPQ blocks, including when Preview remains enabled, and suppresses negative-time events in the zero-crossing block. The original block origin preserves the first downbeat's exact sample offset. Direct MIDI/pad monitoring remains enabled.
- Regression: eight-beat pre-roll, Preview on/off, 32/128/512 frames, first downbeat at sample 17, no duplicate onset, finite output and no audio-thread allocations. This verifies negative-PPQ count-in; no unsupported separate host pre-roll flag is inferred.
- Existing KitLock is retained. MAIN now has a wider LOCK KIT button with KIT FIXED active caption. Locked scope labels are No Changes / Pattern Only / Groove Only / Fill Only / Rhythm + FX / FX Only, including the touch sidebar. The six stored Scope values, parameter IDs and defaults remain unchanged. No Changes intentionally makes no random changes; explicit FX scopes can still change FX.
- Kit lock matrix: 6 scopes × 2 dice modes × 10 rolls retain factory/custom sample selections, role controls, layers and mix trims. Saved lock and effective caption restore correctly. Whole-kit and mixed-part paths are covered.
- Native/ARM32/ARM64 builds, validator (0 errors/0 warnings), full engine/UBSan/layer/state/rules/skin/expansion tests and host_sim (19 passed/0 warnings/0 failures) passed. UI generation tests include conditional scope rows and the larger lock caption. Physical lock-button/Q-Link interaction on the new MPC XL was not independently inspected.
- Installed on MPC XL / OS 3.10.1.12 internal NVMe at `/nvme/VST2 Plugins/kim - VST - Groove Dice`; all 140 files matched installer verification. Personal expansion packages were retained. The new device initially had no plugins on NVMe or attached SSD. The previous device's SD contents were not deleted.
- User explicitly confirmed on the new MPC XL after installation: “예박은 조용하고 녹음부터 정상 재생”. This is a physical count-in/playback confirmation, not a remote screenshot or an all-controls hardware certification.
- Local logs: `/private/tmp/gd-countin-before.log`, `gd-countin-after.log`, `gd-countin-build.log`, `gd-countin-tests.log`, `gd-countin-skin.log`, `mpc-xl-groove-install.log`, `mpc-xl-internal-after.log`. Logs are local evidence, not release payloads. No public Release or website deployment was performed.

# Display-name revision — 1.1 development

-96 core factory kit titles replaced by readable aliases; optional imported code-style kits use a separate title palette. Public factory list97 programs and installed personal profile201 programs/11199 WAV aliases pass uniqueness,31-byte native title limits and source ID/path preservation tests.
- Native/ARM32/ARM64 builds and SDK validation pass (0errors/0warnings); full regression suite passes. Host simulator32/128/512:19PASS/0WARN/0FAIL each. Display-only table rebuild and state load/resave are byte-identical; bank payloads and521 parameter declarations are unchanged by this revision.
- Sample names, core fallback labels, kit browser rows, native programs and pack captions share the display-name layer. Existing readable MIDI titles and user-authored .xpl filenames are retained. Full policy/title list:../../../docs/plugins/groove-dice/naming.md.
- This revision has not yet been visually rechecked on MPC. Existing device captures below document the preceding layer build and therefore still contain the old names. Final device transfer completed on SD drive 2 (36C7-1ACA): all 138 files matched verification. Existing public and personal expansion payloads were retained. The app was not restarted; the running instance needs restart/reload to construct new display tables.

# Development verification — 1.1.0 Sound layers (unreleased)

User-approved implementation: MAIN C drum mix, Kick/Snare four-layer SOUND editor, Sample/Level/Tune/Shift SCREEN controls, ON/SOLO/LOCK, selected-row detail context and saved user kits. Public release remains1.0.0. See [layer specification](../../../docs/plugins/groove-dice/layers.md).

## Automated checks — 2026-11-16

- Native, ARM32 and ARM64 builds pass. SDK validator:0 errors/0 warnings. Compiler retains16 existing style/sign warnings; validator results do not imply a warning-free compiler.
- Full regression suite passes: engine and UBSan, bank integrity,97 factory programs/96 kits, groove/rules/usability/musical/15 Hat Rhythm modes, original bank, layer tests, skin bounds/press visuals and expansion manager. First342 parameter declarations compare exactly with the1.0.0 fixture;521 total, append-only.
- Final host simulator at32/128/512 frames:19 PASS,0 WARN,0 FAIL each. Actual byte-identical chunk resave, random parameter sweep, finite output, no clipping/denormals and teardown pass. No SDK source modification was used; diagnostic hosts were temporary local copies.
- Layer tests with UBSan and guarded process() allocation: signed onset at44.1/48/96kHz ×31/128/511-frame blocks ×−30/−15/0/+30ms; exact incoming MIDI offset preserved. Reported latency matches30ms at each rate. All16 relays across four parts, detail fields/polarity, four voices sharing one logical hit without self-choke, Solo, role stereo routing, pending older-hit choke, locked ROLL, Undo and asynchronous sample changes pass.
- Current/eight saved-kit state roundtrip,342-parameter legacy chunks, missing pack identity→resave→reinstallation, inactive detail relay and unavailable index byte stability pass. Host parameter notification remains outside render callbacks.
- Own-core layered audio render:7.999s mono preview, peak−6.29dBFS/RMS−25.09dBFS, finite and unclipped. Local audition artifact:build/layer-audition.wav. This is rendered output evidence, not a claim of a listening assessment.
- Fixed30ms baseline latency is an intentional cost of negative live Shift; MPC delay compensation/physical feel and worst-case hardware CPU remain separate checks.

## Hardware observations — MPC Live III / OS3.10.1.12

- Official SDK update verifies138 files; source banks and both installed Workshop Sessions/Splat Private packs are retained. Inventory11199 WAV/192 MIDI includes optional packs, not the public base inventory. Old plugin folder was exported first; private payloads remain ignored local build material.
- Saved pre-update settings as gdlayersbefore.xpl. Fresh new instance opens MAIN, shows four0dB mix trims and Init effectsOff. Native PRESET/name remains host-owned.
- Actual six tabs inspected: MAIN/GROOVE/SOUND/KITS/FX/ROUTING. All four SOUND parts display their intended rows; Hat DETAILS shows only Closed/Open. Layer sample text clipping found on the first device pass was fixed with an inset centred Label and transparent editable surface. Text is readable in the final on-device captures.
- Touch adjusts samples/Shift/Low Cut and changes selected-row detail values independently. SAVE KIT at Shift+27.2ms → manual−16.9ms → LOAD KIT restores+27.2ms. Saved gdlayersqa.xpl, restarted the app and loaded it: sample identity, ON, level−12dB, Shift+27.2ms and another layer's Low Cut593Hz restored.
- SOLO can isolate either voice. ROLL SOUND changes enabled unlocked voices while Layer3 LOCK keeps its sample; UNDO restores CORE1/previous Layer2 sample and retains timing/lock. Captures are in design/device-1.1.0.
- Q-Link outline follows rotary focus; library/detail overlays hide background bounds and closing restores them. Sample-only selector focus was missing and corrected. Final device recheck: dragging Sample rows2→3→4→1 moves the native Q-Link outline B→C→D→A, updates the correct sample and enables new layers. All four Kick layers are enabled in the final capture.
- Recorded92.839s stereo44.1kHz via the official Remote Screen Viewer. Three isolated audition groups correspond to combined voices, Layer2Solo and Layer1Solo; peak−16.29dBFS, finite/unclipped PCM. Viewer reported2 stream dropouts. This does not locate the cause or certify worst-case plugin CPU/latency. Audio/video evidence stays local in build/Desktop because it can contain optional personal samples.
- Physical Q-Link rotation across every group, pad timing feel, hardware PDC and worst-case CPU are not certified.30ms baseline delay remains explicit. Screen drag/focus and automated mapping checks are not substituted for a physical-knob test.

---

# Current verification — 1.0.0

Includes Original Workshop only:128 original synthesized WAV,96 kit configurations,two authored beats and93 timing templates. The base package contains no private third-party bank. Main timing/rule/sound/character groups,15 Hat Rhythm modes, fill recording, user-kit state and independent .gdexp expansions are included. Vendor kim, UID KpGy and342 parameter identities are unchanged from0.9.

## Verified

- Native,ARM32,ARM64 build; SDK validator0errors/0warnings; engine/UBSan/bank/factory/groove/rules/usability/musical/hat/skin/expansion-manager suite; host simulator19PASS/0WARN/0FAIL.
- Actual MPC Live III /3.10.1.12: official install completed, fresh MAIN with all installed expansions, Pack Builder-authored Workshop Sessions filter and eight kit list, kit selection and actual preview playback. Saved a new Gdpack.xpl, changed kit and reloaded; Workshop Sessions/Velvet Room and sound selections restored.
- Actual GUI-exported .gdexp contains32 own WAV,2 own MIDI and8 kits. Actual C++ engine test checks all roles, async decoding/non-silent audio, native programs and saved kit identity.
- Device recording49.821s,stereo,peak−7.10dBFS,RMS−26.26dBFS. Viewer dropout counter134→134 during the recording; that does not certify every hardware load. Preview/Audio/Record left off; existing personal packs preserved on the owner's test device.
- Standalone Pack Builder: macOS Apple Silicon actual app launch, UI audition and export; Windows x64 backend/Qt UI/bundle smoke tests passed in CI run43814119104 (commit6af7dfa). The public app has no private samples.

## Installation and compatibility

MPC standalone OS3.10.1+ with Enable Enhancements and External Plugin Support. Extract the complete kim - VST - Groove Dice folder into VST2 Plugins on a drive. Restart/rescan the application; save work before restarting. Read MANUAL.md and VERIFICATION.md in the ZIP.

Expansions remain separate. Reopen the plugin after installing/removing them. Missing required packs mute/report missing sounds and preserve identities for later reinstallation. Older0.8 engines cannot read new expansion identities. The native PRESET list is host-owned; category-first factory-plugin navigation is not promised.

## Known limits

Full physical Q-Link coverage, all MPC multi-output destinations and worst-case hardware load are not certified. External audio sidechain, complete per-part insert chains, MIDI output to the MPC sequencer, live import/time-stretch and native XPN registration are not provided. Windows audio-device compatibility beyond the automated tests is not certified. The app is ad-hoc signed on Mac, unsigned on Windows; no notarization/developer certificate is claimed.

[Full user manual](../../../docs/plugins/groove-dice/manual.md). Actual screenshots are used on the website; installed expansion counts in captures can exceed the base128WAV/95MIDI inventory.


# Historical records

# Current verification — 0.9.0 / original core and expansions

- Public project now at mpc-software/plugins/public/GrooveDice. First297 parameter declarations preserved,45 appended (342 total); vendor kim / UID KpGy unchanged.
- Native, ARM32 and ARM64 builds passed after the final kimswork folder move. SDK validation:0errors/0warnings. Engine/UBSan/bank/factory/groove/rules/usability/musical/hat/original-core/skin/expansion-manager regression suite passed. Host simulator:19PASS/0WARN/0FAIL, state roundtrip. Compiler style warnings are distinct from SDK validator results.
- Original core:128 independently synthesized WAV,96 kit combinations,2 beats and93 timing templates. Relocated standalone generator produced a byte-identical19,734,488-byte bank (SHA b98c4b813ff381fb5befca49bfa7385c5c7ace9d54355739569596cf8b81bb8f). No external audio/MIDI/model input is used. Full provenance/source manifests stay with the expansion project.
- Public build, parameter generation and ZIP assembly passed while the private expansion path was absent. Actual public ZIP contains one own-core bank and zero private payloads. The bundled expansion manager executes standalone outside the repository. Add/remove/private-opt-in/duplicate/path-traversal/hash guards passed. Shared CLI is now in mpc-software/shared/groove-dice-expansion.
- Pre-migration0.9 combined-bank tests covered192kits/193programs, catalogue values beyond former fixed limits, filtered randomization, missing-pack mute, save-while-missing/reinstall, catalogue reorder identity and no process allocations. Logs copied to build/qa/v090-*.log. These are automated tests, not full hardware certification.
- Actual MPC Live III /3.10.1.12: the original-core instance showed128WAV/95MIDI, dynamic kit browsing/filtering, ROLL and native PRESET selection; preview playback and a real screenshot were recorded. Correct personal install then transferred107files with matching sizes; fresh instance showed11167WAV/190MIDI and192kits. Private-pack filter/list and selection worked, and old0.8Gdhat.xpl restored its Splat sounds and rhythm without substituting core sounds. Device captures: v090-core-main-device.png, v090-private-browser-device.png; core preview MOV also retained. Device installed folder stays independent of local source renaming.
- Remote-viewer dropout counters increased during the1.4GB upload. This is not evidence of engine glitch-free performance or worst-case CPU. Full physical Q-Link/maximum-pack load/all destination routing are not certified. A new two-pack device save/reload after the folder migration has not been performed. Existing0.8 fill verification remains historical.
- Source public status does not imply a public Release. Pack Builder is an approved image/proposal, not a built macOS/Windows app. No new plugin Release was created by the repository migration.
- Migration evidence: build/qa/structure-*.log and structure-verification.json. Standalone .gdexp target product/UID fields are still a proposed compatible extension; current GDEXP001 is Groove Dice-specific.

# Historical verification — 0.8.0 / Hat Rhythm

- User-approved MAIN D4 replaces Duck with15 Hat Rhythm choices. Duck is retained onFX; old294 parameter definitions are identical, three appended (297total). SCREEN relayQ16/Programfixedmap/MAINvalue/sidebaragree. All15rows accessible via8-row/two-page sidebar withorderedmodalguidehiding. Existing6-page guide geometry and caption-safe press surfaces retained.
- native/ARM32/ARM64build passed. Full engine/UBSan/bank/library/factory/groove/rules/usability/musical/newhat/skin suite passed. Hostsim19PASS/0WARN/0FAIL,297params/state roundtrip. SDKvalidator0errors/1unchangedContent-sizewarning. No bank/sample/MIDI changes.
- Newhat test:15choices×20ROLL=300, no audio allocations/finite output, snare2/4 and kickexclusions retained; actual scheduler delivers all recorded hats; dotted phases cross barlines; allFILL/ROLLchoices add actual notes even withPocketrule; Q16 changesRhythmwithoutchangingDuck; LOCK/Undo/newstate/old294paramstate checked. Existing320phraseROLL/50lowAmount/1000boundedROLL/etc tests retained. Legacy32ndRoll now adds its short roll onresponsebars2/4, so actual fourbar hatcount is68 rather than0.7's72; regression count updated to approved phrase design.
- Offline88BPM/4bar/identicalkitandROLLseed DOTbase/FILL/ROLL WAVs inbuild/qa/hat-rhythm. Constant gain only to-2dBFS; source rawpeak-8.63dBFS. Files are engine renders, not device captures or a claim of human listening.
- Official0.8update completed onMPC LiveIII3.10.1.12 / SD36C7-1ACA,94file sizes matched. Removed/readdedinstance; freshMAIN showsHatRhythm. LegacyGdmusic.xpl restoredLoose04/1/8/ScopeKit+Pattern. Both pages of15choice sidebar visible; pagebuttons/lastchoice/selectionhighlight/outside-dismiss work; backgroundQguide hidden whileopen and restoredafterclose. Touchdrag offrontHatknob changed14→0; FXDuck remained21%. GROOVEshares the same15choice selector. This is actualremote touch/drag, not physicalQ-Linkrotation certification.
- Selected1/8DOT+ROLL, KitLockOn, PatternROLL andPreview actualdeviceplayback recorded23.638s stereo atpeak-7.68dBFS/RMS-26.52dBFS. No clipping by measuredpeak. Viewerremote dropout counter39→41, so no hardwareCPU/glitch-free certification is claimed. Saved newGdhat.xpl without overwriting olderpresets; changedselectiontoquarterthenreloadedGdhatandDOT+ROLLrestored. FillEditor stillcontains4previousrecordednotes; leftMAIN/PreviewOff/AudioOff/RecordOff. Temporarycaffeinateterminated; no permanentpowersettingchanges.
- MPCchoicereplacementstringsoverrideformatValueinLCD, so actualMAINshowsfull1/8DOT+ROLLnameat18px, notthecompacthostdisplayform. Nativecaptures:v080-main-device.png andv080-rhythm-sidebar-device.png. DeviceMOV/WAVandmetrics:v080-device-playback.*,v080-device-audio.json.
- Logs:build/qa/v080-{build,tests,validate,install}.log andv080-verification.json. Privateproject: no publicRelease/Pages/push.

# Current verification — 0.7.0 / musical phrase generation

Userapprovedsnare2/4,hatgridwithboundedvariation,kicksawayfrom2/4,Percsupport,four-barphrases; existingRULESremainvariationstrength.

- Native/ARM32/ARM64built. Fullengine/UBSan/bank/library/factory/groove/rules/usability/skin suitepassed. Hostsim19PASS/0WARN/0FAIL,294params/restoredchunkbyte-identical; SDKvalidator0errors/1unchangedContent-sizewarning. First292IDs/orderretained; HatGrid/PhraseStateappended. Extra-stateformat unchanged.
- NewmusicalregressionunderUBSan/no-audio-allocationguard:320ROLLs across4grids andAmount10/100. Fourbars,8mainbackbeats,no2/4kick, boundedhatcounts. Actualscheduled4barshave8backbeats and16/32/64/72closedhats forPocketquarter/eighth/sixteenth/32ndRoll; oldbudgetnolongererasesthedensegrid. BusymaxSwing/Nudge/Ghosttested;ghosts≤2/bar andquieterthanmain.
- ActualpartialLOCKhat eventtimes/velocities/loopphase retained onlegacy1bar; fourbargenwhenunlocked. Undo/restoresnote/length/actualhatgrid/origin.50lowAmountROLLs preservechosenkickcore. Old292paramchunksrestorelegacyrhythmwithoutrewrite. ExistingallLOCK/PatternLock/KitOnly/loadreplay/concurrencytestsretained. Prior14notes/onebar assertionswereupdatedtofourbar/per-barchecksbecausetheuserchangedthestructuralcontract,notbecauseanunrelatedregressionwasignored.
- NewGROOVEfooter4touchchoicesleaveall16Q-Linkassignmentsand0.6.1guidegeometryunchanged. SelectedgridappliesnextPatternROLL. NewinstanceScopedefaultKit+Pattern; legacyScoperestoredverbatim. PatternROLLkeepsrecordedfills; Fill/Allstillusesexplicitfillvariation.
- FivecomparisonWAVsinbuild/qa/musical-five: samekit/BPMaspriorroll-dice-five,actual0.7PatternROLLwithKitLockforfaircomparison,peak-normalisationonly. Exactenginestatesandrenderer/manifestretained. NotMPCrecordings; objectiveWAVmetricsnotclaimofhumanlistening.
- OfficialMPCupdateinprogress; hardware0.7UI/PLAYchecksnotyetclaimed. PriorGdsafe.xpl/Loose04isrestorepoint. No publicrelease/push/siteupdate forprivateproject.

# Current verification — 0.6.1 / Q-Link guide correction

The0.6whole-panel guide test wasinsufficient: it checked theouterpanel fit butallowed theMPC Q badge tocoverA–D. User requestedactualfix. Newrule supersedesearlieroutside-panelmargins: header/badgeoutside; names/knobs/valuesinside; nativeQhasitsownleftgutter.

- Every614×138panel usesguide(x+4,y+26,606,110). Four138×108controlcomponentsstart(x+44+j*140,y+28); LCDremains116×24 andfonts unchanged. The native40×40Qmark touchesneitherfirstname'sbounds northeknob. This appliesMAIN/GROOVE/DRUMS/KITS/FX/ROUTING andeditor'sunusednativefallbackdefinition; dynamicFILLstillhidesbackgroundguides.
- Native/ARM32/ARM64built. Hostsim19PASS/0WARN/0FAIL,292parameter/state roundtripunchanged. SDKvalidator0errors/1understoodContent-sizewarning. NoDSP/sample/MIDI/parameterorder/Q-Linkmapping change; onlyproductversion601andvisualcoordinates.
- Revisedregressiontestseverypage: guideisinsidepanel, allheader/badge rectanglesdisjoint, fourcontrolscontained, firstcaption/knobdisjointfrom40pxQbadge. Previouswhole-panelcontainmenttestremovedbecauseitcodifiedthebadlayout. PopuphideQLinkBoundsorderedserializationand33presssurfaceregressionsstillpass.
- Local logs build/qa/v061-{build,host,validate,skin,install}.log. Official0.6.1 updatecompleted,93filesizesmatched. ActualMAIN A–D guidefocusverified onMPC LiveIII3.10.1.12. RestoredGdsafe.xpl/Loose04; oldfilesuntouched.

# Current verification — 0.6.0 / 2026-11-16

Latest approved change: remove redundant screen drum pads; physical pads own drum performance. MAIN lower BROWSE/CREATE/PLAY lanes are60px high. The earlier8-screen-pad candidate is superseded and not the final layout. Bank/engine are private, not a public release.

- Native/ARM32/ARM64 built. SDK validator0errors/1understood >256MB Content warning. Host simulator19PASS/0WARN/0FAIL;292parameters restored and byte-identical state; desktopaverage0.67% (not MPC CPU certification). first291parameter IDs/order retained; read-only ActionInfo appended.
- MIDI95 bank paths and exported filenames renamed uniquely. Byte audit still verifies all11134original payloads bySHA256+CRC32, WAV metadata/MIDI headers. Original MIDI names are retained only in provenance/source research; bank/runtime names are all new.2beats/93timingtemplates, not95newbeats.
- UBSan usability regression: all244Percsets apply4valid WAV IDs/settings; Kick/Snare/Hats/pad notes/output/FX/fill preserved. Four distinct aux roles produced nonzero finite output.100MixParts rolls retain complete Perc ensembles. SHA-state roundtrip byte-identical. Other engine/library/factory768auditions/1000ROLL/rules/concurrency/load replay tests passed.
- Structural skin checks:33command press visuals use UI Local gates/continuous ResetOnGestureEnd surfaces; one real engine Mouse Down command, independent release. All MAIN actions are belowpanels and at least60pxhigh. No screen drum Pad commands onMAIN/FILL; existing enginePad/MIDI IDs retained. Guide bounds moved withpanels and ordered hideQLinkBounds maintained.
- Host simulator exposed a restored SoundBrowserPage reset while load protection wasactive. Fixed group-change/page clamp so they do not overwrite restoredpage during0.5audio-second guard; full19checks then passed. Existing invalid data and late MPC parameter replay tests remain.
- Logs build/qa/v060-{build,tests,skin,validate,usability,bank}.log andverification.json. v060-tests.log was run before final screen-pad removal; finalgeometry/33command surfaces are covered byv060-skin.log. The final change is skin-only; DSP binary was rebuilt afterward unchanged.
- Random-loop audition: actual0.6engine,88BPM,4bars/481091frames, V04/Loose07,22ROLLs drawn by systemrandomness, defaultRules. No post-edited hits; onlyconstant gain. WAV build/qa/groove-dice-random-88bpm.wav, peak−2.19dBFS/RMS−21.51dBFS. Generatedaudio and exactenginestate retained privately. Not a hardware recording or a claim of human listening.
- Final0.6.0 installed through the official tool;93file sizes verified. MAIN startup,33press-surface implementation withactualROLL/UNDO holds, role/kit/pattern/MIDIbrowsers, Preview/queuedROLL/fill editor andsixpagevisuals checked onMPC LiveIII3.10.1.12. Existing device state restored fromuniqueGdsafe.xpl, oldfiles untouched. Physical16knobrotation/newphysicalfillrecording were not re-certified in this update.

# Current verification — 0.5.0 / 2026-11-16

User-confirmed: replace MAIN B PATTERN with four part RULES; keep C SOUND. No C-mode toggle. Fix MAIN guide fit and guide overlay on kit/pattern browsers.

- Native/ARM32/ARM64 builds passed. Host simulator19PASS/0WARN/0FAIL;291parameters andextra state byte-identical. Source identityfirst287IDs/order retained,4Rule choices appended. SDK validator0errors/1understoodContent-size warning. Audio bank unchanged from0.4.
- Guide geometry now derives from614×138MAIN panels with4pxhorizontal/2pxvertical outer margin (622×142). Structural regression verifies realpanel containment. Native version4 hideQLinkBounds field moved afterdisableCoarseDataWheel/beforecomponentsData perSDK ordered contract, across allchoice/role/library/editor modals. Prior structural tests only checkedpresence andmissed serialization order; newtest checksorder.
- Rules tests underUBSan:100AllROLLs withallLOCK preserve actual scheduled event positions/velocities/times despite oldGhost100/Velocity50/Variation100/Nudge30. POCKET Amount100 has14seed notes andno added/deleted hits orauto-ghostflood. Empty locked roles remainempty; fill-onlyroll preservesmain seed. Separate test constrains128-notelegacy input atplayback evenwithoutPatternScopeROLL. ManualMIDI/touch isunrestricted.
- Per-role seeds separate kit randomness fromrhythm. GrooveROLL uses small per-part offsets, retainsmanual Aknobs andlocked parts. New84byte optionalGRP5 state, backward-compatibleoldextra lengths. Undo restoresoffsets/seeds/RNG/KEEP aswellasexistingdata.
- Full engine/bank/library/factory/relative-input/concurrency/skin regressions passed. Matching rounded NoteNudge retains fractionalnormalised state onreset, fixing byte-roundtripwarning exposed by291-param sweep. Initial exact .1f test assertion corrected to comparestorednormalised value; native source range conversion neednotreturnliteral .1f.
- Logs:build/qa/v050-{build,tests,validate,host-final}.log. Hardware observations are appended afterfinal installation; do notconflate software contract checkswithverified device guide rendering. UserLoose04 state saved innew Gdprior.xpl before refreshing; earlier files untouched.

# Current verification — 0.4.0 / 2026-11-16

Approved changes: bounded original-based randomisation and MAIN C role sound selectors. The later SOUND/RULES profile proposal is not implemented/accepted in this candidate.

- Native/ARM32/ARM64 build passed. SDK validator0errors/1understood Content-size warning. uiversion0.1.0.0 unchanged, product0.4.0, UIDKpGy. First261IDs/order retained;26appended=287. MAIN C relay/Program mapping intentionally changed; actual fill parameter identities remain stable.
- Full test runner passed, host simulator19PASS/0WARN/0FAIL with byte-identical287parameter+extra state. Desktop average0.53% block CPU, not a hardware guarantee.
- New UBSan tests:1000patternROLLs preserve original kick downbeat/snare2·4backbeats, bounded role caps/density. Original baseline retained across rolls and state load.400grooveROLLs at10/80%Amount stay within scaled timing bounds; no cumulative drift.20templateROLLs preserve chosen template and avoid additional random Swing.
- Legacy compatibility regression: loaded128-note over-randomised state without the new baseline extension; preserved it immediately afterload, nextPatternROLL regenerated a bounded candidate from immutable factory beat. Recorded fill banks remain separate.
- Next-bar queue tested: no mutation beforeboundary; onecommit atnextbar; transport stop cancels. ScopeFill while recording does not rewrite the ongoing recording. Musical quality still needs listening; algorithmic constraints alone are not a final sound approval.
- Role pools=440Kick/546Snare/376HatPairs/244Perc, total1606. First/last selector values map to actual source IDs, Hat switches both roles withchoke1, partial relative inputs accumulate, concurrent UI writes survive audio readback. Source file metadata lives only inbank, no derived PCM/MIDI compiled intoengine. Loop boundary minimum spacing and role count limits applied.
- Existing engine15cases normal/UBSan, library160live replacements/48voices/cache lifetime,96factorykits×8role auditions, full11039WAV+95MIDI SHA/CRC audit and new1606record catalogueCRC/SHA validation passed. During asynchronous selection the prior loaded source is retained untilnew source ready; explicit failed/missing assets still report errors.
- UI structural tests: MAIN C four sound selectors and caption-safe modal launchers;8-row role browser; guide hidden behindmodal; GROOVE D exposes originalFillSlot/FillDensity/FillEvery/Variation; existingMAIN/FILL mappings besidesC preserved. Existing physical hardware tests are historical; current physical rotation matrix not silently recertified.
- Logs:build/qa/v040-{build,tests,validate}.log. Actual device install/results appended below aftercompletion. Current device Honey03/BankBeat02 state preserved innew Before.xpl before refreshing; old user files notoverwritten.

# Current verification — 0.3.0 / 2026-11-16

Supersedes0.2 statements about only3kit choices and a single native program. Private development build, no public release.

- Native/ARM32/ARM64 built; SDK validator0errors/1size warning(Content>256MB). UID unchanged. First254parameter IDs/order retained,7appended(261). Only existing Scope choice captions changed to describe kit-inclusive randomisation; indices unchanged.
- Full regression runner passed including normal/UBSan engine15cases, bank corruption checks, library/cache/concurrency tests, source SHA/CRC audit, factory-kit UBSan tests and skin structure checks. Host simulator19PASS/0WARN/0FAIL: all261parameters and full state restored byte-identically.
-96same-volume reconstructed kits (6packs×16) registered as native programs plus Init=97. Not an extraction/reproduction of vendor factory presets. All96sample tuples distinct, all768role auditions produced finite nonzero output in isolated/reset renders. Per-role peaks measured offline and gains adjusted; this is not a human audition of every kit.
-100WholeKit ROLLs selected a different kit even under Kit+Groove Scope. MixParts changed role-matched content; KitLock/Amount0 preserved kit and reported why. Undo restored full kit/source/FX. Header/current-program reporting and actual parameter notifications tested.97choice relative increments accumulated instead of being overwritten by canonical readback.
- Native preset changes preserve pad notes, output routing, master level, groove and recorded fills. Original parameter compatibility checked against0.2snapshot. Duplicate-SHA files retain selected directory entry; matching legacy choices retain fractional values on restore. Added regression verifies byte-identical saved state for both.
- All11039WAV+95MIDI raw payloads remain identical. New bank1,481,963,096bytes includes optional96record GDKITS03 catalogue, CRC32 perrecord and SHA→Kits linkage checked. No process() file I/O/allocation/lock/host notification added.
- DRUMS SCREEN groupA=FactoryKit/KitDiceMode/RandomAmt/KitLock; B/C/D=Kick/Snare/ClosedHat Sample/Level/Tune/Decay. MAIN/FILL unchanged. Paged97choice sidebar, modal guide hiding and caption-safe nativeROLL button validated. Duplicate FactoryKit knob component key found by regression and fixed with distinct definitions.
- Software logs build/qa/v030-{build,tests,validator,host-final}.log. Real device observations follow after installation; software mappings do not substitute for a new physical knob rotation test while user asleep.

# Current verification — 0.2.0 / 2026-11-16

This section supersedes the historical0.1 statements below about64-only packaging, no full library and44px knobs.

- native/ARM32/ARM64 build completed. UID KpGy and first214parameter identities/order preserved;40 appended,254total.
- SDK validator0errors/1understood warning: Content exceeds256MB. Scripted `uiversion` must remain0.1.0.0: it is the host schema version, not product version. A proposed0.2 value was rejected by validator and removed **before installation**. Product version is0.2.0.
- host simulator19PASS/0WARN/0FAIL. All254parameters and extra state restored byte-identically; finite audio at32/128/512frames; no main-pass clipping/denormals; CPU~0.55% average onMac, not a hardware CPU guarantee.
- Existing15engine regression cases repeated normally/UBSan, including actual16MAIN/8FILL relay targets, concurrency, partial relative input accumulation, count-in/overdub/finite recording/host loop, note editing, choke, routing and state.
- Full bank1,481,886,264bytes. Every11,134entry compared to source SHA256+CRC32; all WAV rate/channel/frame metadata and MIDI headers verified. Source collection unchanged. All11,039WAV included; not a claim every factory preset MIDI in the original app was extracted.
- New library UBSan test: counts2411Kits/8043Chops/585Loops/2Beats/93Grooves; all95MIDI parse to3822note-ons. First+last samples of all3WAV categories decode/play; deselect/reselect same file works;160live replacements with old voices; no allocations on audio thread;64slot cache releases all held slots; observed test resident PCM4,915,200bytes, theoretical current-collection cache ceiling281,018,368bytes plus legacy/index/temporary I/O.
- Library SHA identity and user-kit restore tested; both imported beats produce events;93templates render finite audio. Original files retained losslessly inside bank. Templates modify timing; Chops/Loops are one-shots, no BPM stretch.
- Restore bug fixed: saved folder must not reset when first process follows state load. Identity reattachment now preserves the existing normalised value when the content index is unchanged. Reset also preserves matching fractional Gate/Chance UI values rather than rewriting them from integer note storage, while inactive C/D editor relays immediately return zero. These together restore byte-identical chunks.
- Cache bug fixed: a quick0→sameID cycle could be missed by a loader remembering only lastID. Audio-side request generation counter ensures every selection transition can be observed.
- Runtime assets rebuilt from approved v3 material direction: dark espresso chassis,138px ivory cards,64px knob frames, tactile material plates. Baked production background1280x628, unique mutable component definitions, native6tabs, bounded8-row library sidebar.
- Device deployment and screenshot observations are recorded below once completed. Current physical hardware knob tests are historical0.1, not silently recertified for the new build.

# Groove Dice — 개발판 검증 기록

2026-11-15. vendor kim / UID KpGy / 0.1.0. 공개 출시가 아닌 private 개발판이다. Donut Theory와 별도 플러그인이며 해당 플러그인의 기존 Release를 변경하지 않았다.

## 자동 검증 — 통과

- native(macOS), ARM32, ARM64 빌드 성공. 고정 SDK validator0errors/0warnings.
- 최신 host simulator **19 PASS / 0 WARN / 0 FAIL**. MIDI 발음·유한 출력·가변32/512블록·파라미터 변화·저장 복원·종료 후 무음·desktop timing 검사. 중간 후보의 state readback 경고/실패는 수정했으며 최신로그와 구분한다.
- 엔진 회귀15항목을 일반/UBSan 각각 통과:16MAIN+8FILL relay, 미할당C/D 보호, MIDI전용 녹음, Replace/Overdub, 카운트인/취소/자동 종료, 필인 queue/play/return, 실제 노트 저장 복원, 킷 잠금/저장 로드/초크 기본값, 노트 Nudge/Velocity relay 편집,16출력 분리/합산, seek·가변블록·process 할당 금지, 호스트 loop를 가로지르는 녹음, 샘플 ID 재연결, 빈 필인 랜덤 생성과Undo, UI/audio 동시 relay write, 단계형 Fill Select의 작은 입력 누적.
- 사운드 뱅크:64WAV+108note-on 로딩/샘플 ID/레이트·채널·길이 확인. 손상 헤더/절단/범위 밖 offset/checksum/잘못된 패턴 값 거부.
- 스킨:6개 함수 탭,16MAIN relay, native44px127frame knobs,1280×628배경,64grid cells, page bounds/중복 component key/편집 overlay guide숨김 검사.
- process()에 파일 읽기·할당·잠금·로그·host 알림을 넣지 않았다. host 알림은 render-thread token 검사 후 UI value read/idle에서 배출한다.

로그: `build/qa/build.log`, `tests.log`, `validator.log`. 설치 후보파일 SHA256은`installed-candidate-sha256.json`. 재현 명령은README를 따른다. host simulator WAV는build/test.wav이며 모델이 청음한 것으로 주장하지 않는다.

## 실제 MPC 확인

MPC Live III / OS3.10.1.12 / aarch64. 공식 SDK mpc_remote.py로 Donut Theory와 같은 SD(VST2 Plugins)에 설치, 파일크기 검증. Remote Screen Viewer를 직접 조작했다. 현재 코드의 16output 선언은 디바이스에서 로드됐으나 수신 믹서별 신호 전체는 아래 미검증 항목이다.

1. 새 인스턴스가 MAIN으로 직접 열림, 시작 도움말 없음. `.gdbank`64sounds 로딩 표시. 예술 원본 자동축소 실패로 주사위가 잘리던 문제는runtime1280×628 이미지로 고쳤다. knob filmstrip도44px native 크기로 맞춰 잘림을 제거했다.
2. MAIN/필인 편집 진입·BACK 복귀. BACK의 일반radio 방식이 반응하지 않아 raw Mouse Down Toggle Switch로 변경 후 정상 복귀 확인. FOLLOW MPC/PREVIEW가 반응하지 않던 FilmStrip Image 기반 후보를 native Button+캡션 위 hit surface로 바꾸어 OFF/ON 표시와 실제 미리듣기 정지를 확인했다.
3. 모든6페이지 진입·배치 확인. 공용 panel definition key가 섹션 제목을GROOVE로 잘못 공유하던 문제를 unique key로 수정하고 정적 중복 검사 추가. 실제 전체 페이지 캡처는build/qa에 있다(중간 후보 캡처도 포함하므로 파일명과 이 기록을 우선한다).
4. MAIN KIT 왼쪽426px 선택 목록에서 Basement Soul 선택·닫힘, 팝업 뒤 가이드 숨김/닫은 뒤 복원을 확인했다. 선택/좌우 명령도 실행했다. 모든 목록의 모든 항목을 물리 손가락으로 반복 검사한 것은 아니다.
5. 실제 Q-LINK 팝업 Screen 모드16개 이름/순서가 설계와 일치. 사용자의 첫 물리 회전 무반응 보고 후, 값 callback 재조회 경쟁과 실제 target에 대한 UI알림 누락을 수정했다. 진단 후보376입력과54→56%를 관찰했고, 진단 문구를 제거한 후보에서 사용자 회전으로 **Swing54→70%** 확인. 편집 진입 후 같은 물리 노브로 **Fill Select A→C** 확인. source unit test는 전체16/8매핑과 작은 choice 입력 누적도 검사한다. 물리16개×모든페이지 회전 행렬을 전부 측정한 것은 아니다.
6. 화면 패드 두 타격을 필인2노트로 기록, COUNT-IN/RECORDING/RECORDED 상태 전환·finite 종료 확인.
7. 물리 기본값은 기존에 측정한 MPC C1 Major 배열24/26/28/29/31/33/35/36을 사용했다. Pad Perform을 바꾸지 않았다. 사용자가 A1–A4에서 킥/스네어/닫힌하이햇/열린하이햇의 소리가 각각 구분된다고 확인했다. 마지막 실제 MIDI29→OPEN HAT도 읽었다.
8. 사용자가 직접 REC FILL을 누른 후 A1–A4를 눌렀고 **4노트**가 저장됐다.4bar/Count-inOff/Replace, BAR2에서 네 역할의 노트가 표시됨을 확인했다. 원래 연주의 Velocity38, Nudge약-2ms가 그대로 보존됐다. 이는 연주가 유효하다는 기능 확인이며 음색 품질 최종 승인과 구분한다.
9. 실제 호스트 프리셋 **Gdqa.xpl**을 User의kim - VST - Groove Dice폴더에 새로 저장했다. 선택된 킥 노트 삭제로4→3노트 변경 후 Gdqa를 다시 불러와 **4노트·해당 킥·Velocity38·Nudge-2ms·4bars·BAR2** 복원을 확인했다. MPC 전체 프로젝트 저장/재시작 복원과는 별개다.
10. 마지막 상태는 MAIN, Follow MPC Off / Preview Off이며 사용자 녹음은 유지하고 Gdqa.xpl에도 보관했다. 다른 프로젝트 파일을 덮어쓰지 않았다.

주요 증거: `main-physical-qlink-device.jpg`, `fill-physical-qlink-device.jpg`, `physical-drums-device.jpg`, `physical-fill-bar2-device.jpg`, `preset-restored-device.jpg`, `main-final-device.jpg`. Remote Screen Viewer 창을 포함한 실제 화면이며 콘셉트 이미지를 실기기 캡처로 소개하지 않는다.

## 남은 항목 / 출시 조건

- 초기 개발판이다. 원본 전체 비트/MIDI 라이브러리·패턴/음색 큐레이션은 완료하지 않았다. 현재 private 뱅크에는 역할 재배치한 비트1개와샘플64개가 있다.
- 외부 다른 트랙과의 사이드체인 미구현/미검증. 현재 Duck은 내부 킥→다른 드럼에만 적용한다.
- 개별16채널 신호는 자동 회귀를 통과했으나 실기기 믹서 목적지별 전부·다른 MPC 모델·최악 CPU/장시간/대량입력 검증은 남았다.
- 파트 insert FX와 독립 master FX 체인의 완전 분리, 사용자 킷 이름 변경/삭제·파일 브라우저·WAV 실시간 import·멀티뱅크 전환은 미구현이다. 기본 FX/모듈레이션과8개 사용자 킷 슬롯 저장/불러오기는 구현했다.
- Gate는 Gated Samples 호스트 파라미터가 켜져 있을 때만 작동한다. 스킨에 Gated 스위치를 별도 배치하지 않았다. 빈 선택 노트의 B컨트롤은 표시되지만 실제 편집을 무시한다.
- 새 인스턴스MAIN 확인. 저장 state의 Editor context는 복원되며, 모든 프로젝트 재시작 조건에서 강제MAIN 진입하도록 하는 별도 transient UI-state 정리는 후속 검토 대상이다.
- 초기화/샘플 로딩 때만 메모리/파일 작업. 현재 고정8역할×8샘플64개를 메모리에 올린다. 모든11039개를 설치하거나 로드하지 않는다.
- public 전환·Release·Pages 배포는 하지 않았다. 외부 콘텐츠 권한은 개인 사용과 별개로 해결해야 하며 자체 제작 음원/사용 허가된 패턴으로 대체해야 한다.

## 0.2 interim device observations — 2026-11-16

The first0.2 candidate installed on the same SD via official SDK tool:69files matched. Re-selected the plugin (no app restart), loaded MAIN directly with11,039WAV/95MIDI status. DRUMS→LIBRARY showed426px sidebar,49kit folders/7pages. Entered Vol1/Kicks:62files/8pages; selected S2C1-1 808.wav and sidebar closed. Other tabs remained on their page behind the sidebar and Q-Link guides hid while open. Started Preview and observed BAR1→5→9, stopped successfully; captured actual streamed audio + screen with Remote Screen Viewer. Transfer congestion caused remote audio dropouts, so that recording is evidence of device output, not a dropout-free performance certificate.

The actual first UI showed LCD values too close to panel bottom; final138px cards,81px LCD offset and row spacing fix this. Selected-sound/playback labels, role-specific DRUMS headers, ROLL dice icon and library-aware Undo are in the final candidate. The second full-package install completed:70files matched; final screenshots/observations are appended after it completes. Source tests for that candidate are19/0/0 again, validator0errors/1size warning. Every revision remains private.

Current user settings were saved as a new Init Pocket.xpl before refreshing the instance. Existing Gdqa.xpl with four physically recorded notes remains available; neither was overwritten by new QA captures. Restore these before handing back.

## Native choice captions and Q-Link clearance

MPC replacementStrings override host display text for indexed choices. The bank state was correct but MAIN could still show the legacy Pattern name and Sample1..8 numbers. Native conditional captions now bind the active library beat's float display and show a static LIBRARY badge for LibrarySample>0; indexed choice data/hit targets remain unchanged. Scope gets a native static title. No parameter IDs or audio code changed in this caption/guide pass.

The hardware focus Q badge extends below the top of qlinkBounds. Detail pages could obscure the beginning of PreviewBPM while that group was focused. Guide rectangles now start12px above the card and retain its bottom boundary; the badge stays above the first knob caption, preserving the full functional mapping. The upper edge may cross a decorative panel/tray border; it does not cover a value or control caption. Final device captures must confirm this before claiming page verification.

## Device capture timing

Remote Screen Viewer reports queued transport latency while the1.4GiB installer is transferring. get_app_state can therefore capture a previous tab after a click even though the click is queued. Multi-click captures made during transfer (`v020-groove/kits/fx/routing-device.jpg`) are **interim/unverified tab labels**, not proof of four successful page switches. Final per-page observations must match the visible page title and active native function key after transfer completes. This is a remote-capture limitation, not evidence of a plugin tab failure.

Device first-pass recording is `build/qa/v020-device-first-playback.mov` (51.5s, native1280x800 +44.1kHz stereo PCM). Extracted WAV peak=-0.088dBFS, RMS=-23.57dBFS; isolated direct-role hits and subsequent Preview output are present. It contains remote-stream dropouts during file transfer and reaches the engine's safety ceiling on some mixed hits. Whole-library loudness/kit curation is still pending; this is not a claim all11,039source files are mix-normalised. Audio extraction used a temporary imageio-ffmpeg tool under/tmp, never linked or shipped with the plugin.

##31-byte host display contract

Actual GROOVE/KITS footer exposed a second issue: VST effGetParamDisplay truncation could turn `GROOVE: OFF` into `GROOVE: O`, and long folder/file names could lose identifying suffixes. Runtime display strings now fit31UTF8 bytes: folder rows use pack number/leaf folder, filenames omit .wav/.mid and keep their tail when shortened, selected sound includes a compact role prefix, playback line shows compact beat + GRV number/OFF. UTF8 boundaries are preserved. Archive paths/raw files/state IDs are unchanged. New regression sweeps all folder/MIDI pages and all8role captions under the31-byte limit. native/ARM32/ARM64 rebuilt and all tests/host simulator19/0/0 passed again before install.

## Final device result — MPC Live III3.10.1.12 /2026-11-16

- Final package installed with the official SDK updater on the same SD,70files matched. No system files, other plugins, Pad Perform settings, public Git/Pages/Release were changed. Refreshed the instrument instance; no app restart was required.
- Fresh instance opens MAIN directly. Actual MAIN/Editor/GROOVE/DRUMS/KITS/FX/ROUTING inspected. Page titles and active native tabs matched after transfer finished. Unused FX C/D guide regions absent; focused GROOVE C PreviewBPM and ROUTING C PadNote captions stay fully visible after guide offset. Images are real Remote Screen Viewer captures.
- LIBRARY onDRUMS: KIT folder→62kick files, NEXT through8pages, selected **S2C1-1 KICK57**, beyond the old8kick variants. Closed automatically; Sample1LCD now **LIBRARY**, selected-sound footer identifiesKICK57. Source file loaded and direct/automatic device output recorded.
- BEATS selects AppleJacks; MAIN correctly shows **AppleJacks Kit -84BPM**, not stale Pocket01. GROOVE shows93templates/12pages, selected **010-MPC3K8-59%**. Compact31-byte footer no longer truncatesOFF. Folder/file names are display abbreviations only; original source names and bytes remain intact in bank.
- Final actual record `v020-final-device-playback.mov` / `.wav`:54.98s,44.1kHz stereo,3,518,904nonzeroPCM samples; remote dropout counter26before/26after (no additional reported remote dropouts). Monitor volume was temporarily zero to keep local playback quiet; recorded source PCM remained present. Preview stopped; remote monitor volume reset using native double-click full-volume control and AudioOff restored. This is evidence of actual device output, not full long-duration/CPU/other-model certification. Existing output safety ceiling remains; full-bank kit loudness curation is pending.
- Loaded the pre-update **Init Pocket.xpl** and verified original settings(Swing64/SnareLate16/HatShift-2/FillD/70% etc). Opened FillA/BAR2 and verified **4notes,4bars,Count-inOff,Velocity38,Nudge-2ms**, including the four original role events. No user recording was replaced by the QA playbacks. Re-loaded Init Pocket.xpl again to restore the original full state, returned toMAIN, PreviewOff, FollowMPCOn as saved. Gdqa.xpl remains untouched.
- MAIN Scope choice popup opens over the current page, hides the background guide, outside touch closes it and the guide returns. Library/choice modal definitions do not share mutable children. The temporary local caffeinate timers were stopped after verification; no permanent power/security setting was changed.
- Hardware16knob physical rotation matrix was not repeated while the user was asleep. Previous0.1 physical Swing/FillSelect and A1–A4 tests remain historical; current fixed16/8relay targets, relative increments, callback concurrency and physical MIDI defaults are covered by software regressions and unchanged mappings. Hardware mixer destinations/external sidechain remain pending as documented.

Final evidence: `v020-main-complete-device.jpg`, `v020-library-assigned-device.jpg`, `v020-library-lastpage-device.jpg`, `v020-library-verified-device.jpg`, `v020-library-beat-device.jpg`, `v020-playing-imported-beat-device.jpg`, `v020-user-four-notes-restored-device.jpg`, `v020-user-state-final-device.jpg`, `v020-choice-guide-hidden-device.jpg`, and `v020-*-verified-device.jpg` for pages. Candidate hashes, complete install log, software logs and audio metrics are beside them in build/qa. Source/bank stay private and have not been pushed or published.

## 0.3 final MPC result — Live III3.10.1.12

- Official SDK updater installed0.3.0 on existing SD36C7-1ACA,71files matched. Refreshed instrument instance without app restart. No public push/Release/Pages and no Donut Theory changes.
- Fresh MAIN, native header PRESET list now contains Init and Kits/V01..V06 entries. Selected Velvet01 and Velvet02 from the native selector; internal FactoryKit name and audio settings followed. Source dump confirms97programs.
- ROLL caption centre selected Pocket11 and second press on dice icon selected Loose01; counter1→2 and native header tracked both. Undo restored Pocket11 and exact visible timing/FX. Kit Lock retained Pocket11 and displayed KIT LOCKED/3 while Kit+Groove still changed unlocked groove. Amount0 displayed AMOUNT ZERO.
- DRUMS A displayed DrumKit/DiceMode/RandomAmt/KitLock, B/C/D retained Kick/Snare/ClosedHat edit controls. Dragging DrumKit changed Dust14 with native header and selected filename update. Internal list traversed13pages to Dust16, selected it and closed; behind-popup Q-Link guide hidden then restored. KITS page inspected with FactoryKit and original user/choke controls.
- MixParts selected by touch, ROLL changed the role combination and internal name became Custom/Legacy; native program index0 represents this custom state and header displays Init/Pocket. This host label does not mean MixParts reloaded Init. WholeKit results display their actual registered program name.
- Actual device recording v030-device-playback.mov/.wav:37.703s,44.1kHz stereo,1,763,156nonzero samples,peak−4.49dBFS,RMS−28.71dBFS. Includes four direct-role hits, Preview and live MixPartsROLL. Remote dropout counter27before/27after. Monitor temporarily muted; volume reset by native double-click and AudioOff restored, RecordOff/PreviewOff.
- Pre-update Kitqa.xpl exists beside untouched Gdqa/InitPocket. Loaded it, inspected FillA/BAR2:4notes/4bars/Velocity38/Nudge−2ms retained. Changed native kit toVelvet02 while that editor remained open and verified all four notes/settings survived. Reloaded Kitqa afterQA, returnedMAIN. Kitqa restores Swing50/Snare15/Hat8/Nudge1 in the saved file; the pre-save screen had shown69/12/−2/5, so exact host save timing/settings fidelity beyond verified fill and restored FX is not certified by this UI run. No saved user file overwritten. Software chunk tests are byte-identical.
- Physical knob rotation cannot be newly exercised while user asleep. New DRUMS fixed16map and relative choice accumulation are software-tested; on-screen knob control tested ondevice. Previous physical MAIN/FILL knob tests remain historical. Full hardware routing/other models/external sidechain remain pending.
- Actual captures: v030-native-presets-device.jpg, v030-roll-changing-kit-device.jpg, v030-drums-kit-control-device.jpg, v030-kit-lastpage-device.jpg, v030-kits-device.jpg, v030-native-kit-preserves-fill-device.jpg. QA artifacts remain private.

Temporary owned caffeinate7200timer stopped after this verification; no permanent sleep/security setting changed.

### Native file save follow-up

Saved the visible0.3state as new Saveqa.xpl (existing files untouched), then ROLL changed it and reloaded Saveqa. Swing50/Snare15/Hat8/Nudge1,100% pattern controls and Drive53/Dust46/Room8/Duck53 restored to the exact pre-save visible values. This confirms a new0.3native host-file round-trip; the older Kitqa pre-save screenshot discrepancy remains a historical observation rather than a reproduced0.3save failure. Evidence:v030-native-save-roundtrip-device.jpg. Final device MAIN, restored Saveqa settings (same Kitqa content), Preview/Record/AudioOff.

## 0.4 initial device pass and restore correction

Initial72-file package: MAIN C440/546/376pair/244role selectors all opened the correct modal and selected source captions; guides hid/reappeared. GROOVE D fill controls visible. PreviewROLL showedNEXTBAR then factory/header/name changes. Actual57.566s/44.1kHz stereo recording containsAmount100%Kit+Pattern changes:3,426,441nonzero samples,peak−3.81dBFS,RMS−26.91dBFS,remote dropouts27→27. File:v040-device-pattern-playback.mov/.wav.

Before.xpl restore exposed late host parameter replay re-triggering saved command states (ROLL counter3→4, unwanted preview/pattern movement). This candidate is superseded by a post-load0.5audio-second command/side-effect guard. Added regression simulates chunk load followed by interleaved per-parameter replay; noROLL/REC/navigation fires and userROLL works aftersettle. Full tests/hostsim19/0/0 pass afterfix. Do not claim the initial restore screenshot is successful preservation. Reinstall and device restore observation remain required below.

Restore guard regression also replays a stale oldQ9=1 and verifies it equals newSoundKick afterguard finishes. Final protected block invalidates relay seen-target caches to restore correct hardware-relative starting values. Rebuild/native+ARM and fullrunner19/0/0 passed again.

## Final0.4 device restore result

Final protected package installed on same SD; official updater72files match. Refreshed instance withNONE→GrooveDice. Loaded untouched Before.xpl: native/header **V05/Honey03**, Pattern **BankBeat02**, Swing59/Snare27/Hat−5/Nudge7, Ghost100/VelSpread50/Amount100, FX10/8/9/18, FillD restored. **PreviewOff, DiceREADY, no unsolicitedROLL/REC/navigation**. This corrects the failed interim restore, rather than accepting that changed state. Inspected FillA/BAR2:4notes,4bars,Count-inOff,Velocity38,Nudge−2ms retained. Captures:v040-final-restored-main-device.jpg/v040-final-four-notes-device.jpg. Reloaded Before again torestore savedMAIN context/FillD afterinspection. Existing savedfiles notoverwritten.

Positive latestguard test and negative-control(logs inQA) cover mismatching saved command values betweenchunk andlate per-param replay plusstale oldQ9=1. Withguard disabled regression fails, withguard enabled no command fires andQ9resynchronises. Real nativefile restore now confirms samefix. Source UI files unchanged from prior successful MAIN/rolepopup/GROOVE observations; current binary includesguard. Allactualcaptures are Remote Screen Viewer images, notconcepts.

New physical Q-Link rotation has notbeen performed. Fixed mappings, relative increments and concurrency tested insoftware; onscreen roleknobs and rolelistspages tested ondevice. Original userstate still has strong100%Ghost/50%VelocitySpread/100%Variation values; loading preserves these intentional savedvalues. The saferROLL algorithm doesnot silentlyresetmanual performance controls.

### Live input at hand-back

After the successful Before restore/fill inspection, a later fresh screenshot showed MIDI29/OPENHAT plus changed role selections and the native vertical Q-Link popup labelled Kick/Snare/Hat/Perc. These were new live-device inputs while the agent was not sending such controls; treated as user activity and left untouched. v040-final-main-device.jpg records this later live state, not the preservedHoney03 baseline. The clean restored screenshot andfour-note screenshot above remain the restore evidence. NativeQ-Link displayed names/values matchnewCgroup; this observation is not a controlled sixteen-knob rotation matrix. Preview/RemoteRecord/Audio wereOff before the live activity. Owned temporarycaffeinate3600/1800timers stopped; no permanent power/security settings changed.

## Final0.5 MPC result — Live III3.10.1.12

- Official updater72files match on existingSD, refreshedNONE→GrooveDice, freshMAIN. MAINB showsRULES(defaultPOCKET/POCKET/LOOSE/LOOSE). Allfour rule choice menus selectedLOCK andreturned correctvalues. Rulepopup includesplain descriptions andKitLock distinction. No publicpush/Release/Pages.
- MAIN A/B/C/D focus inspected byonscreen knobtouch. Eachorangeguide enclosesits fullcreamcardwithoutsidepadding; noLCD/caption clipping. Captures:v050-main-{a,b,c,d}-guide-device.jpg. This isactualguidefocus, notjust sourcegeometry.
- Withright-side B/Dguidefocused, openedKIT andPATTERN: guide/Qbadge disappeared behindbothpopups; X/outsideclose restoredthe priorguide. Rulepopup alsohidesguide. Captures:v050-kit-guide-hidden-device.jpg, v050-pattern-guide-hidden-device.jpg, v050-rules-popup-device.jpg. Fixingspecified orderedserialization resolvedobservedoverlay; no customguidehack used.
- ActualPreview+ROLL recorded withallLOCK andKitLockOn/PatternScope, thendefaultPOCKET/POCKET/LOOSE/LOOSE atAmount100/Ghost100/Velocity50. NEXTBAR→completed counter, stablemanualAknobs, kit/header/sampleupdates observed. GROOVE containsoldPatterncontrols andrule-limitcaption. Physical16-knob rotation matrix wasnot repeated; onscreencontrol actions plusfixedsource mappings/relative/concurrency regressions verified.
- Device recordv050-device-rules-playback.mov/.wav:176.776s,44.1kHzstereo,10,887,073nonzero samples,peak−0.088dBFS,RMS−21.93dBFS. IncludeslegacyInit loudsource/safetyceiling andsilencewhileediting aswellasfactorykit playback. Remote dropoutcounter27→29duringrapidUIoperations, so notadropout-free orclip-free performancecertificate. Musicalquality stillsubjective; constraint/eventtests verifyrules precisely. Monitorvolume restored, Audio/Record/PreviewOff.
- Loaded untouchedGdprior.xpl:Loose04/BankBeat02,manual57/27/−6/7,FX13/4/5/21 restored; newB defaultsPOCKET/POCKET/LOOSE/LOOSE; no unintendedcommandreplay. Fourphysicalfillnotes remainstored(Velocity38/Nudge−2ms). This savedfile hadA length1bar despite notesinBAR2; temporarilyexpandedto4bars andrefreshedBAR1→BAR2 tovisuallyconfirmallfour, thenreloadedGdprior withoutoverwriting it. Evidence:v050-four-notes-preserved-device.jpg. Otheroldpresets untouched.

Finalsources/manual/SDKstate remainprivate. Existinglimits(externalsidechain/route-destinationmatrix/othermodels) remainasdocumented.

Final v050-final-main-device.jpg shows restoredLoose04/B RULES/newAguide, withPreview/Audio/RecordOff. Owned temporary3600s caffeinate stopped afterverification; no permanent systemsetting changes.

## 0.6 first device pass (before wording polish)

Official update installed0.6.0 toSD36C7-1ACA,93file sizes verified. Removed/re-added plugin; newinstance openedMAIN, no helper popup. SavedpriorstateGdsafe.xpl untouched.

- MAIN lower action lanes60px confirmed; no on-screen drum pads. ROLL shortpress #1Pocket11, two-secondhold #2Honey06; release didnot triggeranotherroll. UNDO returnedPocket11. MOV rawframes10.861/10.927 showolivepressed ROLL,12.978orange again: actual2.117sec colourchange/release, not anoffline mockup.
- Perc browser lists244sets across31pages; choosingV01Set1 changed C4 while Kick46/Snare67/Hat68 remained. Kit/Pattern/Role/Library overlays hid backgroundnativeguide, restored onclose. Actualkit/pattern/MIDIpopups captured. Globalnativeheader showsInit/PocketforCustom as documented.
- LIBRARY BEATS showsnewVelvetBackbeat84/BasementPocket90; GROOVE shows93newtemplates across12pages. ChoseVelvetSketch55, returnedMAIN, Previewplayed andpositionadvanced. LiveROLLshowedNEXTBAR thenappliednextkit once (counter3).
- EnteredFillEditor; no screenpads; lowerMAIN/REC/AUDITION/CANCEL/DELETE/UNDO visible andreturnworks. Visually inspectedGROOVE/DRUMS/KITS/FX/ROUTING, nooverlapping controls. No freshphysical16knobmatrix/newfillpadrecording certified inthispass.
- Found rawSoundPick0 commandname infeedback; finalformatters changedtoSOUNDSELECTED/LIBRARYSELECTED/NEXTPAGE/etc andPerccustomshowsCUSTOMSET. Formattingtests+nativeARMbuild+hostsim19PASS runbefore finalreinstall. AudioDSP unchanged.
- Firstrecordingbuild/qa/v060-first-device-ui.mov/.wav (220.1s); metricsin v060-first-device-audio.json. Remote dropouts29→35duringrapidUI, soaudioobservationnotworst-caseMPCorremoteperformancecertification. PressframePNG0showsnativepressedstate. Device UI duringsecondlargecopy canlag; do notinterpret unchangedcachedframesasverifiedMAINfocus.

## 0.6 final device completion

Secondofficialupdate93files matched. FreshinstanceopenedMAINwithoutmessages. Finalwordingcapture v060-final-wording-device.jpg shows1/SOUNDSELECTED andV01SET1. Freshcustomstate showsCUSTOMSET. ActualMAINB/Dfocusbounds inspected andcaptured in v060-guide-{B,D}-device.jpg; A/Cobserved inearlierpress/Percpasses. All4nativeguideenvelopes fitpanelswiththeplannedoutermargins.

LoadedGdsafe.xpl andverifiedV04/Loose04, BasementBounce(oldPattern1name), Timing57/27/−6/7, FX13/4/5/21, originalsamplecaptionsandRules1/1/2/2. Sampleloadingtextremainedwhileidleafterrestore; startingPreviewletprocessing/labelupdatecomplete, libraryreadytextreturnedandpositionadvanced; finalstopcaptureBAR12. Previewthenstopped; noaudio/recordinglefton. Thisidleloadingstatuslatencyisrecordedasobserved, notafailedWAVdecodeorclaimofinstantUIrefresh. Finalnative1280×800capture v060-final-main-native.png; Desktopappcapturealso retained. No publicpush/Release/Pages made.

Temporarycaffeinate3600process49343usedonlyforthisQAwasstoppedafterverification; permanentpower/securitysettingsunchanged.

## 0.6.1 device evidence

- FreshinstanceopenedMAIN. LoadedGdsafe.xplwithsameLoose04/Timing57,27,-6,7/FX13,4,5,21. MAIN A/B/C/D capturedindividually; QbadgeisbelowtheA–Dheaderandbesidethefirstcaption, allfourname/knob/LCDgroupsfitinsideorangeoutline. Noheaderbadge/textoverwrite.
- Everydetailpagewasvisuallyinspected. ActualfocusedguidescapturedforGROOVE A/C(longPreviewBPMcaption), DRUMS A, KITS A, FX B andROUTING A. Firstcaption/knobgutterworksforlongnames. FXAmarkerwasnotshownwiththecurrenttouchfocus, so do notclaimallhardwarebanksrotatedoralltouchfocushandlerscertified; theallpagegeometryregressioncoversallnonemptyrectangles.
- Kit/Pattern/KickRule/Perc/LibrarysidebarsopenedwithoutbackgroundQguides; X/outsideclose restoredthem. FilEditoralsohidbackgroundguidesandMAINreturnrestoredfocus. Captures v061-{kit,pattern,rule,perc,library}-popup-device.jpg andv061-editor-device.jpg.
- Finalnative1280×800capture v061-final-main-native.png displaysBbadgeandQseparately. ActualMAIN/6detailpageJPEGsretained inbuild/qa. AudioOff/RecordOff/PreviewOff; noMPCapprestartorpublicrelease/push. NoDSP/bank/parameter/mappingchanges beyondversion601andUIcoordinates.

## 2026-11-17 — 1.3.1 local QA

623 parameters; previous 536 parameter declarations preserved exactly. Appended fixed-size source browser, independent 3×12 cymbal sound fields and a 64-kit identity set. Existing GDLY0001 size/order stays fixed; GDCY0001 and GDPL0001 trailers are peeled before old data. Sample browser and selected-kit browser suppress native guide overlays. Every sample row has a press/release visual and isolated preview; preview does not call recordInput. A newly exposed state-context bug was fixed: restore now seeds page/slot and Q-Link observation caches so loaded browser folders and fine relay values are not rewritten as fresh navigation. A seeded 200-parameter regression reproduces and verifies byte-identical recovery.

See tests/sample_browser_test.cpp and tests/state_context_test.cpp. SDK output validation: 0 errors, 0 warnings. Actual physical touchscreen/Q-Link verification is pending owner review; no fresh device screenshot is claimed.

### Decay-first SOUND mapping

All 17 sound/layer targets now expose Sample / Level / Decay / Shift. Details expose Start / Tune / Low Cut / High Cut, including the dedicated closed/open-hat detail groups and selected-cymbal D group. No parameter definition, ID, audio value, user kit or state trailer layout changed for this rearrangement. `decay_controls_test.cpp` checks every relay target, Tune/Shift isolation, detail access and actual rendered tails: after 100ms, a 20ms setting has less than 0.1% of the tail energy of a 1000ms setting. The same test passes with all four owner-profile expansions installed. End was intentionally not added after the owner clarified the goal was shortening hi-hat tails.

Device deployment: MPC XL internal /nvme, local owner QA 1.3.1. SDK install verified 167 files by size. Independent SHA256 matched ARM32/ARM64 binaries, TUI/Q-Links and all four expansion banks, including the new owner-only Tape Series. Registry preserved all four entries. SDK app restart was requested successfully. Physical Decay/Q-Link feel remains for owner confirmation.

Final Decay run: full test.py completed with exit 0; all engine/UBSan/count-in/layer/browser/kit/identity/skin/expansion checks passed. Host simulator: 18 PASS, 1 WARN, 0 FAIL. The remaining warning is re-saved chunk bytes differing after its random parameter sweep; all 623 parameters restore with matching display text. Do not describe this run as 19/0/0 or claim byte identity for every VST-host sweep. The dedicated engine 200-write context regression is byte-identical, and 17-sound Decay audio tests pass with core and owner profile.

## Device blank-screen regression — owner screenshot, 2026-11-17

Owner screenshot shows normal MPC track/plugin/preset header with no plugin body or function-key tabs. This supersedes any implication that SDK/file-hash checks proved successful UI loading. SDK ui-reference section 1.3 identifies no tabs as a TUI parsing failure. A skin-only compatibility recovery based on the previously working 1.2.2 TUI (with Decay/detail labels and new MAIN D bindings adapted) was installed; the 1.3.1 engine and four packs were retained. Owner confirmed “본문과 하단 탭이 보임”. This isolates the regression to the new skin rather than requiring an engine/content rollback.

Comparison found 20 fixed-Text labels with an empty text string, introduced by captionless sample/list hit buttons; the working skin had none. A full-feature candidate now omits these empty caption widgets while retaining command press/release surfaces and external dynamic labels. Source build_skin.py now omits blank captions, and skin_test.py prevents their return. Skin-only candidate installed with matching SHA256; SDK restart completed. Hardware confirmation of this full-feature candidate is pending; empty text is a suspected parse trigger, not yet a proved firmware rule. Working compatibility recovery is retained on NVMe in an owned diagnostic directory. No public release or website deployment occurred.

2026-11-18 hardware follow-up: after the full-feature candidate omitted the 20 empty fixed-Text captions, the owner confirmed “본문과 탭이 보이는데”. This verifies body/function-key tab recovery on the MPC XL for the full skin, superseding its pending display status above. The engine/packs were unchanged during the skin-only recovery comparison. Owner then reported that MAIN knobs do not move on Roll Dice; this is being evaluated separately from skin parsing.

## 2026-11-18 — 1.3.2 per-control Dice QA

Appended 16 Dice Lock parameters (623–638); all previous 623 declarations unchanged. MAIN has 44px lock targets with no empty fixed-Text captions. Each target maps to the actual front parameter; Dice writes visible values, and refreshes SCREEN relay/display state. Timing and mix are bounded absolute draws. Undo now includes all front parameters and Hat Rhythm page state; locks themselves stay as currently configured. Hidden legacy timing offsets are no longer generated by Dice.

front_dice_test.cpp passes UBSan/allocation guards: each lock survives 25 All rolls with exact value preservation; manual Q-Link remains live; all-lock works across six extra modes; lock state saves/restores; Undo restores all 16 values; 300 unlocked rolls preserve kick/backbeat structure and range constraints; Amount 0/10/35/80/100 cases are covered. Fixed-control pattern/division test fixtures now explicitly enable the corresponding Dice locks, while the dedicated unlocked tests exercise the new behaviour. KIT FIXED audio assertions now cover actual role/layer/cymbal sound fields rather than unrelated later UI/style parameters.

The new seeded parameter count exposed another genuine restore issue: lastKit was not seeded after effSetChunk, so the legacy-kit poll could treat loaded values as a fresh selection and replace expansion sources. Restore now seeds lastKit and lastSoundGroup along with page/relay observations; state_context_test again saves byte-identically. Hardware installation/visual confirmation will be recorded when completed.

1.3.2 final automated run: full test.py exit 0, UBSan/allocation guards pass, all previous 623 parameter declarations remain identical, SDK native/ARM validation 0 errors/0 warnings, host simulator 19 PASS / 0 WARN / 0 FAIL. This supersedes the 1.3.1 host re-save warning: seeding the legacy-kit observation after state restore resolves the new reproduced overwrite path. Main lock-image composition checked offline; it is not a fresh hardware screenshot.

Device deployment: MPC XL internal NVMe, version 1.3.2. Official SDK update verified 169 files. Independent SHA256 matched both ARM binaries, TUI/Q-Links, both 44px lock PNGs and all four owner expansion banks. Registry retained Cosmic Vintage Drums Private, Workshop Sessions, Splat Private and Tape Series Private. SDK restart reported running again. Owner was asked to verify full skin, 16 lock buttons and Swing-lock preservation versus unlocked visible Dice changes; this physical confirmation is pending. No release/website publishing was performed.

1.3.2 owner confirmation: “다 잘 되는데” in response to the full skin / 16 padlocks / Swing-lock versus unlocked Dice test. The owner then requested removal of the redundant LOCK style option.

## 2026-11-18 — 1.3.3 RULES selector QA

Four new seven-style selector parameters append at 639–642; previous 639 definitions remain unchanged. Touch widgets and default Program/Track Q-Links now bind these selectors, and MAIN SCREEN relays map their full range to legacy style indices 1–7. Fine relative inputs retain their accumulator and do not fan out notifications. New selectors cannot select the legacy 0=LOCK value. Old project LOCK states retain their held rhythm while displayed as closed padlock + BASIC; unlocking explicitly releases into BASIC. Saved old MAIN Q-Link ranges are recalculated for the seven-state selectors. Native selector names remain concise.

Targeted UBSan/allocation-guard tests pass for all four seven-choice lists and 4004 SCREEN fine steps; real VST direct selector/legacy/SCREEN inputs, bounded notifications and reentrant value echoes; old639/new643state migration and exact new-state recovery; and all 16 Dice locks. Legacy metadata snapshots remain exact. Full build/regression and device result follow below when complete. No public release/website publish is authorised by this local QA change.

## 2026-11-18 — 1.3.3 release qualification

The owner confirmed the installed 1.3.3 MAIN display, individual locks, visible Roll Dice changes and RULE selectors work, then authorized public release and direct MPC screenshots. Native/ARM32/ARM64 and SDK validation passed; the complete regression suite passed with host simulator19 PASS/0 WARN/0 FAIL. Final caption visibility correction upgrades conditional LOCK KIT bounds to version2; skin regression checks require it. No engine change followed the owner confirmation.

The public package is assembled from the original core only and audited after ZIP creation: one default.gdbank, byte-identical to the original source bank; no expansion payload. Optional owner banks are preserved independently. Direct captures use MPC Remote Screen Viewer rather than rendered mockups. Full physical-control coverage, every separate route and worst-case MPC load remain outside the verified scope.

## 2026-11-18 — 1.3.4 SOUND pages and reverse candidate

User approved the two-page layout, explicit previous/next arrows and page counter, with polarity retained as Ø on page2. Page1=Sample/Level/Tune/Shift; page2=Start/Decay/LowCut/HighCut. Each row retains its fixed SCREEN group. Both arrows are48×48, page changes affect every visible row, category/cymbal changes preserve page, and the separate detail modal is removed. REV is independent for17sources; all source/kit/preset compatibility limits remain.

Targeted UBSan tests passed: all17sources×2pages×4Q-Link targets, no cross-row writes, independent REV/polarity, inactive HatC/D and CymbalD input isolation, current-page state roundtrip and sample-drawer ownership/restoration. Decay tail tests cover17sources, and reverse playback tests cover17sources×3rates×3blocks, stereo interpolation, Start from the reverse end, safe one-frame/empty/beyond-end termination and render allocation guards. Skin checks pass for page labels, arrows, bounds, guide gutter, sample preview/tap surfaces, caption safety and preserved native order.

Native/ARM32/ARM64 builds and SDK validation pass. The final full regression suite passed with exit0, including host simulator19 PASS/0 warnings/0 failures. The local run log is /private/tmp/gd134-pages-full-tests.log. The completed SOUND skin tree expands to1487widgets, below the2000 regression limit. Installed through the official SDK to MPC XL internal /nvme:156files matched, followed by app restart and matching engine/skin/Q-Link/bank hashes. Four owner expansions remain registered. These file/build results are not a physical touch or encoder confirmation; direct new device captures and physical page/REV verification remain pending. Public release remains1.3.3 until the new candidate is qualified/published.

## 2026-11-18 — direct MPC SOUND UI verification completed

After the user-requested MPC app restart, the native Mac viewer could capture but its desktop mouse path did not deliver input. A temporary authenticated RSCRv2 client successfully sent remote input directly to the MPC service; the service allows only one viewer connection at a time. No device preferences were changed. Main-mode plugin preview opens the external GUI; the pencil icon edits the track, and the Pad Mixer shortcut is not the plugin editor.

Actual MPC XL1.3.4 screen frames are preserved in design/device-1.3.4 with checksums and transport provenance. Verified using remote clicks: MAIN opens first, SOUND1->2->1 page arrows/counter, changed row labels, REV highlight/retention, independent Ø, page retention across sound categories, and correct two-row hats/three-row cymbals. Remote clicks are not physical encoder/finger or hardware-audio certification. The captures show the owner profile's optional-expansion counts; the public ZIP still includes only Original Workshop.

## 2026-11-18 — 1.3.4 final release touch correction

Actual MPC XL remote input found that overlaid dynamic captions swallowed center taps on sample plates and list entries. Command captions now live inside the command wrapper, before its final transparent ResetOnGestureEnd press surface. Legacy library/role/kit rows also keep exactly one command target above their labels. Regression checks cover this ordering and prevent empty fixed-Text widgets. No DSP/parameter change followed the full engine regression. Native/ARM builds and SDK validation passed again; final skin checks and host simulator passed (19 PASS / 0 WARN / 0 FAIL).

On the installed correction, center taps open sample lists, selecting text changes Dry Kick02 to Dry Kick01, kit-list text selects Velvet Sunday, and selected-kit checkbox text toggles the stored pool/count. A 17-note fill remains after changing kits. MAIN, both SOUND pages, sample drawer, cymbals, fill editor and Workshop Sessions were captured directly from authenticated MPC Remote Screen frames, without retouching. Image checksums and capture profile are in design/device-1.3.4/verification.json. Original Workshop was selected using the UI while all four owner packs remained installed; the visible global library totals therefore exceed the public ZIP's 128 original WAVs. No physical encoder, hardware-audio or worst-case CPU certification is implied.

Public release qualification: 1.3.4 core-only ZIP, manual, release notes and website use the final installed skin and direct device captures. Publication and live download/deployment checks are tracked in the release record.

## 2026-11-18 — 1.4.0 candidate final verification

All36new rules have distinct raw and actual scheduled fingerprints. Variable32/512sample blocks preserve backbeats2/4, exclude kick2/4 and retain other-part/fill data. All16hat styles ×15Hat Rhythm choices remain nonempty, including UPBEATS with1/4. Full suite passes (host19PASS/0WARN/0FAIL), native/ARM32/ARM64 SDK0errors/0warnings. Real VST bridge checks4×16selector values and4004fine SCREEN increments with bounded notifications and synchronous reentrant echoes. New styles cannot collapse to legacy fallback aliases during host notifications; canonical writes notify their visible target. First672metadata stay exact;676total.

Installed using official SDK on MPC XL internalNVMe and restarted. Final official export matches143public runtime files;13owner expansion files/four packs are preserved separately and excluded from public source/packages. Direct MPC remote captures under design/device-1.4.0 show the16-choice paged drawer, END DOUBLE/UPBEATS1/4, a live remote knob change to LATE ANSWER and Dice selecting EARLY PAIR/TWO ON ONE/LATIN THREE. Kit Fixed retained the kit/mix; preview was stopped afterward. These checks do not certify fresh physical encoder rotation, hardware output listening/recording, full project round-trip or worst-case CPU. Public stable remains1.3.4; no public release/site publication was made.

## 1.4.0 publication approval

The owner explicitly authorized release and website updates after final candidate checks. Public packaging audits143matching runtime files plus an empty generated expansion registry; the sole bank is byte-identical Original Workshop. No optional owner expansion payload is included. Current manual, direct MAIN/RULES captures and the complete16-style feature description accompany the release. Publication outcome is recorded in design/device-1.4.0/publication.json. Earlier local-only lines are historical.
