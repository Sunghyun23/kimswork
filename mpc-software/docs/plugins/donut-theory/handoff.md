# Donut Theory — 프로젝트 인계 / 재개 기준

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
