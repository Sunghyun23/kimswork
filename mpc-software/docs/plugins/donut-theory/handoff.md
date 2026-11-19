## 2026-11-19 — 1.14.1 device QA and release authorized

The user explicitly requested MPC inspection and publication of the current1.14.1implementation. Earlier local-only/no-device/no-release statements below are historical. Current public outcomes are recorded in `plugins/public/DonutTheory/design/device-1.14.1/publication.json` after completion. Groove Dice1.6.0 is already released separately; the earlier no-Triplet drum audit predates that release.

# Donut Theory — 프로젝트 인계 / 재개 기준

## Local1.14.1 — Sync encoder fix and dotted/triplet rates (2026-11-19)

Supersedes the five-step Sync behavior in the local1.14.0candidate. Public/installed version remains1.13.14; no device, Release or website update is part of this task.

Confirmed native reproduction:200relative+0.01inputs to the old Sync rate relay returned to cursor0/first division every block. The render-side readback had snapped the continuous Q-Link cursor to the center of its quantized choice, discarding all sub-step movement. The fix retains fractional cursor movement while it denotes the current choice; explicit division edits or a Sync-mode change rebind it to the appropriate source. Selected-LFO front relays share that cursor. Clockwise now increases frequency in both Hz and Sync; free Hz is retained when Sync is toggled.

LFO1, LFO2 and shared Tremolo/Auto Pan each provide15beat periods:1/16,1/8,1/4,1/2,1bar with straight/Dotted/Triplet variants. The single Rate knob traverses them from slow to fast; no extra Q-Link or front button is added. Compact LCD labels use D/T; the sidebar spells out Dotted/Triplet and pages8+7rows. Dotted is3/2and Triplet is2/3of the straight period.1bar retains the original four-quarter-note period.

First388parameter declarations stay exact. Append Lfo1SyncDivision388,MotionSyncDivision389,Lfo2SyncDivision390(391total). Existing five-choice Division/ModDivision/Lfo2Division IDs/ranges remain forward aliases; old states seed the matching straight periods, while new dotted/triplet states restore independently. The new choices share existing Rate/Sync Dice locks. Factory recipes retain their original periods; Dice can choose all15, and exact-normalized Undo restores them. Voice mode, harmony, MIDI, Delay and artwork/layout policies remain as1.14.0.

Validation includes slow relative0.001turns both directions for all3modules and selected-LFO control, all15periods at60/88/120/200BPM, PPQ phase checks, free-rate memory, state migration, locks/Undo and VST callback stability. Physical MPC encoders/rendering/audio/CPU are not certified by local tests. See design/implementation-1.14.1 for final logs/status. Groove Dice was separately audited; it does not use the same LFO Sync relay. Its existing MAIN rhythm selectors, dotted hat grids and free-Hz FX ModRate passed the targeted native/UBSan/VST audit without engine or parameter changes. Its current hat list has no Triplet option; this audit did not add one.


## Local implementation 1.14.0 — shared performance editors (2026-11-19)

User approved implementation of the seven tab mockups. Public stable and installed copies remain **1.13.14**; this is a **local candidate**, not a published release or hardware-certified update. Groove Dice, Crate Press, website and MPC drive files are unchanged.

- Six function keys: **POLY / MONO / OSC / ENV / MOD / FX**. CHORD remains the POLY key's second native subtab. Initial view is POLY, with no startup message. A native tab switch has no DSP write actions.
- All sound values, locks, chord/key/MIDI settings are shared. POLY/MONO tabs select an editor only. **VOICE POLY/MONO** explicitly switches the actual voice mode; Mono's LEGATO preference is separate and retained. Factory selection and Dice preserve actual voice mode. This supersedes historical factory/Dice mode changes.
- Both fronts: A Wave / Pitch / Level / Drift (OSC1/OSC2 edit selector, Tune/Fine resolution selector); B Filter Cutoff / Resonance / Body / Drive; C Rate / Depth / Delay / Target (LFO1/LFO2 edit selector, local Shape/Sync). POLY D Attack / Release / Motion Rate / Motion Depth. MONO D Glide / Decay / Release / Pitch Drop, with local Legato/Glide Mode and compact Drop Time.
- OSC detail shows Wave/Tune/Fine/Level per oscillator, Sub/Drift/PW1/PW2 and Noise/FM/Tone. ENV combines both native ADSR graphs/faders and Filter Response. MOD exposes both independent LFOs and shared Tremolo/Auto Pan. FX keeps eleven populated Q-Links; no unused fourth output knob.
- LFO Delay is per-voice elapsed note-on time, **0–5000ms**, default0; a10ms gain fade follows the wait. Zero is a bypass. Strum-delayed gates start their wait when sounding; held connected legato does not restart it. LFO phases and the other module continue when edit focus changes. Both modules offer five waveforms and five targets through append-only full selectors; legacy four-choice LFO1 ranges remain intact.
- Tune/Fine values are retained separately and summed. Oscillator levels and pulse widths are independent. Existing Mix/Detune/Octave/Pulse automation remains a forward alias; new states preserve canonical values through the guarded legacy replay window. Old353states migrate their ratio/pitch/pulse and relevant locks. Sub follows OSC1 one octave below; Drift remains shared.
- Dice Target Auto follows actual voice mode, never the editor tab (Mono/Legato: Bass or Lead; Poly: Keys/Synth/Lead/Pad). Explicit Bass/Keys/Synth/Lead/Pad targets generate that family without changing mode. Dice preserves chords, MIDI, key, master, Pitch Drop, both LFO Delays and editing focus; three Sync values are forced ON when their rate lock allows it. New pitch/level/PW locks protect against legacy-alias writes too. One-step **UNDO** restores the preceding sound, not lock preferences or performance settings; undo history is not saved. UNLOCK ALL changes locks only. Choice locks remain in selector-sidebar headers; Rate/Sync/Division share ownership.
- Factory programs remain66 and harmony remains32types/96progressions/16slots. The first353parameter declarations and IDs are exact; append35for **388total**. Seven new canonical value locks extend54to61; legacy section locks/retired aliases are retained for compatibility. Nonrandomized Delay/mode/chord/master controls have no misleading Dice lock.

Validation: native/ARM32/ARM64 and SDK validation; full engine/voicing/analog/bridge+UBSan; SDKhost19PASS/0WARN/0FAIL; new selection routing, mode/state migration, both300ms audio delay gates, independent440/110Hz OSC octaves and452.9Hz fine tuning, lock/Undo checks.64focus/resolution changes produce stereo PCM identical to an untouched reference. Three identical-state native renders match1.13.14 PCM exactly; that is not a blanket sound-equivalence claim for every possible state. CHORD construction stays under2500widgets. All seven actual generated skins are checked for bounds/maps/modal order/assets/blank Text widgets. Offline previews are approximate code renders, **not MPC screenshots**.

Reproduce: `python3 mpc-software/scripts/build_plugin.py DonutTheory --target all`, then `python3 mpc-software/scripts/test_plugin.py`. A fresh MPC instance/reset Q-Link defaults may be needed for cached maps. Actual MPC rendering, physical Q-Links, multitouch, hardware audio/CPU and full project save/reopen are **not verified for1.14.0**. Do not publish or silently replace installed/released versions on the basis of these local checks.


## Current release 1.13.14 — native PRESET fixed (2026-11-19)

This supersedes the unresolved1.13.3 investigation and all temporary1.13.4–1.13.13 probes. Public stable is1.13.14; official Release and Pages publication completed on2026-11-19. The installed internal-NVMe candidate retains the approved54shared Dice-value locks/353parameters,66programs and original UI/Q-Link maps.

The exact repeating-notification defect was in `setFromEngine`: Transpose Interval219 requested plain0, but SDK round-trip returned approximately1.49012e-7. Comparing only plain values emitted an engine update on every block;89idle render blocks produced89Automate and89UpdateDisplay calls. Those updates drained during host UI queries and prevented the native PRESET list from remaining visible. Compare both semantic plain equivalence and the stored normalized representation; do not quantize fractional normalized legacy-choice echoes. An additional query-side-effect fix makes effGetProgram pure and effGetChunk a callback-free snapshot. Serialize a sanitized copy of transient gates/commands; never release live touches or consume commands while saving state.

Native/ARM32/ARM64 and SDK validation pass. Full engine/voicing/bridge/skin+UBSan passes; SDKhost19PASS/0WARN/0FAIL. All66programs now emit zero notifications during89settled unchanged render blocks each. Regression tests also cover exact legacy normalized echoes, live-state-preserving snapshots and transient-free restoration.

Actual MPC XL/3.10.1.12 verification: native list from fresh Init, Bass/Pocket Anchor with changed values, indices64/65Dusty Sunday/Velvet Melt, Dice→native list→different factory selection, MAIN/CHORD transition. Official SDK export matches all177runtimefiles on internal `/nvme`, with no extra diagnostic files. The native host treats its currently checked Random Sound row as the current custom state; tapping that checked row does not send a fresh program selection. Do not claim native same-checked-row factory reload. Local explicit factory selection/dispatcher same-program reload remains tested.

Groove Dice1.5.1 and Crate Press0.2.4 native lists/selection were verified working; neither needed a source change. Physical encoders/multitouch, new hardware audio/worst CPU and full MPC project save/reopen are not newly certified. Evidence: `mpc-software/plugins/public/DonutTheory/design/device-1.13.14/native-preset-fix/verification.json`.

## Historical 1.13.0 section-lock candidate (superseded)

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

## Latest official status — 1.12.0 published (2026-11-18)

**Stable Donut Theory1.12.0** is public: [Release](https://github.com/Sunghyun23/kimswork/releases/tag/donut-theory/v1.12.0), [product](https://kimswork.app/plugins/donut-theory/), [manual](https://kimswork.app/plugins/donut-theory/manual/). Source949dcfbd0dfe7856ab3451908f5ceb289cf43daa; Release449932324; Pages44301107512success. ZIP5,982,630bytes / SHA256`5dfbdf164e16dcb13f877af2a354c702e1a7f10ebe180c7666ae006b48f21e1f`.

Current tabs MAIN/CHORD,OSC,ENV,PLAY,MOD,FX. ENV combines both ADSRs; PLAY D adds shared Amp Release/Drop Amount/Drop Time. MAIN filters remain intact. UID kim/Krql,293parameters with first291exact,66programs. InternalNVMe173files match the release. Seven latest MPC screenshots are published. Engine/UBSan/bridge/analog/harmony/skin and19host checks pass;21public and77responsive checks pass. Physical encoder/multitouch/hardware audio/CPU/full-project certification remains limited. Groove Dice and Crate Press are unchanged. `design/device-1.12.0/{verification,publication,public-verification}.json` hold evidence. Candidate-only and older release-state paragraphs below are history.


## 1.12.0 local candidate — Release / Pitch Drop

Latest user-approved PLAY D adds **Amp Release / Drop Amount / Drop Time / unassigned**. Amp Release is the same parameter30 used on ENV; editing either view updates the other. PLAY A/B/C keep their previous controls, arranged with D in a2×2 layout. MAIN and its filter group remain unchanged; ENV still holds both ADSRs.

Drop Amount0–24st (0 displays Off); Drop Time20–2000ms, default250ms. Note-on stays at its normal pitch. At actual gate-off the current unbent glide pitch is frozen and a per-voice downward semitone ramp begins; it reaches the chosen interval in Drop Time if Amp Release leaves enough audible tail. A short Release may end the sound first. Sustain delays gate-off; a legato fallback to an older held key does not drop. Chord voices use equal intervals, pending strum notes are cancelled, new/reused voices clear the old ramp, panic/reset removes it. Existing normal Glide and pitch bend remain independent. Amount/time are snapshotted at release, so changes affect subsequent releases. Drop settings are stored with the project and preserved across factory sound changes and ROLL DICE.

UID kim/Krql and first291parameter declarations are exact; append ReleaseDropAmount291 and ReleaseDropTime292 for293total. Old291states migrate to Off/250ms. Existing66factory programs and other projects remain unchanged. Source/native/ARM32/ARM64/SDK, full engine and UBSan/bridge/skin checks pass; direct device evidence is under `design/device-1.12.0/`. Physical encoder/audio/CPU certification is separate. This candidate supersedes1.11.0's empty PLAY D and unchanged-DSP statements; public stable remains1.10.1 until a separately authorized release.


## 1.11.0 local ENV / PLAY candidate — 2026-11-18

User-approved detail reorganization supersedes the older FILTER/AMP split below. Native tabs are MAIN / OSC / ENV / PLAY / MOD / FX, with the existing MAIN↔CHORD subtab. Original paper/chocolate artwork, Roboto Bold lettering and donut knobs are retained. Public stable remains **1.10.1**; this change is not a new public release.

| Page / Screen Q-Link bank | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| ENV A · Amp envelope | Attack | Decay | Sustain | Release |
| ENV B · Filter envelope | Attack | Decay | Sustain | Release |
| ENV C · Filter response | Cutoff | Resonance | Env Amount | Key Track |
| PLAY A · Performance | Volume | Expression | Glide | Bend Range |
| PLAY B · Chord play | Trigger Base | Chord Spread | Chord Slot | Unassigned |
| PLAY C · Voice / input | Voice Mode | Glide Mode | Input Mode | Unassigned |

Both pages leave bank D unassigned and draw only three Q-Link guides. Filter Type is a touch list on ENV. Each envelope has its own native editable ADSR diagram and four vertical faders; both edit the same existing parameters. These diagrams show parameter relationships, not a sample-accurate live waveform or a calibrated millisecond axis. Trigger Base affects consecutive pad mapping; individual MIDI/LEARN remains on CHORD. Body/Sub remain on MAIN/OSC and are not removed.

All 291 parameter declarations, 66 sounds, UID, DSP, MAIN/CHORD/OSC/MOD/FX trees, Program defaults and all non-ENV/PLAY Screen mappings remain unchanged. Existing projects may retain cached old Screen assignments; use a fresh instance or reset the host map to defaults when comparing. Local build/native/ARM/SDK and host simulator pass. Final device evidence is recorded in `design/device-1.11.0/`; physical encoder/audio/CPU coverage must not be inferred from remote testing.


기준일: **2026-11-16 (Asia/Seoul)**. 이번 공개 배포 대상은 **1.2.3 BODY→SUB**, MPC XL 내장 NVMe 설치 완료다. 다음 프로젝트 때문에 Donut Theory를 새 저장소로 분리하거나 루트를 단일 플러그인 프로젝트로 바꾸지 않는다.

## 읽는 순서와 문서의 권위

1. 루트 [AGENTS.md](../../../../AGENTS.md)와 [저장소 정책](../../../../docs/repository-policy.md).
2. 이 문서: 최신 확정 사항, 배포 상태, 검증 범위와 재개 지점.
3. [제품 사양](spec.md), [디자인](design.md), [엔진](engine.md), [공장 음색](presets.md).
4. [사용자 매뉴얼](manual.md), [1.2.2 릴리스](releases/1.2.2.md), [구현·검증 이력](implementation.md).
5. [플러그인 개발 README](../../../plugins/public/DonutTheory/README.md)와 작업 종류에 맞는 고정 SDK 스킬.

spec/design/implementation/README의 버전별 기록은 당시 상태다. 과거의 ‘미구현·재시작 보류·배포 전’ 문구를 현재 상태로 해석하지 않는다. 1.2.2 이후 최신 사용자 확인이 오래된 시안·설명보다 우선한다. 새로운 요구가 생기면 관련 사양과 검증 상태를 함께 갱신한다.

## 현재 배포와 저장소

| 항목 | 확정값 |
|---|---|
| 이름 / vendor / UID | Donut Theory / `kim` / `Krql` |
| 플러그인 경로 | `mpc-software/plugins/public/DonutTheory/` |
| 개발 문서 | `mpc-software/docs/plugins/donut-theory/` |
| 제품 브랜드 / 사이트 | Kim’s Work / [kimswork.app](https://kimswork.app/) |
| 현재 정식 버전 | **1.2.2** |
| 태그 / 태그의 소스 커밋 | `donut-theory/v1.2.2` / `10ace34e7e6698421feb5bcb7ee3332771d5594f` |
| 이후 사용자 확인 기록 커밋 | `1696b860f1a4aa31fc58880fb1bf584adafd73f2` |
| 공개 Release | [Donut Theory 1.2.2](https://github.com/Sunghyun23/kimswork/releases/tag/donut-theory/v1.2.2) |
| ZIP / 크기 | `donut-theory-1.2.2-mpc.zip` / **3,592,700 bytes** |
| ZIP SHA256 | `f27d2ee20f18ec693f2ba82e1314061f8c3cf7d7cd8f821eb707b884f3a78bb9` |
| 제품 / 매뉴얼 | [제품](https://kimswork.app/plugins/donut-theory/) / [매뉴얼](https://kimswork.app/plugins/donut-theory/manual/) |
| 공유 SDK | `inmusic-software/mpc-plugin-builder-sdk` @ `febd2ca5bd9eb16c4a412837196e145566bc981b` |
| 설치 폴더 | `VST2 Plugins/kim - VST - Donut Theory/` |
| 조건 / 실제 확인 기기 | standalone OS 3.10.1+, Enhancements와 External Plugin Support / MPC Live III OS **3.10.1.12** |

Release ZIP과 SHA256은 공개 확인했다. 제품·매뉴얼·실제 스크린샷 3개는 HTTP 200 및 버전/이미지 일치를 확인했다. Pages 배포 `43682503695`(1.2.2), `43697399918`(사용자 확인 문서 후속)은 success였다. 사용자 확인 후 온라인 릴리스 노트/매뉴얼만 보완했으며 이미 공개한 ZIP·태그를 재작성하지 않았다. 따라서 ZIP에 동봉된 문서에는 그 후속 문장이 없을 수 있다.

## 제품·음색의 확정 방향

- J Dilla를 연상시키는 소울·힙합 작업실 정서. 빈티지 아날로그 **리드·패드 중심**이면서 Bass/Keys/Synth도 제공한다. 공식 아티스트 제품이나 특정 음반 소리 복각을 표방하지 않는다.
- **프리셋만 Moog풍으로 조정하는 작업은 최종 목표가 아니다.** 1.2.0부터 기본 음원 엔진 자체가 Moog-inspired다. 새 모드/엔진 선택기/UI 테마는 추가하지 않았다.
- 기존 SDK의 band-limited 파형 → 비선형 OSC 믹서 → 4단 비선형 래더 Low-pass → 둥글게 포화되는 VCA. 래더 내부 2x 처리, 17-tap FIR, 동시 Newton 해석. 기존 ADSR·모듈레이션·FX와 결합한다.
- BP/HP/Notch는 기존 12dB SVF이며 래더의 변형 모드가 아니다. 래더는 특정 Moog 모델의 부품 단위 정밀 복각이 아니며 자발적 self-oscillation을 제공하지 않는다. 자세한 컷오프/레벨 응답과 출처는 engine.md.
- **64음색:** Init 1 / Bass 12 / Keys 15 / Synth 12 / Lead 12 / Pad 12. 63개 연주 레시피를 새 엔진에 맞춰 재작성·출력 보정했다. 이름·순서·카테고리는 유지한다.
- ‘mix-ready lowfilter’의 사용자 뜻은 **Low-cut(불필요한 저역 정리)** 이다. 음색용 Low-pass와 혼동하지 않는다. Init은 Low-cut/FX Off이나 새 엔진 고유 질감은 적용된다.
- UID/208개 파라미터 호환은 구버전과 같은 소리라는 뜻이 아니다. 1.1.2 이전 엔진의 정확한 소리가 필요하면 구버전 보관·오디오 렌더가 필요하다. 1.2.1/1.2.2에서 엔진·음색 레시피는 바꾸지 않았다.

## MAIN / Q-Link / 디자인

| 위치·그룹 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| 좌상 A Sound / Play | OSC Pair | Osc Mix | Body | Drift |
| 우상 B Amp Envelope | Attack | Decay | Sustain | Release |
| 좌하 C Filter / LFO | Filter Cutoff | Resonance | LFO Rate | LFO Depth |
| 우하 D FX / Tremolo | Drive | Glide | Tremolo Rate | Tremolo Depth |

- 총 4×4=16개. 화면 읽기 순서와 Screen/Program 매핑을 맞춘다. 실제 하드웨어 번호는 열 기준 A=13/9/5/1, B=14/10/6/2, C=15/11/7/3, D=16/12/8/4다.
- OSC Pair 바로 뒤가 Osc Mix. Body를 삭제하지 않는다. Drift는 느린 wow/빠른 flutter, Detune은 고정 음정차로 별개다. Space는 FX 상세, Glide는 MAIN D와 AMP 상세에 남는다.
- Filter 종류·LFO Shape/Target은 C의 터치 선택. Sync/Division은 MOD. 중복 LFO SETTINGS 팝업을 재도입하지 않는다. 독립 Tremolo는 자체 Rate/Depth를 가진다.
- MAIN/OSC/FILTER/AMP/MOD/FX의 6개 함수 키, 첫 키에만 MAIN↔CHORD 두 서브탭. 새 인스턴스는 MAIN. MAIN을 다시 누르면 CHORD, 다시 누르면 MAIN이다. 슬롯마다 서브탭을 만들지 않는다. 안내 팝업으로 이동 기능을 대체하지 않는다.
- 갈색 질감 헤더, 크림 본체, 올리브 LCD, 도넛 노브, 큰 가로형 도넛·레코드·슬리브 장식이 현재 방향이다. 헤더와 그림의 배경 접합을 보존한다. CSS 원형 도넛으로 대체하지 않는다.
- 원래 승인 콘셉트 [main-approved.png](design/main-approved.png)는 미술 방향의 이력이다. 현재 배치 기준은 **실제 1.2.2 화면**과 최신 매핑이다. B Drive / Play 등 초기 배치를 되살리지 않는다.
- 실제 화면: [MAIN](../../../../site/public/images/donut-theory-main-1.2.2.webp), [CHORD](../../../../site/public/images/donut-theory-chords-1.2.2.webp), [카테고리 목록](../../../../site/public/images/donut-theory-browser-1.2.2.webp). 오프라인 근사 렌더를 실기기 캡처로 소개하지 않는다.

## 코드 연주·편집 계약

- 기본 4, 최대 16슬롯. CHORD의 +/−로 4개씩 활성화하고 화면 안에서 1–4/5–8/9–12/13–16을 선택한다. 축소해도 비활성 슬롯 설정은 보존한다.
- MAIN은 **222×70 패드 4개**와 페이지 화살표를 유지한다. 코드 프리셋 화살표 44×44, 페이지 화살표 표시 44×32/터치 44×44. HOLD TO PLAY 문구·슬롯별 EDIT·불필요한 전면 STOP은 없다. 비상 정지는 CHORD ADVANCED의 ALL NOTES OFF.
- 화면 코드는 **누르는 동안 발음**, 떼면 릴리스. 코드 캡션 위의 투명 연속 Slider ResetOnGestureEnd가 release를 담당하고 Mouse Down raw Toggle Switch가 ON을 담당한다. 일반 Button+ResetOnGestureEnd로 바꾸면 실제 기기에서 ON/OFF가 한 블록에 끝났던 문제가 재발한다.
- 실제 패드 MIDI는 화면 페이지와 독립적이다. **MPC Pad Perform 설정 변경을 요구하지 않는다.** 각 슬롯 MIDI Note 수동 지정/LEARN으로 맞춘다. LEARN 첫 입력은 무음 등록, 다음 입력부터 코드 연주다.
- **수동 PAD LOCATION 표시 기능은 1.2.2에서 제거했다.** A–H 위치 선택/작은 4×4/MAIN 위치명은 없다. 실제 MIDI/LEARN 기능은 삭제하지 않았다. 저장 호환용 PadPosition1–16(192–207)은 선언·값을 보존하되 UI에서 사용하지 않는다.
- CHORD Screen Q-Link A–D=보이는 네 슬롯, 각 Root/Type/Voicing/Octave. MIDI Note는 Q-Link 범위에서 제외. 16개 고정 편집 중계 파라미터가 실제 선택 뱅크를 읽고 쓴다. Program/Track 모드는 신스 매핑을 유지한다.
- Voicing 저장값 0=Root/1=First/2=Open 유지, 추가 전위는 3/4/5. 실제 구성음 수에 맞는 전위를 제공한다. Octave −2..+2, 저장 인덱스2=0oct.
- 코드 프리셋은 현재 보이는 네 슬롯의 Root/Type만 바꾸고 다른 슬롯·보이싱·옥타브·MIDI 매핑을 보존한다. 네 뱅크 선택은 독립 저장. MOD PRESET 1–4는 첫 뱅크 전용. 32개의 4코드 레시피이며 32개의 16코드 곡이 아니다.
- Mono/Legato는 NOTES/SPLIT의 멜로디에 적용하고 코드는 별도 다성 소유자다. 코드가 전용 mono voice0을 빼앗지 않게 한다. 화면 코드는 내부 오디오 발음이며 MPC 시퀀서로 MIDI를 보내지 않는다.

## 해결한 문제와 재발 방지

| 문제 | 현재 처리 / 주의 |
|---|---|
| 프리셋·버튼 캡션 중앙 터치 무반응 | 실제 기기에서 Label이 터치를 가렸다. 투명 Button hit를 캡션 위에 둔다. 이미지 자동 축소를 가정하지 않는다. |
| 선택 시 페이지 전체가 바뀜 | 현재 페이지를 유지하는 426px 왼쪽 사이드바. 선택 후/×/바깥 터치 닫힘. |
| 상단 MPC 회색 PRESET의 평면 목록 | 호스트 소유. 플러그인 SOUND PRESET에서 category→sound 제공. 정식 플러그인도 분류 불가능하다고 일반화하지 않는다. |
| 모달 위로 코드 화살표가 보임 | 모든 탭의 일반 형제 다음에 modal 루트를 정렬. version4/hideQLinkBounds로 가이드 숨김/복원. |
| CHORD 진입 멈춤·불안정 위치 표시 보고 | 위치 UI를 제거해 CHORD 펼친 위젯 10,889→1,913개, TUI 19,568,032→13,542,804bytes. 재귀 cycle 검사·위젯 2,500 이하 제한. 원인 인과관계를 통제 A/B로 입증한 것은 아니다. |
| 초기 원격 클릭 미응답 | normal-window 좌표 오류와 프레임 지연이 섞였다. fullscreen에서 정확한 기기 영역으로 검증. 잘못 전달된 클릭을 기기 멈춤 재현으로 단정하지 않는다. |
| 중계 값이 저장 코드를 덮어씀 | stateLoading 보호, 오디오 readback의 thread token+index 식별을 보존한다. |
| LEARN 이후 화면 값 갱신 지연 | 오디오 스레드에서 host 알림 금지. non-render value read/idle에서 큐를 배출하는 방어를 유지한다. |

## 검증 결과 — 완료와 미확인을 구분

- **자동 검사:** native/ARM32/ARM64 빌드, SDK validator 0 errors/0 warnings, host simulator 19개, engine/UBSan/bridge/analog/skin 통과. MIDI offset·가변 블록·할당 금지·노트 소유권·state migration 포함. 이후 추가한 CHORD 위젯 budget 검사도 통과했다.
- **래더/오프라인:** 네 rate(32/44.1/48/96k), 빠른 cutoff/res stress solver residual<1e−4. 64개 finite/non-silent, 테스트 피크 0.177732..0.288887FS. 이는 모든 연주에서의 음량·헤드룸 보장이 아니다.
- **실기기 엔진:** 1.2.0의 64개 프로그램 순서·발음 확인. 1.6초 Dm9 화면 hold 각 gate non-silent, 전체 녹화 peak0.683899FS, 마지막1초0.0. 카테고리별 적절한 음역을 달리한 음악적 전수 청음 승인과는 다르다.
- **터치 gate:** 네 MAIN 패드 각 약1.8초 hold·release, 오디오4영역/마지막 무음. 과거 진단에서는 사용자 손가락 hold도 정상 확인했지만 1.2.2 전체 다중 터치 재인증은 아니다.
- **실기기 부하:** Release5000ms 코드 중첩에서 CPU41%, 해제 후12% 관찰. 정확한 16held voice/최악 부하/모든 FX 조합 인증은 아니다.
- **저장 복원:** native `.xpl` preset의 Notch /1.34kHz/46.4%/Release5000을 1.2.0 저장→1.2.1 재시작 복원 확인. MPC 전체 프로젝트 round-trip과 구분한다.
- **1.2.2:** 새 MAIN 진입, 원격 반복 MAIN↔CHORD, inversion 선택, 4→8→12→16과 bank13–16 확인. **사용자가 기기 화면의 첫 탭을 직접 두세 번 눌러 ‘정상 전환됨’ 확인**(2026-11-15). 정상 전환은 확인됐고 원래 멈춤 원인은 미확정이다.
- **사이트:** 영어/다크, PC·일반 모바일·폴드 대응. 이전 검증 폭390/768/1440에서 가로 넘침 없음. 1.2.2 변경은 텍스트·실제 이미지·다운로드 링크이며 사이트 빌드/로컬44참조/공개200 확인.

남은 제한/재검증 후보: Remote Viewer output dropout 원인(전송/Mac재생/DSP) 미분리, 모든 물리 Q-Link 회전·새 물리 LEARN·동시 손가락 multi-touch·전체 MPC 프로젝트 저장/재시작 복원·다른 MPC 모델·최악 CPU. 이 항목을 완료로 보고하지 않는다. 즉시 진행 중인 작업이나 사용자 승인 대기 상태는 없다. 추가 요구가 생길 때 해당 검증을 이어간다.

## 소스·도구 지도와 재현

| 파일 / 도구 | 역할 |
|---|---|
| `mpc-software/plugins/public/DonutTheory/src/DonutTheory.cpp` | 엔진·MIDI/코드 소유권·중계·호스트 알림·VST bridge |
| `mpc-software/plugins/public/DonutTheory/src/AnalogLadder.h` | 원본 MIT 비선형 래더 DSP |
| `mpc-software/plugins/public/DonutTheory/tools/generate_catalog.py` | 파라미터/프리셋 관련 생성. 버전을 임의 덮어쓰지 않는다. |
| `mpc-software/plugins/public/DonutTheory/tools/sound_bank.py`, `calibrate_factory.py` | 레시피와 렌더 기반 출력 보정 |
| `mpc-software/plugins/public/DonutTheory/tools/build_skin.py` | 커스텀 스킨 생성, 마지막 modal 형제 순서 정리 |
| `chord_slots.py`, `chord_qlinks.py`, `sidebars.py` | 단일 CHORD/16슬롯·Q-Link·왼쪽 선택 목록 |
| `pad_positions.py` | **폐기한 표시 UI의 원본**. 현재 build_skin에서 호출하지 않는다. 재활성화하지 않는다. |
| `mpc-software/plugins/public/DonutTheory/tests/` | 엔진/analog/bridge/skin 및 UBSan 회귀 |
| `site/src/data/plugins.ts` | 사이트 버전·명시적 Release/ZIP 링크·이미지·소개 데이터 |
| `site/src/pages/plugins/[slug].astro` | 제품 상세·엔진 A/B·실제 화면 |
| `site/src/pages/plugins/donut-theory/manual.astro` | 저장소 manual.md를 사용하는 웹 매뉴얼 |

저장소 루트에서:

```sh
python3 mpc-software/scripts/bootstrap_sdk.py
python3 mpc-software/scripts/build_plugin.py DonutTheory --target all
python3 mpc-software/scripts/test_plugin.py
python3 mpc-software/third_party/mpc-plugin-builder-sdk/tools/package.py mpc-software/plugins/public/DonutTheory
ASTRO_TELEMETRY_DISABLED=1 npm --prefix site run build
npm --prefix site run check:links
```

현재 테스트 래퍼는 macOS 기준. SDK의 지원 플랫폼과 이 래퍼를 혼동하지 않는다. Zig cache 쓰기 제약이 있으면 작업용 writable cache를 지정할 수 있다. SDK 본체/시스템 파일은 수정하지 않는다. 패키지 생성 후 MANUAL.md/RELEASE_NOTES.md/ENGINE.md를 포함하고 ZIP CRC·중복 이름·manifest버전/UID·양쪽 ARM·런타임 전용 이미지·SHA256을 검증한다. ZIP은 Git에 넣지 않고 plugin 전용 Release asset으로 배포한다. 이미 게시한 1.2.2 ZIP/태그를 바꾸지 말고 다음 코드 변경은 새 patch 버전을 사용한다.

원격 설치는 **공식 `tools/mpc_remote.py`만 사용**한다. 현재 주소/드라이브는 다시 확인하고 임의 SSH/scp·시스템 수정은 하지 않는다. 앱 재시작 전 미저장 작업 보존 여부를 현재 세션에서 확인한다. 이전 QA 사본 불필요·변경 폐기 허용은 과거 테스트의 권한이며, 새 사용자가 작업한 프로젝트를 무조건 버리는 영구 규칙이 아니다. 접속 비밀번호·토큰은 공개 문서에 적지 않는다.

## 증거 보존과 다음 프로젝트

- 실제 공개 스크린샷은 `site/public/images/*1.2.2.webp`에 추적된다. 승인/제작 원본은 `mpc-software/plugins/public/DonutTheory/assets/`와 디자인 문서에 보존한다.
- 상세 MOV/WAV/JSON/빠른 전환 캡처는 이 개발 작업공간의 ignored `mpc-software/plugins/public/DonutTheory/build/qa-1.2.0/`, `qa-1.2.2/`에 있다. `/tmp` 빌드 로그와 Desktop 녹화도 임시/로컬 증거이며 새 clone에서 자동 복원되지 않는다. 핵심 결과는 이 문서와 implementation.md에 영구 기록했다.
- `.codex-attachments/`는 사용자 첨부물이며 현재 Git 미추적이다. 자동 stage/삭제하지 않는다. 빌드·dist·캐시도 Git에 넣지 않는다.
- 다음 플러그인은 별도 `plugins/<NewName>/`와 `mpc-software/docs/plugins/<new-slug>/`, 고유 UID, 독립 버전/태그/Release를 쓴다. 제조사는 실제 생성 전에 확인한다. 기존 Donut의 UID/208 IDs/음색을 새 프로젝트 때문에 바꾸지 않는다.
- 사이트는 같은 카탈로그에 새 항목/상세/해당 Release 링크를 추가한다. 저장소 공통 `releases/latest`를 쓰지 않는다. 실제 공유 수요 없이 Donut 전용 코드를 shared/로 옮기지 않는다.

## 최신 개발 변경 — 1.2.3 / 2026-11-16

사용자 요청으로 A SOUND/PLAY의 BODY를 기존 SUB로 교체했다. A=OSC Pair/Osc Mix/Sub/Drift. 서브는 OSC1 -12반음 사인파이고 공통 필터/ADSR을 지난다. Body는 숨긴 레거시 ID로만 남고 음성 경로에서는 제거했다. 공장64음색의 옛 Body 믹스/서브 기여를 각각 Mix/Sub에 이전했다. 자동 엔진·주파수·상태·Q-Link·스킨 검사 및 전 플랫폼 빌드 통과. 실제 SUB 기기 조작·청음은 아직 확인하지 않았다.

현재 기기는 MPC XL / OS3.10.1.12이며 플러그인 설치 기본 대상은 내장 NVMe다. Donut Theory1.2.3(64파일), Groove Dice1.1.0(140파일, 개인 뱅크 포함), Crate Press0.2.2(128파일)를 `/nvme/VST2 Plugins/`에 설치하고 검증했다. 기기 교체 시 드라이브를 재조회한다. 과거 Live III SD 설치 기록을 새 기본값으로 사용하지 않는다. 공개 Release/웹사이트는 이번 작업으로 갱신하지 않았다.

Groove Dice 예박 후속 수정(1.1.1): 실제 MPC XL 양수 예박 전환을 계측해 자동 감지로 수정했고, 사용자가 예박 On/Off 두 경우 정상 동작을 확인했다. 추가 예박 설정은 넣지 않았다. 내장 설치의 현재 Groove Dice 버전은1.1.1이다.

## 2026-11-17 배포 승인

사용자가 1.2.3 정식 배포와 사이트 갱신을 승인했다. SUB 전용 물리 조작·청음은 별도 확인하지 않았다는 한계를 릴리스에 유지한다.

## 2026-11-18 — approved key reference / transpose (1.3.0 candidate)

User approved the Key/circle CHORD concept and explicit TRANSPOSE confirmation. Root/Major-or-natural-Minor selects reference only; circle and diatonic spelling are generated from musical pitch/letter data, not AI text artwork. The four visible degree badges identify scale-root membership, with case from chord quality and dash for chromatic roots. Existing four row controls and sixteen SCREEN assignments remain unchanged; MIDI/LEARN move to PAD SETUP, not deleted.

TRANSPOSE FROM defaults C for older projects and is editable for preexisting progressions. TO is the selected reference key. APPLY moves all active roots by one shared nearest interval −5..+6 (tritone +6), updates FROM, and preserves type/voicing/octave/MIDI/sound and inactive slots. Out-of-range MIDI24..84 rejects the whole operation; no individual clamping. Key-mode changes do not rewrite chord quality. Modal command states are cleared on save/load; no automatic apply on project recall.

Parameters208..219 append: key root/mode, source root/mode, preview/apply/status, four degree readouts, interval. First208 declarations are identical; UIDKrql and synthesis recipes unchanged. Public release remains1.2.3 until publication. Automated and device verification are recorded below as completed.

## 2026-11-18 — 1.3.0 candidate implementation and MPC verification

Implemented the approved Key/circle layout, reference-only Major/natural-Minor selection, degree badges, explicit all-active-slot TRANSPOSE confirmation and PAD SETUP. First208 full parameter declarations match a committed1.2.3fixture;12parameters append, total220. UIDKrql, sound programs and sixteen CHORD SCREEN assignments remain unchanged. All576root/source/active-count combinations, inactive-slot/field preservation, atomic whole-operation bounds rejection, current-state/old208migration, saved gesture clearing and allocation guards passed. Full native/UBSan/analog/host-bridge/skin suite passed. Host simulator:19PASS/0WARN/0FAIL. Final native/ARM32/ARM64 builds and SDK validator0errors/0warnings. Source logs remain locally under /private/tmp/dt130-*.log.

Actual MPC XL/OS3.10.1.12, internalNVMe: SDK install/restart, then official SDK export compared all114installed files byte-for-byte with the final build. Remote clicks verified MAIN on entry; KeyC->D without changing notes; confirmation+2semitones/4slots; APPLY changed Dm9/G13/Cmaj9/Am9 to Em9/A13/Dmaj9/Bm9; Major/Minor reference changes;16slots/bank13–16; PAD SETUP, LEARN arming/cancel and closing. Fixed two observed visual defects before final capture: the bottom enharmonic minor label overlapped neighbors, and a legacy MIDI heading was duplicated inside PAD SETUP. Final captured pages have both corrections.

Actual1280x800frames and SHA256 provenance are in ../../../plugins/public/DonutTheory/design/device-1.3.0/verification.json. No physical encoder rotation, physical MIDI LEARN registration or new hardware-audio/CPU certification was performed. A native guide did not appear on remote knob click, so modal focus-transition behavior is not newly hardware-certified; regression checks preserve exact row bounds, static mappings and hideQLinkBounds. CHORD expands to2137widgets, below2500. No shared SDK modification.

Installed development candidate1.3.0; public stable Release/site remain1.2.3. The user approved implementation; this record does not claim a newly published release.

## 2026-11-18 — approved SMOOTH VOICING (1.4.0 candidate)

User approved top-note voice leading with current-four/all-active scope, before/after preview, audition and explicit APPLY. First selected chord's existing voicing/octave anchors the loop; candidate inversions/Open and octave−2..+2 are selected by lexicographic total closed-loop top movement, closest-tone/bass movement, then edit distance. Roots/types/Key/MIDI remain intact; only selected voicing/octave fields are applied. Unsafe/clipped/collapsed registers are rejected; spans do not exceed max24semitones/originalspan.

Planning runs only on a non-render UI poll/idle behind the reentrancy guard. Audio consumes a consistent atomic publication and checks source freshness; no search, allocation, lock, I/O or direct host notification runs in process. PREVIEW uses separate owner4096 and a copied proposal, one chord/beat and automatic end; CANCEL/X/outside closes/release without applying. Shared16voice limits still apply. Saved UI/apply/preview commands are cleared.

Preserve first220 parameter declarations; SmoothOpen..SmoothRow16 append220..241 (242total). Existing16CHORD Q-Link/guide geometry, current Key/circle UI and synth recipes remain. SMOOTH VOICING is a44px utility above TRANSPOSE; the426px drawer hides background guides and shows only active scoped top-note rows. Public stable remains1.2.3; candidate validation/device results follow in implementation records.

## 2026-11-18 — SMOOTH VOICING1.4.0 implementation and final verification

User-approved current4/all-active top-note optimization, explicit preview/cancel/apply and loop connection are implemented. The default four-chord loop achieves E3/E3/E3/E3, with42->0semitones travel. First220 full declarations match their fixture;242total, UIDKrql and existing synth/CHORD maps remain. Native/ARM32/ARM64 builds and SDK validation passed. Full engine/UBSan/analog/bridge/skin suite passed; host simulator19PASS/0WARN/0FAIL. Independent exhaustive enumeration proves the example optimum;400varied progressions verify no increase, valid registers and exact reapplication;20000atomic publications exercise consistent reads. Preview ownership/end/stop/state and stale source/apply checks passed, including a UI recompute arriving before the next audio block. The planner never runs on the render thread.

Final MPC XL/OS3.10.1.12 internalNVMe installation was exported via the official SDK: all116installed files byte-identical to final build. Remote clicks verified MAIN entry, utility placement, current-four proposal, PREVIEW start/second-click stop/automatic end, CANCEL preservation, current-four APPLY, all16rows/Apply/bank13–16 and Already smooth0->0after reopening. The initial native indexed PREVIEW button selected ON again on second tap; replaced it with a single Mouse Down Toggle Switch, actual-state artwork and independent local reset surface. Added the regression and reverified the corrected toggle on hardware. Enlarged proposal rows to24px.

Direct frames and checksums are in design/device-1.4.0/verification.json; CHORD2188expanded widgets stays below2500. Physical Q-Link rotations/fresh MIDI LEARN, hardware audio listening/output, worst-case CPU and native guide focus transitions are not newly certified. No SDK modification, public Release, push or website deployment was performed for this development candidate; public stable remains1.2.3.

## 2026-11-18 — 1.5.0 harmony / CHORD GLIDE / STRUM candidate

User approved expansion of both chord types and progression recipes, dedicated chord glide using existing Glide time, and single-shot strum (not a repeating arpeggiator). Current library32types/96four-slot recipes. First11types/32recipes stay unchanged;64new harmonic fingerprints are unique under transposition,8each NeoSoul/Hip-hop/Vamp/RnB/Modal/Gospel/Borrowed/Turnaround. Five-tone voicings explicitly omit optional extensions/fifths as documented in harmony-library.md.

First242declarations/ranges remain exact. Appended canonical16types,4preset banks,4type edit relays,4tone-count readouts, ChordGlide and current-bank preset relay:272total. Legacy automation aliases retain11/32ranges. Old state seeds full-range choices; new state guards legacy normalized replays for roughly0.5audio-seconds. A single current-bank preset browser replaces4hidden96-row trees; Program/Track synth mapping and CHORD’s16controls stay, with Type Q-Link using its full-range relay. MOD stays bank1-only. Voicing menus share triad/seventh/extended classes rather than32duplicated trees.

Chord Glide owns a newest-held chain per MIDI channel; screen holds share channel1. Exact common tones retain voices/envelopes, remaining notes use bounded minimum-distance matching. Latest release falls back to an older held chord; nonoverlap snaps, pedal/channel/duplicate input and page-change ownership are covered. Ordinary melody Mono/Legato remains separate. STRUM reuses existing Spread0..40ms; new onsets and connected moved-pitch transitions roll low-to-high. Early release/panic/reset cancels both pending gates and pitch starts. Touch GLIDE/STRUM appear beside a latched CHORD GLIDE button above the input-mode buttons; no extra Q-Link group.

Build, device and validation outcomes are appended when complete. Public stable remains1.2.3 while this candidate is under test.

## 2026-11-18 — 1.5.0 implementation and device verification

Implemented32chord types,96four-chord recipes, CHORD GLIDE and single-shot STRUM. Existing first242full parameter declarations/ranges, first11types/32recipes,64sound programs and UIDKrql remain; total272parameters. Added bounded nearest-voice matching with common-note preservation, per-channel newest-held fallback and delayed moved-pitch starts. Early release/panic/reset cancel pending strum notes and pitch starts. Legacy/new-state migration, normalized replay, all16type relays/four bank recipes, MIDI/touch/pedal/channel/duplicate/page ownership,400extended smooth progressions and20000atomic publications pass regression. Native/ARM32/ARM64 builds and SDK validator0errors/0warnings; host19PASS/0WARN/0FAIL.

Installed via official SDK on MPC XL/OS3.10.1.12 internalNVMe. Read-only export confirms all120installed files byte-identical to final build (exportSHA256 fb862524caf560b5b3bc286c9318fc2f364b6dbfbe0cb33e1716d0028435d7eb). Direct Remote Screen verified fresh MAIN entry, paged32-type selection/Minor11,96-preset Neo page and selection, fourth inversion menu, GLIDE/STRUM horizontal dragging (194.4ms/11.2ms), native Glide double-tap enlargement, CHORD GLIDE toggle, SMOOTH preview start/stop/apply (36->4semitones) and independent bank2 preset/bank1 preservation. Actual1280x800frames, widget counts and all runtime hashes are recorded in design/device-1.5.0/verification.json.

Full local manual/site build and260local links passed. Physical Q-Link rotations/MIDI, device audio listening/recording and worst-case CPU are not newly certified; deterministic native DSP tests cover voice/pitch/strum timing. This is an installed development candidate; no public Release, push or website deployment was performed. Public stable remains1.2.3.

## 2026-11-18 — approved shared Tremolo / Auto Pan (1.6.0 candidate)

User chose a single motion module instead of independent simultaneous tremolo/pan pages. MAIN D is FX / MOTION: Drive, Glide, Mod Rate, Mod Depth. Existing rate/depth IDs/ranges and all16MAIN/CHORD mappings remain unchanged; native legacy parameter names are retained for compatibility, UI aliases show Mod Rate/Depth. Touch selectors choose Tremolo or Auto Pan and Sine/Triangle/Saw/Square/S&H. No new D page or fifth Q-Link group. FX detail additionally exposes base Pan (L100..Center..R100). Rate remains free0.1..15Hz; no new tempo-sync promise.

First272full declarations stay exact; ModMode272, ModShape273 and Pan274 append (275total). Old states default Tremolo/Sine/Center. Shared oscillator runs independently of filter LFO; mode crossfades over15ms, volume gain smooths2ms and stereo balance5ms, including hard-shaped wave changes. Tremolo remains before chorus/delay; Auto Pan balances final stereo after effects/low-cut, preserving existing stereo instead of summing to mono. Base pan offsets the motion and clamps at channel endpoints; Depth0 disables motion but retains base pan. No render allocation/lock/I/O. Actual implementation/device outcomes follow separately.

## 2026-11-18 — 1.6.0 final implementation and verification

Shared Tremolo/Auto Pan is implemented with the existing D-group Rate/Depth, touch Mode/Shape selectors and base Pan in FX. First272parameter declarations match the1.5.0fixture;275total. Native/ARM32/ARM64 builds, SDK validator0errors/0warnings, full native/UBSan/analog/bridge/engine/skin suite and host19PASS/0WARN/0FAIL passed. New audio regressions verify alternating stereo energy, zero-depth/center behavior, hard pan, common-channel tremolo, both modes/all five shapes, allocation-free variable blocks and current/old272state migration. Native UI labels override legacy Tremolo names without changing host metadata.

Installed final1.6.0 on MPC XL/OS3.10.1.12 internalNVMe via official SDK. All120read-back files match final build byte-for-byte (exportSHA256747e992d63224e314370b50be35fc6da9ae6823b7cc75c897cb1496b201f82ac). Direct remote input verified MAIN on entry, Auto Pan selection, rate/depth dragging to0.3Hz/29.7%, FX base Pan R44%/Center, both mode segments, waveform dropdown/selection and other six tab screens. The first device pass found an obsolete SPACE caption overlapping the fourth FX row and native legacy Tremolo labels; both were corrected, rebuilt, reinstalled and rechecked. The waveform menu now opens directly on touch. Its modal hides the focused Pan guide.

Unaltered1280x800frames, expanded widget counts and runtime hashes: design/device-1.6.0/verification.json. Local manual/site build and260links passed. Physical encoder movement, hardware audio listening/recording and worst-case CPU were not newly tested. This remains an installed development candidate; no push, public Release or website deployment. Public stable remains1.2.3.

## 2026-11-18 — 1.6.1 right-aligned motion selectors

User correction: the mode belongs on the right with its motion knobs. MAIN D now centers Mode directly above Mod Rate (964,356,136,32) and Shape above Mod Depth (1114,356,136,32). Removed the cramped MODE/WAV captions between dropdowns; actual selected values remain visible. Drive/Glide positions, all275metadata declarations, Q-Link maps and DSP behavior are unchanged. Only package/native version advances to1.6.1.

Native/ARM32/ARM64 builds, SDK0errors/0warnings, existing skin regressions and host19PASS/0WARN/0FAIL passed. Installed on MPC XL/OS3.10.1.12 internalNVMe and exported: all120files byte-identical. Direct remote taps confirmed the moved Mode drawer/Auto Pan selection; Rate/Depth drag confirmed0.3Hz/29.7%. Unaltered captures and runtime hashes: design/device-1.6.1/verification.json. No new physical encoder/audio/CPU certification, push, Release or website deployment; public stable remains1.2.3.

## 2026-11-18 — independent tempo sync (1.7.0 candidate)

User requested Sync for both filter LFO Rate and shared Motion Rate. MAIN keeps the existing sixteen knobs. Each rate gets an adjacent SYNC touch toggle; Off uses Hz, On makes that same knob/Q-Link select1/16,1/8,1/4,1/2,1bar. The filter reuses existing Sync/Division; Motion appends its own pair. Each module retains its free frequency while synced and returns to it when Sync is disabled. Existing MOD Sync/Division remains connected. The right-aligned Motion Mode/Shape placement from1.6.1 is retained.

First275metadata declarations stay exact. Append MotionSync275,MotionDivision276,LfoRateControl277,ModRateControl278 (279total); the latter two are nonautomatable UI relays whose writes route to the actual frequency/division parameters. MAIN Screen and Program/Track rate assignments use these relays; all other assignments/guide rectangles stay. Existing cached custom maps may need reset or a fresh instance. Engine readback uses the existing render-thread/index guard; state loading and initial legacy replay window cannot let relays overwrite canonical values.

Both synced oscillators follow host tempo, lock phase to PPQ during valid playback, and continue running at the last/current beat-derived frequency while stopped. Missing/invalid tempo falls back to120BPM. Motion's control-rate free-frequency update is disabled while synced, so it cannot overwrite the tempo setting. No extra independent oscillator, D subpage or Q-Link group. Build/device outcomes are recorded separately.

## 2026-11-18 — 1.7.0 sync implementation and device verification

Implemented independent filter/Motion SYNC buttons with five divisions and shared per-module Rate controls that route to Hz or division. First275full declarations remain exact;279total. Free frequency is preserved across Sync changes. Motion Sync is not overwritten by its per16sample free-Hz update. Native/ARM32/ARM64 and SDK0errors/0warnings passed; full native/UBSan/engine/smooth/analog/bridge/skin tests and host19PASS/0WARN/0FAIL passed. New tests verify both rate relays, division independence, free-rate memory, BPM/PPQ phase alignment, stopped/fallback tempo and old275state migration.

Installed final1.7.0 on MPC XL/OS3.10.1.12 internalNVMe via official SDK; all122exported files are byte-identical to the final build. Direct remote touch verified persistent SYNC On/Off, Rate drags to filter1/8/Motion1/2, Off restoring1.2Hz/4Hz, MOD Sync mirroring MAIN, FX availability and right-aligned Auto Pan selection. Increased RATE/SYNC typography after the first device inspection; rebuilt/reinstalled/rechecked the corrected final skin. Captures/hashes/counts: design/device-1.7.0/verification.json. Physical encoder movement, device audio/CPU are not newly certified. Local manual/site build and260links pass. No push, Release or website deployment; stable remains1.2.3.

## Pending design discussion after1.7.0

User asked whether rarely used ADSR should move off MAIN, and requested LFO2. Proposed moving MAIN B ADSR access to the existing AMP tab and using B for independent LFO2 Rate/Depth/Shape/Target with its own Sync. Existing A/C/D remain. An async question asks whether this is a genuinely independent routable LFO2 (recommended) or relabeling the existing Tremolo/Auto Pan module. This proposed replacement is NOT implemented in1.7.0; MAIN B still shows ADSR. Read the user's response before implementing further UI/DSP changes.

## 2026-11-18 — approved1.8.0 main groups, active Body and independent LFO2

This decision supersedes the pending B=LFO2-only proposal. User approved A=OSC Pair/Osc Mix/Detune/Drift; B=Filter Cutoff/Resonance/Body/Sub; C=LFO1Rate/Depth/LFO2Rate/Depth; D=Drive/Glide/MotionRate/Depth. Body and Sub are both visible. MAIN has no ADSR; Amp ADSR remains on AMP, filter ADSR/Env Amount/Key Track on FILTER. C has per-LFO Shape/Target touch selectors and independent Sync beside each Rate. Motion Mode/Shape remains right-aligned above D Rate/Depth. Screen/Program/Track defaults follow these sixteen controls; CHORD keeps Root/Type/Voicing/Octave. Old cached track maps can require resetting to defaults/new instance.

Preserve first279full declarations. Append BodyTone279 and LFO2 Shape/Rate/Depth/Target/Sync/Division/RateControl280..286:287total. Legacy Body2 stays inert so old stored values do not unexpectedly activate processing. New Body defaults0; new LFO2 depth defaults0. Factory program selection initializes these new settings. A saved custom state retains them. LFO2 is independent of filter LFO1 and the shared Tremolo/Auto Pan module, with Pitch/Filter/Pulse Width/Amp/OSC Mix targets, Sine/Triangle/Saw/Square/S&H, free0.05..20Hz or the existing five beat divisions. Rate relay uses the same state/render guard; no new Q-Link bank.

Body is an original broad140..650Hz density stage after voice sum and before Drive. Two fixed one-poles extract the low-mid band, apply bounded soft saturation and blend with up to roughly3dB broad emphasis; a fixed1/(1+0.18*amount) gain compensates the boost. It generates no sub-octave oscillator. Body0 is an exact output bypass. Sub remains the independent octave-below sine, so both can be adjusted together. LFO2 uses a2ms signal smoother and10ms target crossfade; parameter smoothing and bounded mix/pulse/filter ranges remain. No render allocation/lock/I/O. Build/device evidence follows after validation.

## 2026-11-18 — 1.8.0 final implementation / actual MPC verification

Approved sixteen-control layout is implemented: A OSC Pair/Mix/Detune/Drift, B Cutoff/Resonance/Body/Sub, C LFO1Rate/Depth/LFO2Rate/Depth, D Drive/Glide/MotionRate/Depth. Amp/filter ADSRs remain on their existing detail tabs. Independent LFO2 has five shapes, five targets, Sync, free-rate memory and a mode-aware rate relay. Body is an original low-mid saturation/emphasis stage with fixed gain compensation; new active Body is appended while legacy Body2 stays inert. First279full declarations remain exact;287total. Factory program selection defaults Body/LFO2Depth0; old279states migrate to bypass.

Native/ARM32/ARM64, SDK0errors/0warnings, full engine/UBSan/smooth/analog/bridge/skin and host19PASS/0WARN/0FAIL passed. New regressions measure Body band response/bypass/level bound, all five LFO2targets/shapes, depth0baseline isolation, independent sync/PPQ/free memory, state migration and allocation-free variable blocks. Structural tests assert the approved Screen/Program mapping and retention of both detailed ADSRs.

Final1.8.0 installed on MPC XL/OS3.10.1.12 internalNVMe via official SDK. All122read-back files byte-identical to final build. Direct remote input verifies fresh MAIN, Detune0->6ct, Body0->29.7%, LFO2Target/Shape, LFO1Sync1/8versusLFO2Sync1/2, LFO2Depth29.7% and mirrored MOD values, all seven tabs and detailed ADSRs. Compact Pulse Width initially abbreviated; expanded the value area, rebuilt/reinstalled/rechecked the full label. Final unaltered1280x800frames and runtime hashes: design/device-1.8.0/verification.json. Remote frame capture now commits whole tile batches so saved captures cannot mix partial protocol frames.

Local manual/site build and260links passed. Physical encoder movement, hardware audio listening/recording, full MPC project round-trip and worst-case CPU were not newly certified. Old cached/custom MAIN maps may need reset/new instance. No push, public Release or website deployment; public stable remains1.2.3. This final entry supersedes the earlier pending ADSR/LFO2 layout discussion.

## 2026-11-18 — original Soul Studies audition patches

User requested an actual J Dilla-inspired sound. Created two original1.8.0studies without changing the64factory programs, plugin code, installed MPC files or public release: Keys / Dusty Sunday and Synth / Velvet Melt. Both use the original82BPM Cmaj9–Am9/E–F6/9/D–G9sus4/Dphrase with top D4/C4/C4/C4. First is sine phase-modulation/triangle keys with8ms strum; second is soft saw/triangle with6ms strum and65ms chord glide. Body/Drift/independent LFO2mix and slow Auto Pan are included. No outside sample/recording/effect is used.

Audio renders from the restored native1.8.0preset chunks, not from hardware output. All287parameter values round-trip; audio is finite/unclipped and all voices release. Each clip15.4573s,48kHz/stereo/16bit PCM. Saved Sound Trim matches average levels (RMS0.06488/0.06563; peaks0.58248/0.35302); no external normalization/mastering. Reproducible tools render_soul_studies.cpp/export_soul_studies.py and preset recipes/WAVs/path-specificXPL/verification are in presets/soul-studies. XML/state64 equality is verified; actual MPC preset-browser import is not tested. XPL targets the known internalNVMe ARM64install and includes the demo chord bank/default individual pad routing. Do not describe these as installed or new built-in factory programs.

## 2026-11-18 — Soul Studies wider Auto Pan revision

User found the18%/24% audition depths too subtle. Added an optional --wide-pan render variant at70%/80%, preserving rates0.22/0.18Hz and all other timbral/harmonic settings. Saved Sound Trim is recalibrated for matched average level. Original files remain intact. Wide-state287parameter round-trip, finite/unclipped audio, note release and WAV/XPL checks pass. Measured100ms stereo windows over0.3..11.6s: Dusty Sunday L/R balance range grows−0.68..+0.77dB to−6.15..+6.30dB; Velvet Melt−1.23..+0.82dB to−9.55..+8.80dB. Metrics and new user-preset/WAVs are in presets/soul-studies/wide-pan. Engine/factory bank/device installation unchanged; actual MPC preset import remains untested.

## 2026-11-18 — Soul Studies88BPM / Auto Pan1/8

User requested88BPM and eighth-note Auto Pan. Renderer now accepts --bpm/--pan-sync while retaining the original/wide variants. New sync-88-eighth files use host tempo/PPQ88, ModSync1/Division1, depths70/80%; one cycle0.340909s/2.93333Hz. Native-state round-trip, finite unclipped audio, full note release and saved-setting/WAV/XML checks passed. Clips14.6591s. Presets do not set host tempo; MPC must use88BPM. Engine/factory bank/installed MPC unchanged; MPC preset import remains untested.

## 2026-11-18 —1.9.0 official publication preparation

User authorized official Release and website deployment after approving88BPM/1/8Auto Pan auditions. Both accepted timbres append as factory65/66; first64programs and first287full declarations retained, canonical SoundCurrent287(total288) keeps legacy46range64. Original1.8.0DSP/layout retained. Full build/tests and final display-name skin checks pass. InternalNVMe MPC XL final read-back122files byte-identical; actualscreens/hashes are in design/device-1.9.0/verification.json. Native factory auditions preserve the approved result (DustyexactPCM,Velvetmax1LSB). Detailed release/manual/README and site are current. Website build/links/responsive tests pass; final publication URLs/status will be appended after deployment. Hardware audio/CPU/fullproject/physicalcoverage limits remain as release notes.

## CURRENT — official1.9.0 published (2026-11-18)

**Public stable is now1.9.0.** This supersedes all earlier “stable1.2.3 /development candidate /no push” status lines above. GitHub Release donut-theory/v1.9.0 is public, with the official MPC ZIP and SHA256SUMS. Release commit39723c8, Release ID449664476. Public anonymous ZIP download matches the packaged original (SHA25642a0e0f62bdf92273f0a26f4bbc04bf6811909f3c0678df8e5e1598785da69e1). Existing GitHub Pages workflow44261562442completed successfully. https://kimswork.app/plugins/donut-theory/ and /manual/ expose1.9.0; home/Software thumbnail, six actual MPC screenshots and two WAVs are live. Thirteen public HTTP/content/hash checks pass; both public native audio controls start playback and advance (14.659083seconds each). Local browser verification covers56route/width combinations, all lazy images, English/dark/no-overflow;270local references pass.

Owner MPC XL /OS3.10.1.12 has final1.9.0installed on internalNVMe. All122exported runtime files match build; actual remote MAIN/CHORD/all detail pages,66sound selection/host names and final OSC category launcher checked. Source identity/compatibility: kim/Krql,288parameters,66factory programs; first287full declarations and original64programs retained, legacy46range64 unchanged. MAIN/Body/LFO2/sharedMotion/sync map is1.8.0approved map. Key/circle/transpose/smooth/32types/96recipes/glide/strum are official. Hardware audio/CPU/full project round-trip/complete physical encoder and multi-touch limitations remain; do not promote native or remote checks into those claims. Full publication evidence: plugins/public/DonutTheory/design/device-1.9.0/publication.json and verification.json. No other product version was changed for this publication.

## Local1.10.0 candidate — whole-sound RANDOM (2026-11-18)

User cancelled the song render and requested a single RANDOM button, then explicitly chose complete oscillator-to-FX generation rather than current-preset variation. MAIN adds a48×52px dice/RND touch button at910,26 in the header gap beside INPUT. It retains all existing controls, the donut artwork and sixteen Q-Links. A UI-local press gate gives held feedback; one Mouse Down sends the command and the top continuous surface owns release.

Generated timbre correlates one of five playable envelope families (bass/keys/synth/lead/pad) with newly drawn oscillator pair/mix/detune, pulse/FM/sub/noise, filter/envelopes, Body/Drift, modulation and FX. All three Sync switches (LFO1, LFO2, shared Tremolo/Auto Pan) are forced ON; each Division is independently chosen from1/16,1/8,1/4,1/2,1bar. Cutoff, resonance, pitch modulation, drive and wet levels are bounded; this is not uniformly random full-scale noise. Free rates remain valid for later Sync-off editing.

Master Volume, reference Key, all16chord root/type/voicing/octave/MIDI settings, Input/Trigger/PadMap, slot count/page, Strum and CHORD GLIDE preference are preserved. Ordinary Voice Mode/Glide time are timbral choices. Sound change releases sounding voices as factory selection does. Display reads Random Sound rather than claiming a factory recipe. Re-selecting the same factory program reloads it; factory selection clears Random status. No new factory program is inserted.

First288declarations are unchanged. Randomize288 is a nonautomatable transient command, Randomized289 a persisted nonautomatable status; FactoryReload290 explicitly reapplies a selected factory row:291parameters total. Save/load/reset disarm commands without regenerating a saved sound; old288states default both fields0. RNG seeds once in the constructor; process uses fixed arrays and xorshift, without allocation, locks, file access or host calls.

Native/ARM32/ARM64 and SDK0errors/0warnings; complete tests including native/UBSan60randomizations, all16chord/pad preservation, forced Sync, finite/non-silent variable-block audio, state and same-program reload pass. Host19PASS/0WARN/0FAIL and skin/guide/legacy tests pass. Actual-device outcomes are recorded separately after installation. This is a local candidate; public stable stays1.9.0 until an authorized new release.

### 1.10.0 candidate installed and remote UI verified

MPC XL /OS3.10.1.12, internalNVMe. Official SDK post-install export matches all124runtime files. Direct device captures in plugins/public/DonutTheory/design/device-1.10.0 show clean MAIN, orange held RND/cream release, Random Sound in both host and internal selectors, all three Sync ON, OSC selector state and reselecting the same Init factory program back to defaults. First288declarations remain exact;291total. Complete native/UBSan random/state/compatibility and skin tests pass; host19PASS/0WARN/0FAIL; native/ARM32/ARM64 SDK validation0errors/0warnings. Existing chord mapping and sixteen Q-Links are retained. Physical encoders, hardware audio/CPU, multitouch and full MPC project round-trip were not newly certified. Public stable stays1.9.0; no release/site push was performed.

## 1.10.0 release authorization

The owner explicitly requested official release and website updates after candidate completion. The source release and site include whole-sound RND, three forced Sync switches, preserved chord/master settings and actual1.10.0 MAIN capture. Historical local-only statements above no longer constrain this approved publication. Runtime/build evidence and honest physical/audio limits remain unchanged. Publication IDs and asset checksums are recorded in design/device-1.10.0/publication.json after publishing.

## Local1.10.1 — approved action footer

User rejected the redesigned mockup typography/art and approved rearranging the original interface only. Groove Dice is the fixed reference and must not be modified. MAIN moves Sound Preset/Voice/Input to a44px browse row at y456; orange ROLL DICE and chord-preset/page actions occupy y502. All sixteen conditional hold pads use305×70cells at y550. Header/logo/donut source art and native Roboto Bold type remain; original main knob art is retained at72px to make room. Local filter/LFO/mode/Sync controls remain adjacent. Four guides follow the compact panels94/278with176px height. Detailed pages and all Q-Link assignments are byte-identical to1.10.0;291parameters/66programs/UID remain unchanged. No DSP change beyond binary version11001. Native/ARM32/ARM64 and SDK validation0errors/0warnings, skin regressions and host19PASS/0WARN/0FAIL pass. Actual device results follow. Public stable remains1.10.0; this is not a new release/site publication.

### 1.10.1 final local verification

Official SDK final export matches165files on MPC XL internalNVMe. Direct captures in design/device-1.10.1 show the approved original-art footer; ROLL DICE, three Sync switches, category/factory reload, chord preset/hold pad, Voice/Input choices, MAIN/CHORD and all five detail pages were checked. The initial native TO ellipsis was corrected by retaining the original18px prefix and covered by a regression check. Original background/header-donut/knob source PNGs are byte-identical. All Q-Link JSON and detailed tab component trees are byte-identical. Physical encoders/multitouch, hardware output listening/recording, full project recall and worst-case CPU were not certified. Public Release/site remain1.10.0; no push/publication performed.

## 1.10.1 publication authorization

The owner approved distribution after seeing the actual footer implementation. The current source/manual/site use1.10.1, bottom ROLL DICE/BROWSE/CREATE and actual device captures. Historical candidate-only statements above no longer constrain publication. Groove Dice remains unchanged. Final release/asset/Pages results are recorded in design/device-1.10.1/publication.json.


2026-11-18: User explicitly authorized official Donut Theory1.12.0 release and website publication. Earlier candidate-only statements are historical. ENV/PLAY and Release Pitch Drop are the current specification; preserve recorded physical/audio/CPU limits. Final publication details go in design/device-1.12.0/publication.json.

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


### Latest native PRESET investigation — installed1.13.0
Actual MPC XL/3.10.1.12: the host-owned top PRESET popup is blank in the existing Random Sound state and a fresh Init instance. Native SDK API enumerates all66programs; this is not evidence that the UI works. In a new Groove Dice1.5.1 instance, the same native popup displays kit names and selecting Velvet Basement succeeds (254programs with owner banks). Do not generalize a shared SDK/host limitation or claim top preset selection works for Donut. Exact cause remains unproven; investigate program-name/dispatch differences. No source fix, deployment or release in this comparison. Captures and findings: plugins/public/DonutTheory/design/device-1.13.0/native-preset-investigation.


### 2026-11-19 native PRESET investigation / local1.13.3
Donut native popup is blank in Random Sound and fresh Init. Groove Dice1.5.1 and Crate Press0.2.4 native list/selection work on the same MPC XL. Slash-label replacement alone was tested and did not fix Donut; original names restored. A separate confirmed bridge defect: effGetProgram drained host notifications, causing UI callbacks on a render-thread metadata read. New regression fails baseline and passes after making effGetProgram side-effect-free. All353IDs/66program indices/recipes and UI/Q-Link maps remain. Final1.13.3native/ARM/SDK/full regression pass; temporary file logging is removed. Native-popup causality/hardware completion is NOT confirmed: device became unreachable at192.168.0.56 before final install. Installed version at disconnect was temporary1.13.2probe. Restore/install final1.13.3upon reconnect, then continue controlled API/skin comparison until native list and selection actually work. Do not report this candidate as the confirmed native-popup fix. See DonutTheory/design/device-1.13.3/native-preset-fix/verification.json and CratePress/design/device-0.2.4/native-preset-check/. Public releases unchanged.


### 2026-11-19 reconnect investigation continuation
Reconnected MPC XL: original full-art temporary1.13.4 again showed a blank native picker. Actual device program trace confirms all66 indexed names/count are valid (value=-1). An unmodified SDK dispatcher/value-reader comparison1.13.5 displayed the native list; original bridge with notification flush disabled1.13.6 remained blank. Removing only the early NeedIdle request1.13.8 also remained blank. Runtime idle-routing comparison1.13.7 gave a visible list but multiple writes advanced the mode, so this is a lead, not exact cause proof. Network/Remote/SSH all dropped again while loading a fresh QA project under1.13.9, before the plugin was opened. Installed1.13.9 is a TEMPORARY mode-comparison build; replace it before final delivery. Scratch1.13.10 tests only idle return0 and has no QA logging/controls; built, NOT installed/tested. Product source remains safe local1.13.3 (pure program metadata reads), NOT a confirmed native-list fix. No release/site publication. See design/device-1.13.3/native-preset-fix/reconnect-investigation.json. Crate Press0.2.4 and Groove Dice1.5.1 native preset list/actual selection were verified working; no source fix needed for their native picker.

2026-11-19 publication complete: Donut Theory1.13.14 and Groove Dice1.5.1 are official stable Releases from sourced23da9a. ZIP/checksum digests verified anonymously; public Pages run44419559175 succeeded;77responsive route-width checks and current actual MPC screenshots published. Crate Press0.2.4 remains the unchanged public preview. See each device-version/publication.json. Earlier candidate/public1.12.0/1.4.0 statements are historical. SD migration remains pending: the user requested a portable SD install, but SDK drive discovery twice showed only internal nvme and removable SSD. Do not claim card installation or choose SSD without identification. Preserve owner expansions and existing internal copies until the SD destination is confirmed and verified.

2026-11-19 SSD installation completed (supersedes SD-pending notes): user explicitly selected the removable drive named SSD for portable use. Official SDK add/export copied Donut Theory1.13.14, Groove Dice1.5.1 and Crate Press0.2.4 to SSD/VST2 Plugins. All470files matched SHA256 against the actual internal-NVMe originals (177/165/128). Groove Dice includes all four existing personal expansion packs; these were copied locally only and remain excluded from public releases. Internal originals were preserved. Current MPC mounts SSD noexec; execution from SSD was NOT verified and must not be claimed. On another MPC enable External Plugin Support and restart/rescan. Local operation report: mpc-software/plugins/public/GrooveDice/build/device-ssd-2026-11-19/verification.json (Git ignored). This is a portable SSD copy, not an SD-card installation.

2026-11-19 final storage state (supersedes SSD copy, identity hold and SD-pending notes): user confirmed the same MPC and inserted a64GBcard, discovered as36C7-1ACA (/dev/sdb1 at/media/36C7-1ACA). Backed up all three old card folders, then installed Donut Theory1.13.14, Groove Dice1.5.1 and Crate Press0.2.4 from verified NVMe-derived originals. All470SD files matched SHA256, including four personal expansion packs. Only after this verification, removed exactly our three plugin folders from/media/SSD using the official SDK at explicit user request. Final SDK inventory: NVMe latest three intact; SD latest three present; SSD zero plugins. No device writes targeted NVMe. SD is mounted noexec and execution from it is NOT verified; do not equate file installation/hash verification with runtime certification. Old card backups remain local at/private/tmp/kimswork-sd-before-update. Current operation report: mpc-software/plugins/public/GrooveDice/build/device-sd-2026-11-19/verification.json (Git ignored). No new release or website publication is implied by this storage operation.
