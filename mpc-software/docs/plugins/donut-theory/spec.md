# Donut Theory — 제품 및 구현 사양

상태: **1.2.2 정식 배포·웹 갱신 완료(2026-11-15)**. 사용자 직접 MAIN↔CHORD 터치 정상 전환 확인. [인계·재개 기준](handoff.md)을 먼저 읽는다. 수동 패드 위치 UI 제거 및 CHORD 경량화 후 실기기 원격 반복 전환/보이싱/16슬롯 확인. 1.2.0 엔진 실기기64음색 발음/CPU 관찰과1.2.1 UI·음색 저장 복원을 확인했다. 물리 입력 전체·최악 부하·프로젝트 복원은 미인증. 아래 개발 이력보다 최신 버전 항목을 우선한다. 현재 동작과 남은 항목은 [구현 기록](implementation.md), [프로젝트 README](../../../plugins/public/DonutTheory/README.md)를 우선 참고한다. 이 문서에는 장기 목표도 포함된다.

## 제품 정체성

- 이름: **Donut Theory**. Pocket Butter, Crate Soul 등 과거 이름은 후보이며 사용하지 않는다.
- Akai MPC standalone용 악기 플러그인. 베이스·리드·코드·키보드·패드를 다루는 종합 신스다.
- 따뜻한 소울·힙합 음색, J Dilla에게서 연상되는 느슨한 연주감과 레코드 작업실 분위기를 지향한다.
- 특정 곡·기기 소리의 정확한 복각이나 공식 아티스트 제품이라고 표방하지 않는다.
- 소스 경로 계획: `mpc-software/plugins/public/DonutTheory/`, slug: `donut-theory`.
- 제조사/vendor: `kim` (사용자 확정). 영구 UID: `Krql`. 실기기 QA 대상은 MPC Live III / OS 3.10.1.12다.

## 음원 엔진 방향

- 오실레이터 2개, 사인 서브, 노이즈. 사인·삼각·톱니·펄스, 펄스 폭 및 디튠.
- 합성 기반 일렉트릭 키·벨·플럭을 위한 약한 FM/위상 변조. 샘플 기반 피아노나 Rhodes 정밀 복각은 첫 범위가 아니다.
- 필터: Low-pass / Band-pass / High-pass / Notch 선택, 컷오프·공진·키 트래킹 및 필터 ADSR.
- 앰프 ADSR, 벨로시티 반응, 글라이드, 피치 벤드, 모듈레이션 휠, 서스테인.
- Mono / Legato / Poly, 총16보이스. NOTES/SPLIT 멜로디에 모드 적용, 코드는 별도 다성 소유자다. 실기기 중첩 릴리스 CPU41%를 관찰했으며 최악 부하 보장은 아니다.
- 1.2.0 이후 Low-pass는 프로젝트 원본 비선형4단 래더(점근24dB/oct), 내부2x 처리/FIR/동시해석이다. BP/HP/Notch는 SDK12dB SVF 확장이다. 정확한 컷오프 정의·공진·호환성은 [engine.md](engine.md)를 따른다.
- 느린 드리프트, 코러스, 템포 딜레이, 새추레이션/디스토션.
- 실시간 할당 없이 MIDI sample offset에 맞춰 처리하고 파라미터 변화는 스무딩한다.

## 전면 Q-Link — 최신 확정 매핑

4개씩 4그룹, 총 16개. 내부 파라미터 인덱스와 별도로 매핑한다.

| 그룹 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A · SOUND / PLAY | OSC PAIR | OSC MIX | BODY | DRIFT |
| B · AMP ENVELOPE | ATTACK | DECAY | SUSTAIN | RELEASE |
| C · FILTER / LFO | FILTER CUTOFF | RESONANCE | LFO RATE | LFO DEPTH |
| D · FX / TREMOLO | DRIVE | GLIDE | TREMOLO RATE | TREMOLO DEPTH |

- BODY는 반드시 유지한다. FILTER CUTOFF/RESONANCE를 넣기 위해 BODY를 삭제하지 않는다.
- 컷오프 이름은 `FILTER CUTOFF`로 표시하고 Hz/kHz 값을 보인다.
- BEND는 메인 Q-Link에서 제외하고 상세 PLAY 설정에 둔다. 피치 벤드 기능을 삭제하는 것이 아니다.
- OSC PAIR는 두 파형 조합을 선택한다. 전면 OSC1/OSC2 직접 선택기는 제거한다. 기존 OSC 상세 페이지와 파라미터는 호환성을 위해 유지한다. 필터 종류와 LFO Shape/Target은 C 그룹의 터치 선택기다.
- 왜곡 종류는 D 그룹 첫 번째 DRIVE 노브 바로 위의 선택기다. 선택값 Tape/Tube/Fuzz를 표시한다.
- 최신 사용자 승인으로 DRIVE는 D 그룹 첫 번째에 둔다. GLIDE는 D 그룹 두 번째다. C의 필터/LFO와 D의 FX/Tremolo 사이에 구분선을 둔다.

## 매크로와 모듈레이션 역할

| 컨트롤 | 역할 |
|---|---|
| TONE | 전체 음색 성격을 바꾸는 매크로. 직접 컷오프와 구분해 배음·음색 변화의 범위 설계 |
| BODY | 서브·오실레이터 믹스 등을 통한 두께. DRIVE와 독립적으로 설계 |
| MOTION | 느린 불규칙 드리프트·코러스 움직임 |
| SPACE | 코러스·딜레이·스테레오 폭을 통한 공간감 |
| DRIVE | 선택한 왜곡 모델의 강도 |
| GLIDE | 음 사이 이동 시간, 0은 Off. 짧은 구간을 세밀하게 조절 |
| VIBRATO | 전용 규칙적인 피치 비브라토 |
| EXPRESSION | 벨로시티에 따른 음량·밝기 반응 |
| LFO | 위 매크로와 별도로 설정하는 모듈레이터 |

- LFO 파형 후보: Sine, Triangle, Square, Sample & Hold. 타겟: Pitch, Filter, Pulse Width, Amp.
- 템포 동기화·세부 라우팅은 상세 MOD 화면에 둔다. Rate에 Hz 또는 음표 분할을 명확히 표시한다.
- 매크로 이름·위치는 프리셋 사이에 유지한다. 깊이·범위·커브는 프리셋에 맞춰 조율한다.
- 세부 파라미터 기본값과 매크로 변조를 분리한다. 매크로가 저장된 기본값을 덮어쓰지 않게 한다.
- TONE과 FILTER CUTOFF 등 중복 영향이 생기는 매핑은 유효값 표시와 변조 범위를 설계하고 문서화한다.
- 매크로 상태와 필요한 매핑을 프리셋/프로젝트에 저장한다.

## 드라이브

- Tape: 부드러운 포화와 압축감.
- Tube: 배음과 두께.
- Fuzz: 거친 왜곡.
- 이 명칭은 목표 음색이며 회로·테이프의 정밀 모델링이 구현되었다는 뜻이 아니다.
- 출력 보정, Dry/Wet 및 상세 설정은 FX 페이지에 둔다.
- BODY에 왜곡량을 숨겨 DRIVE와 중복되지 않게 한다. 급격한 레벨 증가와 과도한 클리핑을 검증한다.

## 입력과 연주 모드

입력 모드와 발음 모드는 다른 개념이다.

| 입력 모드 | 동작 |
|---|---|
| NOTES | MIDI 노트를 그대로 연주 |
| CHORDS | 지정 트리거 노트 4개가 코드 슬롯 1~4를 연주 |
| SPLIT | 낮은 지정 노트 4개는 코드, 나머지는 멜로디 |

- 발음 모드는 Mono / Legato / Poly로 별도 선택한다. 코드 확장은 다성부 처리와의 상호작용을 정의해야 한다.
- MIDI 키보드와 MPC 물리 패드 모두 지원한다. 플러그인은 물리 패드 번호가 아닌 MIDI 노트를 받는다.
- 권장 물리 배치는 맨 아래 패드 1~4에 코드 1~4. 실제 송신 노트는 기기/프로그램 설정을 확인해 매핑한다.
- 키보드에서도 같은 트리거 노트를 누르면 같은 코드가 나온다. 옥타브 이름 관례가 다를 수 있으므로 실제 MIDI 번호를 함께 기록한다.
- SPLIT의 코드와 멜로디는 첫 버전에서 같은 음색을 쓴다. 멀티팀버럴 악기로 소개하지 않는다.
- 겹쳐 누를 때만 Glide 적용하는 옵션과 레가토 재트리거 정책을 상세 설정에 둔다.
- NOTES/CHORDS/SPLIT 및 Glide 적용 옵션은 기능 사양에 포함되지만 승인 이미지에는 아직 보이지 않는다. 실제 UI 구현 때 접근 경로를 마련한다.

## 코드 패드와 진행

- 서로 다른 코드 4개를 독립 슬롯에 저장한다. 하나의 코드 종류를 모든 패드에서 이조하는 기능만으로 대체하지 않는다.
- 코드 진행 프리셋 하나를 선택하면 슬롯 4개가 함께 변경된다.
- 슬롯마다 루트, 코드 종류, 보이싱을 편집할 수 있게 한다. Minor/Major 7·9, Sus 계열 등으로 시작한다.
- 코드 화면 예시: Dm9 / G13 / Cmaj9 / Am9.
- 베이스와 멜로디 음역을 고려해 보이싱을 설계한다. 코드 기호와 실제 구성음이 일치해야 한다.
- SPREAD는 구성음 사이 발음 시간차다. 상세 코드 편집에 둔다.
- 화면 코드 버튼은 누르는 동안 연주하고 손을 떼면 Amp Release로 감쇠한다. STOP은 모든 터치 코드 게이트를 끄며 EDIT 1–4는 별도 루트 편집이다. 물리 패드 코드 연주도 유지한다.
- 실제 MIDI 코드 연주와 화면 클릭 시안을 혼동하지 않는다.
- 코드 중첩·릴리스·서스테인·트리거 재입력·프리셋 변경·모드 변경에서 음이 남지 않게 설계한다.
- 5음 코드 두 개만 겹쳐도 8보이스를 초과할 수 있다. 보이스 수와 stealing 정책을 검증한다.
- MPC 자체 코드 기능과 이중 확장되지 않도록 NOTES를 기본 시작점으로 고려한다.

## 프리셋

사용자는 많은 프리셋과 빠른 전환을 요구했다. 다음 수량은 제안된 출시 목표이며 제작/검증 완료가 아니다.

| 음색 분류 | 목표 |
|---|---:|
| Bass | 12 |
| Lead | 12 |
| Keys | 16 |
| Chords / Pads | 16 |
| Texture | 8 |
| 합계 | 64 |

코드 진행은 32개 목표: 따뜻한 소울, 어두운 마이너, 재즈 확장 코드, 열린 코드 각 8개. 구체적인 수량과 내용은 제작 시 확정한다.

- 음색 프리셋과 코드 진행 프리셋은 독립 선택한다.
- 둘 다 `이전 화살표 / 이름 / 다음 화살표`와 이름을 통한 목록 선택을 지원한다. 카테고리 탐색도 제공한다.
- 코드 진행 선택이 음색을 바꾸거나 음색 선택이 코드 진행을 덮어쓰지 않게 한다.
- 각 음색의 출력 레벨·벨로시티·매크로 범위를 조율하고 실제 청음한다. 이름만 다른 중복으로 수량을 채우지 않는다.
- 기존 HTML 시안에는 음색 이름 4개, 코드 진행 6개만 있었고 발음 기능은 없었다. 64/32개가 이미 구현되었다고 쓰지 않는다.

## 상세 화면

MAIN / OSC / FILTER / AMP / MOD / FX를 기본 탭 구조로 한다. PLAY 설정은 AMP 또는 적절한 하위 화면에서 접근하도록 구현 시 정리한다. 코드 편집의 진입 동작도 실제 SDK 지원에 맞춰 정한다. 승인 이미지의 탭은 디자인이며 아직 작동하지 않는다.

## 구현 시작 전 결정 및 검증

- 제조사 `kim`과 UID `Krql`은 결정됨. 실기기 MPC Live III / OS 3.10.1.12를 확인했다. 외장 키보드 검증은 남아 있다.
- SDK 버전 고정, 오실레이터/FM/필터 알고리즘, 보이스 수와 코드 stealing 정책.
- NOTES/CHORDS/SPLIT 선택 및 상세 코드 편집의 실제 UI 경로.
- 터치 발음은 0.1.4에 구현했다. MIDI 노트 매핑과 물리 연주/청음, 프리셋 변경 중 held note의 실기기 검증을 이어간다.
- 매크로 유효값 표시, 단위와 범위, 공진·왜곡·FM 안정성, 패치 저장/복원.
- native/ARM 빌드, host simulator, 청음, 실기기 CPU와 UI 검증.

## 추가 확정 (2026-11-13)

코드 진행 프리셋 선택기 바로 위에 **CHORD PRESET** 라벨을 표시한다. 상단 SOUND PRESET과 구분한다.

## 추가 확정 및 구현 (2026-11-14, 0.1.4)

사용자는 **빈티지 아날로그 리드·패드 중심** 음색과 **화면 터치·물리 패드 모두 코드 연주**를 요청했다. 앞선 터치 발음 미확정 항목은 이 결정으로 대체한다. SDK에 검증된 release 이벤트가 없어 화면은 탭 토글 방식이며 물리 MIDI는 note-on/off 방식이다. 화면 조작은 MIDI 출력이 아닌 내부 발음이다.

64개 중 Lead 12/Pad 16의 개별 설정을 재작성했다. 총수와 순서는 유지한다. FX 페이지에 Custom 및 7개 적용 레시피를 제공하고 MAIN에 Tape/Chorus/Delay와 FX 위치를 안내한다. 기존 16 Q-Link는 유지한다. 음색의 음악적 청음 승인은 아직 아니다.

## 0.1.5 승인 변경 — 2026-11-14

최신 승인 이미지: `design/main-donut-approved.png`. 4×4 매핑은 위 최신 표를 따른다. 과거 이미지·문서의 DRIVE 첫 위치와 ADSR 터치 전용 제안보다 이 결정을 우선한다. Tone/Motion은 상세 페이지, Vibrato/Expression은 전면 터치 및 상세 페이지, LFO Shape/Target/Sync/Division은 전면 터치 선택으로 유지한다. 연속 회전 컨트롤 12개는 초콜릿 도넛 노브, AMP ADSR 4개는 금속 페이더다.

독립 Tremolo는 sine 기반 진폭 변조, 0.1–15 Hz / 0–100% Depth. 기존 LFO 타겟과 동시에 사용한다. Depth 0은 bypass이며 Drive 다음, Chorus/Delay 앞에서 적용하므로 FX tail은 자연스럽게 남는다. 음색 프리셋 선택 시 Tremolo는 4 Hz / 0%로 초기화한다. 상태 저장은 사용자가 설정한 값을 보존한다.

OSC Pair는 4×4=16개 순서 있는 조합이며 파형만 변경한다. Mix/Detune/Filter/ADSR은 보존한다. 기존 Wave1/Wave2를 호스트나 상세 UI에서 바꾸면 Pair 표시도 갱신한다. 새 파라미터 TremRate/TremDepth/OscPair는 기존 74개 뒤에 추가한다.

코드 패드의 사용자 요구는 **누르는 동안 발음, 손을 떼면 Note Off 후 Amp Release로 감쇠**다. 토글 연주 지침을 대체한다. `ResetOnGestureEnd` 핸들 modifier로 구현하며 실제 기기의 해제 이벤트 검증은 구현 기록에 구분한다.

Q-Link 물리 매핑: Column 뱅크 A=(13,9,5,1), B=(14,10,6,2), C=(15,11,7,3), D=(16,12,8,4). 화면 내 그룹은 가로 배열이지만 하드웨어 번호를 1–4씩 연속 배정하지 않는다. Screen/Program 모두 같은 열 순서를 사용한다.

## 코드 편집 접근성 및 물리 패드 설정 — 2026-11-14

MAIN의 EDIT 1–4는 해당 슬롯의 루트·코드 종류 11개·보이싱 3개(Root Position / First Inversion / Open Voicing)를 한 화면에서 편집한다. 기존 Root/Quality/Voicing 파라미터를 사용하고 터치·MIDI 코드 연주가 같은 슬롯을 참조한다. 편집 화면 하단에서 Input Mode, Trigger Base(MIDI BASE), Spread를 함께 설정한다. MIDI Base부터 연속 4개 노트가 슬롯 1–4를 연주한다. Notes는 단음, Chords는 지정된 네 노트만 코드, Split은 지정 노트 코드와 나머지 단음이다. 플러그인이 물리 패드 번호를 직접 수신한다고 설명하지 않는다. 실제 MPC Pad Perform/노트 설정에 따라 송신 노트가 달라질 수 있으므로 실기기 확인이 필요하다.

### 2026-11-14 후속 변경: 통합 코드 편집 및 패드 노트

사용자는 CHROMATIC/C1에서 물리 코드 발음을 확인했으나 호스트 패드 설정을 바꾸는 해결을 거절했다. MPC를 원래 NOTES / C Major / C0로 복원했다. 새 인스턴스는 Split과 개별 Pad Notes(24/26/28/29)를 기본으로 하며, 다른 호스트 배치는 각 슬롯 LEARN으로 등록한다. 기존 77-파라미터 상태는 연속 Trigger Base 라우팅을 유지한다. 새 파라미터 77–82를 끝에 추가했고 기존 ID/Q-Link는 그대로다. 등록 입력은 무음으로 소비하며 중복 노트는 두 슬롯을 교환한다.

앞선 EDIT 1–4 설계는 사용자 최신 요청에 따라 폐기하고 EDIT ALL 하나로 대체했다. 네 행에 Root/Type/Voicing/MIDI Note/LEARN을 동시에 표시한다. MAIN의 코드 패드/캡션/hold 터치 표면을 모두 195×70으로 확대했다. A 보조 컨트롤을 노브 위로 옮겨 값 표시 겹침을 제거하고 A/B 노브 높이, 패널 좌우 18px 여백, 하단 8px 안전 여백을 맞췄다.

0.1.6 native/ARM32/ARM64 빌드와 host_sim 19 PASS, 엔진/UBSan/스킨 검사를 통과했다. 현재 작업은 Qapads라는 새 QA 사본으로 저장 후 0.1.6 설치·재시작했다. 시작 화면의 원격 터치가 반응하지 않아 사용자에게 QA 프로젝트를 직접 열도록 요청했다. 정렬 후속 수정은 로컬 스킨까지 생성했으며, 실제 기기 표시와 통합 편집/새 MIDI 매핑 확인 전에는 완료로 간주하지 않는다.

### DRIFT 확정 변경 — 2026-11-14

사용자 승인: MAIN/Q-Link A를 OSC Pair / Body / DRIFT / Glide로 변경하고 Detune은 OSC 상세 페이지에 유지한다. DRIFT는 Wow+Flutter 피치 변조이며 Tremolo(음량 변조)와 별개다. 0.1.7에서 append-only 인덱스83, 기본0%, 제곱 강도 곡선, 느린0.43/0.137Hz와 빠른6.73Hz, 최대43 cents로 구현했다. 기존 프로젝트는0%, 새 상태는 저장값을 복원한다. SDK 공유 소스는 수정하지 않았다. 현재 기기 적용/실화면 확인 여부는 검증 기록을 따른다.

### 0.1.8 구현 기준: 확정 헤더

최근 사용자 승인을 반영한 현재 디자인은 갈색94px 헤더/크림 로고·라벨, 우측 끝까지 채우는318×126 도넛·레코드 그림, 프리셋240px와Voice/Input104px, 전체 폭을 채운 둥근 섹션 제목 바다. 선택기 뒤 예전 불투명 배경을 제거한다. 이전 소형 스티커/600px 배경/찢어진 외곽 시안은 채택하지 않는다. DSP·84개 파라미터 및16개 Q-Link 순서는0.1.7과 같다.


### 0.1.10 사용성 확정
MAIN STOP 제거, 상세 ADVANCED에 모든 입력용 ALL NOTES OFF 제공. EDIT CHORDS 한 화면에서 네 코드/보이싱/미리듣기/Pad Note/LEARN 편집. 코드 패드222×70, 슬롯번호와 HOLD TO PLAY 표시. ROOT는 단일 루트, 미리듣기 패드는 완성된 코드 이름. Pad Note는 옥타브+MIDI 번호. 호환 라우팅은 ADVANCED로 이동. LFO Sync/Division은 LFO SETTINGS, Shape/Target은 전면 유지. 독립 Tremolo와 구분한다. 기존16 Q-Link와84개 인덱스 보존, LearnStatus84번 추가. 자세한 검증 상태는 implementation.md를 따른다.


### 0.1.11 공장 음색 전면 교체

사용자 요청으로 기존64개 공장 음색을 제거하고 Init1/Bass12/Keys15/Synth12/Lead12/Pad12의 새64개 레시피로 교체했다. Lowfilter는 사용자 확인에 따라 Low-cut(불필요한 저역 정리)이다. 전면/OSC의 SOUND PRESET을 카테고리 먼저 선택하는2열 브라우저로 교체했다. MPC 공통 상단 팝업은 SDK에서 직접 변경할 수 없는 호스트 UI이므로 category/name 내장 프로그램 목록을 제공한다.

LowCut85번, OutputTrim86번을 append했다. Init은 단일Saw, Low-cut0, 공간/왜곡/변조Off. 나머지 음색은 음역별12dB/oct 스테레오HPF를 사용하고 FX에서 조절/해제 가능하다. 기존DC 블로커의 약35Hz 감쇠가 베이스와 충돌하므로 약5Hz로 낮췄다. 기존 상태에는 새LowCut/OutputTrim을0으로 복원한다. 입력모드/코드 진행/Pad Note/마스터 볼륨은 음색 전환과 독립적이다.

명시적 파형/엔벌로프/FM/필터/레벨 레시피와 카테고리별 목적은 presets.md와 tools/sound_bank.py에 기록했다. 기준 연주에서 Bass/Lead 피크−15.1~−15.0dBFS, Keys/Synth/Pad−12.2~−11.9dBFS. 짧은 플럭은 평균음량을 강제 정규화하지 않는다.64개 각각2초hold/3초release 오프라인 렌더, 비무음/NaN·Inf/범위 검사 및 low-cut 보존/감쇠 회귀 통과. 음악적 청감 최종 승인을 의미하지 않는다.

카탈로그 생성기가 plugin.json 버전을0.1.9로 덮어쓰는 기존 문제를 발견하여 제거했다. 선언 버전과 바이너리 metadata 일치 검사도 추가했다. Native/ARM 빌드, SDK validator, 엔진/UBSan/VST bridge, host_sim19 PASS/0 warnings/0 failures, 카테고리/Init/필터/UI 검사 통과. 전체 기록은 presets.md.

Mac 잠금 때문에 실제MPC 카테고리 터치/청감 검증은 남아 있다. 공식 SDK update로0.1.11을 설치 폴더에 전송하되 프로젝트 상태를 볼 수 없어 재시작하지 않는다. 실행 중인 인스턴스는 이전 버전이다. 공개push/Release 없음.


### 0.1.12 코드 편집 Q-Link — 2026-11-14

사용자는 A–D=코드1–4, 각 그룹 Root/Type/Voicing/Pad Note 순서의 Q-Link 편집을 승인했다. SDK UI 계약의 Q-Links.json은 실제 탭/서브탭별 정적 매핑이며 팝업 가시성에 따라 매핑을 바꾸는 API가 없다. 기존 MAIN 팝업 위에 코드 컨트롤만 올려두고 MAIN 신스 매핑을 유지하는 잘못된 구성을 피하기 위해 실제 CHORD 서브탭으로 편집기를 옮겼다.

진입: 하단 MAIN을 한 번 더 누르면 CHORD. 같은 첫 번째 탭을 다시 누르면 MAIN으로 돌아온다. EDIT CHORDS 버튼은 이 이동 방법을 안내한다. 이전 AMP의 구형 CHORD 서브탭은 같은 새 CHORD 화면으로 대체했다. Screen Mode (Tab1,SubTab2)에서 A=(13,9,5,1): Root1/Quality1/Voicing1/PadNote1, B=(14,10,6,2): 슬롯2, C=(15,11,7,3): 슬롯3, D=(16,12,8,4): 슬롯4. Bank Direction=Column. MAIN(Tab1,SubTab1)과 Program/Track의 기존 신스16개 매핑은 변경하지 않았다. 화면별 전환은 Q-LINK SCREEN 모드에서 사용해야 한다.

행별 qlinkBoundsData와 마지막 편집 슬롯에 따른 배경 강조를 적용했다. A·1/B·2/C·3/D·4 미리듣기 버튼을 표시한다. Type/Voicing 컨트롤은 선택 목록을 열기 전에 해당 파라미터의 Q-Link bank를 선택하도록 했고, 미리듣기 역시 해당 슬롯 Root bank를 선택한다. ROOT/Pad Note는 기존 knob의 Q-Link action을 유지한다. LEARN/입력모드/미리듣기 gate는16개 할당에 포함하지 않는다.

화면 미리듣기를 누른 상태에서 Root/Type/Voicing이 바뀌면 기존 보이스를 재사용하여 새 코드를 들을 수 있다. 줄어든 구성음은 정상 Release로 종료하며 MIDI로 누른 다른 음의 소유권과 분리한다. Pad Note/라우팅 변경은 MIDI 소유 음과 서스테인만 정리하고 화면 미리듣기는 유지한다. 손을 떼면 기존 hold 계약으로 해제한다. 터치로 유지 중인 코드의 프리셋/파라미터 변경도 이 갱신에 포함되며 MIDI로 유지 중인 코드는 다음 트리거부터 변경된다. 새 파라미터를 추가하지 않고87개 인덱스/UID를 유지한다.

Native/ARM32/ARM64 빌드 및 SDK validator, 엔진/UBSan/VST bridge, host_sim19 PASS/0 warnings/0 failures, UI 회귀 검사를 통과했다. 회귀 검사에는16개 코드 매핑과 MAIN/Program 매핑 보존, 행 bounds, popup 오인 사용 방지, Root 옥타브 변경 시 실제 오디오 zero-crossing 증가, Type/Voicing 보이스 중복 방지, Pad Note 변경 중 touch hold 유지/MIDI 음 정리/해제 검사를 포함했다.

Mac 잠금으로 실제 기기 탭 전환·새 Type/Voicing 터치·물리 Q-Link·페이지 전환 중 hold 종료는 아직 검증하지 못했다. 파일 전송 후 앱 재시작은 현재 프로젝트 상태 확인 전까지 보류한다. build/chord-editor-preview.png는 오프라인 근사 이미지다. 공개push/Release 없음.


### 최신 사용자 확정 — 초기 화면·Q-Link 가이드

플러그인을 불러오면 안내 팝업 없이 MAIN을 표시한다. 이전 EDIT CHORDS의 탭 이동 안내 팝업은 폐기한다. 첫 번째 하단 탭을 다시 누르면 CHORD 편집으로 이동하며 다시 누르면 MAIN이다. Q-Link 가이드는 실제 할당 컨트롤 영역에만 표시하고 선택/설정 팝업에서는 숨긴다. 모든 기본 탭과 팝업의 실제 기기 검증 결과는 implementation.md에 기록한다.


### 0.1.17 — 레가토·전위·코드 옥타브

NOTES와 SPLIT의 일반 멜로디는 Voice Mode의 Mono/Legato/Poly를 따른다. 코드 트리거 및 화면 코드는 완전한 화음을 유지한다. Mono/Legato에서는 멜로디 한 보이스를 예약하고 나머지15보이스를 코드에 사용한다. CHORDS에서는16보이스를 코드에 사용할 수 있다. 단음 fallback은 코드 트리거를 멜로디로 재생하지 않는다. sustain/CC121 해제는 MIDI 코드·멜로디를 함께 정리하고 누르는 중인 화면 코드는 보존한다. Legato에서 음을 뗀 뒤 다시 누르면 기존 Release가 남아 있어도 Attack을 재시작한다.

전위: Root/1st/2nd/3rd/4th, 추가 Open voicing. 3화음은2전위,4음 코드는3전위,5음 코드는4전위까지 제공한다. 현재 Dominant13은 root/3rd/7th/9th/13th의5음 구성이다. 전위는 선택한 코드 구성음을 bass로 두고 나머지음을 위로 배치한다. Spread는 bass부터 올라가는 순서다. 코드 종류를 바꾸면 존재하지 않는 전위는 가능한 마지막 전위로 보정한다. 기존0=Root/1=First/2=Open 저장값은 유지한다.

각 코드의 OCT 선택기는 -2/-1/0/+1/+2옥타브를 이동하며 Root 표시·Pad Note는 바꾸지 않는다. 신규 파라미터 인덱스87–90, 기본 인덱스2(0oct), 총91개다. MAIN 신스16개와 CHORD Root/Type/Voicing/Pad Note의16 Q-Link는 그대로 유지한다. 화면 코드를 누르는 중의 옥타브 변경은 기존 보이스를 재사용해 갱신하고 MIDI 코드는 다음 트리거부터 적용한다. MIDI0–127의 발음 범위는 기존처럼 제한한다.

D·LFO는 정적 헤더다. 중복 설정 팝업과 CHANGE 버튼을 제거한다. Shape/Target은 MAIN에서 선택하고 Sync/Division은 MOD에서 설정한다. 독립 Tremolo는 자유 Hz Rate/Depth이며 LFO Sync와 별개다.

### 0.1.18 — 최신 코드 편집·선택 목록 규칙

이 항목이 이전 CHORD Pad Note Q-Link/터치 전용 옥타브 및 전체화면 선택 목록 규칙을 대체한다. CHORD SCREEN A–D=슬롯1–4, 각 그룹 Root/Type/Voicing/Octave다. Program/Track 및 MAIN의 신스16개는 유지한다. 행 가이드는 x96..804로 MIDI Note를 제외한다. 옥타브는 x726의76×90 도넛 컨트롤(56px 그림)과 값 터치 목록이다. MIDI Note 도넛은 제거하고 등록값 텍스트와 더블탭 수동 지정, LEARN을 유지한다.

선택은426×628 왼쪽 사이드바에서 수행한다. 뒤쪽 페이지는 반투명 검정으로 어둡게 유지하며 가이드는 숨긴다.58px 행/최대8개, 선택 후 자동 닫힘, X/바깥 터치 닫힘, 긴 목록 PREV/NEXT다. Sound Preset은 카테고리→음색을 유지하고 Voicing은 실제 구성음 수에 맞는 전위만 표시한다. MPC 공통 상단 프리셋 브라우저는 호스트가 관리한다.

PAD LEARN은 첫 물리 입력으로 등록(무음), 다음 입력부터 코드 발음이다. 등록 즉시 같은 화면에서 LEARN 해제·완료 문구·MIDI 번호를 갱신한다. 실제 기기의 입력 등록 후 즉시 표시를 확인했으며 구현과 검증 범위는 implementation.md를 따른다.

### 0.1.19 — 연주 빈도에 따른 그룹 순서와 값 표시

사용자 확정 순서는 A SOUND / PLAY(좌상), B AMP ENVELOPE(우상), C FILTER / LFO(좌하), D FX / TREMOLO(우하)다. A는 OSC Pair/Osc Mix/Body/Drift, B는 Attack/Decay/Sustain/Release, C는 Filter Cutoff/Resonance/LFO Rate/LFO Depth, D는 Drive/Glide/Tremolo Rate/Tremolo Depth다. MAIN Screen과 Program Q-Link도 이 순서를 사용한다. CHORD 매핑과 영구 파라미터 ID/순서는 유지한다. 기존 프로젝트의 저장된 호스트 사용자 매핑은 새 인스턴스와 별도 비교한다.

필터 종류는 C 왼쪽, LFO Shape/Target은 C 오른쪽 두 노브 위에 둔다. Drive 종류는 D 첫 노브 위, Tremolo 자유 속도 안내는 D 오른쪽에 둔다. LCD 값은29→24px, 선택값22px, 보조 숫자22px로 줄이고 기본 값 라벨은122px 폭으로 가운데 배치해138px 플레이트 안에 여백을 준다. 터치 영역은 유지한다. 실제 검증 결과는 implementation.md를 따른다.

### 0.1.20 — 코드16슬롯 확정

사용자는 기본4/최대16,4개 단위 확장을 승인했다. CHORD의+/-로 활성 개수를 바꾸며 축소해도 슬롯별 코드/전위/옥타브/패드 매핑은 보존한다. MAIN은4개의222×70 패드와 위쪽 왼편 페이지 넘김(현재 범위/전체 개수)으로 구성한다. 물리 패드 매핑은 화면 페이지와 독립적이다. 화면을 넘기면 숨겨진 터치 hold를 해제하되 물리 MIDI 발음은 유지한다.

최신 사용자 변경에 따라 MAIN↔CHORD 두 서브탭만 사용한다. MAIN을 한 번 눌러 편집 화면으로 가고 그 안의1–4/5–8/9–12/13–16 버튼으로 묶음을 선택한다. 메인과 편집 페이지를 공유하며 활성 범위만 선택 가능하다. SDK의 Q-Link는16개 편집 중계 파라미터에 고정하고, 플러그인에서 선택한 실제 슬롯에 값을 전달한다. 새 handler나 팝업 매핑 API를 만들지 않는다. 실제 슬롯 파라미터는 automation/저장 대상이며 편집 중계는automation을 비활성화한다. 안내 팝업은 만들지 않는다.

프리셋 선택은 현재 MAIN의4슬롯에만 루트/종류를 적용한다. 다른 묶음의 코드와 모든 보이싱/옥타브/패드 매핑은 유지한다. 네 묶음의 프리셋 선택은 독립 저장하며 MOD의 PRESET 1–4는 고정 첫뱅크 전용이다.16코드 진행32개를 새로 작곡한 것은 아니다. 동시 발음16보이스는 유지한다. 기존91개 파라미터의ID/순서를 보존하고 총192개로 append한다. 실제 검증 상태는 implementation.md를 따른다. B 헤더 위 native Q-Link 선은 사용자가 그대로 두라고 하여 변경하지 않는다.


### 1.0.1 — Main oscillator mix / glide placement
User-approved MAIN and Screen/Program Q-Link order: A = OSC Pair, OSC Mix, Body, Drift; D = Drive, Glide, Tremolo Rate, Tremolo Depth. B/C are unchanged. Space remains on FX, outside the MAIN sixteen controls; Glide also remains on AMP. Existing parameter IDs and DSP behavior are unchanged. OSC Mix is the existing balance parameter; Body still contributes its established mix/sub shaping.


### 1.1.0 — 기존 음색 전체 재조정(로컬 후보)
모드나UI를 추가하지 않고 기존63개 연주 음색의OSC/Low-pass/엔벌로프/Tape/효과를 따뜻한 아날로그 성격으로 재조정한다.64개 이름/순서, Init과개별Low-cut, UI/Q-Link, UID/파라미터는 유지한다. 엔진의12dB SVF를24dB 래더로 설명하지 않는다. 스테레오 피크를 재측정해레벨을 보정하며 실기기청음 승인은 별도다. presets.md가 최신 레시피·레벨 기준이다.


### 1.1.1 — 코드 영역 화살표 터치 확대
사용자 불편에 따라 코드 프리셋 이전/다음은44×44 표시·터치, 슬롯 페이지는44×32 표시와44×44 터치로 확대한다. 코드 패드222×70을 유지한다. 아래프리셋행y580–624, 페이지hit y536–580으로 분리하며 위필터LCD(y534까지)와 겹치지 않는다. 네뱅크 선택기는220×44로 함께 키우고 기본프리셋명 글씨는 유지한다. HOLD문구통합·코드이름위계·누름피드백강화·EDIT CHORDS진입버튼은 아직 제안이며 구현하지 않았다.

- 최신 사용자 결정: HOLD TO PLAY는 공통 안내도 표시하지 않는다. MAIN 패드에는 슬롯 번호와 코드 이름만 남긴다.

### 1.1.2 — 수동 물리 패드 위치 표시 (로컬 후보)

사용자가 수동 위치 선택+LEARN 방식을 승인했다. MAIN의 각222×70 코드 연주 패드에는 등록한 위치명(A1–H16)과40×40 패드 배열을 표시한다. 미등록 위치는 숨긴다. 별도 코드별 EDIT 버튼을 추가하지 않고 기존 공통 CHORD 편집 화면에서 PAD LOCATION을 설정한다. 왼쪽426px 선택창에서 A–H 뱅크,4×4 위치(왼쪽 아래1,오른쪽 위16), CLEAR LOCATION을 제공한다. 위치 지정은 MIDI 라우팅을 변경하지 않으며 실제 패드 입력 연결은 기존 LEARN으로 등록한다. MPC Pad Perform 배치가 바뀌면 위치와 MIDI 연결을 다시 확인한다.

기존192개 파라미터의ID/순서를 보존하고 PadPosition1–16(192–207)을 append한다. 위치는 프로젝트에 저장하며 automation/Q-Link에서 제외한다. 기존 프로젝트는 미등록으로 복원한다. 슬롯 축소·진행/음색 프리셋 선택으로 위치를 삭제하지 않는다. 실기기 검증 결과는 implementation.md에 별도로 기록한다.


### 1.2.0 — Moog-inspired engine (supersedes preset-only scope)
사용자가 프리셋 조정만으로는 부족하다고 정정했고, 기존 엔진 자체를 Moog-inspired로 변경하도록 확정했다. 새 모드/UI 없이 비선형 믹서·4단 래더Low-pass·레벨 의존VCA를 기본 경로로 사용한다.2x 내부 래더 처리와 출력FIR, 공진 피드백을 구현하며 다른 필터 종류·Low-cut·FX는 보존한다. 전체63개 프리셋은 새 엔진에 맞춰 재설계한다. 기존208개 파라미터/UI/Q-Link/UID는 유지하지만 저장값이 같아도 소리는 달라진다. 음색 동일 복원을 보장하지 않는다. 설계·연구·제한은 [engine.md](engine.md), 실제 검증 상태는 implementation.md를 따른다. 사용자가 검증·정식 릴리스·사이트 갱신까지 추가 수락 없이 진행하도록 승인했다.


## 1.2.2 수정 작업 — 수동 패드 위치 표시 폐기

사용자가 불안정한 패드 위치 표시 기능 제거를 허용했다. 위치 선택/미니4×4/뱅크 라벨은 UI에서 제거하고 실제 MIDI Note/LEARN 및 코드 연주는 유지한다. 저장 호환을 위해 PadPosition1–16(192–207)은 삭제·재정렬하지 않고 비활성 저장값으로 보존한다. 수정본 실기기 원격 전환·편집은 동작했다. 구버전 실패와의 인과관계는 통제된 A/B로 확정하지 않았다.

## 2026-11-18 — approved key reference / transpose (1.3.0 candidate)

User approved the Key/circle CHORD concept and explicit TRANSPOSE confirmation. Root/Major-or-natural-Minor selects reference only; circle and diatonic spelling are generated from musical pitch/letter data, not AI text artwork. The four visible degree badges identify scale-root membership, with case from chord quality and dash for chromatic roots. Existing four row controls and sixteen SCREEN assignments remain unchanged; MIDI/LEARN move to PAD SETUP, not deleted.

TRANSPOSE FROM defaults C for older projects and is editable for preexisting progressions. TO is the selected reference key. APPLY moves all active roots by one shared nearest interval −5..+6 (tritone +6), updates FROM, and preserves type/voicing/octave/MIDI/sound and inactive slots. Out-of-range MIDI24..84 rejects the whole operation; no individual clamping. Key-mode changes do not rewrite chord quality. Modal command states are cleared on save/load; no automatic apply on project recall.

Parameters208..219 append: key root/mode, source root/mode, preview/apply/status, four degree readouts, interval. First208 declarations are identical; UIDKrql and synthesis recipes unchanged. Public release remains1.2.3 until publication. Automated and device verification are recorded below as completed.

## 2026-11-18 — approved SMOOTH VOICING (1.4.0 candidate)

User approved top-note voice leading with current-four/all-active scope, before/after preview, audition and explicit APPLY. First selected chord's existing voicing/octave anchors the loop; candidate inversions/Open and octave−2..+2 are selected by lexicographic total closed-loop top movement, closest-tone/bass movement, then edit distance. Roots/types/Key/MIDI remain intact; only selected voicing/octave fields are applied. Unsafe/clipped/collapsed registers are rejected; spans do not exceed max24semitones/originalspan.

Planning runs only on a non-render UI poll/idle behind the reentrancy guard. Audio consumes a consistent atomic publication and checks source freshness; no search, allocation, lock, I/O or direct host notification runs in process. PREVIEW uses separate owner4096 and a copied proposal, one chord/beat and automatic end; CANCEL/X/outside closes/release without applying. Shared16voice limits still apply. Saved UI/apply/preview commands are cleared.

Preserve first220 parameter declarations; SmoothOpen..SmoothRow16 append220..241 (242total). Existing16CHORD Q-Link/guide geometry, current Key/circle UI and synth recipes remain. SMOOTH VOICING is a44px utility above TRANSPOSE; the426px drawer hides background guides and shows only active scoped top-note rows. Public stable remains1.2.3; candidate validation/device results follow in implementation records.
