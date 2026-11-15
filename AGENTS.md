# Repository instructions

## 작업 시작

- 이 저장소는 Akai MPC용 플러그인 모노레포다. MCP(Model Context Protocol) 프로젝트가 아니다.
- README와 `docs/repository-policy.md`를 먼저 읽는다.
- Donut Theory 작업에서는 `docs/plugins/donut-theory/spec.md`, `design.md`, 승인 이미지를 확인한다.
- 새 사용자 지시가 기존 결정을 바꾸면 관련 문서도 갱신한다. 확정 사항, 제안 목표, 미구현 상태를 구분한다.

## 구조와 SDK

- 플러그인은 `plugins/<PluginName>/`의 독립 프로젝트로 관리한다. 루트를 단일 플러그인으로 바꾸지 않는다.
- 공식 `inmusic-software/mpc-plugin-builder-sdk` 한 벌을 공유하고 도입 시 커밋을 고정한다. 출처와 라이선스를 기록한다.
- SDK를 프로젝트마다 복제하거나 임의 수정하지 않는다. 필요한 변경은 이유와 패치로 기록한다.
- 실제 공유 수요가 생길 때만 `shared/`로 코드를 추출한다.
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

- 공장 음색은 Init/Bass/Keys/Synth/Lead/Pad, 카테고리→음색 탐색을 따른다. Init은 Low-cut/효과Off. 기존 뱅크는 사용자 지시로 전면 교체했으며 현재 기준은 docs/plugins/donut-theory/presets.md다. Low-cut과 음색용 Low-pass를 혼동하지 않는다. 사용자 확인값은 저역 정리용Low-cut이다. 카탈로그 생성기가 버전을 임의로 덮어쓰지 않도록 한다.

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

- 슬롯 페이지 때문에 MAIN 키를 여러 번 누르게 하지 않는다. MAIN↔CHORD 두 서브탭만 유지하고 CHORD 안에서 묶음을 선택한다. Q-Link는16개 고정 편집 중계 파라미터에 연결하고 선택한 실제 슬롯을 읽고 쓴다. 실제192개 파라미터 중 중계16개는automation 대상이 아니다. 상태 복원 중 중계값으로 실제 코드를 덮어쓰지 않도록 보호한다. 오디오 스레드의 중계 readback은 별도 스레드 토큰+인덱스로 구분해 UI 동시 쓰기를 무시하지 않는다.

- 최신1.0.1 전면/Q-Link는 A=OSC Pair/Osc Mix/Body/Drift, D=Drive/Glide/Tremolo Rate/Tremolo Depth다. Space는 FX 상세에 유지하고 전면에서 제외한다. Glide는 AMP 상세에도 유지한다. B/C와 CHORD 매핑은 유지한다.

- 로컬1.1.2의 물리 패드 위치는 수동 힌트다. 공통 CHORD 편집에서 A–H/1–16을 선택하고 기존 LEARN으로 MIDI를 연결한다. MAIN에 코드별 EDIT를 추가하지 않고 위치명+작은4×4만 표시한다. 미등록은 숨기고 자동 감지로 설명하지 않는다. PadPosition1–16은192–207에 append하며 Q-Link/automation에서 제외한다. 기존 MIDI 매핑과 위치 정보는 별개이며 실제 MPC 배치 변경을 자동 추적하지 않는다.

- 사용자 확정1.1.0 음색 재조정은 UI·모드 추가 없이 기존64개 이름/순서를 유지한다.63개 연주 음색은 따뜻한 듀얼OSC/Low-pass/Tape 성격으로 조율하고 Init은 기본 출발점으로 보존한다. 이 프리셋 작업을24dB Moog 래더 필터 구현으로 설명하지 않는다. 레벨 보정은 스테레오 양 채널 피크로 측정하고 실제 청감/기기 검증 여부를 구분한다.

- 1.1.1 코드영역의 이전/다음 화살표4개는44×44 터치영역을 보존한다. 코드프리셋행의44×44 버튼과슬롯페이지44×32표시/44×44hit를 구분하며 원본image와위젯실제크기를 맞춘다. 필터값·코드패드·다른화살표와hit가겹치지 않게 한다. 나머지UI제안은 사용자승인없이 완료로표시하지 않는다.

- 최신 사용자 결정: HOLD TO PLAY는 공통 안내도 표시하지 않는다. MAIN 패드에는 슬롯 번호와 코드 이름만 남긴다.

- 최신 사용자 확정1.2.0: 프리셋만의 Moog풍 조정에서 엔진 자체 재설계로 범위를 변경한다. 별도 모드/UI 없이 nonlinear mixer → four-stage ladder Low-pass → rounded VCA를 기본 적용하고 전체63음색을 재설계한다. 엔진 설계는 docs/plugins/donut-theory/engine.md를 따른다.208개 파라미터·UID·매핑은 보존하되 구버전과 동일 음색 복원을 주장하지 않는다. BP/HP/Notch는 기존SVF 확장으로 구분한다.

- 선택 사이드바를 만든 뒤 일반 위젯을 추가하더라도 모든 탭의 modal 루트를 형제 목록 맨 뒤에 유지한다. MAIN 코드 페이지 화살표가 목록 위에 떠 있던 실기기 버그를 재발시키지 않는다. 스킨 회귀 검사와 실제 목록 중앙/바깥 닫기에서 확인한다.
