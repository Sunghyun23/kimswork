# Donut Theory v0.1.0 개발 알파 — 2026-11-13

## 현재 구현

- 제조사 `kim`, UID `Krql`, 68개 파라미터, 16보이스, 64개 내장 음색.
- 32개 진행(8개 기본 진행의 네 조옮김), 코드 슬롯별 루트/종류/보이싱.
- NOTES/CHORDS/SPLIT, Mono/Legato/Poly, 서스테인, pitch bend, glide, 드라이브 3종, LFO, chorus/delay.
- 음색/코드 진행의 독립 저장과 선택, 프리셋 이전/다음 버튼.
- 메인 16 Q-Link, 6개 기능키 + AMP의 CHORD 서브탭. 실제 JSON 스킨을 생성한다.
- 하단 **CHORD PRESET** 라벨 반영. 승인 디자인에서 분리 제작한 배경과 노브 자산 사용.

## 검증 결과

환경: macOS arm64, Python 3.13.7, Apple clang 16, Zig 0.17.1, SDK `febd2ca5bd9eb16c4a412837196e145566bc981b`.

| 검사 | 결과 |
|---|---|
| native dylib | 빌드 성공 |
| ARM32 / ARM64 | 빌드 성공, VSTPluginMain 확인, GLIBC 최대 요구 2.27 |
| SDK validator | 0 errors, 0 warnings |
| host_sim, 5초, 노트 48/52/55 | 19 PASS, 0 warnings, 0 failures |
| host_sim 블록 크기 | 32/128/512 통과 |
| 상태 round-trip | 68개 파라미터 복원, 재저장 byte-identical |
| 일반 C++ 회귀 테스트 | 통과, 오디오 콜백 new/new[] 금지 검사 포함 |
| UndefinedBehaviorSanitizer | 동일 회귀 테스트 통과 |
| AddressSanitizer | macOS sanitizer 초기화 CHECK 실패로 실행 불가. 통과로 간주하지 않음 |
| 스킨 자체 검사 | 16 매핑, 4 그룹 bounds, 6 기능키+서브탭, 라벨, 이미지 참조 통과 |
| TUI 오프라인 렌더 | 1280×628 MAIN 프리뷰 확인. 실제 호스트 검증 아님 |

host_sim의 Mac CPU 평균은 약 0.28%, 최악 블록 약 1.16%였다. 이 값을 MPC CPU 사용률로 환산하거나 보장하지 않는다. 렌더 peak 약 -15.40 dBFS, NaN/Inf·clipping·denormal 경고 없음.

회귀 테스트 범위: 코드 중첩 소유권, 서스테인 해제, mono 마지막 누른 음 복귀, 모드 변경 정리, 음색 변경 시 코드 보존, 사용자 수정 상태 복원, 이전/다음 버튼 wrap, spread 예약음 취소, sample-offset 이전 무음, 64개 음색 발음, 32개 진행의 네 패드 중첩.

## 남은 일 / 실제 검증의 한계

- 사용자 MPC 모델·OS 확인 및 실기기 설치/CPU/노브·터치·서브탭/저장·복원 검증.
- 프리셋별 음악적 청음 튜닝, gain matching, 강한 FM·왜곡의 aliasing 및 voice stealing 클릭 검증.
- 화면 패드는 선택/편집이다. 터치 발음은 구현하지 않았다.
- 별도 카테고리 필터, 프리셋별 매크로 라우팅 편집은 미구현이다. 현재 카테고리는 이름 접두사이며 매크로는 고정 범위를 사용한다.
- 승인 콘셉트와 실제 SDK UI의 폰트/선택기 표현 차이가 있다. 추가 미술 QA가 필요하다.
- 전체 해상도 knob 자산의 실기기 메모리/회전 렌더 비용을 확인해야 한다.
- 실제 사용자가 WAV를 듣고 음색을 확인해야 한다. 자동 발음 테스트는 청음 승인이 아니다.
- GitHub Pages, CI, GitHub Release 발행, 기기 설치는 수행하지 않았다.

## 재현

```sh
python3 scripts/bootstrap_sdk.py
python3 scripts/build_plugin.py DonutTheory --target all
python3 scripts/test_plugin.py
```

생성물은 프로젝트의 `build/`, `skins/`, `dist/`에 있으며 Git에서 제외한다. 빌드 종속성 안내와 설치법은 [프로젝트 README](../../../plugins/DonutTheory/README.md)를 따른다.

## v0.1.1 스킨 수정 — 2026-11-14

사용자 실행 화면에서 배경·파라미터 값·선택기는 표시되지만, 노브는 점으로 축소되고 그룹 제목 및 상하단 고정 라벨은 보이지 않는 것을 확인했다. 기기 모델과 OS는 아직 미확인이다.

- RotatingImage의 추가 imageScale을 64/1254에서 1.0으로 수정했다. 호스트 캡처는 이 배율이 bounds 크기에 다시 적용되는 현상과 일치한다.
- 고정 그룹의 빈 핸들 및 Hide 조건을 제거하고 고정 라벨에도 유효한 핸들을 지정했다. 라벨 텍스트와 파라미터 값은 별개이며 오디오 파라미터를 변경하지 않는다.
- 오프라인 프리뷰가 imageScale을 무시하던 문제를 수정했다. 이전 프리뷰는 실제 노브 축소를 잡지 못했다.
- UID와 파라미터 순서는 유지했다. 수정 후 실기기 표시 확인은 아직 필요하다.

사용자는 프리셋 화살표 및 메뉴 터치도 동작하지 않는다고 보고했다. 화살표는 부모 Toggle Switch 액션 대신 SDK 기본 Button 위젯으로 바꾸었다. 메뉴는 단일 탭 외에 SDK 표준 double-tap/Enter 경로도 추가했다. 이 변경으로 실기기 문제가 해결되었는지는 재설치 후 확인해야 한다.

## v0.1.2 버그 수정 및 재검증 — 2026-11-14

검토에서 재현한 프로그램 번호 불일치, 마스터 볼륨 초기화 차이, 다른 채널의 sustain 해제 간섭, CC121 mono fallback 누락을 수정했다. VST 진입점 어댑터는 프로그램 번호/이름을 SoundPreset에서 읽고, 프로그램 로드는 Volume을 보존한다. UID Krql, 68개 기존 파라미터 순서/ID는 유지했다.

내부 파라미터 변경은 원자적 플래그에 기록하며 UI 쪽 표시값·이름·프로그램·저장 조회 및 UI idle 호출에서 호스트 알림을 보낸다. 오디오 process에서는 호스트 UI 갱신을 실행하지 않는다. 구형 NeedIdle 요청은 SDK 시뮬레이터에서 미지원이므로 사용하지 않는다. 실제 MPC의 UI 조회 시점에 따른 갱신 지연은 실기기 검증이 필요하다.

사인 파형을 wavetable로 변경하고, 샘플 루프와 MIDI 처리에서 블록 파라미터 스냅샷을 사용한다. 16샘플 제어 카운터를 순환시켜 장시간 signed overflow를 제거했다. Sync Rate 표시, SYNC/DIV 메뉴, DRIVE/PLAY 구분선, 메뉴·노브 겹침도 보완했다. 메뉴는 native ComboBox와 더블탭/Enter 오버레이로 구현했다.

검증 결과:
- native·ARM32·ARM64 빌드 성공. SDK validator 0 errors / 0 warnings. uiversion은 호환성 버전 0.1.0.0 유지.
- 기존 엔진 검사와 추가 pedal/CC121/preset/volume/rate 검사: 일반·UBSan 통과.
- 신규 host_bridge_test: 실제 VST 진입점을 통해 프로그램/상태 복원, 볼륨 보존, 렌더 중 UI 콜백 없음, UI 조회 시 알림 전달, 재진입/중복 갱신 방지 검증. 일반·UBSan 통과.
- host simulator: 19 PASS / 0 warnings / 0 failures, peak -15.38 dBFS. Automate 109회 / UpdateDisplay 3회 확인. Mac CPU 평균 0.35%, 실기기 CPU 수치 아님.
- 스킨 검사: Q-Link, 이미지 참조, 라벨, native dropdown, 그룹 구분선, 선택기·노브 비겹침 통과.
- 오프라인 MAIN 프리뷰 확인. MPC 터치/ComboBox/배경·노브 표시 및 실시간 화면 갱신 검증은 아직 미완료.
- WAV 생성 및 유한값/클리핑 자동 검사는 통과했으나 음악적 청음 승인은 하지 않았다.

SDK 원본 수정, 기기 원격 설치, Git push, Pages, Release 발행은 하지 않았다. 설치 ZIP은 donut-theory-0.1.2-mpc.zip이다.

## v0.1.3 후보 / 실기기 조사 — 2026-11-14

검증 기기: **MPC Live III, OS 3.10.1.12, aarch64**. Remote Screen Viewer를 computer use로 조작했다. SD의 기존 설치를 SDK export로 백업하고 설치된 0.1.2 스킨·Q-Link·배경을 로컬 파일과 비교해 일치함을 확인했다. 이전 설치 잔여물만의 문제는 아니다.

### 0.1.2에서 직접 재현한 현상

- MAIN 프리셋 화살표가 반응하지 않음. 코드 이름 더블탭 메뉴는 빈 목록으로 열림.
- MOD의 Chord Preset 슬라이더는 진행을 바꾸며 MAIN의 이름과 네 코드가 함께 갱신됨. 코드 엔진 자체와 터치 목록을 구분해 조사함.
- MAIN 연속 노브의 값 변경은 동작함. Shape/Target은 그림과 값 표시의 갱신이 일치하지 않음.
- OSC 파형 라디오 버튼은 선택되지만 캡션이 보이지 않음.
- 배경은 원본 이미지 크기로 잘려 그려져 로고가 선택기와 겹침.

### 후보 수정 및 컴퓨터 검증

- 빈 native 메뉴 대신 명시적 선택 버튼 패널을 생성함. 음색 64개는 32개씩 두 페이지, 진행은 32개 한 페이지. 닫기 버튼을 제공함. 이 방식의 실제 기기 동작은 검증 대기.
- 화살표 명령을 처리한 후 0으로 되돌려 반복 입력을 허용함. 일반/UBSan 테스트에서 양쪽 선택기의 반복·순환 동작을 검사함.
- 모든 choice 변경에 UI 갱신을 예약함. 오디오 콜백에서 UI 호스트 호출을 수행하지 않는 규칙은 유지함.
- 고정 캡션의 핸들 연결을 보완하고 배경 런타임 PNG를 실제 콘텐츠 크기 1280×628로 생성함.
- native·ARM32·ARM64 빌드 및 SDK validator 통과. 엔진·VST 어댑터 일반/UBSan 통과. host_sim 19 PASS / 0 warning / 0 failure. 정적 스킨 검사 통과. 이것은 실기기 UI 통과를 뜻하지 않음.

### 설치·보존 상태

사용자 허가로 개발자 모드를 활성화하고 공식 `mpc_remote.py update`로 같은 SD의 설치를 갱신했다(30개 파일 전송 검증). SDK 원본은 수정하지 않았다. 사용자 허가 후 현재 프로젝트를 **[User] MPC Documents/Projects/donuttheoryqa20261114.xpj**로 별도 저장했다. 저장 진행창 종료를 확인한 다음 SDK `restart`로 앱을 재시작했다. 재시작 후 Remote Screen Viewer 재연결에서 인증 오류가 발생해 사용자에게 앱에서 직접 재연결을 요청한 상태다. 후보의 실기기 메뉴/배율 수정 결과는 아직 확인하지 못했다. 개발자 모드 종료도 검증 종료 후 확인해야 한다.

Git push, Pages 및 Release 발행은 하지 않았다.

재연결 후 QA 사본의 존재 및 로드를 화면에서 확인했다. 0.1.3 첫 후보에서 배경 크기/로고 겹침은 해결됐고, 더블탭 선택 패널도 표시됐다. 그러나 버튼 중앙은 무반응이었다. **같은 버튼의 라벨 밖 테두리를 누르면 선택되는 것을 재현**하여, 현재 호스트에서는 보이는 Label이 버튼 터치를 차단한다는 근거를 얻었다. SDK 문서의 '라벨은 터치를 가로채지 않는다'는 설명과 실제 기기 동작이 달랐다.

모든 native Button에 동일한 bounds·핸들·동작을 갖는 투명 터치 표면을 캡션 뒤(최상단)에 추가했다. MAIN 화살표/메뉴/코드 슬롯, 선택 패널, 상세 라디오 스위치를 함께 다룬다. 정적 검사에 터치 표면의 순서·범위·핸들 동일성을 추가했고 통과했다. 재빌드 및 host_sim 19 PASS 후 다시 설치(31개 파일 확인)했다. 두 번째 후보의 실기기 결과는 아래에 이어 기록한다.


### 0.1.3 최종 UI 수정 검증 — 2026-11-14

앞선 후보의 대기 상태를 아래 결과로 갱신한다. MPC Live III / OS 3.10.1.12에서 Remote Screen Viewer를 직접 조작했고, 마지막 설치는 SDK update에서 31개 파일 일치를 확인했다. 이후 앱 재시작과 QA 사본 로드를 완료했다.

| 범위 | 실제 관찰 결과 |
|---|---|
| MAIN 아트·노브·제목 | 1280×628 배경, 로고/선택기 비겹침, 16개 노브·4개 그룹 제목·CHORD PRESET 표시 |
| 음색 프리셋 | 이름 중앙 단일 탭, 64개 목록의 2페이지 전환·선택·DONE, 이전/다음 반복 및 마지막→첫 항목 순환 |
| 코드 프리셋 | 32개 목록 선택·DONE, 이전/다음 반복·순환, 네 코드 이름 동시 갱신, 음색과 독립 유지 |
| 상세 선택 막대 | OSC 음색 드래그: Round Tape→Nasal Soul, 값 이름 더블탭 목록: Sunday Keys 선택. MOD 코드 목록: Sunday Soul 선택. 이름·값 대비 정상 |
| OSC/FILTER/AMP/MOD/FX | 탭 표시, 파형·발음 모드 라디오 중앙 터치, ADSR 페이더, FX 6개 컨트롤 드래그, LFO enum/속도 표시 갱신 확인 |
| CHORD | AMP 두 번째 탭, 코드 종류 목록·보이싱 표시, 정수 Trigger Base, 네 패드 각각의 올바른 Root 오버레이, Root 1 변경 시 해당 코드만 변경 |
| 재시작/복원 | 저장된 QA 사본의 음색·코드 진행·수정 파라미터 복원 확인. QA 기준 파일을 덮어쓰지 않았음 |

최종 메모리상의 테스트 상태는 Keys / Sunday Keys, Sunday Soul이며 MAIN 화면으로 복귀했다. 현재 테스트 변경은 저장하지 않았다. 기준 사본은 `[User] MPC Documents/Projects/donuttheoryqa20261114.xpj`에 있다.

추가 수정: 코드 패드의 오버레이를 슬롯마다 Root 핸들로 연결했다. 상세 GenericLegacy slider는 자체 흰색 값 때문에 대비 설정을 무시하여 ColourBar+Name/Value로 교체했다. 라벨의 Data→Data 자기 참조가 값을 숨기는 현상을 제거하고 회귀 검사에 추가했다. SDK 원본은 수정하지 않았다.

최종 빌드: native/ARM32/ARM64 성공, validator 0 errors/0 warnings. 엔진·VST 어댑터 일반/UBSan 통과, host_sim 19 PASS/0 warning/0 failure, peak -15.38 dBFS. 스킨 회귀 검사 7그룹 통과. CPU 평균 0.29%는 Mac 시뮬레이터 값이며 MPC 수치가 아니다.

남은 검증: 물리 Q-Link 회전·뱅크, 실제 패드 벨로시티/외부 MIDI 키보드·페달, 기기 오디오 청음, 장시간 부하와 모든 프리셋의 음악적 밸런스. 원격 화면 클릭 검증을 물리 연주·청음 통과로 대체하지 않는다.

개발자 SSH는 검증 종료 후에도 켜져 있다. Info의 버전/시리얼/펌웨어 탭 시퀀스는 종료 스위치로 동작하지 않았고, SDK 읽기 전용 list 연결이 계속 성공했다. 시스템 명령으로 서비스를 임의 변경하지 않았다. 사용자에게 종료 미완료를 알린다.

## 0.1.4 — 가독성·터치 연주·리드/패드·FX (2026-11-14)

사용자 답변: 빈티지 아날로그 리드·패드 중심, 터치와 물리 패드 모두 코드 연주.

구현: 주요 글자 확대, 네 개 독립 터치 토글 게이트와 STOP, 별도 EDIT 1–4, MIDI와 분리된 터치 노트 소유권, 모드/음색 변경·reset 시 게이트 해제. Mono 일반 노트용 보이스를 예약했다. 기존 68개 파라미터 뒤에 Touch1–4/TouchStop/FXPreset을 추가했다. Lead 12개/Pad 16개 개별 레시피 재설계, FX 7개 레시피와 Custom을 추가했다. SDK 원본 변경 없음.

검증: native/ARM32/ARM64 빌드, validator 0 errors/0 warnings; 일반·UBSan 엔진/브리지 테스트 통과. 터치 발음 수, MIDI 동시 발음, 재탭/STOP/CC123/reset 정리, FX 레시피 적용과 Dry의 Motion 제거 검사 포함. host_sim 19 PASS/0 warning/0 failure, peak -15.38 dBFS, Mac 평균 CPU 0.29%. 스킨 회귀 7그룹 통과. 64개 프리셋 렌더 검사를 통과했으며 청음 승인은 아님.

공식 SDK update에서 31개 파일 일치를 확인하고 MPC 앱을 재시작했다. QA 기준 파일을 다시 불러와 Remote Screen Viewer로 다음을 관찰했다.

- MAIN 코드1 탭 주황색, 재탭 해제; 코드2·3 동시 켜짐 후 STOP으로 모두 해제.
- EDIT 1 중앙 탭으로 Chord 1 Root 오버레이 열림.
- FX Preset 값 더블탭으로 8항목 목록 표시. Space Echo 선택 후 Tape/Drive18/DriveMix70/Chorus18/Delay35/Feedback42/점4분음표/Space18 적용 표시.
- 음색 목록 두 페이지와 큰 글자 표시. Lead / Late Night, Pad / Velvet Chords 선택 후 MAIN 이름·파형·수치 동시 갱신.
- INPUT Split 선택 확인. 최종 MAIN은 Velvet Chords / Sunday Soul +5 / Split, 터치 게이트 전부 꺼짐. 테스트 변경은 저장하지 않았고 QA 기준 파일을 보존했다.

실기기 터치 게이트 표시를 오디오 청음 통과로 주장하지 않는다. 물리 MIDI 패드/키보드/Q-Link, 실제 출력 청음 및 장시간 CPU 검증은 남아 있다. 개발자 SSH는 여전히 켜져 있다. Git push, Pages, Release 발행 없음.

## 0.1.5 구현·자동 검증 — 2026-11-14

사용자가 최신 도넛 노브/4그룹 시안을 승인하고 구현 시작을 요청했다. 승인 시안과 별도 제작한 투명 도넛 노브/빈티지 배경을 저장소에 보존했다. 새 MAIN은 12개 회전 노브 + 4개 세로 ADSR 페이더이며 Q-Link 4×4와 일치한다. Shape/Target은 터치 메뉴, Vibrato/Expression은 작은 터치 컨트롤, Tone/Motion은 상세 화면이다.

OSC Pair 16조합은 기존 Wave1/Wave2와 동기화하되 Mix/Detune/Cutoff/ADSR을 보존한다. 독립 sine Tremolo를 Drive 뒤/Chorus·Delay 앞에 추가하고 Rate/Depth를 스무딩한다. 0%는 bypass다. 새 파라미터는 74–76 인덱스에만 추가했다.

SDK ui-reference §7.2에서 `ResetOnGestureEnd/<handle>`를 확인했다. 이전의 'release 이벤트가 없어 토글만 가능' 판단을 정정한다. MAIN Touch1–4 핸들에 이 modifier를 적용해 gesture 끝에 기본값 Off로 복귀하도록 구성했다. 기존 DSP의 떨어지는 gate가 Note Off를 발생시키고 Amp Release로 감쇠한다. 실제 기기의 modifier 처리는 재시작 후 검증해야 한다.

검증 결과: native/ARM32/ARM64 빌드 및 SDK validator 통과(0 errors/0 warnings), 일반·UBSan 엔진/브리지 통과, host_sim 19 PASS/0 warnings/0 failures, 77개 파라미터 상태 round-trip 통과. peak -15.38 dBFS, Mac 평균 CPU 0.37%(기기 수치 아님). Pair의 파형 변경·레거시 파형 동기화·다른 설정 보존, Tremolo 0% 출력 일치·100% 감쇠·필터 LFO 독립성·저장 복원, 터치 gate off 후 릴리스 테스트 통과. 스킨 회귀 8그룹과 오프라인 프리뷰 확인.

공식 SDK update로 SD 설치를 갱신했고 35개 파일 일치를 확인했다. 새 UI 스캔에는 앱 재시작이 필요하다. 기기 화면이 마지막 QA 상태와 달리 Golden Pluck으로 변경되어 있어 현재 사용자 작업 저장을 요청했다. 저장 완료 확인 전 재시작하지 않았다. 새 스킨/hold/ADSR의 실기기 결과는 아직 대기다. 0.1.5 ZIP CRC 및 35개 빌드 파일 일치 확인. SDK 원본 변경, push, Pages, Release 발행 없음.

## 0.1.5 실기기 재검증 및 미해결 항목 — 2026-11-14

사용자가 현재 변경 폐기를 허용한 뒤 공식 SDK로 재시작하고 저장된 QA 사본을 다시 열었다. 새 도넛 노브, ADSR 4개 페이더, OSC Pair 16선택, 독립 Tremolo Rate/Depth 표시를 확인했다. Attack 3→132.8ms, Decay 51.6ms, Sustain 50.4%, Release 359.7ms 등 드래그 변경, OSC Pair Saw+Pulse 선택과 다른 값 보존, Tremolo 15Hz/66.6% 변경, Shape Triangle/Target Pitch 선택, 음색/코드 화살표 전환 및 Velvet Chords 목록 선택을 확인했다.

사용자가 실제 Q-Link 매핑 이상을 보고했다. 이전 MAIN Row 배열을 Program Mode에 그대로 복사한 구성을 교정해 두 모드 모두 Column 뱅크 순서 A=(13,9,5,1), B=(14,10,6,2), C=(15,11,7,3), D=(16,12,8,4)로 통일했다. 테스트도 단순 1..16 배열 비교 대신 물리 뱅크 순서로 검사한다. 재빌드와 스킨 회귀 8그룹 통과, 공식 update 35파일 일치 후 재시작했다. 그러나 터치 후 Q-Link 포커스가 대응 그룹과 어긋나는 상태가 여전히 관찰됐다(Body 변경 후 B 강조 등). 이 변경만으로 해결됐다고 주장하지 않는다. 실제 물리 노브를 돌렸을 때 바뀌는 항목과 현재 Screen/Program 모드를 추가 확인해야 한다.

터치 코드: Instant→Hold, toggle→2-step radio, modifier를 부모 remap에서 Button 직접 핸들로 이동했다. 두 차례 Remote Screen 녹화에서 버튼이 켜지는 구간을 확인하지 못했고 오디오 샘플은 전부 0이었다. 두 번째 녹화에는 Body 드래그로 인한 화면 변화가 기록되어 녹화 자체는 동작했다. 따라서 ResetOnGestureEnd가 해당 호스트에서 실제 gate를 전달한다고 입증되지 않았으며 터치 hold 기능은 미해결이다. DSP gate-release 단위 검사를 실기기 발음 검증으로 대체하지 않는다.

현재 기기에는 위 진단 수정본이 설치되어 있다. QA 원본 파일은 덮어쓰지 않았다. 개발자 SSH는 켜져 있다. 물리 MIDI/Q-Link, 기기 청음 및 터치 hold를 완료로 표시하지 않는다. push/Pages/Release 없음.

### Q-Link 실기기 재점검 — 2026-11-14

사용자가 첫 노브 회전 시 OSC Pair 대신 LFO Shape가 바뀐다고 보고했다. 공식 SDK export로 SD 설치본을 `/tmp`에 가져와 Q-Links.json을 로컬 빌드와 바이트 단위로 비교했으며 일치했다. 설치본은 35파일이며 하드웨어 전용 Q-Link 변형 파일은 없다. 공식 list로 nvme와 MPCLIVE_SH도 확인했고 두 드라이브에 VST2 Plugins 폴더가 없어 중복 설치는 배제했다.

`version.xml`의 uiversion=0.1.0.0은 UI 수정 번호가 아니라 펌웨어 스킨 형식 호환성 버전이다(SDK ui-reference §2). 캐시 무효화를 위해 올리지 않는다. 현재 원인은 확정하지 않았으며, 물리 Q-LINK 버튼으로 설정 화면을 열어 실제 Screen/Program 모드와 저장된 할당을 확인하는 단계다. 설치 파일 일치만으로 실제 매핑 통과를 선언하지 않는다.

후속 원격 확인: Q-Link 설정 화면은 Screen 모드였고 Q-Link 16이 선택되어 있었다. Track 모드로 바꾸자 4×4 표에 OSC Pair/Body/Detune/Glide, Cutoff/Resonance/Drive/Space, ADSR, LFO Rate/Depth/Tremolo Rate/Depth가 정확히 표시됐다. 따라서 Program Mode 매핑 로딩은 실제 기기에서 확인했다. Screen 모드의 실제 파라미터 매핑은 아직 미확정이며, Screen과 Track의 파라미터 인덱스 해석 차이 가능성을 추가 확인한다.

원격 탐색 경로: MAIN → 왼쪽 주황색 Instruments 아이콘 → EDIT INSTRUMENT. 플러그인에서 상단 손잡이를 아래로 드래그 → 빠른 설정 오른쪽 화살표 → 현재 Q-LINKS 영역 → Q-LINKS → Screen/Track → 번호 선택으로 물리 버튼 없이 모드 전환 가능. 이는 설정 모드 선택 경로이며 실제 16개 할당 팝업과는 구분한다.

Screen 팝업을 추가 관찰한 결과, 실제 Q-Link 1–16은 Tone, Body, Motion, Space, Drive, Glide, Vibrato, Expression, Osc Mix, Detune, Filter Cutoff, Resonance, LFO Shape, LFO Rate, LFO Depth, LFO Target 순서였다. 첫 열 맨 위(Q-Link 13)가 LFO Shape여서 사용자 보고와 일치한다. 이 배열은 설치된 Screen 매핑과 다르며, 앞서 제기한 단순 ±1 오프셋 가설을 뒷받침하지 않는다. 기존 QA 트랙과 새 인스턴스를 비교하기 전까지 캐시 또는 저장된 매핑 여부를 단정하지 않는다.

### Q-Link 신규 인스턴스 검증 완료 — 2026-11-14

기존 QA 프로젝트의 Plugin 001을 보존한 채 Plugin 002를 새로 만들고 Donut Theory를 신규 로드했다. Remote Screen Viewer의 Q-LINKS 팝업에서 **Screen** 선택 상태 및 최신 16개 할당을 직접 확인했다: A OSC Pair/Body/Detune/Glide, B Filter Cutoff/Resonance/Drive/Space, C Amp ADSR, D LFO Rate/Depth/Tremolo Rate/Depth. 사용자도 물리 노브를 직접 돌린 뒤 “이제 모두 정상이야”, “했어. 정상임”으로 확인했다. 원격 화면에서 여러 그룹의 값 변경도 관찰했다. 스킨 회귀 검사 8개 항목 통과.

현재 설치된 매핑은 새 인스턴스에서 정상이다. 이전 QA 트랙의 구버전 배열과 신규 트랙의 정상 배열 차이는 기존 인스턴스/저장 상태에 남은 할당 가능성을 뒷받침한다. 저장 파일 내부의 원인이나 기존 트랙의 자동 마이그레이션은 검증하지 않았으므로 확정하지 않는다. 이번에는 소스·바이너리를 변경하거나 재설치하지 않았다.

업데이트 후 이전 배열이 남으면 먼저 기존 프로젝트와 음색을 보존하고 **새 플러그인 트랙에 신규 인스턴스**를 로드하여 확인한다. 정상 동작 확인 없이 기존 트랙을 삭제하거나 음색을 초기화하지 않는다. 현재 Plugin 001은 보존했고 Plugin 002가 선택된 MAIN 화면으로 돌아왔다. QA 기준 프로젝트 파일은 덮어쓰지 않았다. 신규 트랙과 테스트 값은 메모리에 있으며 저장/재로드 후 매핑 지속성은 아직 검증하지 않았다. 터치 코드 hold 문제의 해결 여부는 이번 Q-Link 확인과 별개다.

### 터치 hold 입력 계측 조사 — 2026-11-14 (진행 중)

사용자는 토글/자동 짧은 연주로 대체하지 않고 hold 동작을 계속 조사하도록 명시했다. 진단 전용 `DONUT_TOUCH_DIAGNOSTICS` 빌드와 `tools/diagnose_touch_skin.py`를 추가했다. 기존 ID/순서는 유지하며, 호스트 setParameter 콜백에서 원자적 카운터만 증가시킨다. process에서 증가시키는 프레임 카운터로 ON/OFF 간 처리 시간을 측정한다. 파일 I/O/로그/잠금은 사용하지 않는다. 일반 빌드에는 계측이 포함되지 않는다.

일반 Button Hold/Instant는 ON을 전달하고 해제 후에도 ON이 남았다. ResetOnGestureEnd를 붙이면 무반응처럼 보였지만, 계측에서 ON/OFF가 모두 전달되는 것을 확인했다. 따라서 'modifier 핸들이 무효여서 아무 입력도 없다'는 추정은 철회한다. 2초 원격 클릭에서 reset Hold toggle/radio 및 acceptsHWFocus Yes 변형 모두 OFF 1회, ON/OFF 사이 처리 프레임 0이었다. 사용자 실제 화면 터치 후에도 3번 진단 버튼이 `15:0`(OFF 누적 15회, 마지막 gate 길이 0ms)으로 나타났다. 오디오 블록이 gate ON을 관찰하기 전에 초기화되는 현상과 일치한다. Button의 Hold만으로 지속 게이트가 유지되지 않는다. 다음 비교는 연속 터치 Slider의 gesture 수명이다.

진단 스킨은 제품 UI가 아니며 포장/릴리스하지 않는다. 계측 패키지를 일반 0.1.5 배포 ZIP과 혼동하지 않는다. 진단 종료 시 일반 스킨과 비계측 바이너리를 다시 빌드/설치해야 한다.

후속 실기기 계측: 부모 `Mouse Down → Toggle Switch`는 원래 Touch 파라미터에 연결하고, 최상단 연속 입력 위젯은 `ResetOnGestureEnd/Parameter`에 연결한 조합에서 지속 게이트를 관찰했다. 2번 Knob 조합을 2초 원격 클릭한 결과 `1:1985`, 4번 Slider 조합은 `1:2011`이었다(OFF 횟수:ON→OFF 처리 시간 ms). 손을 뗀 뒤의 화면 갱신은 다음 조작 때 보일 수 있었다. STOP 후에도 측정값이 유지되었다. 이는 기존 Button 조합의 0ms와 다른 결과다. 실제 손가락 터치·청음, 드래그 시 조기 해제, 반복 터치, 다중 터치, 시각적 눌림 표시 검증은 남아 있다. 아직 제품 수정 완료로 간주하지 않는다.


### 터치 hold 정상 조합 확인 및 네 패드 적용 — 2026-11-14

4번 Slider 조합을 실제 손가락으로 누른 뒤 사용자가 “했음. 잘됨”으로 발음·해제를 확인했다. 후속 화면은 `2:888`로 마지막 실제 입력의 gate가 888ms 유지됐음을 표시했다. 요청한 약 3초와 실제 측정값을 혼동하지 않는다. 원격 2초 입력의 2011ms 계측 및 사용자 청음 확인을 함께 근거로 사용했다.

제품 스킨의 네 패드를 같은 Mouse Down → raw Touch Toggle Switch + 최상단 ResetOnGestureEnd Slider 방식으로 변경했다. Button은 raw gate에 연결한 상태 이미지로 두고 코드 이름 위에 투명 Slider를 배치한다. 이전 Button용 투명 hit 복제는 제거한다. 진단 숫자는 일반 빌드에서 사라지며 기존 파라미터/순서와 Q-Link를 보존한다. 원래 Button의 Hold가 지속 gesture를 보장한다는 주석은 제거했다.

native/ARM32/ARM64 일반 빌드 및 SDK 검증 통과. 엔진/브리지 일반·UBSan 통과, host_sim 19 PASS/0 warnings/0 failures. 기존 Button hit 전용 스킨 테스트는 새 chord 연속 gesture 표면 검사로 변경하고 8그룹 통과. 진단 스킨은 배포 패키지로 만들지 않았다. SDK update 후 QA 사본으로 최종 UI 재검증 중이다. 다중 터치/드래그 경계와 모든 음색의 청음 검증까지 완료했다는 의미는 아니다.


최종 일반 빌드 설치 후 QA 사본을 다시 열고 코드 이름 Gm9/C13/Fmaj9/Dm9 복원을 확인했다. Remote Screen 녹화에서 네 패드를 각 1.8초씩 순서대로 눌렀다. 10fps 프레임 분석상 주황색 활성 구간은 각각 12.3–14.0s, 15.1–16.9s, 18.0–19.8s, 20.9–22.7s로 각 누름 구간과 일치하고 해제 후 모두 꺼졌다. 녹화 오디오는 peak 0.2693, 마지막 1초 peak 0이었다. FX 잔향 때문에 발음 구간은 이어질 수 있으나 마지막 해제 후 완전 무음을 확인했다. 기기에는 진단 define 없는 일반 바이너리와 네 패드 수정 스킨이 설치되어 있다. QA 원본은 저장/덮어쓰기하지 않았다. 원격 오디오는 원래 Off로 되돌렸다. push/Release/ZIP 재생성 없음.

### 승인 디자인 복원 — 2026-11-14

실기기와 승인 시안 사이의 과도한 차이에 대한 사용자 지적에 따라 `tools/restore_design.py` 시각 후처리를 추가했다. Roboto Bold, 대문자 라벨, LCD 값 표시, 둥근 패널, 노브 눈금, 84px 도넛, 입체 코드/EDIT/화살표, 한 줄 헤더와 작은 로고로 복원했다. 변경되는 값/이름은 실제 파라미터 라벨이며 배경에 숫자를 구워 넣지 않았다. `main-background-v3.png`는 built-in image_gen으로 생성한 프로젝트 자산이다. 원본 도넛 노브 및 기존 hold 입력 조합을 유지했다.

native/ARM32/ARM64 및 validator 0 errors/0 warnings, host_sim 19 PASS/0 warnings/0 failures, 스킨 회귀 8그룹 통과. 실제 MPC에서 음색 화살표 Round Tape→Midnight Root, 코드 화살표 Sunday Soul+5→Basement Loop+5, 코드 목록 Sunday Soul 선택, Body 40→70.9% 드래그를 확인했다. Q 배지가 제목 첫 글자를 가리는 문제를 관찰하여 제목 시작 여백을 50px로 보완했다. 기존 QA 트랙의 이전 Q-Link 포커스 캐시는 별도 기록대로 남아 있으며 이번 디자인 변경에서 매핑 인덱스는 변경하지 않았다.

최종 제목 여백 수정본을 설치하고 재시작한 뒤 실제 표시를 확인했다. 새 위치의 4개 코드 패드를 각 1.5초 누른 녹화에서 활성 색 구간은 6.1–7.6 / 8.6–10.2 / 11.2–12.8 / 13.8–15.3초였고, 녹화 오디오 마지막 1초 peak는 0이었다. 디자인 변경 뒤에도 네 터치 hold/해제 동작이 유지됨을 확인했다. 실제 화면 캡처는 `plugins/DonutTheory/build/design-restored-device.png`(생성물, 미커밋)에 보관한다. QA 원본을 덮어쓰지 않았고 원격 오디오는 Off로 복원했다. push/Release 발행 없음.

### 물리 패드 연결 및 코드 보이싱 편집 — 2026-11-14 (검증 중)

사용자가 물리 패드의 코드 연결과 보이싱 편집 부재를 보고했다. 원격 화면에서 현재 Input=Notes를 확인하고 Split으로 변경했다. 이 설정에서는 Trigger Base부터 연속 네 MIDI 노트가 기존 코드 엔진을 사용한다. 실제 물리 패드 송신 노트 일치는 사용자 테스트 결과를 기다리며, 아직 물리 연결 해결로 단정하지 않는다.

`tools/chord_editor.py`를 추가하여 MAIN EDIT 1–4가 루트만 조절하는 오버레이 대신 루트·코드 종류·보이싱 및 Input/MIDI Base/Spread 설정 패널을 열도록 변경했다. 77개 파라미터의 ID/순서와 DSP, Q-Link 매핑을 변경하지 않는다. 기존 보이싱 엔진의 Root/First/Open 선택을 노출한다. 네 슬롯의 개별 선택 바인딩과 캡션 위 터치 표면을 스킨 검사로 확인했다. native/ARM32/ARM64 및 validator 통과, host_sim 19 PASS. 새 스킨 설치·실기기 편집 검증은 현재 설정 보존 여부 응답을 기다린다. 현재 기기는 이전 디자인 복원본이며 Input만 Split으로 변경돼 있다.

### A뱅크 하단 네 물리 패드 녹음 — 2026-11-14

사용자 요청으로 Remote Screen 오디오·화면 녹화를 켜고 A뱅크 하단 네 패드를 왼쪽부터 연주하도록 요청했다. 사용자가 완료를 알린 뒤 녹화를 종료했다. 당시 Golden Pluck / Poly / Split, After Hours+2 상태였다. 녹화 `Remote Screen 2026-11-14 18.16.48.mov`는 20.57초, peak 0.09283이며 발음 구간 네 개(약 5.1–5.7, 6.3–7.1, 7.5–8.5, 8.9–9.7초)를 검출했다. 강한 주파수는 약 33.0, 37.0, 41.4, 43.7Hz로 저음 C/D/E/F 계열 단음 패턴을 시사한다. 이는 합성음의 스펙트럼 분석이며 정확한 수신 MIDI 번호/패드 매핑을 입증하지 않는다. 화면 코드 패드의 색은 MIDI gate에 연결되어 있지 않으므로 화면 무반응을 MIDI 미수신으로 해석하지 않는다. MAIN 화면에서 실제 패드/스케일/옥타브 설정을 확인하도록 사용자에게 MAIN 이동을 요청했다. 원격 오디오는 Off로 복원했다.

MAIN → 우측 하단 4×4 아이콘 → PAD PERFORM에서 실제 설정을 확인했다. NOTES / C Major, A01–A04=C0/D0/E0/F0였다. 녹음의 C/D/E/F 계열과 일치하며, 플러그인의 연속 반음 트리거 배열과 스케일/옥타브가 달랐다. 사용자 요청의 패드 연결 문제 해결을 위해 CHROMATIC으로 바꾸고 OCTAVE +를 한 번 눌렀다. 화면의 A01–A04가 C1/C#1/D1/D#1로 바뀐 것을 확인했다. 변경 후 물리 네 패드 재녹화 중이며 최종 코드 발음 확인은 아직 대기다. 프로젝트 저장은 하지 않았다.

### 2026-11-14 후속 변경: 통합 코드 편집 및 패드 노트

사용자는 CHROMATIC/C1에서 물리 코드 발음을 확인했으나 호스트 패드 설정을 바꾸는 해결을 거절했다. MPC를 원래 NOTES / C Major / C0로 복원했다. 새 인스턴스는 Split과 개별 Pad Notes(24/26/28/29)를 기본으로 하며, 다른 호스트 배치는 각 슬롯 LEARN으로 등록한다. 기존 77-파라미터 상태는 연속 Trigger Base 라우팅을 유지한다. 새 파라미터 77–82를 끝에 추가했고 기존 ID/Q-Link는 그대로다. 등록 입력은 무음으로 소비하며 중복 노트는 두 슬롯을 교환한다.

앞선 EDIT 1–4 설계는 사용자 최신 요청에 따라 폐기하고 EDIT ALL 하나로 대체했다. 네 행에 Root/Type/Voicing/MIDI Note/LEARN을 동시에 표시한다. MAIN의 코드 패드/캡션/hold 터치 표면을 모두 195×70으로 확대했다. A 보조 컨트롤을 노브 위로 옮겨 값 표시 겹침을 제거하고 A/B 노브 높이, 패널 좌우 18px 여백, 하단 8px 안전 여백을 맞췄다.

0.1.6 native/ARM32/ARM64 빌드와 host_sim 19 PASS, 엔진/UBSan/스킨 검사를 통과했다. 현재 작업은 Qapads라는 새 QA 사본으로 저장 후 0.1.6 설치·재시작했다. 시작 화면의 원격 터치가 반응하지 않아 사용자에게 QA 프로젝트를 직접 열도록 요청했다. 정렬 후속 수정은 로컬 스킨까지 생성했으며, 실제 기기 표시와 통합 편집/새 MIDI 매핑 확인 전에는 완료로 간주하지 않는다.

### DRIFT 확정 변경 — 2026-11-14

사용자 승인: MAIN/Q-Link A를 OSC Pair / Body / DRIFT / Glide로 변경하고 Detune은 OSC 상세 페이지에 유지한다. DRIFT는 Wow+Flutter 피치 변조이며 Tremolo(음량 변조)와 별개다. 0.1.7에서 append-only 인덱스83, 기본0%, 제곱 강도 곡선, 느린0.43/0.137Hz와 빠른6.73Hz, 최대43 cents로 구현했다. 기존 프로젝트는0%, 새 상태는 저장값을 복원한다. SDK 공유 소스는 수정하지 않았다. 현재 기기 적용/실화면 확인 여부는 검증 기록을 따른다.

DRIFT 검증: native/ARM32/ARM64 빌드, 엔진 및 UBSan, VST 브리지, host_sim 19 PASS/0 warnings/0 failures, 스킨 회귀 통과. 0% 중립, 100% 편차 제한, 느린/빠른 성분, 50% 제곱 taper, 가변 127-frame 렌더의 음 변화 및 상태 복원을 검사했다. 공식 SDK update로 0.1.7을 기기에 설치하고 53개 파일 일치를 확인했다. 이번 업데이트 후 재시작은 하지 않았으며 실행 중 인스턴스의 DRIFT 표시·물리 Q-Link·실기기 청감은 미검증이다. 공개 push/Release 없음.

### 0.1.7 실기기 후속 검증 — 2026-11-14 19:00 KST

Mac 잠금/원격 인증 복구 후 기존 donuttheoryqa20261114 프로젝트를 열었다(Qapads 사본은 덮어쓰지 않음). 실제 MAIN에서 DRIFT 0→73.8→0→100→20.2% 드래그를 확인했고, A/B 노브·값 표시의 수평 정렬, 좌우 패널 여백, 커진 코드 패드 및 EDIT ALL 표시를 확인했다. EDIT ALL에서 Split/Pad Notes를 선택하고 Chord 1 Voice를 Root→Open으로 바꿨을 때 통합 편집 화면의 Open 반영을 확인했다. OSC 상세에는 Detune 24.6ct가 그대로 남았다. 이전 QA의 Notes/Legacy 설정은 로드 호환성에 따라 유지됐으며 테스트 중에만 Split/Pad Notes로 변경했다. 프로젝트 저장/원본 덮어쓰기는 하지 않았다.

원격 녹화 Remote Screen 2026-11-14 19.00.15.mov에서 동일 코드 패드를 DRIFT0%와100%로 각각6초 눌렀다. 오디오 28.02초, peak0.3590, 발음+효과 꼬리 구간 약0.6–8.0초/14.9–22.4초, 마지막1초 peak0.0. 녹음의 네트워크 dropout 표시는 최대3회였으므로 기기 DSP 결함으로 해석하지 않는다. 실제 캡처 build/drift-device.png, 비교 오디오 build/drift-device-comparison.wav. 오디오는 Off, DRIFT20.2%, Split/Pad Notes 상태로 종료했다. 물리 Q-Link 회전/물리 패드 LEARN은 원격 마우스로 수행할 수 없으므로 이번 실기기 확인에 포함하지 않는다. 코드 편집 오버레이에 MAIN Q-Link 포커스 테두리가 남는 호스트 표시도 관찰됐으며, 전면 정렬과 별도 항목이다.

### 헤더 도넛 콜라주 — 2026-11-14

사용자가 사진형 도넛을 평범하다고 평가하여, 거친 망점/어긋난 인쇄색/찢어진 슬리브/레코드/작은 SIDE A·33⅓ 스탬프의 콜라주로 교체했다. 배경과 분리된154×154 투명 자산을 MAIN1108,0에 배치하여 헤더 경계를 넘되 조작부와 겹치지 않게 한다. VOICE/INPUT은 각각792/936으로 이동한다. 기능 및 Q-Link는 그대로다. 로컬 미리보기 단계이며 기기 설치/실화면 확인은 별도다.

헤더 모서리 잔색 수정: SoundPrev/Next의 backgroundData=ffd6c298, SoundPreset/VoiceMode/InputMode=ffc9cd92가 투명 PNG 모서리 아래 노출되는 것을 확인했다. 해당 컴포넌트의 focused/unfocused 배경을00000000으로 변경하여 갈색 헤더가 보이게 했다. 올리브/크림 버튼 원본, 바인딩과 터치 bounds는 그대로다. 로컬 미리보기 및 스킨 검사 통과, 기기 적용 전.

### 0.1.8 배포 및 검증 — 2026-11-14

사용자가 구현 및 검증을 요청했다. 최종 헤더 디자인을 배포 스킨으로 생성하고 버전0.1.8/108로 올렸다. Native/ARM32/ARM64 빌드, SDK 검사, 엔진/UBSan 및 VST bridge, host_sim19 PASS(0 warnings/0 failures), 스킨 검사를 통과했다. 추가 검사로 헤더5개 컨트롤의 focused/unfocused 투명 배경, 축소 선택기 전체 자식의 일치 bounds, 우측318×126 이미지,94px 경계 및 섹션 바 양끝 불투명 채움을 확인했다.

공식 SDK update로 MPC의36C7-1ACA에 설치했고66개 파일 일치와 성공 종료를 확인했다. Mac이 잠겨 CUA가 두 번 차단되었으므로 현재 MPC 프로젝트 상태 확인/QA 저장/재시작/실기기 터치 검증은 진행하지 않았다. 사용자에게 Mac 잠금 해제를 요청했다.0.1.8 실기기 검증 완료로 보고하면 안 된다. 공개push/Release 없음.

### 0.1.8 실기기 검증 완료 범위 — 2026-11-14 19:30 KST

잠금 해제 후 화면이 직전 에이전트 QA 상태와 일치함을 확인했다. 이전 승인 범위에서 앱 재시작 후 저장된 donuttheoryqa20261114를 다시 열었다. 원본 프로젝트는 덮어쓰지 않았다. 실제 MPC Live III3.10.1.12에서 갈색 헤더/94px 경계/우측 full-bleed 그림/섹션 바 양끝 채움/헤더 밝은 사각 잔색 제거를 확인했다.

축소 헤더 중앙 터치로 Sound Preset 목록,Voice Mode,Input Mode 메뉴가 열렸다. 다음 화살표 Round Tape→Midnight Root,목록 Sunday Keys 선택,Input Split 선택 확인. 코드 다음 화살표 Sunday Soul+5→Basement Loop+5 확인. EDIT ALL에서 Pad Notes와 Chord1 Root→Open 보이싱 변경이 반영됐다. DRIFT0→32.1% 드래그 확인.

화면 네 코드 패드를각1.8초 눌러 녹화했다. Remote Screen2026-11-14 19.30.10.mov 분석: 활성 색 구간0.7–2.6/3.3–5.0/5.8–7.5/8.2–10.0초,오디오17.90초 peak0.34794,마지막1초 peak0.0. hold 해제와 잔향 종료 확인. 스크린샷 build/018-device-verified.png는 실기기 캡처다. 원격 Audio/Record Off로 종료했고 프로젝트 저장은 하지 않았다.

검증 한계: 물리 Q-Link 회전과 물리 패드 LEARN은 이번 원격 검증에 포함하지 않는다. 메뉴 위 MAIN Q-Link 포커스 테두리가 남는 기존 호스트 표시 현상은 유지된다. 공개push/Release 없음.


### 0.1.9 디자인 재작업 및 실기기 검증 — 2026-11-14 20:02 KST

사용자가 거절한 광택/접힌 구멍의 full-bleed 그림과 분리된 top-down natural 후보를 대체했다. 승인된 큰 도넛·중첩 레코드·사선 슬리브 구도는 유지하고 얇은 무광 코팅과 열린 타원 구멍으로 수정했다. `assets/header-donut-soul-final-v2.png`를318×126으로 배치한다. 배경은 실제 헤더와 비교해 두 번 보정했다. 원본/프롬프트는 assets/README.md. 독립 시각 리뷰와 native-size 프리뷰를 거친 뒤 MPC에 적용했다. 사용자 미적 최종 승인을 대신하는 객관적 판정이라고 주장하지 않는다.

Native/ARM32/ARM64 빌드와 SDK validator0 errors/0 warnings, host_sim19 PASS/0 warnings/0 failures, 스킨12개 검사 그룹을 통과했다. 리뷰에서 native-only 빌드에는 패키지가 만들어지지 않는 SDK 분기를 확인하여 이미지 정리는 all 타깃에서만 실행하도록 수정했고 native 경로도 실행 성공했다. UID/84개 파라미터 순서/16 Q-Link/오디오 알고리즘은 변경하지 않았다.

실기기 재로드에서 값 표시창과 EDIT ALL의 작은 Image 배경이 누락되는 문제를 발견했다. PNG/알파/bounds/복사본 해시는 정상이었고, 사용하지 않는 이미지 제거와 파일 이름 변경만으로는 해결되지 않았다. 장식의 실크기와 위치를 보존하면서 투명 캔버스를 노브146×144, ADSR146×168, 보조 값286×32로 확장하자 실제 기기에서 복원됐다. EDIT ALL은110×32로는 안 보였고110×64에서 복원됐다. 이는 이 기기에서 확인한 우회이며 SDK의 보편적 최소 크기 규칙이나 내부 GPU 원인을 확정한 것은 아니다. 별도로 DTKnob/DTChoiceKnob의 공유 children 리스트가 장식을 두 번 삽입하던 문제를 deepcopy로 수정했고 회귀 검사에 추가했다.

SDK가 assets의 제작 원본까지 모두 설치 폴더에 복사하여 패키지가50.9MB였다. 참조된 런타임 PNG만 남겨6.4MB로 줄였다. 제작 원본은 지우지 않았고 SDK는 수정하지 않았다. 최종 공식 update에서47개 파일 일치와 성공 종료를 확인했다. 반복 재시작은 기존 사용자 승인 범위에서 에이전트의 QA 조작만 버리고 수행했으며 원본 프로젝트는 덮어쓰지 않았다.

MPC Live III / OS3.10.1.12에서 실제1280×800 캡처 `build/019-device-verified.png`를 저장했다. 캡처의 헤더 그림 영역은 최종 런타임 PNG와 픽셀이 완전히 일치했다. 도넛의 구멍·빵 테두리·레코드가 구분되고 INPUT/B 제목/DRIVE 메뉴를 가리지 않는다. 값 표시창과 EDIT ALL 테두리도 모두 표시된다.

직접 터치 검증: Sound Preset 메뉴에서 Sunday Keys 선택, Chord Preset 오른쪽 부분(확장한 EDIT 이미지 아래) 터치 및 Sunday Soul 선택, EDIT ALL 열기와 Split/Pad Notes 선택, 그림 아래 DRIVE 메뉴 Tape→Tube, DRIFT0→29.7%, Attack3.0→10.9ms 드래그. 화면 네 코드 패드를 각1.6초 눌러 `Remote Screen 2026-11-14 20.01.43.mov`로 녹화했다. 오디오22.77초 peak0.23950, 마지막1초 peak0.0으로 발음/해제 후 잔향 종료를 확인했다. Audio/Record Off, Sunday Keys/Poly/Split/Pad Notes 상태로 종료했다.

검증 한계: 이번 작업에서 물리 Q-Link 회전/물리 패드 LEARN/다중 터치를 재검증하지 않았다. 메뉴 위에 MAIN Q-Link 포커스 테두리가 남는 기존 호스트 표시 현상은 유지된다. 공개push/Release 없음.


### 0.1.10 사용성 개선 — 2026-11-14

사용자가 전체 UI 검토 제안을 모두 승인했다. 승인한 투명 헤더 배경(header-collage-transparent.png)을 유지한다. MAIN 메뉴 이미지에 펼침 표시를 넣고, 선택 캡션이 화살표와 겹치지 않도록 조정했다. 전면 STOP을 제거하고 네 코드 패드를222×70으로 확대하여 오른쪽20px 여백까지 사용한다. 패드마다 슬롯 번호와 HOLD TO PLAY를 표시한다. EDIT ALL 표시를 EDIT CHORDS로 변경했다.

D 그룹은 LFO SETTINGS와 독립 TREMOLO로 명확히 구분한다. Shape/Target은 전면에 유지하고 Sync/Division은 LFO SETTINGS에 둔다. 독립 Tremolo는 자유 Hz이며 Depth0=Off를 표시한다. Voice/Input 선택 화면에는 각 모드 설명과 Poly에서 voice 재사용 시 Glide가 적용되는 조건, AMP의 Glide Mode 경로를 명시한다. FX 화면에 Space가 Chorus 폭/Delay wet에 영향을 준다는 안내를 넣었다.16 Q-Link 순서/UID/기존84개 파라미터 순서는 유지한다.

코드 편집은 각 행 왼쪽에 누르는 동안 연주하는 미리듣기 패드를 둔다. 원래 검증된 Mouse Down+ResetOnGestureEnd Slider 계약을 복제하며 새 토글 재생을 만들지 않는다. ROOT에는 루트 음이름/옥타브만 표시하고 완성된 코드 이름은 미리듣기 버튼에 둔다. 기존 Root 파라미터 표시/저장 형식은 유지하고61개 조건부 루트 라벨로 UI 표시를 분리했다. Pad Note에는 MPC 표기(24=C0)와 MIDI 번호를 함께 표시한다. LEARN 버튼의 선택 색과 상단 상태 문구로 대기/등록 완료를 알린다. 완료 상태용 파라미터 LearnStatus를 인덱스84에 append했다.

Legacy Base 표시는 편집 화면에서 제거하고 ADVANCED에서 CONSECUTIVE/INDIVIDUAL 호환 라우팅을 선택한다. ALL NOTES OFF 역시 ADVANCED에 두며 기존TouchStop 인덱스72를 유지한 채 모든 채널의 held/sustain 상태·터치 게이트·보이스·delay/chorus 꼬리를 정리한다. 프로젝트 상태는85개 파라미터를 그대로 왕복 복원한다. 저장 당시 활성 LEARN도 복원하므로 기다리는 슬롯은 강조와 상태 문구로 표시되고 CANCEL LEARN으로 해제한다.

Native/ARM32/ARM64 빌드, SDK validator0 errors/0 warnings, 엔진/UBSan/VST bridge, host_sim19 PASS/0 warnings/0 failures 및 UI13개 검사 그룹을 통과했다. 전체 입력을 누른 상태의 ALL NOTES OFF 회귀 검사, 네 미리듣기 hold 연결, 조건부 루트 라벨, 신규 피드백 인덱스 검사를 추가했다.

공식 SDK update로0.1.10 파일을 MPC 설치 폴더에 전송했다. Mac 잠금으로 CUA가 두 차례 차단되어 현재 프로젝트 상태 확인·재시작·이번 버전의 실제 화면/터치 검증은 아직 수행하지 않았다. 수정 완료/실기기 검증 완료를 동일하게 보고하지 않는다. 현재 실행 중인 MPC 인스턴스는 재시작 전 버전이다. 로컬 프리뷰 build/main-preview.png 및 chord-editor-preview.png는 근사 렌더다. 공개push/Release 없음.


### 0.1.11 공장 음색 전면 교체

사용자 요청으로 기존64개 공장 음색을 제거하고 Init1/Bass12/Keys15/Synth12/Lead12/Pad12의 새64개 레시피로 교체했다. Lowfilter는 사용자 확인에 따라 Low-cut(불필요한 저역 정리)이다. 전면/OSC의 SOUND PRESET을 카테고리 먼저 선택하는2열 브라우저로 교체했다. MPC 공통 상단 팝업은 SDK에서 직접 변경할 수 없는 호스트 UI이므로 category/name 내장 프로그램 목록을 제공한다.

LowCut85번, OutputTrim86번을 append했다. Init은 단일Saw, Low-cut0, 공간/왜곡/변조Off. 나머지 음색은 음역별12dB/oct 스테레오HPF를 사용하고 FX에서 조절/해제 가능하다. 기존DC 블로커의 약35Hz 감쇠가 베이스와 충돌하므로 약5Hz로 낮췄다. 기존 상태에는 새LowCut/OutputTrim을0으로 복원한다. 입력모드/코드 진행/Pad Note/마스터 볼륨은 음색 전환과 독립적이다.

명시적 파형/엔벌로프/FM/필터/레벨 레시피와 카테고리별 목적은 presets.md와 tools/sound_bank.py에 기록했다. 기준 연주에서 Bass/Lead 피크−15.1~−15.0dBFS, Keys/Synth/Pad−12.2~−11.9dBFS. 짧은 플럭은 평균음량을 강제 정규화하지 않는다.64개 각각2초hold/3초release 오프라인 렌더, 비무음/NaN·Inf/범위 검사 및 low-cut 보존/감쇠 회귀 통과. 음악적 청감 최종 승인을 의미하지 않는다.

카탈로그 생성기가 plugin.json 버전을0.1.9로 덮어쓰는 기존 문제를 발견하여 제거했다. 선언 버전과 바이너리 metadata 일치 검사도 추가했다. Native/ARM 빌드, SDK validator, 엔진/UBSan/VST bridge, host_sim19 PASS/0 warnings/0 failures, 카테고리/Init/필터/UI 검사 통과. 전체 기록은 presets.md.

Mac 잠금 때문에 실제MPC 카테고리 터치/청감 검증은 남아 있다. 공식 SDK update로0.1.11을 설치 폴더에 전송하되 프로젝트 상태를 볼 수 없어 재시작하지 않는다. 실행 중인 인스턴스는 이전 버전이다. 공개push/Release 없음.

최종0.1.11 전송은 SDK update 종료0 및65개 파일 일치를 확인했다(설치 폴더7.6MB). 재시작은 보류했다. 로컬 브라우저 미리보기는 build/preset-browser-categories.png 및 preset-browser-keys.png, 실제 오프라인 음색 샘플은 build/factory-demo.wav이다.


### 0.1.12 코드 편집 Q-Link — 2026-11-14

사용자는 A–D=코드1–4, 각 그룹 Root/Type/Voicing/Pad Note 순서의 Q-Link 편집을 승인했다. SDK UI 계약의 Q-Links.json은 실제 탭/서브탭별 정적 매핑이며 팝업 가시성에 따라 매핑을 바꾸는 API가 없다. 기존 MAIN 팝업 위에 코드 컨트롤만 올려두고 MAIN 신스 매핑을 유지하는 잘못된 구성을 피하기 위해 실제 CHORD 서브탭으로 편집기를 옮겼다.

진입: 하단 MAIN을 한 번 더 누르면 CHORD. 같은 첫 번째 탭을 다시 누르면 MAIN으로 돌아온다. EDIT CHORDS 버튼은 이 이동 방법을 안내한다. 이전 AMP의 구형 CHORD 서브탭은 같은 새 CHORD 화면으로 대체했다. Screen Mode (Tab1,SubTab2)에서 A=(13,9,5,1): Root1/Quality1/Voicing1/PadNote1, B=(14,10,6,2): 슬롯2, C=(15,11,7,3): 슬롯3, D=(16,12,8,4): 슬롯4. Bank Direction=Column. MAIN(Tab1,SubTab1)과 Program/Track의 기존 신스16개 매핑은 변경하지 않았다. 화면별 전환은 Q-LINK SCREEN 모드에서 사용해야 한다.

행별 qlinkBoundsData와 마지막 편집 슬롯에 따른 배경 강조를 적용했다. A·1/B·2/C·3/D·4 미리듣기 버튼을 표시한다. Type/Voicing 컨트롤은 선택 목록을 열기 전에 해당 파라미터의 Q-Link bank를 선택하도록 했고, 미리듣기 역시 해당 슬롯 Root bank를 선택한다. ROOT/Pad Note는 기존 knob의 Q-Link action을 유지한다. LEARN/입력모드/미리듣기 gate는16개 할당에 포함하지 않는다.

화면 미리듣기를 누른 상태에서 Root/Type/Voicing이 바뀌면 기존 보이스를 재사용하여 새 코드를 들을 수 있다. 줄어든 구성음은 정상 Release로 종료하며 MIDI로 누른 다른 음의 소유권과 분리한다. Pad Note/라우팅 변경은 MIDI 소유 음과 서스테인만 정리하고 화면 미리듣기는 유지한다. 손을 떼면 기존 hold 계약으로 해제한다. 터치로 유지 중인 코드의 프리셋/파라미터 변경도 이 갱신에 포함되며 MIDI로 유지 중인 코드는 다음 트리거부터 변경된다. 새 파라미터를 추가하지 않고87개 인덱스/UID를 유지한다.

Native/ARM32/ARM64 빌드 및 SDK validator, 엔진/UBSan/VST bridge, host_sim19 PASS/0 warnings/0 failures, UI 회귀 검사를 통과했다. 회귀 검사에는16개 코드 매핑과 MAIN/Program 매핑 보존, 행 bounds, popup 오인 사용 방지, Root 옥타브 변경 시 실제 오디오 zero-crossing 증가, Type/Voicing 보이스 중복 방지, Pad Note 변경 중 touch hold 유지/MIDI 음 정리/해제 검사를 포함했다.

Mac 잠금으로 실제 기기 탭 전환·새 Type/Voicing 터치·물리 Q-Link·페이지 전환 중 hold 종료는 아직 검증하지 못했다. 파일 전송 후 앱 재시작은 현재 프로젝트 상태 확인 전까지 보류한다. build/chord-editor-preview.png는 오프라인 근사 이미지다. 공개push/Release 없음.

0.1.12 최종 전송: 공식 SDK update 종료0,65개 파일 일치 확인. TUI/Q-Links 소스와 빌드 산출물 동일 및 manifest/binary112 일치 확인. Mac 잠금으로 재시작은 수행하지 않았다.


### 0.1.16 실기기 UI·초기 화면·Q-Link 검증 — 2026-11-14

사용자 승인에 따라 현재 QA 변경을 버리고 공식 SDK update/restart로 MPC Live III / OS3.10.1.12에 적용했다. 잠금 해제 후 실제 Remote Screen을 조작해 확인했다. 아래 결과가 앞선0.1.10–0.1.12의 기기 미검증 기록을 대체한다. 공개push/Release는 하지 않았다.

발견 및 수정:
- 시작/EDIT CHORDS 안내가 MAIN을 덮고 CLOSE도 동작하지 않았다. SDK에 없는 탭 이동을 안내 모달로 대체했던 구성을 제거했다. MAIN이 첫 탭이며 CHORD는 첫 번째 하단 키를 다시 눌러 연다. 안내 버튼/시작 팝업을 생성하지 않는다. 재시작 후 QA 프로젝트 로드와 새 Plugin002 인스턴스의 최초 진입 모두 MAIN을 직접 확인했다. 호스트가 보존하는 기존 인스턴스의 모든 뷰 상태를 DSP가 강제로 재설정하는 API가 있다는 뜻은 아니다.
- 카테고리 브라우저의 투명 전체화면 하위 컴포넌트가 카테고리/DONE의 터치를 가로챘다. 하위 목록·첫 안내 레이어에 ignoreMousePresses를 적용했다. Init/Bass/Keys/Synth/Lead/Pad 목록 표시, Ribbon Whistle/Init 적용, DONE 복귀를 직접 확인했다.
- LEARN 선택 색만 바뀌고 상태는 Ready에 머물렀다. native choice replacementStrings가 동적 formatValue를 덮는 문제를 피하도록 PadLearn을 직접 따르는 조건부 대기 문구를 적용했다. LEARN2에서 PRESS PHYSICAL PAD FOR SLOT2 및 CANCEL 후Ready 복귀를 확인했다. 이번 QA에서 실제 물리 패드 학습 완료는 다시 수행하지 않았다.
- 선택·설정 팝업 위로 배경 Q-Link 가이드가 남았다. 모든 DTChoicePanel 및 DTLfoSettings/DTChordAdvanced를 version4/hideQLinkBounds로 변경했다. 필드 순서도 SDK 규칙대로 유지한다. 팝업을 닫으면 실제 페이지의 가이드가 복원된다.
- OSC/FILTER/AMP/MOD/FX의 기본 가이드가 빈 하단 공간까지 내려갔다. 가이드 폭을288px로 늘려 컨트롤을 포함하고 높이는 해당 열의 마지막 컨트롤+8px로 끝낸다. 행별 원래 파라미터/Q-Link 번호는 유지했다.
- CHORD의 마지막 편집 행 테두리가 현재 native Q-Link 가이드와 서로 다른 행을 둘러쌌다. 별도 행 테두리를 제거해 native 가이드 하나만 사용한다.
- 어두운 상세 선택 버튼의 캡션이 검정으로 표시돼 대비가 부족했다. 버튼 안의 segment 캡션만 크림색으로 바꾸고, 밝은 배경 위의 파라미터 제목은 짙은색으로 유지한다.

실제 화면 확인 범위:
| 화면 | 결과 |
|---|---|
| MAIN | A–D 실제 그룹 영역, 새 Plugin002에서 Body→A / Filter Cutoff→B / Amp Attack→C / LFO Rate→D |
| CHORD | 실제 행별 guide, Type/Voicing 선택 후 해당 행으로 복원, 단일 테두리 |
| OSC | 전체 첫 열과 짧은 D열, 밝은 버튼 캡션/짙은 제목, Sound Preset 선택 팝업 |
| FILTER | A열3개/C·D열2개 높이에 맞는 guide, 빈 하단 영역 제외 |
| AMP | C·D의3개 열과 A·B의4개 열 높이, 선택 버튼 가독성 |
| MOD | 두 줄/세 줄 열의 다른 높이, Chord Preset 팝업에서 가이드 숨김 |
| FX | 열별 높이, FX Preset 팝업에서 가이드 숨김, Init Low CutOff/효과Off |
| 기타 팝업 | Sound Preset, Chord Type/Voice, LFO Settings/Division, Advanced에서 배경 가이드 없음. native 노브 확대 팝업도 없음 |

기존 QA Plugin001의 MAIN에서는 Body를 눌러도 B가 표시되는 오래된 저장 매핑이 남아 있었다. 원본 트랙을 보존하고 Plugin002를 새로 만든 뒤 Body→A를 포함한 네 그룹 정상 동작을 확인했다. 기존 프로젝트에 저장된 사용자/구형 호스트 매핑을 자동으로 덮어쓰지는 않는다. 새 인스턴스와 비교해야 하며 이 호스트 상태를 플러그인 소스 수정만으로 마이그레이션했다고 보고하지 않는다. 최종 원격 화면은 Plugin002 / MAIN / Lead–Ribbon Whistle이며 Audio·RecordOff다. 프로젝트 원본 파일을 덮어쓰지 않았다.

CHORD 미리듣기4개 각1.8초, MAIN 패드4개 각1.5초의 원격 hold/release를 실제 기기 오디오와 함께 녹화했다. MOV의 원본 PCM을 추출해44100Hz/stereo/54.004초를 확인했으며 발음 구간은9–21초 및32–44초, peak0.26758, 마지막3초 peak0(무음)이다. 실제 손가락 다중 터치, 물리 노브 회전, 화면 audition을 손가락으로 유지하며 동시에 Q-Link 편집하는 실기기 조합은 이번 원격 검증 범위 밖이다. 녹화는 build/qa-0.1.16/touch-hold.mov, 오디오/분석JSON 및 실제 화면 캡처도 같은 폴더에 둔다.

최종 Native/ARM32/ARM64 빌드, SDK validator0 errors/0 warnings, 엔진/UBSan/VST bridge, host_sim19 PASS/0 warnings/0 failures와 UI 회귀 검사 통과. 추가 검사에는 popup guide suppression, 처음 MAIN/안내 없음, 전체 카테고리 레이어의 터치 투과, 전체28개 중 상세20개 가이드의 컨트롤 포함/마지막 행 종료, 라디오 캡션 대비, 중복 행 테두리 없음이 포함된다. 마지막 UI 보완 후 스킨 검사를 다시 통과했다. 실기기 성능 수치는 측정하지 않았다.


### 0.1.17 — SPLIT 레가토, 전위·옥타브, LFO 중복 메뉴 정리 (2026-11-14)

사용자가 LEGATO인데 폴리처럼 들린다고 지적했다. 원인은 mono()가 InputMode=NOTES에서만 참이어서 SPLIT의 일반 MIDI 입력까지 폴리로 발음한 것이다. NOTES/SPLIT의 일반 멜로디는 Mono/Legato/Poly를 따르고 MIDI 코드·터치 코드는 독립 다성 발음으로 유지한다. Mono/Legato 멜로디용voice0을 코드 allocation에서 제외하고 fallback은 코드 트리거를 무시한다. Note-off/서스테인/CC121은 각 MIDI 소유 음을 해제하며 누르고 있는 화면 코드를 유지한다. 이전 Release 중에 새 음을 누른 경우는 실제 겹친 음이 아니므로 Attack을 재시작하도록 보완했다.

보이싱은 기존 저장값0=Root/1=First/2=Open을 유지하고3=2nd/4=3rd/5=4th inversion을 추가했다. ChordVoicing.h에서 실제 코드 구성음을 bass로 만드는 음 배열을 만들고 Spread는 낮은 음부터 발음한다.3화음/4음/5음 코드에2/3/4전위까지 제공한다. 현재13화음은5음 구성이다. 코드 종류를 바꿔 존재하지 않는 전위가 되면 가능한 마지막 전위로 보정한다. 별도 OCT는 각 슬롯의 -2..+2oct, 신규 인덱스87–90, 기본 저장값2=0oct다. Root/Pad Note는 유지하고 화면 audition 중의 옥타브 변경도 기존 보이스를 재사용해 반영한다. 이전87개 파라미터 상태를 이미 옥타브가 바뀐 인스턴스에 불러와도 누락된 새 옥타브 값은0oct로 복원한다.

기존 Voicing의 choice 범위가3→6으로 확장됐다. SDK의 plain/normalised 동시 저장 계약을 통해 이전Root/First/Open 프로젝트 값 복원을 검사했다. 이전 버전에서 해당 파라미터에 기록한 정규화 automation 값은 확장된 choice 범위에 맞게 재확인해야 한다. UID·기존 파라미터ID/순서와 MAIN/CHORD의16 Q-Link 매핑은 유지한다. OCT는 터치 선택이다.

MAIN의 D·LFO SETTINGS는 전면 Shape/Target과 MOD의 Sync/Division을 중복한 메뉴였다. 팝업/CHANGE 버튼을 제거하고 D·LFO/TREMOLO 정적 제목으로 바꿨다. Shape/Target은 MAIN, Sync/Division은 MOD에서 유지한다. 독립 Tremolo는 여전히 자유 Hz다.

실제MPC Live III3.10.1.12에 공식 SDK update/restart로 적용했다. 저장된 구형 Q-Link 매핑을 피하기 위해 새 untitled QA 프로젝트/Plugin001 인스턴스를 만들었으며 원래 QA 프로젝트 파일은 덮어쓰지 않았다. 새로 열 때 MAIN/안내 없음, 중복 LFO 메뉴 제거, MOD Sync/Division 존재, 코드1–4의 독립 전위/옥타브 선택과 표시, 선택 팝업의 가이드 숨김·닫은 뒤 행 가이드 복원,5음 코드→3화음 전환 시 유효 전위 목록·자동 보정을 직접 확인했다.60×44 옥타브 Image가 기기에서 누락돼 기존 패딩 이미지 방법(150×64 투명 canvas)으로 테두리를 복원했다. 네 슬롯의 서로 다른 옥타브(+1/-1/+2/-2) 및 전위(2nd/3rd/4th/Open)가 동시에 표시되는 실제 캡처를 저장했다.

LEGATO+SPLIT/Init에서 사용자가 물리A13/A14를 겹쳐 누르고 나중 음을 먼저 떼는 테스트를 수행하고 ‘정상동작함’으로 확인했다. 실제기기MOV/PCM을 보존했으며44100Hz/stereo/63.736초,peak0.083984,마지막3초peak0이다. 이것은 사용자 확인+기기 오디오 기록이며 화면만 보고 물리 연주를 검증했다고 추정한 결과가 아니다. 물리Q-Link의 신규 전위 전체 스텝 회전과 모든 음역/동시다중터치 조합은 이번에 전수 검사하지 않았다.

최종 native/ARM32/ARM64 빌드 및 SDK validator0errors/0warnings, 엔진/UBSan/VST bridge, host_sim19 PASS/0warnings/0failures, UI 회귀 검사 통과. 새 검사는 SPLIT의 단음·독립 코드·채널별pedal·보이스 고갈·떼고 다시 누르기, Cmaj9 모든 전위와 -2..+2oct 실제 음 배열, 구버전87개 상태/옛 normalised voicing 복원, 음색 전환 시 코드 옥타브 보존, held audition의 옥타브 변경에 따른 주파수 증가, 구성음별 전위 목록, 기존16 Q-Link 보존, 작은 옥타브 배경 패딩을 포함한다.

증거: build/qa-0.1.17/{chord-editor.png,voicing-menu.png,main-final.png,legato-physical.mov,legato-physical.wav,legato-physical-audit.json}. 마지막 호환 보완도 설치·재시작했고 MAIN/Init/Legato/Split/코드옥타브0 상태로 두었다. Audio/RecordOff. 공개push/Release 없음.

### 0.1.18 — CHORD Octave Q-Link, PAD LEARN 갱신, 왼쪽 목록 (2026-11-15)

CHORD Screen의 각 슬롯 네 번째 Q-Link를 Pad Note에서 Chord Octave로 변경했다. A–D=슬롯1–4, Root/Type/Voicing/Octave, 물리 Column 번호와 MAIN/Program/Track 신스 매핑은 유지한다. 행 guide 폭은708px(x96..804)로 MIDI Note를 제외한다. 사용자 추가 요청으로 MIDI Note 도넛을 제거하고 OCT 도넛(56px 그림,76×90 컨트롤)으로 대체했다. MIDI Note는 값 표시와 더블탭 수동 지정/LEARN을 유지한다. UID/91개 파라미터 순서·ID·상태 형식은 바꾸지 않았다.

PAD LEARN 진단: 물리 입력 후 LEARN 대기/이전 MIDI 값이 화면에 남았으나, native MAIN을 거쳐 플러그인으로 돌아오니 등록 완료와 새 MIDI 번호가 표시됐다. 입력 실패로 단정하지 않고 UI 갱신 경로 문제로 분리했다. 기존 idle/표시문자열 요청 외에 호스트 getParameter 값 조회에서 큐에 모인 변경 알림을 배출한다. process()가 실행된 native 스레드 토큰 최대4개를 lock-free 원자값으로 기록하고 해당 스레드의 getter에서는 알림을 배출하지 않는다.4개 초과시 getter 배출을 보수적으로 비활성화한다. 호스트 idle 요청도 entry/effOpen에서만 보낸다. 할당/잠금/새 스레드/host UI 알림을 오디오 process()에 추가하지 않았다. 실제 별도 렌더 스레드를 사용하는 bridge 회귀 검사로 렌더/getter 알림 금지와 UI getter 알림 발행을 검사했다.

실기기 확인: 공식 SDK update/restart 후 새 untitled QA Plugin001을 열었다. 사용자가 LEARN1 대기에서 A1을 한 번 누른 직후, 화면 이동이나 다른 컨트롤 터치 없이 `Pad 1 registered — press again to play`, MIDI72(C4), LEARN 선택 해제를 확인했다. 이 입력은 등록용으로 무음 소비한다. 후속 발음 자체의 새 녹음은 이번 확인에 포함하지 않는다. MPC Pad Perform 설정을 바꾸지 않았다. 현재 입력값은 그때 들어온 실제 MIDI72이며 모든 MPC의 A1이 항상72라는 뜻은 아니다.

왼쪽 목록: 공식 MenuOverlay를 Voice Mode에 연결한 실기기 시험에서 왼쪽 native 프레임/체크는 표시됐으나 세 항목 캡션은 비어 있었다. 고정 SDK/현재 기기 조합의 이 결과를 보존하고, 자체426px 사이드바로 구현했다. 전체 페이지 교체 없이 뒤쪽 편집 화면을 dim 처리한다.58px 행/최대8개, 흰 글자/현재 항목 체크, 선택 후 닫힘, X/바깥 터치 닫힘, 긴 목록 PREV/NEXT를 제공한다. Sound Preset은 카테고리→음색, Voicing은 구성음별 유효 전위 목록을 유지한다. 투명 전체화면 하위 그룹은 ignoreMousePresses로 형제 터치를 막지 않는다. MPC 공통 상단 프리셋 브라우저는 호스트 소유다.

실기기 사이드바 검사: Voice Poly→Legato 선택·자동 닫힘, Sound 카테고리→Lead→NEXT(2/2)→Melted Neon 적용, OSC 상세의 동일 목록 재열기와 바깥 터치 닫힘, CHORD2의4전위 선택, OCT +1 선택, MOD의Chord Preset→After Hours 적용을 확인했다. MAIN/CHORD/OSC/FILTER/AMP/MOD/FX의 가이드와 목록 중 숨김/복원을 확인했다. 물리 Q-Link16개 전체 회전은 이번 원격 입력으로 수행하지 않았으며, 파일 매핑/기기 guide/터치 조절과 구분한다.

최종 native/ARM32/ARM64 빌드 및 SDK validator0errors/0warnings, 엔진/UBSan/VST bridge, host_sim19PASS/0warnings/0failures, 스킨 검사를 통과했다. 수정한 OCT 도넛 빌드도 같은 검사를 다시 통과했다. host_sim의 NeedIdle 미지원 응답은 INFO이며 실제 호스트가 그 요청을 지원한다고 가정하지 않는다. 공식 SDK update 파일 검증·재시작 완료. 공개 push/Release 없음. 기기 CPU/다중 터치 전수 검증은 하지 않았다.

증거: build/qa-0.1.18/native-menu-blank.png, learn-registered.png. 마지막 OCT 도넛 스킨 확인은 아래 추가 기록을 따른다.

최종 OCT 도넛 실기기 확인: 새 인스턴스 MAIN/안내 없음 확인 후 CHORD에서 MIDI 도넛 제거·OCT 도넛4개·행 guide 포함을 확인했다. 코드1 OCT를 원격 드래그해0→+2oct로 변경하고 값 터치로 왼쪽 목록을 열어0oct를 선택, 자동 닫힘·값 복원을 확인했다. MIDI Note 텍스트 더블탭의 native 수동 편집도 열리고 바깥 터치로 닫힘을 확인했다. 새 스킨의 CHORD2 Voicing 목록과 MAIN OSC Pair 값 단일 터치 목록, X/바깥 터치 닫힘도 확인했다. 마지막 화면은 새 untitled QA Plugin001 / MAIN / Init / Poly / Split, Audio·RecordOff이며 원본 QA 프로젝트 파일은 덮어쓰지 않았다. 재시작 전 물리 학습 테스트 상태는 이 새 QA 인스턴스에 저장하지 않았으므로 마지막 Pad Note 기본값은24/26/28/29다. 최종 증거: chord-octave-donuts.png, voicing-sidebar.png, main-final.png(모두build/qa-0.1.18/).

### 0.1.19 — MAIN 그룹 재배치·숫자 LCD 여백 (2026-11-15)

사용자 확정: A Sound/Play, B Amp Envelope, C Filter/LFO, D FX/Tremolo. MAIN의 실제 컨트롤 위치·그룹 제목·Screen/Program Column Q-Link를 함께 변경했다. C의 필터 종류와 LFO Shape/Target을 해당 두 노브 위에, D의Drive 종류를 첫 노브 위에 배치했다. C와D 각 그룹 안의 두 기능 사이 divider를 분리했다. 상세 페이지·CHORD의 매핑과91개 파라미터 ID/순서는 유지한다.

MAIN 숫자 LCD를29→24px로 줄이고138px 플레이트 안의 값 라벨을 x12/w122로 중앙 배치해 좌우 여백을 확보했다. OSC Pair 선택값은22px, Vibrato/Expression의 보조 LCD 숫자는22px/w92다. 기존 터치 hit 영역은 축소하지 않았다. Native/ARM32/ARM64 빌드, SDK validator0errors/0warnings, 엔진/UBSan/bridge/host_sim19PASS/스킨 검사 통과. 회귀 검사에 새16개 매핑, 그룹 좌표/선택기 위치, LCD글자크기·여백을 추가했다.

기기 화면이 마지막 QA의Init에서 Synth/Chopped Brass로 달라져 사용자에게 확인했고, 현재 변경을 버리고 재시작하라는 답을 받아 공식 SDK update/restart를 수행했다. 기기 검증 결과는 아래에 기록한다. 공개 push/Release 없음.

별도 제안(미구현·미확정): 사용자가 코드 슬롯 +/- 확장 방법을 질문했다. 기본4/최대8, MAIN은 큰 패드4개 유지+페이지전환, CHORD 편집에슬롯+/-배치, 물리 패드는 화면페이지와 독립 매핑, 축소한 슬롯 설정 보존을 제안했다. 이 질문을 슬롯 확장 구현 승인으로 간주하지 않는다.

실제 MPC Live III3.10.1.12 결과: 재시작 후 새 untitled QA Plugin001에서 MAIN 직접 진입/안내 없음, 네 그룹 제목·실제 위치·LCD 여백을 확인했다. Attack fader 드래그 시 B guide와887.3ms 표시, Filter Cutoff 드래그 시 C guide와2.01kHz 표시, Tremolo Rate 드래그 시 D guide와11.7Hz 표시를 확인했다. 이동한 Filter Type 왼쪽 목록에서 guide 숨김과 닫은 뒤 복원을 확인했다. Sound Preset 다음/이전으로 Init을 복원했고 최종 MAIN/Init/Poly/Split, Audio·RecordOff다. 물리16개 Q-Link 회전은 이번 검증에 포함하지 않았다. SDK update/restart 종료0, 원본 QA 프로젝트 파일 미변경. 실제 캡처: build/qa-0.1.19/main-final.png.

### 0.1.20 — 4–16 code slots (2026-11-15)

사용자가 기본4/최대16,4개 단위 확장을 승인했다. 기존91개 파라미터를 보존하고 슬롯5–16의Root/Type/Voicing/Octave/PadNote/Touch72개, 활성개수/MAIN페이지/명령6개, 뱅크활동4개를 append하여173개로 확장했다. UIDKrql와16보이스는 유지한다. ChordSlot/PadLearn/LearnStatus choice 범위만16슬롯으로 확장했으므로 기존 정규화 automation은 재확인 대상이다.

엔진은 고정16항목 인덱스 테이블로 이전 비연속 파라미터 배치를 지원한다. MIDI 트리거 탐색은 활성 슬롯만 대상이며 화면 페이지와 독립적이다. 슬롯 축소/확대는 설정을 삭제하지 않고 음을 정리한다. MAIN 페이지 전환은 사라진 터치 gesture를 명시적으로 해제하되 물리 MIDI 소유 음은 유지한다. LEARN은16번까지 지원하며 기존 등록 노트를 누르면 슬롯끼리 교환해 유일 매핑을 유지한다. 비활성 LEARN은 해제한다. 구91개 상태를 이미 수정한 새 인스턴스에 로드할 때도 누락된 추가 파라미터는 기본값으로 복원한다.

MAIN은222×70 패드4개와현재범위/전체개수·좌우 이동을 유지한다. 물리 연주 중인 뱅크에 활동 표시를 제공한다. 코드 편집은 SDK 정적 매핑을 따라 첫 함수 키의 네 native서브탭(CHORD/C5–8/C9–12/C13–16)이다. 각 페이지16개 Q-Link는 보이는4슬롯의Root/Type/Voicing/Octave다. 비활성 뱅크는 설정을 미리 편집할 수 있으나 audition/LEARN을 숨기고 INACTIVE BANK를 표시한다. 각 편집기오른쪽 위 +/-로4·8·12·16개를 변경한다. 원래검증된 hold Slider 계약을 clone하여 캡션위의release gesture를 유지한다. 가이드/선택기/sidebar도 페이지별로 독립 복제한다. B 헤더의Q-Link선은 사용자요청대로 유지했다.

코드프리셋32개는 기존4코드 레시피다. 선택 시 네 묶음에 반복하여 초기화하고 각슬롯을 따로 편집한다.16코드 진행32개를 새로 작곡한 것이 아니다. 동시발음16보이스/voice stealing은 기존대로다.

검사: Native/ARM32/ARM64 빌드 및 SDK validator0errors/0warnings, 일반/UBSan 엔진·bridge, host_sim19PASS/0warnings/0failures, skin 검사 통과. 새 엔진 검사는16개 모든 슬롯의MIDI/터치 발음·해제, page4연주 중MAIN페이지 이동, touch만해제/MIDI유지, 축소시음정리·설정보존·페이지clamp, LEARN16+중복swap,173개저장복원,실제91개prefixchunk마이그레이션, 실시간할당금지를 포함한다. UI검사는4개nativeScreen매핑·추가12개pad의각자gate/release/페이지조건과append인덱스를 포함한다.

공식 SDK update/restart로 MPC QA 설치했다. 실제 기기 결과는 아래에 기록한다. 공개push/Release 없음.

0.1.20 최신 사용자 변경(앞선16슬롯 항목의 탐색/프리셋 설명을 대체):
- 프리셋은 현재 MAIN에 보이는4슬롯의Root/Type만 변경한다. 다른12슬롯과보이싱/옥타브/패드 매핑은 보존한다. 네 뱅크의프리셋선택을 독립저장하며 추가3개를인덱스173–175에append했다. MOD의고정첫뱅크Q-Link는PRESET1–4로명시한다.
- 사용자가 MAIN키반복탐색을 거절했다. MAIN↔CHORD 두서브탭만 유지하고 CHORD안의1–4/5–8/9–12/13–16버튼으로묶음을고른다. 활성개수만큼버튼이늘어나고메인과페이지를공유한다. SDK의정적Screen Q-Link는16개편집중계파라미터(176–191)에고정한다. 중계쓰기→선택실제슬롯쓰기,실제슬롯읽기→중계표시를플러그인에서처리한다. 새handler나동적팝업매핑API를발명하지않는다. 총192개이며기존176개파라미터ID/순서는보존한다. 중계16개는automation비활성화,실제슬롯파라미터는독립automation/저장대상이다.
- 상태복원동안중계callback을억제하여이전에저장된중계표시가실제코드를덮어쓰지않게한다. 오디오스레드readback은원자적스레드토큰+현재인덱스로구분하고다른UI스레드동시쓰기를억제하지않는다. 프로세스에잠금·동적할당·스레드생성을추가하지않았다.
- 신규값이변하지않아도prepare/reset에서추가파라미터의초기호스트알림을큐에넣는다. 실제알림배출은기존안전한control/idle/getter경로로제한한다. 복제컴포넌트그래프는의존자식부터결정적인순서로생성한다.
- 저장시화면미리듣기·UI명령·연주표시는정적상태로남기지않고Off로정리한다. 오디오소유보이스에UI스레드가직접접근하지않고원자게이트만해제한다. 다음오디오블록에서화면audition을해제하며물리MIDI소유음은유지한다. 재로드시손가락을떼어놓은화면코드가자동재생되지않는다.
- 잘린화살표:새MAIN페이지화살표의36×32그림/36×28영역불일치를36×28로맞추고,기존preset화살표의하단프레임도bitmap안쪽에배치해투명한마지막행을남겼다. 개수+/-는44×44로확대했다. 단일값조건RangedEnabling/N/N대신IndexedEnabling/N을사용한다.
- 최종자동검사:Native/ARM32/ARM64,SDK validator0errors/0warnings,엔진/UBSan/bridge,host_sim19PASS/0warnings/0failures,스킨검사통과. 추가검사는선택뱅크만preset변경/다른뱅크보존,중계4종쓰기·실제값readback·뱅크전환·상태복원보호,저장시touch만해제/MIDI유지,단일편집기탐색/16 LEARN/프리셋선택기조건/화살표safe edge다.

기기 최종검증은아래추가한다. 원본QA파일은덮어쓰지않았고공개push/Release는하지않았다.


## 1.0.0 release verification — 2026-11-15

- Native, ARM32, ARM64 build and SDK validators passed. Host simulator:19 pass/0 warnings/0 failures. Engine, UBSan, host bridge and skin fixtures passed.
- MPC Live III / OS3.10.1.12: fresh instance opens MAIN without startup popup. MAIN↔single CHORD editor, count4→8→12→16, internal bank buttons, shared MAIN bank, bank13 octave edit and bank1 preservation verified through Remote Screen Viewer. Duplicate CHORDS footer/title placement fixed.
- MAIN and OSC sound browser reopen at categories; Lead selection loads Ribbon Whistle. Host gray PRESET remains flat, documented explicitly.
- Bank13–16 preset arrow selects Basement Loop while bank1–4 remains Sunday Soul. Bottom arrows render completely.
- Website screenshots are device-exported1280×800 PNGs, converted to WebP without UI synthesis.
- Prior user physical hold/release, pad LEARN and mono/legato evidence remains applicable; newest full physical Q-Link matrix, simultaneous finger multitouch, other MPC models and worst-case device CPU are not certified.
- 192 stable parameters; old91 migrate; sixteen editor proxies are nonautomatable. Touch/learn/command transient state is cleared on save to prevent replay on restore.

- Final device sweep: OSC, FILTER, AMP, MOD, FX and CHORD guides visually inspected. Sound sidebars hide background guides and restore on close.16→4→16 restores bank13 Fm9/+1oct and clamps to valid bank on shrink.16 screen gestures recorded at44.1kHz stereo, peak0.632FS (no clipping); recording ended during the last effect tail, so this recording alone is not a post-release silence test.


## 1.0.1 — oscillator balance performance mapping
User requested A=OSC Pair/Osc Mix/Body/Drift and D=Drive/Glide/Tremolo Rate/Tremolo Depth. Space remains on FX; AMP retains Glide. MAIN and Program Q-Link maps use existing stable parameters (no added IDs, no DSP changes). Native/ARM32/ARM64 and SDK validators pass; engine/UBSan/host bridge/host simulator19 and skin regression checks pass. New geometry checks verify A order, Glide position and retained FX Space. Device verification is recorded separately below when performed.

Device check (MPC Live III / OS3.10.1.12): installed with official mpc_remote.py and restarted with user's discard approval. Fresh instance opens MAIN. New A/D layout renders without overlap. MAIN Osc Mix drag0→71.4% is mirrored on OSC; MAIN Glide0→238ms is mirrored on AMP. Screen/Program serialized Q-Link bindings match exactly; physical knob rotation matrix was not repeated remotely. No DSP behavior changed; neutral-Body oscillator endpoint isolation also passes engine/UBSan tests.

- Actual device audio: two1.2s chord holds before/after OSC Mix20→67.6%,44.1kHz stereo20.56s recording. Two separated audible regions, peak0.574FS, final3s peak0.0. FX Space0→33.3% leaves MAIN/AMP Glide238ms unchanged. Fresh preset loading and CHORD4→16 bank13–16 navigation also pass. Four1.0.1 screenshots exported from the device replace the website images; no synthetic UI render is labelled as a device screenshot.


## 1.1.0 local candidate — warm factory-bank voicing

User corrected a misunderstanding: Moog-like sound character was requested, not UI color. Scope: revoice existing sounds, no added modes/UI.

- All63 playable recipes explicitly revoiced in sound_bank.py; Init and64 program names/order retained. Existing Low-cut values remain unchanged. No DSP algorithm,192 parameter declarations, UID, chord recipes or mapping changes. The only DonutTheory.cpp change is version10001→10100. Existing12dB SVF is not presented as24dB ladder modeling.
- Bass/Synth/Lead/Pad have FM0; Keys retain smallFM amounts up to18 for sound identity. Every playable recipe uses existingLP/Tape; lower chorus and drift; each patch has explicit Body/Resonance/Env/Pulse values.
- Stereo peak calibration: render with OutputTrim0, calculate gain through the final tanh using calibrate_factory.py, regenerate and re-render. Bass/Lead measured−15.00dBFS; Keys/Synth/Pad−12.00dBFS. Init unchanged. Trim range−3.00..13.02dB. These are a fixed musical-pattern peak reference, not perceived loudness or guaranteed headroom for every performance.
- audit_presets.cpp renders64×5s at44.1kHz; each has finite non-silent stereo output in range. Max measured peak0.251319FS. Before/after mono WAVs use identical MIDI/hold/release and no post-render normalization; raw/report files are in build/voicing-before, voicing-untrimmed, voicing-after.
- Native/ARM32/ARM64, SDK validators0 errors/0 warnings, engine/UBSan, host bridge and host simulator19 pass/0 warnings/0 failures. Skin regression includes the new bank contract. TUI.json and Q-Links.json SHA256 are identical to1.0.1.
- Before/after demos for Bass Maple Thump, Keys Felt Circuit, Synth Mellow Block, Lead Bent Wire and Pad Draped Velvet generated. A spectral energy centroid falls in33/63; this is only a waveform descriptor, not a metric that proves subjective analog warmth or an audible Moog match. No full64-preset listening approval is claimed.
- No MPC installation/restart, new device CPU/physical-input verification, push, Release or Pages publication performed for this local candidate.


## 1.1.1 — chord navigation target size
The preset arrows grow36×34→44×44 (58% greater area). Page arrows grow36×28→44×32 visually, with44×44 topmost transparent hit surfaces (92% greater hit area). Preset label remains220px wide and grows34→44px high. Four222×70 hold pads and mappings are unchanged. Hardware image dimensions match visible widgets. Upper hits starty536, after filter controls endy534; lower hits starty580, preventing overlaps. Automated native/ARM32/ARM64, validators, simulator19, engine/UBSan and skin checks pass. Other four proposed UI improvements are not implemented. Device results follow when checked.
### 1.1.2 — 수동 패드 위치 구현 및 실기기 확인 (2026-11-15)

- 로컬 후보1.1.2를 공식 SDK update/restart로 MPC Live III / OS3.10.1.12에 적용했다. 기존192개 파라미터 선언 전체가 공개HEAD와 동일한 것을 비교했고, PadPosition1–16만192–207에 append했다. 위치는 비자동화 파라미터이며 Q-Link 매핑을 바꾸지 않는다.
- 공통 CHORD 편집에 PAD LOCATION을 추가하고, A–H/4×4 사이드바에서 위치를 수동 지정한다. MAIN의 기존222×70 hold 패드에 위치명과40×40 배열을 장식으로 추가했다. 별도 코드별 EDIT는 추가하지 않았다. 미등록은 숨기며 HOLD TO PLAY도 제거했다.
- 실제 기기에서 초기 MAIN/안내 팝업 없음, 미등록 숨김, 슬롯1–4에 A1–A4 개별 등록, 슬롯2의 H16 등록, CLEAR LOCATION→Unassigned, 해제 전후 MIDI26 보존, MAIN의 각 위치명과 아래 행 강조를 확인했다. 선택 후 닫힘, 선택창에서 Q-Link 가이드 숨김/닫은 뒤 복원도 확인했다. 초기 셀의 Data→Data 자기 remap이 외부 슬롯 바인딩을 가리는 문제가 있어 제거하고 A1/H16 저장을 재시험했다. 위치 팝업은 CHORD 탭 루트의 형제 컴포넌트로 배치한다.
- 원격 입력에서 기존 포커스 좌표로 클릭이 들어가 MAIN/CHORD가 다시 전환되는 현상이 있었다. 전체화면의 작은 drag 입력으로 위치 선택을 재검증했다. 실제 손가락이나 물리 패드를 누른 시험으로 설명하지 않는다.
- native/ARM32/ARM64 빌드 및 SDK validator0 errors/0 warnings, host simulator19 PASS, 엔진/UBSan/VST bridge 및 스킨 검사 PASS. 위치16개 저장·복원, LEARN/음색과의 독립성,192개 구버전 상태 복원 시 미등록 기본값을 회귀 검사했다. 로그: /tmp/donut-112-build.log, /tmp/donut-112-test.log, /tmp/donut-112-skin-test.log.
- 실제 캡처: plugins/DonutTheory/build/qa-1.1.2/main-viewer.png(원격 뷰어 창1280×900), chord-editor.png(기기1280×800). MAIN은 현재 표시 중인 뷰어 창 원본을 보존했고 CHORD는 Remote Screen Viewer의 Screenshot으로 저장해 편집 없이 복사했다. 표시된 A1–A4는 QA에서 수동 지정한 힌트이며 실제 물리 패드 연결을 재인증한 것이 아니다.
- 기기 오디오 녹화의 마지막45초를 PCM으로 분석했다. 짧은 터치 발음1구간을 검출했고 피크0.093689FS/마지막1초 무음이었다. 원격 입력 문제 때문에 네 버튼의 장시간 hold 시험 완료로 보고하지 않는다. 이번 실행에서 물리 LEARN 입력·물리 Q-Link 회전·손가락 hold·MPC 프로젝트 저장/재시작 복원은 재검증하지 않았다. 기존 hold 제스처 표면은 보존했고 엔진 상태/노트 소유권 검사는 통과했다.
- 공개push/Release/Pages 갱신 없음. 공개판은1.0.1을 유지한다.


### 1.1.2 official distribution preparation — 2026-11-15

사용자가 공식 Release 및 웹사이트 갱신을 승인했다. 앞선 로컬 후보의 공개판1.0.1 유지 기록은 당시 상태다. 이번 배포 대상은1.1.2이며63개 음색 재설계, 화살표 확대, 수동 패드 위치 기능을 포함한다. 사이트 제품 데이터·매뉴얼·변경 내역·다운로드는 동일 버전을 사용한다. MAIN/CHORD는 실제 MPC 캡처로 교체했으며 MAIN은 뷰어의 창 테두리만 제거했다.

최종 native/ARM32/ARM64, SDK validator0 errors/0 warnings, 엔진/UBSan/bridge, host simulator19, 스킨 검사 및 사이트 빌드·로컬링크40개 검사를 통과했다. 사이트는390px 모바일·768px 폴드·1440px PC에서 가로 넘침 없이 확인했다. 패키지는 SDK package.py 결과에 MANUAL.md/RELEASE_NOTES.md를 추가하고 SHA256을 별도 제공한다. 실제 발행 여부는 명시적 donut-theory/v1.1.2 Release와 Pages 배포 결과로 확인한다. 실기기 미검증 범위는 릴리스 노트에 그대로 공개한다.


## 1.2.0 — engine redesign and factory refit (2026-11-15)

- 사용자 정정: 프리셋만의Moog풍 튜닝 대신 엔진 자체를Moog-inspired로 변경한다. 추가수락 없이 엔진·전체프리셋·검증·릴리스·사이트까지 진행하도록 명시 승인했다.
- 프로젝트 원본MIT AnalogLadder.h: 비선형4단 차동쌍, trapezoidal integrator, 최대6회의동시Newton 해석,2x 내부래더와17tap Blackman FIR. 최소1회Newton을강제하여저레벨입력에서절대오차조기종료가만들던deadzone을제거했다. 테스트전용 계측으로 급격한컷오프/rate/res stress에서잔차<1e-4를확인했다. saturation값·미분계산을공유하여inner-loop 나눗셈을줄였다. 믹서/VCA도비선형으로교체하고VCO위상은노트재트리거사이에보존한다. 기존SDK파형생성/BP·HP·Notch/ADSR/FX를유지한다.
- 63개연주음색의cutoff/resonance/env를개별수정하고Tone/Drive를줄였다. Sound Trim은재렌더계측으로−1.98..15.21dB. 64개이름/순서/카테고리/각Low-cut/Init값은공개1.1.2와동일하다. Init도새엔진고유경로를사용하므로오디오동일성은보장하지않는다.
- UID와208개파라미터선언전체가공개HEAD와동일하다. TUI.json/Q-Links.json은공개1.1.2 ZIP과byte-identical이다. 상태호환과음색동일호환을명확히구분한다.
- native/ARM32/ARM64및SDK validator0errors/0warnings, engine/UBSan/bridge/analog/skin, host simulator19 PASS. 추가host_sim48k/512 및96k/32도통과. 실시간할당금지·MIDIoffset·가변블록·코드소유권·상태마이그레이션도회귀검사했다. 로그:/tmp/donut-120-final-build.log,/tmp/donut-120-final-test.log,/tmp/donut-120-host48.log,/tmp/donut-120-host96.log.
- 래더응답:48k/각pole1kHz/작은신호에서50Hz gain0.995019,1kHz0.249722,4kHz0.00333159.90%res에서1kHz gain2.24468.80%res에서small/hot gain1.17372/0.131283로입력레벨의존성을확인했다. 모든64×5s오프라인렌더finite/non-silent,피크0.177732..0.288887FS.
- compare_engines.cpp는같은serialized state/MIDI를구1.1.2/새1.2.0 library에적용한다.16voice desktop평균0.74%→5.78%,최악10.47%(새버전)로CPU증가를측정했다. 새로운host_sim3note 평균1.38%/최악3.49%. 이값을MPC실제부하로환산하거나보증하지않는다. 사이트24초A/B는Bass/공진Lead/4note순서로각old→new이며전체에동일3xgain을적용하고개별정규화하지않았다.
- 공식mpc_remote.py update로기기드라이브36C7-1ACA에73개파일크기검증후설치했다. 재시작은자동승인검토가현재미저장데이터폐기미승인을이유로거절했다. 우회하지않았으며사용자에게명시적현재변경폐기허용을요청했다. 원격상단tray는접근했으나프로젝트저장화면으로이동할수없었다. 현재실행중인엔진은이전버전이며1.2.0실기기오디오/CPU/새저장복원은미검증이다. 사용자물리입력·전체63음색청음승인도확정하지않는다.
- 공개사이트/매뉴얼/Release에는위제한을표시하고실제1.1.2MPC스크린샷이변경없는UI를보여준다고명시한다. 발행결과는donut-theory/v1.2.0및Pages Actions에서확인한다.

## 1.2.1 — hardware engine verification and modal fix (2026-11-15)

사용자가 현재 변경 폐기를 허용하고 사본 저장 불필요를 명시했다. 공식 SDK mpc_remote.py로1.2.0 재시작, 새 프로젝트/새 Plugin 인스턴스에서 MAIN 기본 진입을 확인했다. 이어 실기기에서 발견한 사이드바 레이어 버그를 고친1.2.1을 공식 update/restart로 적용했다. 고정 SDK/시스템 파일 변경 없음.

- 1.2.0 실제64음색 순서·이름과 발음 확인. factory-cycle.mov(416.078초,44.1k stereo)64gate 모두 non-silent, 전체 peak0.6838989FS/마지막1초0. 각 gate는 Dm9 화면 hold1.6초이며 저음/고음별 음악적 평가나 전체 음색 주관 청음 승인을 대체하지 않는다.
- 네 MAIN hold1.8초의 ON 시간과 오디오4영역을 확인했다. initial-four-chords.mov peak0.3793335FS/마지막1초0.0. 실제 손가락 다중 터치가 아닌 원격 hold다.
- Cutoff20..18000Hz, resonance100%, BP/HP/Notch를 확인했다. Init/Poly/Split/Release5000ms의 여러 코드 중첩 뒤 host System Resources CPU41%,6초 후12% 관찰. 정확한16held voice 계측/최악 부하는 아니다. filter-load.mov peak0.3442993FS/clip0이며 종료 직전 Notch 음을 눌러 마지막 구간은 릴리스 중이다.
- native plugin preset ladderqa.xpl 저장 후1.2.1 재시작/새 인스턴스에서 복원했다. Notch/fc1.34k/res46.4/Release5000이 같았다. MPC 전체 프로젝트 저장 복원으로 보고하지 않는다.
- MAIN Filter Type 선택 사이드바 위로 코드 페이지 화살표가 떠 있는 버그 발견. chord_slots가 나중에 추가한 일반 형제를 modal 뒤에 붙이던 원인. 생성기 마지막에 모든 탭의 ordinary→modal stable partition을 수행하고 skin_test에 전 탭 회귀 검사 추가.1.2.1에서 기존 화살표가 목록에 뜨지 않음과 가이드 숨김/복원을 확인했다. 음색 category→list2page→Faded Poly 선택도 확인했다.
- CHORD inversion2 선택,4→8→12→16 확장,13–16 bank,MAIN/OSC/FILTER/AMP/MOD/FX 각 화면을 실제 기기로 확인했다. 수정 전 화면/원격 전달 지연이 섞인 캡처는 공개 자료에서 제외한다.
- 1.2.1 Filter LFO+tremolo 동시 적용 뒤2.8초 hold2개 기록: final-1.2.1.mov(358.153초)peak0.289917FS/clip0/마지막1초0. 원격 뷰어 output dropouts 누적 표시가 있어 transport/Mac playback/DSP 원인은 미분리다. 무결점 오디오나 모든 물리 입력을 인증하지 않는다.
- 증거: plugins/DonutTheory/build/qa-1.2.0/{factory-media,initial-four-chords,filter-load-media,final-media}/analysis.json 및 실제 viewer 스크린샷. 소스 녹화 원본은 Desktop에도 보존. build/ 미커밋. 공개 실제 스크린샷은 뷰어 chrome만 crop한1280×800으로 저장한다.
- native/ARM32/ARM64 SDK0errors/0warnings, engine/UBSan/bridge/analog/skin와 host simulator19PASS. /tmp/donut-121-build.log,/tmp/donut-121-test.log. DSP는version metadata만10201로 변경했다.1.2.0 음성 엔진·64recipes·208params·Q-Link 유지.
- 모든 물리 Q-Link·새 물리 LEARN·실제 손가락 동시누름·다른모델·전체프로젝트 복원은 재인증하지 않았다. 공식 배포 결과는donut-theory/v1.2.1과 Pages Actions에서 확인한다.

## 1.2.2 — retire manual pad-location UI / CHORD construction budget

사용자: MAIN 재탭 시 CHORD 미전환/벽돌 현상과 패드 위치 표시 불안정 보고, 위치 표시 기능 삭제 허용. 위치 스킨 설치 패스만 중단하고 원본 제작 코드/자산은 보존했다. MIDI Note/LEARN·208개 파라미터/UID·Q-Link·DSP·음색은 유지한다. PadPosition192–207은 저장 호환용 미사용 값이다. LEARN 버튼은 위치 표시 도입 이전의180px 크기로 복원된다.

구성 그래프 계측: 1.2.1 MAIN1608/CHORD10889 → 1.2.2 MAIN1576/CHORD1913개. TUI19,568,032→13,542,804bytes. CHORD2500 이하·재귀 cycle 방지 검사 추가. 모든 빌드/SDK0errors0warnings/host19/engine·UBSan·bridge·analog·skin 통과 (/tmp/donut-122-build.log,/tmp/donut-122-test.log). 마지막 budget 검사 별도 통과.

공식mpc_remote.py update로64개파일 검증 후 기존 사용자 폐기/재시작 허용 범위에서 재시작했다. 새 untitled QA 프로젝트/Plugin001에 로드. 초기 MAIN→CHORD 성공, 12개 연속 전환 입력 후 CHORD 응답 및 inversion 선택,4→8→12→16 증가,13–16 bank 확인. 초기8개 연속 캡처는 MAIN/CHORD 교대, 뒤 일부는 원격 프레임 지연으로 이전MAIN이 찍혔으며 이후 최신 CHORD와 편집 성공을 확인했다. 캡처만으로 정확한 전환 지연/12회 각 프레임 성공을 인증하지 않는다.

초기 normal-window 좌표 입력이 제대로 전달되지 않은 것을 fullscreen에서 확인했다. 앞선 MAIN 미응답 재현 주장은 이 제약 때문에 근거가 부족하며 정정한다. 사용자 실제 멈춤 원인을 통제된 구버전/신버전 A/B로 확정하지 않았다. 제거한 대형 UI가 불필요한 구성 비용을 만들던 것은 그래프 계측으로 확인했다. 물리 재탭 확인은 사용자에게 요청했으며 응답 전에는 물리 검증 완료로 표시하지 않는다. 실제 screenshots: build/qa-1.2.2/*.jpg, 사이트는 fullscreen의 기기영역1080,308,1280,800만 crop한 실제WebP다. 공개 결과는1.2.2 Release/Pages를 따른다.
