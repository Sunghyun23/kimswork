## Current release — 1.9.0 (2026-11-18)

The approved 1.8.0 MAIN layout/engine is retained. Two approved 88 BPM /1/8 Auto Pan Soul Studies append as built-in programs65/66: Keys / Dusty Sunday, Synth / Velvet Melt. Factory bank66; first64order/names/recipes retained. Legacy SoundPreset46 remains64choices; appended SoundCurrent287 has66choices. Total288parameters; first287declarations match the1.8fixture exactly. Sound selection preserves master/chord/key/strum/glide-preference state. See the current manual/release notes for the coherent public specification; historical sections below are development history.

# Donut Theory factory bank v4 — 1.2.0

현재 정식판1.2.2에서도 이1.2.0 엔진/레시피를 유지한다. 최신 배포·사용자 확인·검증 한계는 [인계 기준](handoff.md)을 따른다.

새 Moog-inspired 엔진을 위한63개 연주 레시피다.1.1.2는 기존 엔진의 프리셋 조정 버전이며,1.2.0은 비선형 믹서·4단 래더·VCA 자체를 바꾼다. 엔진 설계는 [engine.md](engine.md)를 따른다. 모든 연주 음색의 컷오프·공진·엔벌로프를 개별 재설계하고 Tone/Drive를 정돈했으며 새 스테레오 렌더로 레벨을 다시 보정했다.64개 이름/순서·카테고리·개별Low-cut은 유지한다. Init의 값과Low-cut/FX Off는 유지하지만 새 엔진의 고유 질감은 적용된다.

## 음색 방향

- 기존64개 이름·카테고리·번호와32개 코드 진행을 유지한다.63개 연주 음색을 모두 개별 재설계했고 Init은 기존 단일Saw·Low-cut/효과Off 출발점으로 유지한다.
- Bass: Saw/Pulse/Triangle 배합, 정돈된 저역·서브, 짧은 필터 어택과 punch.
- Keys: FM과 금속성 배음을 줄이고 짧은 아날로그 필터 플럭·펄스/삼각파 질감으로 조정한다. 벨·틴 계열은 약한 FM으로 구분을 남긴다.
- Synth: 두 OSC의 두께와 적당한 공진, 필터 엔벌로프로 brass/stab 성격을 구분한다.
- Lead: Saw/Pulse의 중심감, 단음Legato와 짧은 glide, 과하지 않은 detune.
- Pad: 느린 필터/앰프 엔벌로프와 부드러운 듀얼OSC. 넓은 코러스·피치 흔들림은 줄인다.
- 모든 연주 음색은 새 래더Low-pass/Tape 설정을 사용한다. 서로 다른 cutoff/resonance/envelope/pulse/body 값은 sound_bank.py의 개별 레시피에 명시한다.

## 선택·저장 호환성

SOUND PRESET은426px 왼쪽 목록에서 카테고리→음색을 선택하며 다시 열면 카테고리부터 시작한다. 상단 회색MPC PRESET의 현재 내장 프로그램 통합은 평면 목록이다. 정식 플러그인 분류와 외부SDK 등록 방식의 차이는 별도 조사 대상이며, 호스트 전체에서 카테고리가 불가능하다고 단정하지 않는다.

UID와208개 파라미터의ID/순서는 유지한다. 저장한 프로젝트의 파라미터는 그대로 복원하지만 새 엔진을 거치므로 음색은 달라진다. 새 공장 프로그램을 선택하면 재조정된 음색을 불러온다. 음색 선택은 마스터볼륨·코드·보이싱·패드 매핑을 변경하지 않는다.

## Low-cut·출력 레벨

기존 음색별Low-cut을 유지한다: Bass18–28Hz, Keys85–160Hz, Synth85–140Hz, Lead100–160Hz, Pad120–200Hz. Init은Off다. 음색용Low-pass와 혼동하지 않는다.

보정 기준은44.1kHz, master−8dB, velocity96, Bass MIDI36 / Lead MIDI72 단음, 나머지MIDI60 minor7 코드,2초hold/3초release다. audit_presets.cpp --untrimmed로 스테레오 양 채널의 최댓값을 측정하고 calibrate_factory.py로 Sound Trim을 계산한다. Bass·Lead 기준피크−15dBFS, Keys·Synth·Pad−12dBFS. 체감음량 정규화나 모든 연주 조건에서의 mix-ready 보장은 아니다.

## 프리셋 목록

| Category | Name | Low Cut Hz | Sound Trim dB |
|---|---|---:|---:|
| Init | Init | 0 | 0 |
| Bass | Pocket Anchor | 18 | 5.93 |
| Bass | Maple Thump | 20 | 8.62 |
| Bass | Brick Bounce | 22 | 6.19 |
| Bass | Molasses | 18 | 9.74 |
| Bass | Crate Knock | 25 | 10.43 |
| Bass | Soft Cone | 18 | 11.44 |
| Bass | Motor Hum | 20 | 4.72 |
| Bass | Felt Punch | 24 | 9.91 |
| Bass | Honey Slide | 20 | 7.52 |
| Bass | Walnut Pluck | 28 | 6.73 |
| Bass | Low Canvas | 18 | 4.88 |
| Bass | Dust Motor | 22 | 8.28 |
| Keys | Felt Circuit | 85 | 2.11 |
| Keys | Worn Tines | 100 | 0.3 |
| Keys | Honey Reeds | 110 | 0.52 |
| Keys | Kitchen Bell | 140 | -0.01 |
| Keys | Soft Hammer | 95 | 3.17 |
| Keys | Velour Clav | 130 | 2.27 |
| Keys | Milk Glass | 120 | 15.21 |
| Keys | Tape Celeste | 150 | -0.65 |
| Keys | Reed Cabinet | 100 | 0.78 |
| Keys | Bronze Mallet | 160 | 1.92 |
| Keys | Evening Felt | 90 | 1.17 |
| Keys | Pocket Organ | 110 | -1.98 |
| Keys | Pearl Hammer | 135 | 9.2 |
| Keys | Cocoa Tines | 105 | 0.8 |
| Keys | Dust Harp | 145 | 7.1 |
| Synth | Chopped Brass | 100 | 2.67 |
| Synth | Mellow Block | 85 | 0.0 |
| Synth | Velvet Stab | 120 | 5.16 |
| Synth | Amber Stack | 110 | 0.41 |
| Synth | Rubber Chord | 130 | 1.25 |
| Synth | Hollow Print | 95 | 3.18 |
| Synth | Record Bloom | 100 | 0.17 |
| Synth | Copper Pluck | 140 | 4.32 |
| Synth | Faded Poly | 110 | 0.92 |
| Synth | Small Ensemble | 90 | 2.77 |
| Synth | Syncopated Ink | 125 | 2.07 |
| Synth | Glass Chord | 115 | 1.88 |
| Lead | Ribbon Whistle | 110 | 8.1 |
| Lead | Bent Wire | 130 | 7.37 |
| Lead | Narrow Alley | 150 | 6.09 |
| Lead | Honey Horn | 110 | 8.03 |
| Lead | Syrup Line | 120 | 5.31 |
| Lead | Ink Flute | 100 | 10.29 |
| Lead | Frayed Saw | 140 | 7.26 |
| Lead | Pocket Squeak | 160 | 6.03 |
| Lead | Golden Thread | 115 | 7.69 |
| Lead | Melted Neon | 135 | 6.89 |
| Lead | Muted Trumpet | 105 | 8.48 |
| Lead | Round Needle | 125 | 10.86 |
| Pad | Draped Velvet | 140 | 0.84 |
| Pad | Paper Halo | 165 | 2.24 |
| Pad | Warm Projection | 130 | 1.69 |
| Pad | Woven Air | 200 | -1.68 |
| Pad | Satin Room | 120 | 2.72 |
| Pad | Faded Choir | 180 | 0.54 |
| Pad | Ember Strings | 155 | 0.57 |
| Pad | Curtain Light | 145 | 2.69 |
| Pad | Soft Horizon | 150 | 0.02 |
| Pad | Opaque Silk | 135 | -0.95 |
| Pad | Dusk Film | 190 | 4.77 |
| Pad | Golden Haze | 160 | -0.65 |

## 검증

64개 전체를5초씩 오프라인 렌더했다. 무음/NaN/Inf/범위 초과 없이 통과했다. 기준 스테레오 피크는0.177732–0.288887FS(Init 포함), 연주 음색 목표는Bass/Lead−15dBFS, 그 외−12dBFS다. Sound Trim은−1.98..15.21dB다. UI와208개 파라미터 선언은1.1.2와 동일하다. 수치 검사는 음악적 청감 승인이 아니며 실기기 범위는 [구현 기록](implementation.md)과 릴리스 노트를 따른다. 원본 렌더는build/ladder-after/에 보존한다.
