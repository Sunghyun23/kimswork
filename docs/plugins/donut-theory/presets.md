> 1.1.2 배포 기준: 아래 재조정 레시피가 기본 공장 음색이다.1.1.0 후보 기록은 설계 이력이며 현재 배포 상태는 [1.1.2 릴리스 노트](releases/1.1.2.md)를 따른다.

# Donut Theory factory bank v3 — 1.1.0 candidate

공개 버전은1.0.1이다. 아래 내용은 사용자 요청으로 기존 음색 전체를 따뜻한 아날로그 감성으로 재조정한 로컬1.1.0 후보를 설명한다. 새 모드·필터 알고리즘·UI·Q-Link 변경은 없다. Moog 계열의 두꺼운 subtractive 질감을 목표로 한 기존 엔진의 프리셋 튜닝이며24dB 래더 회로 복각이 아니다.

## 음색 방향

- 기존64개 이름·카테고리·번호와32개 코드 진행을 유지한다.63개 연주 음색을 모두 개별 재설계했고 Init은 기존 단일Saw·Low-cut/효과Off 출발점으로 유지한다.
- Bass: Saw/Pulse/Triangle 배합, 정돈된 저역·서브, 짧은 필터 어택과 punch.
- Keys: FM과 금속성 배음을 줄이고 짧은 아날로그 필터 플럭·펄스/삼각파 질감으로 조정한다. 벨·틴 계열은 약한 FM으로 구분을 남긴다.
- Synth: 두 OSC의 두께와 적당한 공진, 필터 엔벌로프로 brass/stab 성격을 구분한다.
- Lead: Saw/Pulse의 중심감, 단음Legato와 짧은 glide, 과하지 않은 detune.
- Pad: 느린 필터/앰프 엔벌로프와 부드러운 듀얼OSC. 넓은 코러스·피치 흔들림은 줄인다.
- 모든 연주 음색은 기존Low-pass/Tape 설정을 사용한다. 서로 다른 cutoff/resonance/envelope/pulse/body 값은 sound_bank.py의 개별 레시피에 명시한다.

## 선택·저장 호환성

SOUND PRESET은426px 왼쪽 목록에서 카테고리→음색을 선택하며 다시 열면 카테고리부터 시작한다. 상단 회색MPC PRESET의 현재 내장 프로그램 통합은 평면 목록이다. 정식 플러그인 분류와 외부SDK 등록 방식의 차이는 별도 조사 대상이며, 호스트 전체에서 카테고리가 불가능하다고 단정하지 않는다.

UID와192개 파라미터의ID/순서는 유지한다. 저장한 프로젝트의 파라미터는 그대로 복원한다. 새 공장 프로그램을 선택하면 재조정된 음색을 불러온다. 음색 선택은 마스터볼륨·코드·보이싱·패드 매핑을 변경하지 않는다.

## Low-cut·출력 레벨

기존 음색별Low-cut을 유지한다: Bass18–28Hz, Keys85–160Hz, Synth85–140Hz, Lead100–160Hz, Pad120–200Hz. Init은Off다. 음색용Low-pass와 혼동하지 않는다.

보정 기준은44.1kHz, master−8dB, velocity96, Bass MIDI36 / Lead MIDI72 단음, 나머지MIDI60 minor7 코드,2초hold/3초release다. audit_presets.cpp --untrimmed로 스테레오 양 채널의 최댓값을 측정하고 calibrate_factory.py로 Sound Trim을 계산한다. Bass·Lead 기준피크−15dBFS, Keys·Synth·Pad−12dBFS. 체감음량 정규화나 모든 연주 조건에서의 mix-ready 보장은 아니다.

## 프리셋 목록

| Category | Name | Low Cut Hz | Sound Trim dB |
|---|---|---:|---:|
| Init | Init | 0 | 0 |
| Bass | Pocket Anchor | 18 | 4.63 |
| Bass | Maple Thump | 20 | 7.54 |
| Bass | Brick Bounce | 22 | 4.57 |
| Bass | Molasses | 18 | 8.87 |
| Bass | Crate Knock | 25 | 7.24 |
| Bass | Soft Cone | 18 | 9.92 |
| Bass | Motor Hum | 20 | 3.0 |
| Bass | Felt Punch | 24 | 8.19 |
| Bass | Honey Slide | 20 | 7.13 |
| Bass | Walnut Pluck | 28 | 4.1 |
| Bass | Low Canvas | 18 | 4.34 |
| Bass | Dust Motor | 22 | 6.12 |
| Keys | Felt Circuit | 85 | 2.07 |
| Keys | Worn Tines | 100 | 0.4 |
| Keys | Honey Reeds | 110 | -2.16 |
| Keys | Kitchen Bell | 140 | -0.62 |
| Keys | Soft Hammer | 95 | 3.56 |
| Keys | Velour Clav | 130 | 3.7 |
| Keys | Milk Glass | 120 | 13.02 |
| Keys | Tape Celeste | 150 | -0.64 |
| Keys | Reed Cabinet | 100 | -2.14 |
| Keys | Bronze Mallet | 160 | 1.53 |
| Keys | Evening Felt | 90 | 0.81 |
| Keys | Pocket Organ | 110 | -3.0 |
| Keys | Pearl Hammer | 135 | 8.53 |
| Keys | Cocoa Tines | 105 | 0.17 |
| Keys | Dust Harp | 145 | 7.95 |
| Synth | Chopped Brass | 100 | 0.25 |
| Synth | Mellow Block | 85 | -2.4 |
| Synth | Velvet Stab | 120 | 3.64 |
| Synth | Amber Stack | 110 | 0.35 |
| Synth | Rubber Chord | 130 | 4.48 |
| Synth | Hollow Print | 95 | -0.1 |
| Synth | Record Bloom | 100 | -0.38 |
| Synth | Copper Pluck | 140 | 3.51 |
| Synth | Faded Poly | 110 | 1.21 |
| Synth | Small Ensemble | 90 | 0.7 |
| Synth | Syncopated Ink | 125 | 3.52 |
| Synth | Glass Chord | 115 | 0.6 |
| Lead | Ribbon Whistle | 110 | 5.83 |
| Lead | Bent Wire | 130 | 5.22 |
| Lead | Narrow Alley | 150 | 3.29 |
| Lead | Honey Horn | 110 | 6.75 |
| Lead | Syrup Line | 120 | 2.84 |
| Lead | Ink Flute | 100 | 7.68 |
| Lead | Frayed Saw | 140 | 5.11 |
| Lead | Pocket Squeak | 160 | 2.22 |
| Lead | Golden Thread | 115 | 5.89 |
| Lead | Melted Neon | 135 | 4.06 |
| Lead | Muted Trumpet | 105 | 7.41 |
| Lead | Round Needle | 125 | 7.98 |
| Pad | Draped Velvet | 140 | 0.67 |
| Pad | Paper Halo | 165 | 0.98 |
| Pad | Warm Projection | 130 | 0.45 |
| Pad | Woven Air | 200 | -2.13 |
| Pad | Satin Room | 120 | 0.96 |
| Pad | Faded Choir | 180 | -2.38 |
| Pad | Ember Strings | 155 | -0.51 |
| Pad | Curtain Light | 145 | 1.15 |
| Pad | Soft Horizon | 150 | -0.01 |
| Pad | Opaque Silk | 135 | -2.42 |
| Pad | Dusk Film | 190 | 3.48 |
| Pad | Golden Haze | 160 | -1.32 |

## 검증

64개 전체를5초씩 실제 엔진으로 오프라인 렌더했다. 무음/NaN/Inf/범위 초과 없이 통과했다. 기준 스테레오 피크는0.1777–0.2514FS다. UI TUI.json/Q-Links.json은1.0.1과SHA256 동일하며 신규파라미터·모드·이미지 변경이 없다.

Native/ARM32/ARM64·SDK 검사와 회귀 결과는 implementation.md의1.1.0 항목을 따른다. 수치 검사와 음악적 청감 승인을 구분한다. 새 뱅크의 MPC 실기기 청음·CPU 및 사용자 취향 승인은 아직 확인하지 않았다. 로컬 비교 WAV는 build/voicing-after/에 있으며 공개Release/push/기기 설치는 이번 작업에서 진행하지 않았다.
