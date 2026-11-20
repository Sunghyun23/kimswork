# Donut Theory 작업 지침

루트 AGENTS.md를 함께 적용한다. 작업 시작 시 [최신 인계 기준](../../../docs/plugins/donut-theory/handoff.md), spec.md, design.md, engine.md, presets.md와 해당 SDK 스킬을 읽는다. 문서의 버전별 개발 이력보다 최신 사용자 확인/인계 상태를 우선한다.

- 현재 정식판은1.9.0, vendor `kim`, UID `Krql`, 파라미터288개, 공장음색66개. 첫287개 선언·기존64프로그램·legacy46범위를 보존한다. 최신 매핑은 아래1.8.0/1.9.0지침과 handoff를 따른다.
- 기본 엔진은1.2.0 Moog-inspired mixer/4단 ladder/VCA. 프리셋만의 튜닝 또는 SDK 기본SVF를 래더로 소개하지 않는다. 정확한 구버전 소리 복원은 보장하지 않는다.
- MAIN A=OSC Pair/Osc Mix/Detune/Drift, B=Filter Cutoff/Resonance/Body/Sub, C=LFO1Rate/Depth/LFO2Rate/Depth, D=Drive/Glide/Mod Rate/Depth. ADSR는 AMP/FILTER, Space는 FX 상세다.
- 첫 함수 키는 MAIN↔CHORD 두 서브탭. 초기 MAIN/안내팝업 없음. 사용자 직접 터치 정상 전환 확인 완료. CHORD Q-Link는 Root/Type/Voicing/Octave이며 Program/Track 신스 매핑은 유지한다.
- 수동 Pad Location UI는 제거했다. `tools/pad_positions.py`를 build_skin에 다시 연결하지 않는다. 호환용 PadPosition1–16(192–207)은 유지하되 미사용이다. 실제 MIDI Note/LEARN과 패드 연주는 유지한다.
- CHORD 구성 그래프의 펼친 위젯2500 이하/재귀 cycle 검사, 전 탭 modal 루트의 마지막 형제 순서, hold Slider/Mouse Down 동작과 캡션 위 hit 표면을 보존한다.
- process()에서 할당/lock/I/O/host알림 금지. 중계 state-load 보호·render-thread token+index·노트 소유권을 임의 단순화하지 않는다.
- native/양쪽 ARM·SDK validator·host simulator·engine/UBSan/bridge/analog/skin 결과와 실제 기기 확인을 구분한다. 사용자 확인은 사용자 확인으로 기록한다.
- 모든 물리 입력·최악 CPU·다른 모델·전체 MPC 프로젝트 복원·원격 dropout 해결은 완료로 주장하지 않는다. 최신 상세는 handoff.md에 있다.
- 새 변경은 새 버전으로 배포한다. 이미 공개한1.9.0및 이전 ZIP/태그를 재작성하지 않는다. 다른 프로젝트 작업을 이유로 이 플러그인을 변경하지 않는다.

- 1.3.0 사용자 승인: CHORD 오른쪽 Key/Major·Minor/참조용5도권·다이아토닉 표시, MIDI/LEARN은 PAD SETUP으로 이동. 기존 Root/Type/Voicing/Octave16Q-Link/가이드 범위 유지. Key 선택만으로 노트를 변경하지 않는다. TRANSPOSE 확인창에서 FROM/TO/반음/활성슬롯수를 표시하고 APPLY만 모든 활성루트를 동일간격 이동한다. 타입/전위/옥타브/패드/비활성슬롯 보존,24..84범위를 넘으면 전체거부. 208개기존선언보존, 새208..219append. 새지침은 이전208총수/구형CHORD배치보다 우선한다.

- 1.4.0 SMOOTH VOICING: Current4/AllActive, first chord fixed, closed-loop top-note minimization before inner/bass/edit ties, before/after audition and explicit APPLY. Root/Type/Key/MIDI/unselected slots preserved; only voicing/octave change. Existing220declarations remain; append220..241. No solver on render: UI poll/idle publishes atomic cells, audio rejects stale plans and owns finite preview4096. Save/load clear transient commands. Keep242metadata/state/UBSan/preview ownership/atomic publication/skin budget tests.

- 1.5.0 user-approved32types/96progressions, ChordGlide and one-shotSTRUM. Preserve first242metadata/ranges and original11/32meanings; canonical/relay additions242..271 total272. Do not expand legacy normalized choice ranges. Use existing Glide/Spread parameters; preserve16Q-Links with full32-type relays. Keep MIDI/touch/channel/pedal/fallback/duplicate/page-reset ownership and pending strum cancellation tests. Current-bank preset UI is one relay/browser; MOD is bank1. Avoid reintroducing4hidden96-row browsers or32voicing trees. Extended chord formulas/recipes live in tools/harmony_bank.py and generated ChordTypes.h; unchanged first11 and64distinct new harmonic forms are regression contracts.

- 1.6.0 approved shared motion: MAIN D=Drive/Glide/Mod Rate/Mod Depth, touch Tremolo/Auto Pan +Shape, basePan onFX. No D subpage or independent simultaneous modules. First272metadata exact; append ModMode/ModShape/Pan272..274. Preserve stereo, depth0/base-pan behavior, smoothed mode/shape output, migration and zero-allocation tests.

- 1.6.1 user layout correction: MAIN D Mode/Shape selectors align above the right-hand Mod Rate/Depth knobs (x964/1114); do not put Mode above Glide. Preserve four knobs and Q-Link geometry.

- 1.7.0 sync: keep first275 declarations; append ModSync/ModDivision/LfoRateControl/ModRateControl275..278. Each MAIN rate has its own SYNC toggle and the same physical Rate control selects Hz or division. Preserve free Hz, use guarded nonautomatable relays for Screen/Program/Track, and prevent the per16sample Motion Hz update from overriding Sync. Test independent BPM/PPQ, stop/fallback, division routing, old275 migration and no RT allocation.

- 1.8.0 latest approved MAIN: A=OscPair/Mix/Detune/Drift; B=Cutoff/Resonance/BodyTone/Sub; C=LfoRateControl/Depth/Lfo2RateControl/Lfo2Depth; D=Drive/Glide/ModRateControl/TremDepth. Move ADSRs to existing AMP/FILTER detail; retain Env/KeyTrack there. Legacy Body2 remains inert; activeBodyTone279 and independent LFO2 append280..286,total287. Preserve first279metadata, depth0/Body0 compatibility, per-LFO Sync, target smoothing, saved-state/routing tests and all guide/menu geometry checks.

- 1.9.0 publication: approved Soul Studies append programs65/66;66factory sounds,288parameters. Preserve first287metadata and legacy SoundPreset46range64; SoundCurrent287 owns full selector/host/arrows. Sound selection must not alter master/key/chords/pads/Strum/ChordGlide. Runtime Body/LFO2/sync map remains1.8.0. Use current manual/handoff, not historical stable1.2.2 paragraphs.

- 1.10.0local whole-sound generator: preserve first288metadata; append Randomize288(transient), Randomized289(persisted), FactoryReload290(transient),291total. RND48x52header gap910,26; no moved controls/extraQ-Link. Generate full timbre and force LFO1/LFO2/sharedMotion Sync ON. Preserve master/key/all16chord/pad/input/Strum/ChordGlide preference. Disarm command on save/load/reset; do not regenerate a saved random sound. Same-program reselect must reload factory; status displays Random Sound. Public stable remains1.9.0until new publication authorization.

- 2026-11-18: user approved official1.10.0 release and website publication. Earlier local-only statements above describe candidate history. Retain actual QA limits and explicit product tags.

- Local1.10.1approved footer reflow: preserve original art/type,291parameters/66sounds/16maps; move global browse/actions to bottom, keep local selectors adjacent and70px pads. Groove Dice stays untouched. See handoff for geometry and actual QA.

- User explicitly authorized official1.10.1 publication and website updates after device verification. Earlier local-only statements are history. Preserve original artwork and the recorded physical/audio limits.

- Latest approved local1.11.0 ENV/PLAY supersedes the old FILTER/AMP detail split: ENV A/B=two ADSRs (native editable Envelope + four vertical faders each), C=Cutoff/Resonance/EnvAmount/KeyTrack; PLAY A=Volume/Expression/Glide/Bend, B=TriggerBase/ChordSpread/ChordSlot, C=VoiceMode/GlideMode/InputMode. D unassigned on both, no guide. Preserve291metadata/66programs, MAIN/CHORD/other tabs/maps/DSP and original artwork. Native Envelope field order follows pinned SDK; 81x96px fader texture stays below8192px. Public stable remains1.10.1 pending separate publication authorization.

- Latest1.12.0 local candidate fills PLAY D with shared Amp Release30/Drop Amount291/Drop Time292/unused. Preserve first291declarations/66programs/UID and MAIN filters. Drop is per-voice semitone ramp from actual glide pitch at gate-off, sustain-aware, no drop during held legato fallback, strum cancellation/retrigger/reset/CC120 safe. New settings defaultOff/250ms and persist across sound generation/selection. PLAY2x2 four-bank layout supersedes1.11.0empty-D. Public stays1.10.1; see handoff/evidence.


2026-11-18: User explicitly authorized official Donut Theory1.12.0 release and website publication. Earlier candidate-only statements are historical. ENV/PLAY and Release Pitch Drop are the current specification; preserve recorded physical/audio/CPU limits. Final publication details go in design/device-1.12.0/publication.json.

- Donut Theory1.12.0 publication complete: stable Release449932324, source949dcfbd, ZIP/SHA256 integrity verified anonymously, Pages44301107512success,21public/77responsive checks,7actual latest MPC screenshots. InternalNVMe173runtime hashes match. Earlier candidate/public1.10.1 statements are historical. Preserve recorded physical/audio/CPU limits; Groove Dice/Crate Press unchanged. See design/device-1.12.0/publication.json.

## 1.13.0 local candidate — section Dice locks

Five persistent nonautomatable locks append at293–297 (298total); first293definitions,66programs and every existing Q-Link map remain unchanged. Default is Unlocked; old293states migrate to all unlocked. Locks affect ROLL DICE only: manual controls and explicit factory selection remain available. Factory selection preserves lock preferences. If all five are locked Dice is a no-op, including held voices and the Random Sound status. Partial rolls keep the existing note-clear behavior.

| Lock | Protected settings | Visible control |
|---|---|---|
| OSC | Waves/pair/mix/detune/octave/pulse/sub/noise/FM/tone/body/drift | MAIN A title |
| FILTER | Type/cutoff/resonance/env amount/key track/filter ADSR | MAIN B and ENV B titles |
| AMP | Amp ADSR, Expression, Voice Mode, Glide and Glide Mode | ENV A, PLAY A and PLAY D titles |
| MOD | Both LFOs (shape/target/rate/depth/sync/division), shared Tremolo/Auto Pan, static Pan, Motion/Vibrato | MAIN C title |
| FX | Drive/type/mix, Chorus/Delay/Feedback/Division/Space, Low-cut, Sound Trim | MAIN D title |

Lock symbols use Groove Dice's dark padlock/cream-open/orange-closed convention, with24px art inside44px touch targets at title rails. Targets end before adjacent selectors; no controls or Q-Link slots are removed. The MAIN groups mix some DSP families: Sub/Body are protected by OSC even though they appear in B; Glide by AMP even though it appears in D; motion by MOD even though it appears in D. Chord/Key/pad/input/Strum/ChordGlide/PitchDrop/master controls remain outside Dice. All three Sync switches are forced on only when MOD is unlocked; locking MOD preserves existing Sync states. Writes are filtered before parameter callbacks/host-notification queues rather than restored afterward. Modal roots remain last with hideQLinkBounds.

Native/ARM/SDK and full engine/UBSan/bridge/skin pass, including32lock combinations×8rolls, save/load, all-locked held notes, manual/preset independence and old293migration. Physical/device checks are recorded separately. Public stable remains1.12.0 until a new release is requested. Groove Dice and Crate Press are unchanged.

### Dice-lock placement refinement (local1.13.0)

Latest user correction moves glyph-only locks to a leading column before each section name, away from panel corners and header collage. ENV/PLAY section titles align left with MAIN. Keep44px hit targets; MAIN lower menus shift4px and lower knobs/Sync/dividers3px to leave the targets clear of neighbouring values/menus. Panel and Q-Link bounds, all Q-Link JSON and engine/parameter contracts remain unchanged. Native/ARM/SDK, UI overlap regression and host19checks pass. Latest placement captures and device read-back live in design/device-1.13.0/leading-locks/. Older captures in the parent folder show the rejected right-corner placement. No public release/site update.


## 2026-11-18 공통 Dice 잠금 — 최신 구현 후보

이 결정은 과거 Donut Theory의 5개 헤더 잠금 후보를 대체한다. Donut Theory1.13.0 / Groove Dice1.5.0 로컬 후보이며, 별도 공개 승인 전 정식 버전은 각각1.12.0 /1.4.0이다.

- Dice가 변경하는 값에 개별 자물쇠를 제공한다. 같은 실제 값을 표시하는 페이지/릴레이는 잠금을 공유한다. 수동 터치·Q-Link·명시적 공장 프리셋 선택은 허용한다.
- Donut MAIN16개는 각 노브 왼쪽 위에44px 터치 잠금을 둔다. 다른 페이지의 실제 랜덤 대상도 잠근다. OSC Pair/Wave1/Wave2와 각 LFO의 Rate/Sync/Division은 각각 하나의 잠금을 공유한다. ENV와 PLAY의 Amp Release도 공유한다.
- Groove Dice는 기존 MAIN16개 잠금을 유지하며 SOUND는 실제 역할·레이어·페이지에 따라 잠금 대상을 연결한다. FX의 랜덤 대상과 필인 A–D의 실제 데이터 보호도 제공한다. 랜덤화하지 않는 사운드 파라미터에는 의미 없는 잠금을 추가하지 않는다.
- 기존 킷·패턴·파트·레이어 잠금은 개별 잠금보다 우선한다. UNLOCK ALL은 전체 페이지의 개별 잠금과 상위 보호를 해제하고, Dice 실행·음색/샘플/노트 변경은 하지 않는다.
- master/MIDI/routing 및 항상 보존하는 Donut 코드·키·패드·Pitch Drop에는 Dice 잠금을 만들지 않는다. Q-Link 할당은 바꾸지 않는다. 원래 미술과 글꼴을 보존하며 Crate Press는 변경하지 않는다.
- Donut first293 / Groove first676 파라미터 선언은 유지한다. 새 파라미터는 append하며 총353 /777개다. 공개 ZIP에는 개인 확장팩을 넣지 않는다.
- 자동 검사와 실제 MPC 터치/화면 검증은 분리해서 기록한다. 물리 Q-Link 회전·오디오·최악 CPU는 새 검증 없이 완료로 주장하지 않는다.


### 2026-11-19 native PRESET investigation / local1.13.3
Donut native popup is blank in Random Sound and fresh Init. Groove Dice1.5.1 and Crate Press0.2.4 native list/selection work on the same MPC XL. Slash-label replacement alone was tested and did not fix Donut; original names restored. A separate confirmed bridge defect: effGetProgram drained host notifications, causing UI callbacks on a render-thread metadata read. New regression fails baseline and passes after making effGetProgram side-effect-free. All353IDs/66program indices/recipes and UI/Q-Link maps remain. Final1.13.3native/ARM/SDK/full regression pass; temporary file logging is removed. Native-popup causality/hardware completion is NOT confirmed: device became unreachable at192.168.0.56 before final install. Installed version at disconnect was temporary1.13.2probe. Restore/install final1.13.3upon reconnect, then continue controlled API/skin comparison until native list and selection actually work. Do not report this candidate as the confirmed native-popup fix. See DonutTheory/design/device-1.13.3/native-preset-fix/verification.json and CratePress/design/device-0.2.4/native-preset-check/. Public releases unchanged.


### 2026-11-19 reconnect investigation continuation
Reconnected MPC XL: original full-art temporary1.13.4 again showed a blank native picker. Actual device program trace confirms all66 indexed names/count are valid (value=-1). An unmodified SDK dispatcher/value-reader comparison1.13.5 displayed the native list; original bridge with notification flush disabled1.13.6 remained blank. Removing only the early NeedIdle request1.13.8 also remained blank. Runtime idle-routing comparison1.13.7 gave a visible list but multiple writes advanced the mode, so this is a lead, not exact cause proof. Network/Remote/SSH all dropped again while loading a fresh QA project under1.13.9, before the plugin was opened. Installed1.13.9 is a TEMPORARY mode-comparison build; replace it before final delivery. Scratch1.13.10 tests only idle return0 and has no QA logging/controls; built, NOT installed/tested. Product source remains safe local1.13.3 (pure program metadata reads), NOT a confirmed native-list fix. No release/site publication. See design/device-1.13.3/native-preset-fix/reconnect-investigation.json. Crate Press0.2.4 and Groove Dice1.5.1 native preset list/actual selection were verified working; no source fix needed for their native picker.


## Latest local candidate 1.13.14 — native PRESET fixed (2026-11-19)

This supersedes the unresolved1.13.3 investigation and all temporary1.13.4–1.13.13 probes. Public stable remains1.12.0; no Release/website publication was performed in this task. The installed internal-NVMe candidate retains the approved54shared Dice-value locks/353parameters,66programs and original UI/Q-Link maps.

The exact repeating-notification defect was in `setFromEngine`: Transpose Interval219 requested plain0, but SDK round-trip returned approximately1.49012e-7. Comparing only plain values emitted an engine update on every block;89idle render blocks produced89Automate and89UpdateDisplay calls. Those updates drained during host UI queries and prevented the native PRESET list from remaining visible. Compare both semantic plain equivalence and the stored normalized representation; do not quantize fractional normalized legacy-choice echoes. An additional query-side-effect fix makes effGetProgram pure and effGetChunk a callback-free snapshot. Serialize a sanitized copy of transient gates/commands; never release live touches or consume commands while saving state.

Native/ARM32/ARM64 and SDK validation pass. Full engine/voicing/bridge/skin+UBSan passes; SDKhost19PASS/0WARN/0FAIL. All66programs now emit zero notifications during89settled unchanged render blocks each. Regression tests also cover exact legacy normalized echoes, live-state-preserving snapshots and transient-free restoration.

Actual MPC XL/3.10.1.12 verification: native list from fresh Init, Bass/Pocket Anchor with changed values, indices64/65Dusty Sunday/Velvet Melt, Dice→native list→different factory selection, MAIN/CHORD transition. Official SDK export matches all177runtimefiles on internal `/nvme`, with no extra diagnostic files. The native host treats its currently checked Random Sound row as the current custom state; tapping that checked row does not send a fresh program selection. Do not claim native same-checked-row factory reload. Local explicit factory selection/dispatcher same-program reload remains tested.

Groove Dice1.5.1 and Crate Press0.2.4 native lists/selection were verified working; neither needed a source change. Physical encoders/multitouch, new hardware audio/worst CPU and full MPC project save/reopen are not newly certified. Evidence: `mpc-software/plugins/public/DonutTheory/design/device-1.13.14/native-preset-fix/verification.json`.


2026-11-19: User authorized publishing all current fixes and updating the website: Donut Theory1.13.14 and Groove Dice1.5.1; Crate Press0.2.4 is already current and unchanged. Prior candidate-only statements are historical. User separately requested installing all three on an SD card for another device; SDK currently sees only internal nvme and removable SSD, so card identity/insertion is pending. Do not silently install to SSD or remove the internal copies until the card destination is confirmed and verified. Personal expansions must remain excluded from public packages.

2026-11-19 publication complete: Donut Theory1.13.14 and Groove Dice1.5.1 are official stable Releases from sourced23da9a. ZIP/checksum digests verified anonymously; public Pages run44419559175 succeeded;77responsive route-width checks and current actual MPC screenshots published. Crate Press0.2.4 remains the unchanged public preview. See each device-version/publication.json. Earlier candidate/public1.12.0/1.4.0 statements are historical. SD migration remains pending: the user requested a portable SD install, but SDK drive discovery twice showed only internal nvme and removable SSD. Do not claim card installation or choose SSD without identification. Preserve owner expansions and existing internal copies until the SD destination is confirmed and verified.

2026-11-19 design discussion only: approved front C uses an LFO1/LFO2 editing selector and four Q-Links Rate/Depth/Delay/Target, with Shape/Sync touch choices. Selection never changes either LFO's stored values or enabled state. Independent proposed Delay values default0ms and stay outside Dice. All sound values/locks are shared across POLY/MONO views; tab navigation never switches voice mode or sound. Only a dedicated POLY/MONO button changes voice mode; Dice preserves that mode. The old independent per-tab sound-memory idea is rejected. No implementation/build/device update has occurred for this proposal. See design.md section “2026-11-19 proposed performance UI — accepted editing semantics”.

## Latest approved local1.14.0 implementation

This supersedes historical MAIN/PLAY maps and proposal-only status above. Public/installed1.13.14 remains unchanged. Native keys POLY/MONO/OSC/ENV/MOD/FX; CHORD is POLY subtab2. Navigation only edits a shared sound; explicit VOICE alone changes actual mode. Dice/factory preserve mode. Both fronts A=selected OSC Wave/Pitch(Tune/Fine)/Level/Drift, B=Cutoff/Resonance/Body/Drive, C=selected LFO Rate/Depth/Delay/Target. D differs by performance view; see latest handoff/spec for exact maps and source-generated offline previews.

Preserve first353declarations and66programs;388total. Independent OSC tune/fine/level/PW migrate legacy aliases and lock preferences; guarded state replay must not overwrite new values. Independent LFO Delays(default0,0..5s) gate per-voice note age, never reset module phase on focus change and remain outside Dice.61value locks, selector-header locks, shared rate ownership, one-step transient Undo, mode/chord/master exclusions and unlock-only semantics are regression contracts. No empty literal Text widgets. Keep modal roots last and CHORD expanded graph below2500. SDK/native/ARM/full+UBSan checks pass locally; no1.14.0device, Release/site or other-product update. Do not describe offline code renders as MPC captures. Read latest handoff/spec/design and design/implementation-1.14.0 before further work.

## Local1.14.1 Sync contract

Retain the fractional normalized rate cursor while its15-choice decoded index agrees with the canonical Sync division; never snap it back every render block. Explicit source/mode changes rebind; selection changes are edit-only. Full Sync divisions388..390append after the exact388fixture;391total. Old5-choice IDs/ranges and straight periods stay unchanged; D/T choices share existing rate locks and exact-normalized Undo. Clockwise is faster. No new front controls/Q-Links. See handoff/manual and sync_rate_test.cpp/sync_rate_bridge_test.cpp. Local build/testing only; installed/public1.13.14unchanged.

## 2026-11-19 — 1.14.1 actual MPC release preparation

The user has explicitly authorized real-device inspection and release of1.14.1. The earlier local-only/no-device restrictions describe prior work. Native MPC QA found that performance Value labels with `Data -> Data` self-remaps render blank; inherited value labels must omit this remapping. Keep the no-self-alias regression in skin_test.py. The corrected native/ARM/SDK build,15-step Sync cursor/phase/state/lock tests and all-page actual captures are recorded in design/device-1.14.1. Device UI checks do not imply new physical-encoder/hardware-audio/CPU/full-project certification. Preserve the shared-view/explicit-VOICE semantics and current Groove Dice1.6.0/Crate Press0.2.4.


2026-11-19 Donut Theory1.14.1 publication complete: source0937a4f, stable tag donut-theory/v1.14.1 and anonymous ZIP/checksum digests verified. Pages run44456002969 succeeded;77responsive checks and17public route/image/download checks passed. Current actual MPC screenshots and detailed manual are live. Official SDK read-back matched255runtimefiles on each NVMe and SD36C7-1ACA; no SD-specific runtime-origin certification. Remote real-device UI checks are separate from physical encoders, hardware audio, worst CPU and full project recall, which were not newly certified. Groove Dice1.6.0 and Crate Press0.2.4 unchanged. Evidence: mpc-software/plugins/public/DonutTheory/design/device-1.14.1/publication.json and public-verification.json.

## 2026-11-19 — local 1.14.2 selector alignment

User approved a UI-only refinement: front selectors use 146px or 296px column widths and a common26px height; footer VOICE/Dice Target/Input use matching184px-wide44px-high fields. FREE/SYNC is directly above its Rate on fronts and MOD. LFO focus and shape remain available in the same row; Motion sync no longer occupies its section title. Compact list text has8px leading space and a reserved arrow gutter; compact Drop TIME reserves40px for caption and106px for values/units. Original artwork/fonts, knob geometry, all391parameters and every Q-Link map are unchanged. No DSP change besides the candidate version11402. User authorized official1.14.2 release and website publication; completion is recorded in design/device-1.14.2/publication.json. Native/ARM/SDK, skin and host checks and actual MPC evidence are under DonutTheory/design/device-1.14.2/.

2026-11-19 Donut Theory1.14.2 publication complete: source4a45d88 and stable tag donut-theory/v1.14.2; anonymous ZIP/checksum digests verified. Pages44461789094 succeeded;77responsive checks and15public route/image/download checks passed. Seven actual MPC screenshots and the updated manual are live. NVMe/SD each have247verified runtimefiles from the preceding UI QA. Same391parameters/Q-Link maps and sound engine; Groove Dice1.6.0 and Crate Press0.2.4 unchanged. Earlier local-only status is historical. Limits and audit: DonutTheory/design/device-1.14.2/publication.json and public-verification.json.


## 2026-11-20 — approved1.15.0 Phase/Wave expansion

User explicitly approved implementation, complete verification, device update and official release/site publication alongside Groove Dice1.8.0. Preserve original artwork and all other mappings. Both fronts A=selected OSC Full Wave / Tune-Fine / Level / selected Drift-Phase; A4focus is edit-only and does not change either stored value. OSC1is reference; OSC2relative phase0–360degrees. FREE/RESET is an explicit touch preference beside the Phase value, defaultFREE. RESET aligns new articulations while connected legato/chord glide retains phase. Strum reset occurs at each actual delayed attack.

First391declarations remain exact;399total. Original4wave paths remain; append Soft Saw/Soft Pulse/Organ/Hollow with band-limited construction and live wave/phase crossfades. Old normalized wave ranges remain aliases; new canonical eight-wave fields and relay append. Legacy391states/presets seed classic waves and default82.8degree offset, retaining previous free-running phase behavior. Full waves share the original OSC Pair Dice lock; Phase owns one appended lock. RANDOM covers8waves/phase but preserves restart preference, editing focus, voice/chord/master settings. Undo restores full canonical values without replaying editor relays.

Implementation/device/release verification is in progress. Installed/public1.14.2is not yet updated. NVMe only; SD/SSD untouched. User private Groove Dice packs must be excluded from public packages/screenshots.


Current1.15.0 release qualification is in design/device-1.15.0; user explicitly authorized stable publication/site update. Preserve the first391parameter definitions and default66factory PCM. Actual phase and wave transitions/locks are separate from editing focus. No physical/hardware-audio/CPU blanket certification.
