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
