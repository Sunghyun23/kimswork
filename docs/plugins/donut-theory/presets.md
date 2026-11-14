# Donut Theory factory bank v2 — 0.1.11

사용자 승인: 기존 음색 프리셋을 전면 교체하고 카테고리 우선 선택, Dilla를 연상하는 소울/힙합 Bass·Keys·Synth·Lead·Pad, 음색별 Low-cut 및 Init을 제공한다. 특정 아티스트 소리의 복각/공식 제품이라는 뜻은 아니다. 코드 진행32개는 별도 라이브러리다.

## 선택 방법

플러그인 MAIN/OSC의 SOUND PRESET 목록은 왼쪽 카테고리→오른쪽 음색 순서다. 최초 방문은 카테고리 미선택 상태이며 이후 마지막 탐색 카테고리를 유지한다. 선택 카테고리를 누르는 것만으로 현재 음색을 변경하지 않는다. MPC 공통 상단 PRESET 팝업은 호스트가 그리는 내장 프로그램 목록이며 SDK에서 그 UI를 카테고리 단계로 바꾸는 API는 없다. 그 목록에는 카테고리/이름을 함께 표시한다. 화살표는 전체64개를 순환한다.

## Low-cut / 레벨

FX의 LOW CUT은 출력단 스테레오12dB/oct HPF(Q≈0.707)이며0=Off, 최대250Hz다. 본래 음색용 Low-pass와 독립적이고 Chorus/Delay 뒤에서 처리한다. Bass18–28Hz, Keys85–160Hz, Synth85–140Hz, Lead100–160Hz, Pad120–200Hz. Init은0Hz/효과Off/단일Saw/지속형Amp로 시작한다. 저역 기음 보존을 위해 기존 고정DC 블로커(~35Hz)를 약5Hz로 낮췄다. 따라서 구버전 프로젝트도40Hz 이하 응답은 달라질 수 있다.

SOUND TRIM은 음색별 고정 출력 보정이다. 기준:44.1kHz, master−8dB, velocity96, Bass C2/Lead C5 단음 및 다른 음색 C4 minor7 코드,2초hold/3초release. Bass·Lead 피크약−15dBFS, Keys·Synth·Pad약−12dBFS로 보정했다. 정해진 연주에서의 피크 보정이며 모든 연주/노트/벨로시티에서 같은 체감음량이나 최종 믹스 완성을 보장하지 않는다. 연주에 따라 LOW CUT/SOUND TRIM을 조정한다.

## 제작 및 호환성

tools/sound_bank.py가63개 명시적 레시피와 Init을 관리한다. 이름뿐인 modulo 변형 루프를 제거했다. presets/factory-levels.json은 기준 연주를 바탕으로 한 보정값이다. 음색 프리셋은 입력/코드 진행/패드 매핑/마스터 볼륨을 변경하지 않는다. 파라미터85 LowCut,86 OutputTrim을 끝에 추가하고 이전 인덱스/UID는 유지한다. 이전 프로젝트에는 두 항목0을 적용해 새 프리셋의 EQ/트림을 강제로 적용하지 않는다. 공장 프로그램 번호의 의미/이름은 사용자 지시에 따라 교체되며 저장 프로젝트의 실제 파라미터 값은 보존한다.

## 프리셋 목록

| Category | Name | Low Cut Hz | Sound Trim dB |
|---|---|---:|---:|
| Init | Init | 0 | 0 |
| Bass | Pocket Anchor | 18 | 5.83 |
| Bass | Maple Thump | 20 | 13.74 |
| Bass | Brick Bounce | 22 | 3.31 |
| Bass | Molasses | 18 | 13.03 |
| Bass | Crate Knock | 25 | 9.48 |
| Bass | Soft Cone | 18 | 6.23 |
| Bass | Motor Hum | 20 | 4.32 |
| Bass | Felt Punch | 24 | 10.32 |
| Bass | Honey Slide | 20 | 7.71 |
| Bass | Walnut Pluck | 28 | 3.97 |
| Bass | Low Canvas | 18 | 5.81 |
| Bass | Dust Motor | 22 | 6.2 |
| Keys | Felt Circuit | 85 | 0.49 |
| Keys | Worn Tines | 100 | 1.61 |
| Keys | Honey Reeds | 110 | 4.73 |
| Keys | Kitchen Bell | 140 | 1.09 |
| Keys | Soft Hammer | 95 | 8.92 |
| Keys | Velour Clav | 130 | 0.73 |
| Keys | Milk Glass | 120 | -0.15 |
| Keys | Tape Celeste | 150 | -0.32 |
| Keys | Reed Cabinet | 100 | -0.73 |
| Keys | Bronze Mallet | 160 | 0.93 |
| Keys | Evening Felt | 90 | 9.51 |
| Keys | Pocket Organ | 110 | 0.41 |
| Keys | Pearl Hammer | 135 | 0.85 |
| Keys | Cocoa Tines | 105 | 1.54 |
| Keys | Dust Harp | 145 | 7.01 |
| Synth | Chopped Brass | 100 | 2.1 |
| Synth | Mellow Block | 85 | -1.34 |
| Synth | Velvet Stab | 120 | 5.5 |
| Synth | Amber Stack | 110 | -0.05 |
| Synth | Rubber Chord | 130 | 3.35 |
| Synth | Hollow Print | 95 | 1.01 |
| Synth | Record Bloom | 100 | 1.18 |
| Synth | Copper Pluck | 140 | 3.07 |
| Synth | Faded Poly | 110 | 2.22 |
| Synth | Small Ensemble | 90 | 2.76 |
| Synth | Syncopated Ink | 125 | 3.95 |
| Synth | Glass Chord | 115 | 0.19 |
| Lead | Ribbon Whistle | 110 | 9.49 |
| Lead | Bent Wire | 130 | 6.74 |
| Lead | Narrow Alley | 150 | 3.37 |
| Lead | Honey Horn | 110 | 8.83 |
| Lead | Syrup Line | 120 | 2.93 |
| Lead | Ink Flute | 100 | 6.41 |
| Lead | Frayed Saw | 140 | 7.34 |
| Lead | Pocket Squeak | 160 | 4.16 |
| Lead | Golden Thread | 115 | 7.8 |
| Lead | Melted Neon | 135 | 6.04 |
| Lead | Muted Trumpet | 105 | 9.5 |
| Lead | Round Needle | 125 | 5.48 |
| Pad | Draped Velvet | 140 | 1.7 |
| Pad | Paper Halo | 165 | 1.92 |
| Pad | Warm Projection | 130 | 1.61 |
| Pad | Woven Air | 200 | -1.17 |
| Pad | Satin Room | 120 | 2.15 |
| Pad | Faded Choir | 180 | -0.88 |
| Pad | Ember Strings | 155 | 0.04 |
| Pad | Curtain Light | 145 | 2.17 |
| Pad | Soft Horizon | 150 | 1.67 |
| Pad | Opaque Silk | 135 | -0.35 |
| Pad | Dusk Film | 190 | 2.68 |
| Pad | Golden Haze | 160 | 0.72 |

## 검증 상태

Native/ARM32/ARM64 빌드와 SDK validator, 엔진/UBSan/VST bridge, host_sim19 PASS/0 warnings/0 failures, 카테고리별 개수/배선/독립 레시피/Init/필터 회귀 검사를 통과했다.55Hz 사인 기준18Hz Low-cut의 에너지 유지 및150Hz Low-cut의 감쇠를 검사했다. tools/audit_presets.cpp로64개 전체를5초씩 렌더해 무음/NaN/Inf/범위 초과가 없음을 확인하고 build/factory-audit.csv에 결과를 기록했다. build/factory-demo.wav 및 카테고리별WAV는 실제 엔진의 오프라인 출력이며 음량 정규화하지 않았다.

Mac 잠금으로 실기기 카테고리 선택/전 음색 청감/기기 성능은 미검증이다. 숫자·렌더 검사와 음악적 청감 승인을 구분한다. 공개 Release/push 없음.


2026-11-14 실기기 후속: MPC Live III3.10.1.12에서 카테고리 패널이 형제 버튼 터치를 막는 문제를 수정했다(ignoreMousePresses). Init/Bass/Keys/Synth/Lead/Pad 목록 표시, Lead/Ribbon Whistle와 Init 실제 적용, DONE 복귀를 확인했다. FX에서 Ribbon Whistle Low Cut110Hz와 Init Low CutOff/효과Off 표시도 확인했다. 전체64개 실기기 청음·음악적 승인은 별도다.


## 1.0.0 browser scope
The plugin SOUND PRESET selector always opens the six categories first, on MAIN and OSC. Choose a category, then a sound. The MPC-owned gray PRESET list is flat; category prefixes in program names do not create native folders. Chord presets modify Root/Type only for the currently visible four-slot bank; each bank stores its own choice.
