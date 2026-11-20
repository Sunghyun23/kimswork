# Repository instructions

## 작업 시작

- 이 저장소는 kimswork 소프트웨어 모노레포이며 MPC 작업은 mpc-software/에 둔다. MCP(Model Context Protocol) 프로젝트가 아니다.
- README와 `docs/repository-policy.md`를 먼저 읽는다.
- Donut Theory 작업에서는 `mpc-software/docs/plugins/donut-theory/handoff.md`의 최신 상태를 먼저 확인한 뒤 `spec.md`, `design.md`, 승인 이미지와 실제 최신 캡처를 확인한다. 버전별 과거 기록을 현재 사양으로 되돌리지 않는다.
- 새 사용자 지시가 기존 결정을 바꾸면 관련 문서도 갱신한다. 확정 사항, 제안 목표, 미구현 상태를 구분한다.
- Donut Theory의 매핑·음색·도넛 디자인 등 전용 결정은 다른 플러그인에 자동 적용하지 않는다. 새 프로젝트는 별도 요구·제조사·UID·사양으로 관리하고 기존 Donut 프로젝트는 보존한다.

## 구조와 SDK

- 플러그인은 `mpc-software/plugins/public/<PluginName>/` 또는 `mpc-software/plugins/private/<PluginName>/`의 독립 프로젝트로 관리한다. 루트를 단일 플러그인으로 바꾸지 않는다.
- 공식 `inmusic-software/mpc-plugin-builder-sdk` 한 벌을 공유하고 도입 시 커밋을 고정한다. 출처와 라이선스를 기록한다.
- SDK를 프로젝트마다 복제하거나 임의 수정하지 않는다. 필요한 변경은 이유와 패치로 기록한다.
- 실제 공유 수요가 생길 때만 `mpc-software/shared/`로 코드를 추출한다.
- SDK 도입 후 해당 SDK의 AGENTS.md와 작업별 스킬을 읽는다. Donut Theory의 제조사/vendor는 사용자 확정값 `kim`, UID는 `Krql`이다. 다른 플러그인은 실제 스캐폴딩 전에 제조사명을 확인한다.

## UI·사양 보존

- 새 컨트롤의 자리를 만들려고 기존 컨트롤을 임의 삭제하지 않는다. BODY를 빼고 RESONANCE를 넣었던 접근을 반복하지 않는다.
- Q-Link는 4개씩 4그룹, 총 16개다. 최신 매핑은 spec.md를 따른다. 화면 순서와 Q-Link 순서를 맞춘다.
- 종류 선택기·관련 노브·현재 값은 근접 배치한다. DRIVE 종류는 DRIVE 바로 위에 둔다.
- 기본 매핑 밖의 기능과 상세 페이지로 이동하는 기능은 명세에 명시한다.
- 승인된 이미지의 미술 방향을 보존한다. 도넛을 단순 CSS 원·그라데이션으로 대체하지 않는다.
- 콘셉트 이미지는 작동하는 스킨이 아니다. SDK 영역과 실제 터치 크기에 맞춰 구현한다.
- 배포 스킨에는 TUI에서 참조하는 런타임 이미지만 포함한다. assets/의 고해상도 제작 원본·거절된 시안은 보존하되 설치 패키지에서는 제외한다. 서로 다른 위젯 정의가 수정 가능한 children 리스트를 공유하지 않도록 한다.
- 코드 hold 패드는 일반 Button+ResetOnGestureEnd를 사용하지 않는다. MPC 3.10.1.12에서 즉시 ON/OFF되는 것을 계측했다. Mouse Down의 raw Touch Toggle Switch와 캡션 위의 투명 연속 Slider ResetOnGestureEnd 조합을 유지한다. Button은 상태 그림만 담당하며 추가 Button hit 표면으로 덮지 않는다.
- MPC Live III 3.10.1.12에서는 보이는 Label이 아래 Button의 터치를 차단했다. 버튼 캡션 중앙을 직접 눌러 검증하고, 투명 Button 터치 표면은 캡션 위에 둔다. 이미지의 자동 축소도 가정하지 않는다.

## 오디오·호환성

- process()에서 할당, 잠금, 파일 I/O, 로그 출력을 하지 않는다. MIDI 오프셋, 가변 블록 크기, 노트 종료를 처리한다.
- 파라미터 변화를 스무딩한다. 배포 후 UID, 파라미터 ID와 순서를 안정적으로 유지하고 새 항목은 끝에 추가한다.
- SDK 호스트 계약을 확인하고 내부 파라미터 0은 부작용 없는 마스터 볼륨으로 둔다. 내부 순서와 Q-Link 순서는 별개다.
- 코드 트리거, 서스테인, 모드 변경에서 stuck note와 발음 수·릴리스 처리를 검증한다.
- 빌드, SDK 검증기, host simulator, WAV 청음 결과를 기록한다. 실기기 성능·터치 발음·하드웨어 호환성을 추정만으로 확정하지 않는다.
- 빌드만 성공한 것을 완성된 플러그인으로 보고하지 않는다.

## 문서·배포

- 새 플러그인 추가 시 프로젝트 README와 Pages 카탈로그·상세 페이지를 함께 관리한다. 사이트 미구현 시 필요한 항목을 기록한다.
- 공개 페이지와 릴리스에는 실제 검증 상태를 표시한다. 태그·링크 정책은 repository-policy.md를 따른다.
- 다른 플러그인의 릴리스로 연결될 수 있는 저장소 공통 releases/latest 링크를 사용하지 않는다.
- 사용자 요청 범위 밖에서 자동 push, Release 발행, Pages 공개를 하지 않는다. 규칙 문서 작성과 실제 배포를 구분한다.

- 물리 코드 패드 연결은 MPC Pad Perform 설정 변경을 요구하지 않는다. 플러그인의 개별 Pad Note/LEARN으로 맞춘다. MAIN에는 높이70px 코드 패드 네 개를 둔다. 코드 편집은 첫 번째 하단 탭을 다시 눌러 CHORD로 이동하며 한 화면에서4개 코드·보이싱을 편집하고 총16슬롯까지 제공한다. SDK에 없는 탭 이동을 안내 팝업으로 대체하지 않는다.

- 공장 음색은 Init/Bass/Keys/Synth/Lead/Pad, 카테고리→음색 탐색을 따른다. Init은 Low-cut/효과Off. 기존 뱅크는 사용자 지시로 전면 교체했으며 현재 기준은 mpc-software/docs/plugins/donut-theory/presets.md다. Low-cut과 음색용 Low-pass를 혼동하지 않는다. 사용자 확인값은 저역 정리용Low-cut이다. 카탈로그 생성기가 버전을 임의로 덮어쓰지 않도록 한다.

- 코드 편집은 첫 함수 키의 단일 CHORD 편집 서브탭에서 Q-LINK SCREEN으로 사용한다. A–D=각 화면의 네 슬롯, 순서는 Root/Type/Voicing/Octave다. MIDI Note는 Q-Link와 가이드 범위에서 제외하고 읽기용 값+더블탭 수동 지정/LEARN으로 제공한다. 팝업만 띄워 신스 Q-Link가 남는 구성을 반복하지 않는다. Screen Tab1/SubTab2와 MAIN Tab1/SubTab1을 구분하고 Program/Track의 신스 기본 매핑은 유지한다. SDK에 없는 탭 이동/동적 팝업 매핑 handler를 발명하지 않는다.

- 플러그인 초기 화면은 MAIN이다. 시작 도움말·탭 이동 안내 팝업을 띄우지 않는다. 모든 탭과 선택/설정 팝업에서 Q-Link 가이드 위치를 실제 기기로 확인한다. 팝업은 version4/hideQLinkBounds로 뒤쪽 탭의 가이드를 숨기고 닫으면 복원한다. 투명 전체화면 하위 패널은 ignoreMousePresses로 카테고리/DONE 등 형제 버튼 터치를 가로채지 않도록 한다.

- Voice Mode는 NOTES와 SPLIT의 일반 멜로디에 모두 적용한다. 코드 MIDI/터치 발음은 별도 다성 소유자로 처리하며, 모노 멜로디 전용 voice0을 코드가 빼앗지 않게 한다. fallback은 코드 트리거를 제외하고 note-off/CC64/CC121이 각 소유 음을 정상 해제하도록 검사한다. Legato는 실제로 음을 겹쳐 누를 때만 엔벌로프를 이어간다.
- MAIN의 C·FILTER / LFO는 정적 제목이다. 중복 LFO SETTINGS 팝업을 재도입하지 않는다. Shape/Target은 MAIN, Sync/Division은 MOD에서 조절한다.
- 코드 Voicing의 기존 저장값0=Root/1=First/2=Open을 유지한다.2·3·4전위는3·4·5로 추가했고 실제 구성음 수에 맞는 항목만 표시한다. ChordOctave1–4는 인덱스87–90에 append했으며 -2..+2oct, 저장 인덱스2가0oct다. CHORD의16 Q-Link는 Root/Type/Voicing/Octave다. OCT는 도넛 노브+값 터치 목록으로 제공하고 MIDI Note 도넛은 표시하지 않는다.

- 선택 목록은 현재 페이지를 유지하는 왼쪽426px 사이드바로 제공한다. 카테고리→음색 탐색, 긴 목록 페이지 이동, 선택 후 닫힘, X/바깥 터치 닫힘을 기기에서 검증한다. MPC 공통 상단 프리셋 UI는 호스트 소유다. 문서화된 MenuOverlay라도 현재 고정 SDK/기기에서 항목 글자가 비어 있었으므로 확인 없이 native 목록 성공으로 보고하지 않는다.
- PAD LEARN은 첫 물리 입력을 등록용으로 무음 소비하고 다음 입력부터 코드 연주한다. 등록 직후 같은 화면에서 MIDI Note·완료 문구·LEARN 해제를 확인한다. 엔진 내부 변경의 호스트 알림을 process()에서 보내지 않는다. 값 조회에서 알림을 배출할 때 렌더 스레드 호출을 제외하는 검사를 유지한다.

- MAIN 최신 그룹 순서는 화면 읽기 순서 A(좌상) Sound/Play, B(우상) Amp Envelope, C(좌하) Filter/LFO, D(우하) FX/Tremolo다. 각각 OSC Pair/Osc Mix/Body/Drift, Attack/Decay/Sustain/Release, Filter Cutoff/Resonance/LFO Rate/LFO Depth, Drive/Glide/Tremolo Rate/Tremolo Depth다. Screen/Program 매핑을 함께 맞추며 CHORD 매핑은 별도 보존한다. 숫자 LCD는24px 글자와 양쪽 내부 여백을 유지해 값·단위가 테두리에 꽉 차지 않게 한다.

- 코드 슬롯은 기본4개, 최대16개이며 CHORD의+/-로4개 단위 활성화한다. MAIN은222×70 패드4개와 페이지 넘김을 유지한다. 물리 MIDI/LEARN 매핑은 화면 페이지와 독립적이다. 슬롯 축소는 설정을 삭제하지 않는다. 페이지 이동 시 사라진 화면 hold는 해제하되 물리 MIDI 음은 유지한다. 활성 개수만큼 편집 화면 안의 묶음 버튼을 표시하고 MAIN과 페이지를 공유한다. 기존91개 파라미터의ID/순서는 유지하고 새슬롯/개수/페이지/명령/활동 값은 끝에 추가한다.

- 코드 프리셋은 MAIN에 보이는4슬롯의Root/Type만 변경하고 다른12슬롯 및 보이싱/옥타브/패드 매핑을 보존한다. 네뱅크 프리셋 선택은 독립 저장한다. MOD의 PRESET 1–4는 기존고정 Q-Link가 제어하는 첫뱅크 전용이며 동적값으로 위장하지 않는다. 단일step의가시성 조건에 RangedEnabling/N/N을 쓰지 말고 IndexedEnabling/N을 쓴다.

- 슬롯 페이지 때문에 MAIN 키를 여러 번 누르게 하지 않는다. MAIN↔CHORD 두 서브탭만 유지하고 CHORD 안에서 묶음을 선택한다. Q-Link는16개 고정 편집 중계 파라미터에 연결하고 선택한 실제 슬롯을 읽고 쓴다. 현재208개 파라미터 중 중계16개는automation 대상이 아니며, 마지막16개 PadPosition도 비자동화 호환값이다. 상태 복원 중 중계값으로 실제 코드를 덮어쓰지 않도록 보호한다. 오디오 스레드의 중계 readback은 별도 스레드 토큰+인덱스로 구분해 UI 동시 쓰기를 무시하지 않는다.

- 1.0.1에서 도입해 현재 유지하는 전면/Q-Link는 A=OSC Pair/Osc Mix/Body/Drift, D=Drive/Glide/Tremolo Rate/Tremolo Depth다. Space는 FX 상세에 유지하고 전면에서 제외한다. Glide는 AMP 상세에도 유지한다. B/C와 CHORD 매핑은 유지한다.

- [과거1.1.2 이력; 1.2.2에서 표시 UI 제거] 당시 물리 패드 위치는 수동 힌트다. 공통 CHORD 편집에서 A–H/1–16을 선택하고 기존 LEARN으로 MIDI를 연결한다. MAIN에 코드별 EDIT를 추가하지 않고 위치명+작은4×4만 표시한다. 미등록은 숨기고 자동 감지로 설명하지 않는다. PadPosition1–16은192–207에 append하며 Q-Link/automation에서 제외한다. 기존 MIDI 매핑과 위치 정보는 별개이며 실제 MPC 배치 변경을 자동 추적하지 않는다.

- 사용자 확정1.1.0 음색 재조정은 UI·모드 추가 없이 기존64개 이름/순서를 유지한다.63개 연주 음색은 따뜻한 듀얼OSC/Low-pass/Tape 성격으로 조율하고 Init은 기본 출발점으로 보존한다. 이 프리셋 작업을24dB Moog 래더 필터 구현으로 설명하지 않는다. 레벨 보정은 스테레오 양 채널 피크로 측정하고 실제 청감/기기 검증 여부를 구분한다.

- 1.1.1 코드영역의 이전/다음 화살표4개는44×44 터치영역을 보존한다. 코드프리셋행의44×44 버튼과슬롯페이지44×32표시/44×44hit를 구분하며 원본image와위젯실제크기를 맞춘다. 필터값·코드패드·다른화살표와hit가겹치지 않게 한다. 나머지UI제안은 사용자승인없이 완료로표시하지 않는다.

- 최신 사용자 결정: HOLD TO PLAY는 공통 안내도 표시하지 않는다. MAIN 패드에는 슬롯 번호와 코드 이름만 남긴다.

- 최신 사용자 확정1.2.0: 프리셋만의 Moog풍 조정에서 엔진 자체 재설계로 범위를 변경한다. 별도 모드/UI 없이 nonlinear mixer → four-stage ladder Low-pass → rounded VCA를 기본 적용하고 전체63음색을 재설계한다. 엔진 설계는 mpc-software/docs/plugins/donut-theory/engine.md를 따른다.208개 파라미터·UID·매핑은 보존하되 구버전과 동일 음색 복원을 주장하지 않는다. BP/HP/Notch는 기존SVF 확장으로 구분한다.

- 선택 사이드바를 만든 뒤 일반 위젯을 추가하더라도 모든 탭의 modal 루트를 형제 목록 맨 뒤에 유지한다. MAIN 코드 페이지 화살표가 목록 위에 떠 있던 실기기 버그를 재발시키지 않는다. 스킨 회귀 검사와 실제 목록 중앙/바깥 닫기에서 확인한다.

- 최신1.2.2에서는 사용자가 허용한 수동 Pad Location 표시/선택 UI를 제거했다. 실제 MIDI Note/LEARN은 유지한다. 호환용 PadPosition1–16(192–207)은 삭제·재정렬하지 않는다. CHORD의 숨은 선택 트리를 무한히 늘리지 말고 펼친 위젯 수2500 이하 회귀 검사를 유지한다. 원격 창 좌표 오류/지연을 플러그인 멈춤 재현으로 단정하지 않는다.

- 2026-11-15 사용자는 설치된1.2.2의 MAIN↔CHORD 직접 터치 전환을 “정상 전환됨”으로 확인했다. 최신 상태·배포 해시·검증 한계·다음 프로젝트 분리는 mpc-software/docs/plugins/donut-theory/handoff.md를 따른다.

## 공개·개인용 프로젝트 분리

- 공개 프로젝트는 `mpc-software/plugins/public/`, 개인용은 `mpc-software/plugins/private/`에 둔다. Donut Theory는 public이다.
- private의 안내 파일 외 프로젝트·자료는 Git에서 제외한다. 강제 추가하거나 Pages/공개 Release에 포함하지 않는다. 개인용 프로젝트에는 공개 소개 페이지를 만들지 않는다.
- 개인 사용과 외부 콘텐츠의 추출·변환·배포 권한을 구분한다. 프로젝트 생성 전 vendor/UID 확인 규칙은 동일하다.

## 2026-11-16 확정 구조

- 저장소/로컬 루트 이름은 `kimswork`, GitHub는 `Sunghyun23/kimswork`다. 브랜드 사이트는 루트 `site/`로 유지한다.
- MPC 플러그인/앱/확장팩/공유도구/SDK/개발문서는 `mpc-software/` 아래에 둔다. 루트 docs에는 저장소·사이트 운영 정책을 둔다.
- DonutTheory와 GrooveDice는 public 프로젝트다. GrooveDice 공개 소스/기본 배포는 자체 제작 Original Workshop만 포함한다.
- 확장팩은 `mpc-software/expansions/<product-slug>/{public,private}/<pack-slug>/`로 관리한다. Splat-to-Clat은 groove-dice/private/splat-to-clat이며 자료·패키지·연구·import 도구 모두 Git 제외다.
- GrooveDicePackBuilder는 `mpc-software/apps/public/GrooveDicePackBuilder/`의 독립 프로젝트이며 현재는 macOS/Windows 독립 앱이 구현되어 있다.
- UID/파라미터/릴리스 태그를 경로 변경 때문에 바꾸지 않는다. SDK는 mpc-software/third_party의 고정 한 벌만 사용한다.
- 공개 경로 이동과 GitHub 이름 변경은 새 제품 Release/웹 배포 허가를 의미하지 않는다.

- 2026-11-16 사용자 정식배포 승인: Groove Dice1.0.0, Pack Builder1.0.0 및 자체제작 Workshop Sessions 팩을 별도태그/다운로드로 관리한다. 사이트 Software 하위 Instruments/Apps/Expansions에 연결한다. 물리/최악CPU/Windows오디오기기 검증 한계를 숨기지 않는다.

- 2026-11-16 Crate Press는 public effect, vendor kim, UID K36Z다. 자체 spec/manual/validation 및 프로젝트 AGENTS를 따른다. 사용자가 검증 후 Release/웹 업데이트를 승인했다. Software 첫 목록과 카테고리 제품 카드에는 실제 소프트웨어 스크린샷 썸네일을 먼저 보여주고 Effects 카테고리를 별도로 유지한다. 미검증 물리/오디오/CPU를 완료로 주장하지 않는다.

- Donut Theory latest1.8.0 user-approved layout supersedes historical MAIN maps above: A OSC Pair/Mix/Detune/Drift; B Cutoff/Resonance/Body/Sub; C independent LFO1Rate/Depth/LFO2Rate/Depth; D Drive/Glide/MotionRate/Depth. ADSRs remain on existing AMP/FILTER detail. New Body uses an appended parameter; legacy Body2 stays inert for compatibility. Follow the project AGENTS and latest handoff for287parameter state/routing/UI checks.

- Donut Theory1.9.0 retains the1.8.0MAIN/DSP layout and appends two approved Soul Studies (66programs,288parameters). Old SoundPreset46range64 stays unchanged; SoundCurrent287 owns full selection. First287metadata remain exact. Sound changes preserve chord/key/MIDI/Strum/ChordGlide/master settings.

- 2026-11-18 official publication: Donut Theory1.10.0 (kim/Krql,291parameters,66programs) adds whole-sound RND with all3Sync ON and chord/master preservation. Groove Dice1.4.0 (kim/KpGy,676parameters) provides16RULES per part with unchanged first672metadata. Explicit product tags, ZIP/checksums, updated manuals and actual MAIN/RULES MPC captures are published on existing kimswork.app/Pages. Candidate-only statements are historical. See each design/device-version/publication.json for release IDs, source SHA and verification; do not infer new physical encoder/audio/CPU certification. Private expansions remain owner-only.

- Latest user-approved UI consistency rule: Groove Dice remains unchanged and is the action-layout reference. For Donut Theory/Crate Press, preserve original artwork, lettering, knobs/faders and their own identity; relocate plugin-owned global actions to a bottom footer. Do not redraw the products in a new common theme. Native MPC host header stays host-owned; local parameter selectors/Sync stay with relevant knobs. Current local candidates are Donut Theory1.10.1 and Crate Press0.2.4; public versions remain1.10.0/0.2.3 until separately approved release.

- 2026-11-18 footer publication complete: Donut Theory1.10.1 is stable; Crate Press0.2.4 stays public preview. Both original-art bottom-action layouts, explicit product-tag ZIP/checksums, updated manuals and real MPC XL screenshots are public on kimswork.app. Runtime matches internal-NVMe verification; asset downloads/hash checks and Pages succeeded. Groove Dice1.4.0 source/package/device remains unchanged. Earlier local-candidate/publication-pending statements are historical; see each device-version/publication.json. Do not infer new physical encoder/multitouch, hardware audio or CPU certification.

- Latest approved Donut Theory1.11.0 local UI candidate replaces FILTER/AMP tabs with ENV/PLAY: both ADSRs together with native curves/vertical faders; performance settings on PLAY. Existing MAIN/CHORD and291metadata/DSP remain unchanged. Both new pages use only three populated Screen Q-Link banks; D unassigned. Public release remains1.10.1. See project AGENTS/handoff and design/device-1.11.0 for actual device checks and limits.

- Latest Donut Theory1.12.0 approved local feature fills PLAY D with Amp Release (shared with ENV), Release Pitch Drop Amount and Time. First291metadata retained, two appended for293total. PLAY uses four groups in2×2; MAIN and its filter controls stay unchanged. Earlier1.11.0empty-D/unchanged-DSP notes are historical. Public remains1.10.1.


2026-11-18: User explicitly authorized official Donut Theory1.12.0 release and website publication. Earlier candidate-only statements are historical. ENV/PLAY and Release Pitch Drop are the current specification; preserve recorded physical/audio/CPU limits. Final publication details go in design/device-1.12.0/publication.json.

- Donut Theory1.12.0 publication complete: stable Release449932324, source949dcfbd, ZIP/SHA256 integrity verified anonymously, Pages44301107512success,21public/77responsive checks,7actual latest MPC screenshots. InternalNVMe173runtime hashes match. Earlier candidate/public1.10.1 statements are historical. Preserve recorded physical/audio/CPU limits; Groove Dice/Crate Press unchanged. See design/device-1.12.0/publication.json.

- Latest approved Donut Theory1.13.0 local candidate adds5section Dice locks (OSC/FILTER/AMP/MOD/FX) at293..297,298total. First293metadata and every Q-Link map stay exact. Locks persist across sound selection/save-load and affect Dice only; all-locked Dice must not clear held notes. MOD lock preserves all Sync states. Header icons use44px non-Q-Link touch targets and remain below modal roots. Public stays1.12.0 until separate release authorization. Groove Dice/Crate Press unchanged.


## 2026-11-18 공통 Dice 잠금 — 최신 구현 후보

이 결정은 과거 Donut Theory의 5개 헤더 잠금 후보를 대체한다. Donut Theory1.13.0 / Groove Dice1.5.0 로컬 후보이며, 별도 공개 승인 전 정식 버전은 각각1.12.0 /1.4.0이다.

- Dice가 변경하는 값에 개별 자물쇠를 제공한다. 같은 실제 값을 표시하는 페이지/릴레이는 잠금을 공유한다. 수동 터치·Q-Link·명시적 공장 프리셋 선택은 허용한다.
- Donut MAIN16개는 각 노브 왼쪽 위에44px 터치 잠금을 둔다. 다른 페이지의 실제 랜덤 대상도 잠근다. OSC Pair/Wave1/Wave2와 각 LFO의 Rate/Sync/Division은 각각 하나의 잠금을 공유한다. ENV와 PLAY의 Amp Release도 공유한다.
- Groove Dice는 기존 MAIN16개 잠금을 유지하며 SOUND는 실제 역할·레이어·페이지에 따라 잠금 대상을 연결한다. FX의 랜덤 대상과 필인 A–D의 실제 데이터 보호도 제공한다. 랜덤화하지 않는 사운드 파라미터에는 의미 없는 잠금을 추가하지 않는다.
- 기존 킷·패턴·파트·레이어 잠금은 개별 잠금보다 우선한다. UNLOCK ALL은 전체 페이지의 개별 잠금과 상위 보호를 해제하고, Dice 실행·음색/샘플/노트 변경은 하지 않는다.
- master/MIDI/routing 및 항상 보존하는 Donut 코드·키·패드·Pitch Drop에는 Dice 잠금을 만들지 않는다. Q-Link 할당은 바꾸지 않는다. 원래 미술과 글꼴을 보존하며 Crate Press는 변경하지 않는다.
- Donut first293 / Groove first676 파라미터 선언은 유지한다. 새 파라미터는 append하며 총353 /777개다. 공개 ZIP에는 개인 확장팩을 넣지 않는다.
- 자동 검사와 실제 MPC 터치/화면 검증은 분리해서 기록한다. 물리 Q-Link 회전·오디오·최악 CPU는 새 검증 없이 완료로 주장하지 않는다.


## 2026-11-18 Groove Dice1.5.1 — predictable Dice policy (supersedes1.5.0 common-lock candidate)

- Global ROLL DICE changes the MAIN16controls and regenerates the repeating main pattern, respecting MAIN locks and Pattern Lock. If Kit Lock is off it also chooses sounds within the selected pack/kit pool; Kit Lock preserves the kit. Random Amount0 leaves everything unchanged.
- All four recorded/edited fill banks and all seven master/kit FX parameter values are preserved by global Dice, including Whole Kit replacement. The previous Extra scope selector is removed from the interface; saved Scope values remain compatible but no longer choose global behavior.
- ROLL FILL is a separate bottom action in the fill editor: it varies only the selected A–D bank, uses Random Amount, respects legacy locked rhythm roles, and is blocked during active recording. ROLL FX is a separate bottom action on FX: it randomizes Drive/Dust/Room/Duck only. Both support UNDO and do not regenerate the main pattern or choose a new kit.
- Individual Dice padlocks are visible only beside MAIN16controls. Remove the1.5.0-added SOUND/FX/choke/fill locks and duplicated GROOVE timing locks; existing Kit/Pattern/Role/Layer protections and global UNLOCK ALL remain. Appended1.5.0 lock parameters are retained for state/ID compatibility and no longer restrict non-MAIN randomization.
- GROOVE remains the detailed timing/density/ghost/velocity/random-amount/fill-frequency page; this change does not redesign or delete it. Donut Theory remains unchanged with its approved all-page shared locks.
- First777declarations are unchanged; append RollFill777/RollFx778,779total, same kim/KpGy identity and existing Q-Link maps. This is a local installed candidate; public stable stays1.4.0 until a separate release request. Preserve all personal expansions and keep them out of public packages.


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

2026-11-19 SSD installation completed (supersedes SD-pending notes): user explicitly selected the removable drive named SSD for portable use. Official SDK add/export copied Donut Theory1.13.14, Groove Dice1.5.1 and Crate Press0.2.4 to SSD/VST2 Plugins. All470files matched SHA256 against the actual internal-NVMe originals (177/165/128). Groove Dice includes all four existing personal expansion packs; these were copied locally only and remain excluded from public releases. Internal originals were preserved. Current MPC mounts SSD noexec; execution from SSD was NOT verified and must not be claimed. On another MPC enable External Plugin Support and restart/rescan. Local operation report: mpc-software/plugins/public/GrooveDice/build/device-ssd-2026-11-19/verification.json (Git ignored). This is a portable SSD copy, not an SD-card installation.

2026-11-19 storage identity correction: user reports no SSD. Official SDK discover_drives reads actual /proc/mounts and df: MPC XL192.168.0.56 has /dev/nvme0n1p2 at/nvme and /dev/sda1 (~1TB) at/media/SSD. SSD is a volume name; the SDK internal/removable label is a mount-name-prefix heuristic, not proof of physical medium or external connection. Prior descriptions as a confirmed external SSD were overconfident. The470file copy/read-back is real, but physical storage type and intended MPC identity remain unconfirmed. No further device writes until user clarifies.

2026-11-19 final storage state (supersedes SSD copy, identity hold and SD-pending notes): user confirmed the same MPC and inserted a64GBcard, discovered as36C7-1ACA (/dev/sdb1 at/media/36C7-1ACA). Backed up all three old card folders, then installed Donut Theory1.13.14, Groove Dice1.5.1 and Crate Press0.2.4 from verified NVMe-derived originals. All470SD files matched SHA256, including four personal expansion packs. Only after this verification, removed exactly our three plugin folders from/media/SSD using the official SDK at explicit user request. Final SDK inventory: NVMe latest three intact; SD latest three present; SSD zero plugins. No device writes targeted NVMe. SD is mounted noexec and execution from it is NOT verified; do not equate file installation/hash verification with runtime certification. Old card backups remain local at/private/tmp/kimswork-sd-before-update. Current operation report: mpc-software/plugins/public/GrooveDice/build/device-sd-2026-11-19/verification.json (Git ignored). No new release or website publication is implied by this storage operation.


- 2026-11-19 approved local Donut Theory1.14.0 implementation supersedes historical MAIN/PLAY maps: shared POLY/MONO editors, explicit actual-mode toggle, selected OSC Wave/Pitch(Tune/Fine)/Level/Drift, selected LFO Rate/Depth/Delay/Target, separate oscillator tuning/levels/PW and local-only Undo. Six native keys POLY/MONO/OSC/ENV/MOD/FX; CHORD is POLY subtab2. First353declarations exact,388total. Public/installed1.13.14 unchanged; no device/Release/site update in this implementation. Follow the project's latest handoff/spec for detailed mapping, migration and verification limits.

- Local Donut Theory1.14.1 fixes Sync small-turn accumulation and adds15straight/dotted/triplet periods on each LFO/sharedMotion via appended388..390(391total). First388metadata stable; old five-choice ranges remain aliases. Preserve cursor residuals, shared rate locks and state/Dice/Undo behavior. Groove Dice audit covers its existing relative Q-Link/rhythm/FX paths with no engine/schema/version change; no Triplet rhythm was added to drums by this audit. Public/device/site unchanged.

- 2026-11-19 user authorized Donut Theory1.14.1 actual MPC QA/release/site update. Local-only status above is historical. Actual device discovered blank numeric values from nested Data-to-Data remapping; inherited native Value labels must have no self-alias. Front OSC/Tune-Fine/LFO/Mono-Legato pairs fit one146px knob column (71px per segment). Keep shared edit-only tabs/focus, explicit VOICE,15step Sync fractional cursor retention and Delay/Dice exclusions. Actual/device/public verification is in DonutTheory/design/device-1.14.1. Groove Dice1.6.0 and Crate Press0.2.4 remain separate and unchanged.


2026-11-19 Donut Theory1.14.1 publication complete: source0937a4f, stable tag donut-theory/v1.14.1 and anonymous ZIP/checksum digests verified. Pages run44456002969 succeeded;77responsive checks and17public route/image/download checks passed. Current actual MPC screenshots and detailed manual are live. Official SDK read-back matched255runtimefiles on each NVMe and SD36C7-1ACA; no SD-specific runtime-origin certification. Remote real-device UI checks are separate from physical encoders, hardware audio, worst CPU and full project recall, which were not newly certified. Groove Dice1.6.0 and Crate Press0.2.4 unchanged. Evidence: mpc-software/plugins/public/DonutTheory/design/device-1.14.1/publication.json and public-verification.json.

2026-11-19: User authorized official Donut Theory1.14.2 UI selector alignment release and site update. Previous local-candidate status is historical. Keep existing391metadata/Q-Link maps/engine and other products unchanged; publish current seven actual MPC captures and manual with the versioned package. Evidence and final outcome: DonutTheory/design/device-1.14.2/.

2026-11-19 Donut Theory1.14.2 publication complete: source4a45d88 and stable tag donut-theory/v1.14.2; anonymous ZIP/checksum digests verified. Pages44461789094 succeeded;77responsive checks and15public route/image/download checks passed. Seven actual MPC screenshots and the updated manual are live. NVMe/SD each have247verified runtimefiles from the preceding UI QA. Same391parameters/Q-Link maps and sound engine; Groove Dice1.6.0 and Crate Press0.2.4 unchanged. Earlier local-only status is historical. Limits and audit: DonutTheory/design/device-1.14.2/publication.json and public-verification.json.

2026-11-19 user authorized Open MPC registration. Donut Theory alone submitted as PR1030; portable ARM32/ARM64 archives and checksums were added to its existing1.14.2Release without altering the original SDK assets. Registry/min_os3.10.1/UID and runtime bytes validated; no Verified/tested.json claim. Upstream review is pending. Groove Dice/Crate Press were not submitted or relicensed. Reproducible integration/audit: mpc-software/catalog/open-mpc/. Do not execute Open MPC legacy installers/benchmarks on the device; official SDK mpc_remote.py remains the sole remote installation route.

2026-11-19 Open MPC registration complete: PR1030 merged as864411da, upstream CI and catalog build44517697971success. Public catalog confirms Donut Theory1.14.2, Listed tier, correct original ARM32 SHA256 and live individual page. Earlier approval/review-pending notes are historical. No Verified/tested.json claim; other plugins unchanged. Integration records: mpc-software/catalog/open-mpc/.

2026-11-19 user approved Groove Dice Open MPC release including Pack Builder guidance.1.6.1is docs/licensing/distribution only: engine/UI/782parameters/maps/core bank unchanged, native version10601. Explicit project/original-core/shared-expansion MIT notices exclude optional personal packs and Pack Builder dependencies. Bundle detailed PACK_BUILDER.md and link optional macOS Apple Silicon/Windows x64 app/manual on the website and catalog PR. Publish public-only SDK plus per-CPU Open MPC assets; no new hardware verification/Verified badge. Other plugins and installed owner expansions remain untouched.

2026-11-19 Groove Dice1.6.1release/site complete: release source4a89f6e, Pages44527014146success,91responsive and12public page/download checks passed. SDK and per-CPU archives include optional Pack Builder workflow/downloads and explicit MIT notices; only original core bank, empty expansion registry, no personal payloads. Pack Builder binaries remain1.0.0and separate; no companion-app catalog entry or dependency relicense was added. Open MPC PR1035 submitted (review/merge pending); integration records in mpc-software/catalog/open-mpc/. Device owner installations remain1.6.0with their own packs: this docs/licensing-only release did not modify the MPC or claim new hardware certification. Other plugins unchanged.

2026-11-19 user approved Crate Press Open MPC registration/release.0.2.5is a docs/license/distribution patch only: native205; retain engine/UI/380parameters/32programs/Q-Link maps. Add explicit MIT project/SDK notices, normal SDK plus per-CPU portable archives. Preserve public preview/prerelease status and old actual0.2.4image labels; no Verified/tested.json or new hardware certification. Device installations/other products unchanged; Pack Builder companion registration is a separate future task.

2026-11-19 Crate Press0.2.5public preview release/site complete: sourcee75e395, Pages44528905173success;91responsive and11public page/image/download checks passed. Native/ARM/SDK, full offline UBSan/engine/performance/28pairs/bridge/skin and3host configurations20PASS/0WARN/0FAIL. Explicit MIT notices included; SDK and per-CPU Open MPC artifacts preserve engine/UI/380parameters/32programs. PR1036submitted, Linux CI44529150797success, review/merge pending. Preview/prerelease:true; expected catalogExperimental, no Verified/tested.json. Current MPC installation remains0.2.4; no device update/restart or new hardware certification in this docs/distribution task. Other plugin/app versions unchanged; Pack Builder companion registration remains future work. Records: mpc-software/catalog/open-mpc/ and CratePress/design/release-0.2.5/.

2026-11-19 Open MPC PR1035review correction: all Groove Dice portable releases must declare user_data=[Content/Expansions] via --user-data Content/Expansions; never publish with an empty declaration. Use tools/package_open_mpc.py (public ZIP only), which validates both metadata copies/installers. Linux actual-installer regression must preserve nested packs/registry on upgrade/reinstall/uninstall and reproduce old1.6.1loss.1.6.2is packaging-only; keep782IDs/UI/corebank. Catalog/latest website screenshots must show only public128WAV/95MIDI; no owner-pack totals. Root README explains scoped MIT; private expansions and app dependencies remain separate. Preserve owner device installs by verified backup/restoration; do not execute Open MPC scripts on the actual device.

2026-11-19 Groove Dice1.6.2review fix complete: sourcee181142, stable SDK/per-CPU release; mandatory user_data=[Content/Expansions] and actual Linux installer loss/preservation regression. Pages44538201342success,91responsive/23public checks.11new actual public-only MPC XL128WAV/95MIDIcaptures published; original owner SD167files SHA256-restored, NVMe untouched. PR1035updated, CI44538239063success, response7011180549posted; re-review pending (formal reviewer assignment API404, request made in comment). Root README scoped MIT and summary UI scope included; old1.6.1assets unchanged with upgrade warning. No new physical/audio/CPU certification. See GrooveDice/design/device-1.6.2/ and catalog/open-mpc/.


2026-11-20 approved Groove Dice1.7.0: PERFORM four fills/four held snare rolls, selected four Screen Q-Links, LOOP/STOP pad learn, per-slot generation/locks, actual-note capture/SMF export. First782declarations exact,880total. Open-slot Dice regeneration is bar-synchronized and supersedes the old all-fill-preservation policy; legacy/recorded/edited fills auto-lock. Keep both Content/Expansions and Content/Captures as portable user_data; public content remains original core only. Final full regression/host19PASS/0WARN/0FAIL and both-CPU Linux installer preservation pass. Actual MPC XL3.10.1.12 remote touch and129balanced exported MIDI-note checks pass. NVMe205files and13original owner-data files SHA256-verified. Latest user prohibits SD reads/writes; SD untouched, other plugins unchanged. Physical pads/encoders, hardware audio/full MIDI import and worst-case device CPU are not newly certified. Release/site publication evidence follows in GrooveDice/design/device-1.7.0/.


2026-11-20 Groove Dice1.7.0publication complete: source1341055, stable tag groove-dice/v1.7.0, all six public SDK/per-CPU ZIP/checksum assets anonymously SHA256-verified. Pages44588325375success;91responsive and24public page/image/download checks pass. Twelve actual public-only MPC XL128WAV/95MIDIcaptures and detailed manuals are live. NVMe205runtime/ownerfiles match staged bytes;13original personal data files retained and exported test takes preserved. SD/SSD and other plugins untouched. Physical pad/Q-Link/hardware-audio/full MIDI import/worst CPU limits remain explicit. Earlier candidate/release-prepared notes are historical. Evidence: GrooveDice/design/device-1.7.0/publication.json.


2026-11-20 latest Groove Dice1.7.1user decisions (unreleased correction): PERFORM is first native MAIN/PERFORM subtab only; remove duplicate MAIN PERFORM entry/context, retain distinct EDIT FILL on MAIN with eight existing edit relays. Both fills and rolls finish at a bar boundary with full chosen Length. User explicitly chose next valid bar ending if the start window has passed; never truncate. Latch timing on trigger; preserve full recorded multi-bar phrases. Confirmed physicalB1=48fix and original user_data preservation remain. Source/current candidate changes and actual bar-end MIDI evidence are in GrooveDice/design/device-1.7.1/. Do not read/change SD, do not claim full physical-pad verification or publish until pending validation is resolved.


2026-11-20 owner confirmed final physicalB1/B3select ROLL A/C and complete at the bar ("잘됨"). The targeted confirmation gate is satisfied; finish the previously authorized stable1.7.1release/site update without asking again. Do not claim all8physical pads, encoder/audio/CPU coverage. Final tab-only/bar-end source/UI and actual26-note MIDI/public-only frames are verified; NVMe203files and all17before-timing persistent files retained. No more device restart is necessary for documentation/package publication. SD remains prohibited; Donut phase request is advice only and must not change Donut code.


2026-11-20 Groove Dice1.7.1publication complete: sourcebc69dca, stable tag groove-dice/v1.7.1, all six public assets anonymously SHA256verified. Pages44639127077success;91responsive and24public checks;12final actual public-only frames/manual live. Corrected default BankBmap and targeted physicalB1/B3confirmed; complete bar-end phrases defer late triggers, tab-only MAIN/PERFORM and distinct MAIN EDIT FILL. NVMe203files/all17before-timing persistent files verified; SD/SSD/other plugins untouched. Earlier candidate/pending gates are historical. Full8pad/encoder/audio/CPUlimits remain explicit. Donut phase/wave work is a separate new task. Evidence: GrooveDice/design/device-1.7.1/publication.json.


2026-11-20 user approved completion and publication of Donut Theory1.15.0 Phase/eight waves and Groove Dice1.8.0 musical performances/four hat rolls. Current specifications and validation are in each versioned device evidence folder. Historical no-release statements are superseded. NVMe only; SD reads/writes prohibited, SSD untouched. Preserve all19owner expansion/capture files; public-core QA uses a temporary load-isolation marker, never a destructive public-only replacement. No QA marker ships. Donut first391metadata/399total; Groove first880/917total. Other products unchanged.


2026-11-20 publication complete: Donut Theory1.15.0 and Groove Dice1.8.0 stable source13fa4f0. All12public SDK/per-CPU ZIP/checksum assets anonymously SHA256-verified. Pages44657208518succeeded;91responsive,293local references and46public page/image/download checks passed. Actual MPC XL screenshots/manuals are live. NVMe Donut250/Groove204files match final runtime bytes; all19owner expansion/capture files preserved. SD/SSD untouched. Both native/ARM/SDK/full regression+UBSan/host19PASS0WARN0FAIL. Donut66factory programs sample-identical to1.14.2at default phase; new8waves and Phase/FREE-RESET/locks/Undo checked remotely. Groove adds musical snare/fill templates and4finite hats; actual55balanced MIDI notes/44hat hits verified, plus queued/active performance count-in cancellation regression. Final restart/owner-profile reopening checked. New physical-pad/encoder, hardware-audio, full project recall and worst-case CPU coverage remain separate. Public-only destructive replacement was rejected automatically and did not run; safe QA retained every owner file and used a temporary optional-pack load marker, removed before final installation/package. Evidence in each versioned design/device directory. Crate Press0.2.5public/0.2.4device and Pack Builder1.0.0unchanged.


2026-11-20 user requested actual installation of the approved RETRIG/aligned Phase UI. Donut candidate1.15.1is installed only on internal NVMe:249files independently SHA256-verified. Same399parameter declarations and DSP behavior; only version11501and skin presentation change. Actual remote MPC XL/3.10.1.12 POLY/MONO, RETRIG Off-On-Off colors, conditional hiding in Drift and return toPOLY passed. Native/ARM32/ARM64/SDK, existing skin checks and host19PASS/0WARN/0FAIL passed. Public/site remain1.15.0; do not overwrite its release assets. Groove Dice/Crate Press, SD and SSD untouched. Evidence: DonutTheory/design/device-1.15.1/.


2026-11-20 shared action footer complete (local candidates only): Donut Theory1.15.2/Groove Dice1.8.1/Crate Press0.2.6 use UNLOCK ALL, ROLL DICE, UNDO at identical footer geometry on20pages including fill editor. Crate appends16parameters after exact original380:3commands/13locks; restrained13-control Dice preserves Input/Output/Mix, safety, LATCH and performance states. Service new commands immediately through the native setter even on a stopped empty audio track, with nonwaiting UI/render ownership and no render host callbacks. Actual MPC remote checks include Dice/Undo/locks, two latched effects retained and native32program browsing. All builds/SDK/regressions pass; Donut/Groove host19PASS, Crate20PASS. NVMe244/182/165files independently SHA256verified; all19owner Groove expansion/capture files retained. SD/SSD untouched. Public releases/site remain1.15.0/1.8.0/0.2.5preview; no push/publication authorized for this task. Evidence in respective device-version folders; no new physical encoder/pad, hardware-audio, worst CPU or full-project-recall certification.

2026-11-20 user authorized Crate Press0.2.6 public preview release and website update. Supersede local-only Crate footer notes; Donut1.15.2/Groove1.8.1remain local, not part of this release. Final lock silhouette/padding match Donut both states,44px hit; NVMe165files verified. Publish SDK plus both portable CPU packages and final real captures/manual, preserve preview and hardware limits. SD/SSD untouched.

2026-11-20 Crate Press0.2.6preview publication complete: source2b02d59/tag crate-press/v0.2.6, sixSDK/perCPU ZIP/checksum assets anonymously SHA256verified. Pages44673082720success;35responsive/295local reference/11public page-image checks pass. Final actual MPC captures and Dice manual are live. NVMe165files verified, final Donut-matched lock silhouettes with44px touch; stopped-track Dice/locks/Undo and two latched states checked. SD/SSD untouched. Donut1.15.0/Groove1.8.0public versions unchanged; local footer candidates remain pending. Preview/hardware limits remain explicit. Evidence:CratePress/design/device-0.2.6/publication.json.

2026-11-20 owner explicitly requested stable Crate Press promotion. Promote existing0.2.6GitHub Release to prerelease:false; keep all six public assets/digests and existing tag unchanged. Stable status is a distribution choice, not additional hardware certification. Update site/current manual/README; historical preview artifacts retain initial labels. No device reinstall or other product release is needed.

2026-11-20 Crate Press0.2.6stable promotion complete: GitHub prerelease:false/draft:false, sixassets/digests unchanged, existingtag/runtime preserved. Pages44674703497success from665939b;35responsive/295local reference/11publicpage-imagechecks pass, actual site badgeAvailable now and manualStable release confirmed. NVMe0.2.6needs no reinstall; hardware verification scope unchanged. Donut/Groove public and local candidate statuses unchanged. Evidence:CratePress/design/device-0.2.6/stable-promotion.json.

2026-11-20 user approved stable Groove Dice1.8.1 release/site update after final left-side transparent lock actualQA. Earlier local-only restrictions are historical forGroove. Keep917parameters/engine/maps/scopes unchanged; public-only actual captures use optional-pack load isolation while all19owner files stay retained. Remove marker before final owner install and public packaging. Preserve both portable user_data paths. Donut local candidate remains unpublished; Crate stable0.2.6unchanged. NVMe only; SD/SSD untouched.

2026-11-20 Groove Dice1.8.1stable publication complete: source4f500c0/tag groove-dice/v1.8.1, sixSDK/perCPU ZIP/checksum assets anonymously SHA256verified. Pages44682400399success;91responsive/295local reference/19publicpage-imagechecks pass. Twelve newactual public-core128WAV95MIDIframes/manual live; MAIN remains hero/thumbnail/firstcaption. Native/bothARM/SDK/fullUBSan/regression andhost19PASS0WARN0FAIL; bothCPU actual Linux installer preservation passes for Expansions+Captures. NVMe181files SDKverified, all19ownerfiles retained and normal11633WAV192MIDIprofile reopened; noQAmarker in final installation/packages. Engine917params/maps/scopes unchanged. SD/SSDuntouched; Donut1.15.0public/local1.15.2candidate andCrate0.2.6stable unchanged. Hardware limits remain explicit. Evidence:GrooveDice/design/device-1.8.1/publication.json.
