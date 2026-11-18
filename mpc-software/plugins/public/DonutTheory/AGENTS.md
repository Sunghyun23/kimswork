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
