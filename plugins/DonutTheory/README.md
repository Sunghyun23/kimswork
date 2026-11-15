# Donut Theory

**kim · v1.2.0 · UID `Krql`**

MPC standalone용 소울·힙합 신디사이저입니다. 베이스, 리드, 합성 키보드, 코드와 패드를 한 엔진에서 연주합니다. MPC Live III / OS 3.10.1.12에서 프리셋·메뉴·16슬롯 편집·표시 수정을 직접 검증한 배포 버전입니다. 검증 범위는 아래 기록을 참고하세요. **1.2.0 새 엔진은 자동 검증을 완료했지만 MPC 앱 재시작이 보류되어 아직 실기기에서 실행하지 않았습니다. 이전 UI 검증과 구분하세요.**

[상세 사용자 매뉴얼](../../docs/plugins/donut-theory/manual.md) · [제품 소개](https://kimswork.app/plugins/donut-theory/)

## 1.2.0 Moog-inspired engine

프리셋 조정만이 아니라 비선형 오실레이터 믹서·4단 래더Low-pass·VCA의 DSP 자체를 바꿨습니다. 래더는2x 처리/FIR/동시 피드백 해석을 사용하고, 기존63개 연주 음색을 모두 새 엔진에 맞춰 재설계·레벨 보정했습니다. 별도 모드·UI·파라미터 추가는 없습니다.208개 파라미터와UID/매핑은 유지하지만 기존 프로젝트도 소리가 달라집니다. 정확한 기존 소리가 필요하면1.1.2를 보관하고 트랙을 렌더하세요.

[엔진 구조·연구 출처·라이선스](../../docs/plugins/donut-theory/engine.md). 프로젝트의 새DSP는MIT 원본 구현이며 외부 래더 코드 복사나 SDK 수정은 없습니다. 특정Moog 모델의 부품 단위 복각이 아닙니다.2x 처리는aliasing을 줄이며 제거를 보장하지 않습니다. 전체 MPC 청음·최악 부하 검증을 완료한 것으로 소개하지 않습니다.

## 현재 검증 범위와 남은 항목

- 새 인스턴스의 Q-Link 매핑은 사용자가 정상 동작을 확인했습니다. 기존 프로젝트의 캐시된 매핑과 이번 디자인 수정 후 전체16개 물리 회전 검증은 별도입니다.
- 터치 코드 hold/release는 Mouse Down과 연속 Slider gesture를 조합합니다. MPC Live III 3.10.1.12에서 진단 패드의 실제 손가락 연주·해제 정상 동작을 사용자가 확인했고 네 패드에 적용했습니다. 다중 터치 및 드래그 경계 동작은 추가 검증 대상입니다.

## 구현된 기능

- 16보이스, 오실레이터 2개, 사인 서브, 노이즈, 사인 OSC 1의 위상 변조/FM 계열 음색.
- 비선형4단24dB/oct 래더LP, 기존12dB/oct SVF BP/HP/Notch 확장, 필터·앰프 ADSR, 벨로시티와 키 트래킹.
- Mono / Legato / Poly, 노트 우선순위 복귀, 글라이드, 피치 벤드, CC1 비브라토, CC64 서스테인.
- Tape / Tube / Fuzz 성격의 waveshaping, 코러스, 템포 딜레이, DC 차단, 최종 소프트 제한.
- 선택 가능한 LFO 파형·대상, 자유 속도/템포 동기화, 별도 드리프트와 비브라토.
- 64개 새 음색: Init1, Bass12, Keys15, Synth12, Lead12, Pad12. 카테고리 우선 탐색, 음색별 Low-cut과 출력 보정. [프리셋 설계·목록·검증](../../docs/plugins/donut-theory/presets.md)을 참고하세요.
- 코드 진행 32개: 8개 기본 진행 × 4개 조옮김(0,+2,-2,+5). 32개 서로 다른 화성 작곡이라고 소개하지 않습니다.
- 음색·코드 진행의 독립 선택, 이전/다음 버튼, 메뉴 선택, 프로젝트 상태 저장/복원.
- 승인 이미지에서 분리 제작한 실제 배경·노브 자산, 커스텀 MPC 스킨과 Q-Link 매핑.

## 메인 Q-Link

| 뱅크 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A SOUND / PLAY | OSC Pair | OSC Mix | Body | Drift |
| B AMP ENVELOPE | Attack | Decay | Sustain | Release |
| C FILTER / LFO | Filter Cutoff | Resonance | LFO Rate | LFO Depth |
| D FX / TREMOLO | Drive | Glide | Tremolo Rate | Tremolo Depth |

DRIVE 종류는 DRIVE 위에 있습니다. OSC Pair는 두 파형을 함께 고릅니다. 필터 종류와 LFO Shape/Target은 별도 터치 메뉴입니다. 각 값과 단위를 표시하며 아래 코드 선택기는 **CHORD PRESET**으로 명시합니다. Q-Link는 Column 방식으로 A=(13,9,5,1), B=(14,10,6,2), C=(15,11,7,3), D=(16,12,8,4)에 연결합니다. 각 뱅크의 물리 노브 위→아래가 화면의 왼쪽→오른쪽입니다. Screen/Program 모드에 같은 열 순서를 사용하며 물리 회전 검증은 별도입니다.

전체 파라미터 정의: [parameters.json](parameters.json). 내부 0번은 마스터 볼륨이며 기본 Q-Link 16개와 별개입니다.

## 키보드와 코드 패드

- **Notes**: 들어오는 MIDI 노트를 그대로 연주합니다.
- **Chords**: 활성 슬롯의 개별 Pad Note가 최대16개 코드 슬롯을 트리거합니다. 새 인스턴스 기본값은24/26/28/29이며 그 밖의 노트는 무시합니다.
- **Split**: 활성 슬롯에 지정한 노트는 코드, 나머지는 일반 노트입니다. 코드와 멜로디는 같은 음색입니다.
- **CHORD 페이지**에서 각 슬롯의 Pad Note를 지정하거나 LEARN으로 실제 패드를 등록합니다. MPC의 Pad Perform 설정을 플러그인 때문에 바꿀 필요가 없습니다. 기존 프로젝트는 Legacy Trigger Base 설정을 유지합니다.
- 코드 확장이 켜진 Chords/Split에서는 선택한 Voice Mode와 관계없이 16보이스 다성부를 사용합니다. Mono/Legato는 Notes와 Split의 일반 멜로디에 적용됩니다.
- 코드 슬롯마다 루트·종류·보이싱을 설정하며 Spread는 각 구성음 사이의 ms입니다.
- 같은 트리거를 반복해 누르면 마지막 대응 note-off까지 소리를 유지합니다. 같은 음이 들어 있는 다른 코드의 note-off와 분리해 관리합니다.
- 보이스가 부족하면 릴리스 중인 가장 오래된 보이스부터, 다음으로 가장 오래된 보이스를 교체합니다. 극단적인 voice stealing에서 클릭은 실기기/청음 QA 대상입니다.
- 입력/발음 모드·음색을 바꾸면 현재 음을 정리합니다. 코드 진행만 바꾸면 현재 음은 유지되고 다음 트리거부터 새 코드를 사용합니다.
- 화면의 네 코드 버튼은 **누르고 있는 동안 연주하고, 손을 떼면 Amp Release에 따라 감쇠**하도록 구성했습니다. INPUT 모드와 무관합니다. 0.1.5의 실기기 검증 범위는 구현 기록을 확인하세요. CHORD → ADVANCED의 **ALL NOTES OFF**는 터치·물리 패드·키보드 음과 서스테인, 효과 꼬리를 정리합니다.
- 하단 MAIN을 다시 누르면 **CHORD** 서브탭이 열립니다. 선택한 묶음의 네 코드 루트·종류·보이싱·옥타브를 함께 편집하고 LEARN으로 패드를 등록합니다. 첫 번째 탭을 다시 누르면 MAIN으로 돌아옵니다.
- 터치 코드는 내부 발음 게이트입니다. MIDI 노트를 시퀀서에 보내는 기능은 아닙니다. 물리 패드/MIDI 노트 녹음과 구분하세요.
- NOTES의 Mono/Legato에서도 터치 코드는 다성부로 재생하며 일반 단음 연주용 보이스 하나를 예약합니다.

## 빌드

저장소 루트에서 실행합니다. Python 3.9+, C++17 컴파일러, 개발용 Pillow가 필요합니다.

```sh
python3 scripts/bootstrap_sdk.py
python3 -m pip install -r requirements-dev.txt
python3 scripts/build_plugin.py DonutTheory --target native
python3 scripts/build_plugin.py DonutTheory --target all
python3 scripts/test_plugin.py
```

SDK 버전과 컴파일러 설치: [third_party 안내](../../third_party/README.md).

- `generate_catalog.py`: 파라미터·음색·코드 데이터, C++ 헤더 및 상세 UI 정의 생성.
- `build_skin.py`: ordered JSON을 사용해 중복 version 키를 보존한 커스텀 스킨 생성.
- `render_skin_preview.py`: 실제 TUI의 MAIN을 오프라인으로 근사 렌더링. MPC 호스트 에뮬레이터가 아닙니다. 현재 프리뷰 폰트 경로와 테스트 래퍼는 macOS 기준입니다.
- native 빌드는 컴퓨터용 테스트 라이브러리만 만듭니다. MPC 설치에는 `--target all` 결과를 사용합니다.
- `skins/`, `build/`, `dist/`는 생성물이며 커밋하지 않습니다.

## 설치 결과와 조건

전체 빌드 출력: `build/kim - VST - Donut Theory/`.

```sh
python3 third_party/mpc-plugin-builder-sdk/tools/package.py plugins/DonutTheory
```

출력 폴더를 USB/SD의 `VST2 Plugins`에 복사합니다. SDK 기준 MPC OS **3.10.1 이상**, Preferences → Enhancements의 **Enable Enhancements / External Plugin Support** 활성화가 필요합니다. 실제 사용 모델·OS에서 설치, 노트 연주, Q-Link, 저장/복원을 확인해야 합니다.

## 검증과 제한

- native·ARM32·ARM64 빌드 및 SDK 구조 검증, host simulator 검증 결과는 [개발 검증 기록](../../docs/plugins/donut-theory/implementation.md)에 기록합니다.
- 자동 회귀 검사는 MIDI offset, 코드 소유권, 서스테인, mono fallback, 모드 전환, 상태 복원, 선택기 독립성, 프리셋 발음/유한값을 다룹니다.
- 실기기에서 프리셋·코드 진행의 목록/화살표 전환, 전면 메뉴, 상세 탭, 코드 루트 편집, 노브·페이더 및 값 표시의 수정 결과를 확인했으며, 상세 기록과 미검증 항목은 개발 검증 기록을 참고하세요. 모든 음색의 음악적 밸런스와 물리 MIDI/Q-Link 검증은 별도입니다.
- 플러그인 SOUND PRESET은 카테고리→음색 순서로 탐색합니다. MPC 공통 상단 프리셋 브라우저는 평면 목록입니다. 이름의 category/name 표시는 폴더 계층이 아닙니다.
- FM·drive는 간단한 합성/왜곡 알고리즘이며 특정 아날로그 회로나 테이프 기기의 정밀 복각이 아닙니다. 강한 왜곡과 고음역 FM의 aliasing 청음 검증이 필요합니다.
- 전역 기본 범위를 쓰는 매크로를 먼저 구현했습니다. 프리셋마다 독립된 매크로 라우팅/곡선 편집은 아직 없습니다.
- 코드 설정은 파라미터로 저장됩니다. 프로그램 선택으로 chord 설정을 덮어쓰지 않습니다.
- GUI는 승인 콘셉트의 실제 SDK 적응판입니다. 사용 가능한 폰트와 1280×628 영역에 맞춰 달라진 부분이 있으며 픽셀 단위 복제는 아닙니다.

## 라이선스와 자산

SDK DSP 및 프레임워크: inMusic MPC Plugin Builder SDK, MIT. 출처·고정 커밋은 third_party/sdk.lock.json에 기록했습니다. SDK MIT 고지를 패키지에 포함합니다.

플러그인 코드: MIT, kim. 이미지 자산 제작 기록은 [assets/README.md](assets/README.md)를 참고하세요. 외부 아티스트의 로고·사진·음원·샘플을 포함하지 않습니다.

### 0.1.1 화면 수정

노브가 점으로 보이던 추가 배율 및 고정 그룹 제목의 숨김 조건을 수정했다. SOUND PRESET / VOICE / INPUT / CHORD PRESET 고정 라벨의 핸들 연결을 보완했다. 수정된 화면은 실기기 재확인이 필요하다. 기존 설치 폴더를 새 ZIP의 같은 이름 폴더로 교체하고 MPC를 재시작한다. UID와 저장 파라미터 순서는 동일하다.

### 0.1.2 버그 수정

- MPC 프로그램 번호와 내부 Sound Preset은 같은 선택값을 보고한다. 양쪽 선택 모두 사용자가 설정한 마스터 볼륨과 코드 설정을 보존한다.
- 내부 변경은 원자적 플래그로 모아 호스트의 UI 표시값/이름/프로그램 조회 및 UI idle 호출에서 알린다. 오디오 렌더 중에는 UI 콜백을 호출하지 않는다. SDK 원본은 수정하지 않고 플러그인 진입점의 어댑터로 처리한다. 실제 MPC의 화면 조회 주기와 갱신은 재검증 대상이다.
- Mono/Legato에서 다른 채널의 페달 해제가 음을 끊는 문제, CC121 후 누른 음으로 복귀하지 않는 문제를 수정했다.
- 사인 오실레이터를 wavetable로 처리하고, 렌더 루프의 파라미터는 블록 스냅샷을 사용한다. 제어 주기 카운터는 16샘플마다 순환하여 장시간 실행 시 signed overflow를 방지한다.
- 터치 메뉴는 native ComboBox를 사용하며 더블탭/Enter 메뉴도 유지한다. DRIVE/PLAY 구분선과 전면 SYNC/DIV 선택기를 추가하고 선택기·노브 겹침을 제거했다.
- Sync가 켜지면 Rate 값은 실제 분할을 표시한다. 이때 자유 Hz 노브는 비활성 의미이며 속도 변경은 DIV로 한다. Sync를 끄면 저장된 자유 Hz가 다시 적용된다.

일반/UBSan 엔진 및 VST 어댑터 테스트로 재현 사례를 검사한다. 정적 스킨 검사와 오프라인 프리뷰는 실기기 터치 검증이 아니다. 이 버전의 ComboBox, 메뉴, 이미지 배율, 화면 갱신은 MPC에서 다시 확인해야 한다.

### 0.1.3 실기기 UI 수정

- 배경을 실제 콘텐츠 크기로 생성해 잘림·로고 겹침을 해결했습니다.
- 빈 native 선택 목록을 명시적 선택 패널로 교체했습니다. MAIN에서 이름을 한 번 누르면 열리며, 항목 선택 후 DONE으로 닫습니다. 음색은 32개씩 두 페이지입니다. 상세 탭의 긴 목록은 더블탭으로 엽니다.
- 보이는 라벨이 터치를 막는 실기기 현상을 반영해 버튼 캡션 위에 투명 터치 표면을 배치했습니다. 상세 스위치 캡션도 같은 컴포넌트 안에 넣고 대비를 높였습니다.
- 화살표 반복 입력, choice 값 갱신, 정수로 처리하는 옥타브·트리거 번호 표시를 보완했습니다. UI 파라미터 종류는 호스트 추정값 대신 프로젝트 선언에서 읽습니다.
- 코드 패드의 더블탭은 해당 슬롯의 루트 편집입니다. 슬롯 선택과 실제 MIDI 발음은 서로 다릅니다.

- 상세 선택 막대의 흰색 내장 텍스트를 어두운 명시적 라벨로 교체하고, 잘못된 Data 자기 참조를 제거했습니다. 실제 OSC/MOD/CHORD 페이지에서 이름·값 표시 및 선택을 확인했습니다.

실기기 상태·검증 범위: [구현 기록](../../docs/plugins/donut-theory/implementation.md).

### 0.1.4 가독성·연주·음색 개선

- MAIN 이름/값과 선택 목록 글자를 확대했습니다. Q-Link 4×4와 기존 컨트롤은 유지했습니다.
- Lead 12개와 Pad 16개를 톱니/펄스 조합, 디튠, 필터 엔벌로프, 어택/릴리스, 코러스를 개별 설정하는 방식으로 재설계했습니다. 총 음색 수는 64개입니다. 먼저 Lead / Late Night, Pad / Velvet Chords를 확인하세요. 특정 하드웨어의 정밀 복각은 아닙니다.
- **FX → FX Preset 값 더블탭**: Dry, Warm Tape, Wide Chorus, Slap Delay, Space Echo, Fuzz Box, Dark Dub를 선택합니다. Custom은 현재 값을 그대로 유지합니다. 선택 후 개별 노브를 조정할 수 있으며 FX Preset 이름은 마지막 선택 레시피를 표시합니다. 같은 레시피를 다시 적용하려면 Custom을 선택한 뒤 다시 선택하세요. 음색을 바꾸면 그 음색의 FX 설정이 적용됩니다.
- 화면 코드 연주/정지와 물리 MIDI 코드 확장을 함께 지원합니다. 물리 패드는 INPUT=Chords 또는 Split, 기본 송신 MIDI 36–39를 사용합니다. 패드 번호 1–4가 자동으로 이 노트를 보낸다고 가정하지 마세요.
- 새 파라미터 6개는 기존 68개 뒤에 추가했습니다. UID와 기존 인덱스는 유지했습니다.
- 실기기 화면에서 버튼 토글/STOP/EDIT, 확대된 목록, Late Night와 Velvet Chords 전환, Split 선택, Space Echo의 실제 파라미터 갱신을 확인했습니다. 오디오 엔진 자동 테스트와 실기기 청음은 별개이며 물리 패드·Q-Link·청음 QA는 남아 있습니다.

### 0.1.5 새 전면 배치

- 12개 도넛 노브와 4개 ADSR 페이더로 16 Q-Link 구성. 새 매핑은 위 표를 따른다.
- Tone/Motion은 OSC/MOD 상세 화면에 유지. Vibrato/Expression은 전면 작은 조절부 드래그 또는 이름/값 더블탭으로 조절. LFO Shape/Target/Sync/Division은 전면 터치 선택.
- OSC Pair는 Sine/Triangle/Saw/Pulse의 16개 조합. 파형만 바꾸고 믹스·디튠·필터·ADSR은 보존. 기존 OSC 상세 설정과 자동화 파라미터는 유지한다.
- 독립 Tremolo Rate 0.1–15 Hz, Depth 0–100%. Depth 0은 Off. LFO와 동시에 사용 가능. Drive 뒤/Chorus·Delay 앞의 진폭 변조라 지연 잔향은 남는다. 음색 선택 시 4 Hz / 0%로 돌아가며 프로젝트 저장값은 보존한다.
- 터치 코드는 SDK ResetOnGestureEnd 핸들로 손 떼기 시 Off가 되도록 구현. 물리 MIDI는 기존 Note On/Off 사용. 기존 토글 방식 안내보다 이 항목이 우선한다.
- UID/기존 파라미터 순서는 유지하고 3개를 끝에 추가했다. 기존 프로젝트의 음색·코드 설정을 유지한다.

### 업데이트 후 Q-Link가 이전 배치로 남을 때

MPC Live III / OS 3.10.1.12에서 이전 QA 트랙의 Screen 매핑은 구버전 배열을 유지했지만, 새 플러그인 트랙에 Donut Theory를 신규 로드하면 최신 4×4 매핑이 표시됐고 사용자가 물리 노브 동작 정상도 확인했습니다(2026-11-14). 기존 프로젝트와 음색을 보존한 뒤 새 인스턴스로 비교하세요. 기존 트랙 자동 마이그레이션과 저장/재로드 후 매핑 지속성은 아직 검증하지 않았습니다. 터치 코드 hold 수정·검증 기록은 implementation.md를 참고하세요.

### DRIFT (0.1.7)

전면 A 그룹의 세 번째 Q-Link는 DRIFT입니다. 0–100%로 느린 Wow(0.43/0.137Hz 혼합)와 빠른 Flutter(6.73Hz)의 피치 흔들림을 함께 조절합니다. 낮은 값에서 섬세하게 조절하도록 제곱 곡선을 사용하며 최대 편차는 43 cents 이내입니다. 오디오 테이프 장치의 정밀 모델링이 아닌 전체 발음에 공유되는 피치 모듈레이션입니다. Detune은 기존 OSC 상세 페이지에 그대로 남아 있습니다. Tremolo 및 필터 LFO와 독립적으로 작동합니다.

새 파라미터 Drift는 인덱스 83(총 84개)에 추가했습니다. 기존 파라미터 ID/순서를 유지하고 0.1.6 이하 상태 로드 및 음색 프리셋 선택 시 DRIFT=0으로 초기화합니다. 저장한 새 프로젝트는 DRIFT 값을 복원합니다. 기기 화면/물리 Q-Link 검증은 별도로 필요합니다.

### 0.1.8 UI update

The MAIN header now uses chocolate-brown paper with cream labels, a full-bleed donut/record illustration at the right edge, and an exact94px header boundary. Section title bars fill their rounded panels. Header controls retain their olive/cream artwork with transparent widget backgrounds, so obsolete light rectangular fills no longer show through their corners. Sound preset, Voice and Input hit areas match the compact visual widths. All84 parameter IDs,16 MAIN Q-Link assignments,70px chord hold surfaces and DSP behavior remain unchanged from0.1.7. Hardware verification status is recorded in `docs/plugins/donut-theory/implementation.md`; local preview is not proof of device rendering.


## 0.1.9 디자인 적용 및 검증

큰 도넛·레코드·사선 슬리브의 가로 구도를 유지하고 도넛 형태와 배경 접합을 다듬었습니다. MPC Live III3.10.1.12에서 작은 값 표시창/EDIT ALL 이미지 누락까지 수정·확인했습니다. 프리셋과 코드 프리셋, EDIT ALL, DRIVE 메뉴, DRIFT/ADSR 조절, 네 터치 코드의 hold/release를 직접 점검했습니다. Native/ARM 빌드, SDK 검사, host_sim19항목, 스킨 회귀 검사를 통과했습니다. 설치 폴더는 사용한 런타임 이미지만 포함해6.4MB입니다. 상세 결과와 검증 한계는 [구현 기록](../../docs/plugins/donut-theory/implementation.md)을 참고하세요. 공개 Release는 아직 발행하지 않았습니다.


## 0.1.11 새 공장 프리셋

기존 음색을 Init1/Bass12/Keys15/Synth12/Lead12/Pad12의 새64개로 교체했습니다. SOUND PRESET에서 카테고리→음색을 선택합니다. MPC 공통 상단 팝업은 호스트의 내장 프로그램 목록으로 표시됩니다. FX의 LOW CUT(0=Off)과 SOUND TRIM은 음색별 저역/레벨 보정입니다. 기존 상태는 새 파라미터0으로 복원합니다. 약5Hz DC 블로커 변경으로 기존 프로젝트의 초저역 응답은 달라질 수 있습니다. [전체 목록·측정·검증 상태](../../docs/plugins/donut-theory/presets.md)를 참고하세요. 실제 기기 터치/음악적 청감 검증은 Mac 잠금 해제 후 진행해야 합니다.


## 코드 편집 Q-Link (0.1.12)

하단 MAIN을 다시 눌러 CHORD를 엽니다. **Q-LINK SCREEN** 모드에서 A–D가 코드1–4이며 각 그룹은 Root → Type → Voicing → Octave 순서입니다(0.1.18부터). 첫 번째 탭을 다시 누르면 MAIN 신스 매핑으로 돌아갑니다. Program/Track 모드는 기존 신스 매핑을 유지합니다. 별도의 진입 안내 팝업은 없습니다. 플러그인 최초 화면은 MAIN입니다. 미리듣기 버튼을 누른 채 코드를 바꾸면 소리가 갱신됩니다. 실기기 물리 회전/터치 검증은 Mac 잠금 해제 후 진행해야 합니다.


## 0.1.16 실제 기기 확인

안내 팝업 없이 MAIN으로 시작합니다. 코드 편집은 첫 번째 하단 탭을 다시 눌러 CHORD에서 수행합니다. 모든 기본7개 페이지와 주요 선택/설정 팝업에서 Q-Link 가이드 위치/숨김/복원을 확인했고, 상세 페이지의 빈 하단 공간 및 CHORD 중복 테두리를 제거했습니다. 카테고리/DONE 터치 차단과 LEARN 대기 표시, 상세 선택 버튼 글자 대비를 수정했습니다.

기존 QA 트랙의 MAIN에는 구버전 저장 매핑이 남아 있었고 새 Plugin002에서는 A–D가 정상입니다. 기존 호스트 매핑의 자동 마이그레이션은 구현하지 않았습니다. 터치 미리듣기4개/MAIN패드4개를 실제 기기에서 원격 hold/release하고 녹음 종료 후 무음을 확인했습니다. 물리 Q-Link 회전과 손가락 다중 터치는 이번 확인 범위 밖입니다. 상세 증거와 한계는 [구현 기록](../../docs/plugins/donut-theory/implementation.md)을 따릅니다.


## 0.1.17 — 레가토·코드 편집

- SPLIT에서도 일반 멜로디가 Mono/Legato/Poly를 따릅니다. 코드 패드는 화음을 유지합니다. 실제 A13/A14 겹침 연주에서 사용자가 정상 동작을 확인했습니다.
- 각 코드의 VOICING: Root/1st/2nd/3rd/4th inversion/Open. 구성음 수에 맞는 전위만 표시합니다. OCT는 코드별 -2..+2oct입니다. 새 파라미터4개는 끝에 추가했고 기존16 Q-Link는 유지합니다.
- D·LFO SETTINGS 중복 팝업을 제거했습니다. Shape/Target은 MAIN, Sync/Division은 MOD에 있습니다. 독립 Tremolo는 유지합니다.
- 이전Root/First/Open 저장값과 옛 프로젝트의0oct 복원을 검사했습니다. 확장된Voicing choice의 기존 정규화automation은 재확인이 필요합니다.

Native/ARM 빌드, SDK validator, 엔진/UBSan/bridge, host_sim19항목과 UI 검사 통과. 실기기 전위/옥타브 터치·표시·가이드 확인 결과와 한계는 [구현 기록](../../docs/plugins/donut-theory/implementation.md)에 있습니다.

## 0.1.18 — 코드 Q-Link·PAD LEARN·왼쪽 선택 목록

- CHORD의 A–D는 코드1–4이며 각 그룹은 Root / Type / Voicing / Octave입니다. MIDI Note는 가이드와 Q-Link에서 제외합니다. OCT는 도넛 노브로 조절하며 값 부분을 누르면 -2..+2oct 목록을 엽니다. MIDI Note는 등록된 입력을 보여 주며 더블탭 수동 지정도 유지합니다.
- LEARN을 켜고 실제 물리 패드를 한 번 누르면 등록합니다. 이 첫 입력은 소리 없이 소비하고, 다음 입력부터 코드를 연주합니다. 실제 기기에서 화면을 이동하지 않아도 LEARN 해제·완료 문구·입력 MIDI 번호가 즉시 갱신됨을 확인했습니다. MPC 패드 설정을 바꾸지 않습니다.
- 선택 목록을 왼쪽 사이드바로 변경했습니다. 편집 화면을 뒤에 남기고 선택 후 닫힙니다. X/바깥 터치로 취소하고 긴 목록은 PREV/NEXT로 이동합니다. 음색은 카테고리부터 고릅니다. MPC 공통 상단 브라우저는 호스트 UI입니다.
- Native/ARM32/ARM64 빌드, SDK validator 및 엔진/브리지/스킨 검사를 실행합니다. 실제 확인 범위와 남은 물리 회전 검증은 [구현 기록](../../docs/plugins/donut-theory/implementation.md)을 따릅니다.

### 0.1.19 — 연주 빈도에 따른 그룹 순서와 값 표시

사용자 확정 순서는 A SOUND / PLAY(좌상), B AMP ENVELOPE(우상), C FILTER / LFO(좌하), D FX / TREMOLO(우하)다. A는 OSC Pair/Osc Mix/Body/Drift, B는 Attack/Decay/Sustain/Release, C는 Filter Cutoff/Resonance/LFO Rate/LFO Depth, D는 Drive/Glide/Tremolo Rate/Tremolo Depth다. MAIN Screen과 Program Q-Link도 이 순서를 사용한다. CHORD 매핑과 영구 파라미터 ID/순서는 유지한다. 기존 프로젝트의 저장된 호스트 사용자 매핑은 새 인스턴스와 별도 비교한다.

필터 종류는 C 왼쪽, LFO Shape/Target은 C 오른쪽 두 노브 위에 둔다. Drive 종류는 D 첫 노브 위, Tremolo 자유 속도 안내는 D 오른쪽에 둔다. LCD 값은29→24px, 선택값22px, 보조 숫자22px로 줄이고 기본 값 라벨은122px 폭으로 가운데 배치해138px 플레이트 안에 여백을 준다. 터치 영역은 유지한다. 실제 검증 결과는 implementation.md를 따른다.

## 0.1.20 — 코드 슬롯4–16개

- 기본4개에서 CHORD 오른쪽 위 `− / +`로4·8·12·16개를 선택합니다. 축소해도 각 슬롯 설정은 보존됩니다. 개수 변경 시 울리는 음은 정리합니다.
- MAIN은 큰 코드 패드4개를 유지합니다. CHORD PRESET 라벨 옆의 `‹ 1–4 / 16 ›`로 보이는 묶음을 바꿉니다. 프리셋 변경 화살표는 그 아래에 있습니다. 화면 페이지 변경은 물리 MIDI 입력과 독립적이며 사라진 화면 hold만 해제합니다.
- 첫 하단 MAIN 키를 한 번 누르면 CHORD 편집 화면으로 이동하고 다시 누르면 MAIN입니다. 편집 화면 안의1–4/5–8/9–12/13–16 버튼으로 묶음을 고릅니다. 메인과 편집 화면은 같은 페이지를 공유합니다. A–D는 보이는 네 슬롯의 Root/Type/Voicing/Octave를 조절합니다. 고정 Q-Link 편집 파라미터가 실제 선택 슬롯의 값을 중계합니다. 편집 중계 값은 automation 대상이 아니며 실제 슬롯 파라미터를 사용합니다.
- 선택 가능한 묶음은 활성 슬롯 수에 따라 늘어납니다. +로 활성 개수를 늘리면 추가 묶음 버튼이 나타납니다. 축소 시 현재 페이지를 활성 범위로 보정하고 숨긴 슬롯 설정은 보존합니다.
- 물리 패드는 각 슬롯 LEARN으로 지정합니다. 추가 슬롯의 기본 노트는 C Major 연속 배치이며 MPC 호스트의 Pad Perform 설정을 바꾸지 않습니다. 화면과 다른 묶음을 연주하면 작은 뱅크 활동 표시가 켜집니다.
- 코드 프리셋32개는 기존4코드 진행입니다. MAIN에서 선택하면 현재 보이는4슬롯의 루트/종류만 바꾸며 다른 묶음은 보존합니다. 네 묶음의 프리셋 선택은 각각 저장합니다. 보이싱/옥타브/패드 매핑은 유지합니다. MOD의 PRESET 1–4는 고정 Q-Link를 유지하는 첫 뱅크 전용입니다.
- 16슬롯은 저장할 코드 개수입니다. 동시 발음은16보이스이며 코드 중첩 시 voice stealing이 발생할 수 있습니다.
- 기존91개 인덱스를 보존하고 추가 슬롯·제어·활동 값을 append하여 총192개입니다. 예전 프로젝트는4슬롯/MAIN 패드1–4로 복원됩니다. Chord Slot/Pad Learn/완료 표시의 choice 범위 확장으로 이전 정규화 automation은 재확인해야 합니다. 기존 plain 상태 복원은 검사합니다.

검증 환경과 결과는 [구현 기록](../../docs/plugins/donut-theory/implementation.md)을 따릅니다. B 헤더 위 Q-Link 선은 사용자 요청대로 유지했습니다.

## 1.1.2 — 수동 패드 위치 안내

공통 CHORD 편집의 PAD LOCATION에서 A–H 뱅크와4×4 위치를 지정한 뒤 LEARN으로 실제 패드의 MIDI 노트를 연결합니다. MAIN의 연주 패드 안에 위치명과 작은 배열을 표시하며 미등록은 숨깁니다. 위치는 자동 감지 정보가 아닙니다. CLEAR LOCATION은 위치 안내만 지우며 MIDI 연결은 유지합니다. 메인에 코드별 EDIT를 추가하지 않았습니다. 기존192개 파라미터를 보존하고 위치16개를192–207에 append했으며 Q-Link/automation에서는 제외합니다. 저장·복원 및192개 상태에서의 마이그레이션을 검사했습니다. 1.1.2 배포에 포함하며 실기기 검증 범위는 구현 기록을 따릅니다.
