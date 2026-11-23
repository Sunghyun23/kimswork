> Latest owner decision (2026-11-23): Loop Club0.2.0 has eight active loop tracks, a source timeline above a4×2loop-status board, and sixteen instrument pads. Older sixteen-slot project data is preserved as inactive compatibility storage. This supersedes historical sixteen-active-track counts below. REC still enters the recording/performance workspace directly; explicit SEQ opens the full editor. Current local device validation is in mods/public/LoopClub/docs/validation-0.2.0.json.

# MPC Sample 16트랙 루퍼 — SRS 초안 0.1

문서 상태: 설계 검토용. 제품명은 **Loop Club**, 제조사는 **kim**이다(2026-11-22). 아래 요구사항은 제품 구현 완료를 뜻하지 않는다.

표기: **확정**은 사용자가 정한 방향, **권장안**은 구체화를 위한 설계 제안, **미정**은 구현 전 선택이 필요한 항목이다. 버튼·패드의 동일 동작은 화면 문맥 안에서 해석한다.

## 1. 제품 목적과 범위

- **확정:** MPC Sample에서 16개 트랙으로 오디오 루프, 내장 신스 MIDI 루프, 드럼 킷 MIDI 루프를 조합한다.
- **확정:** 현재 BPM, 녹음 전 프리롤, 트랙마다 독립적인 루프 길이를 사용한다. 첫 루프의 길이로 BPM을 추정하지 않는다.
- **확정:** 실제 패드·왼쪽 페이더·K1–K3 회전 노브·DATA·화면 위 B1–B3 버튼으로 연주한다. 화면은 비터치 320×240이다.
- **확정:** 믹싱과 개별/전체 FX를 메뉴를 거치지 않고 빠르게 전환하는 퍼포먼스 악기를 목표로 한다.
- **설계 기준:** 순정 앱을 확장하는 Rust 모드로 시작한다. SampleBoot에서 선택할 수 있는 패키지를 목표로 하며, 내부 펌웨어 전체를 교체하는 작업으로 정의하지 않는다.

## 2. 용어와 공통 상태

- 트랙: 16개 루프 채널 중 하나. 종류는 AUDIO, SYNTH, DRUM이다.
- 사운드: DRUM 트랙의 킷 안에 있는 개별 악기. 트랙과 별개이며 킷 전체가 한 트랙을 차지한다.
- 마스터: 16트랙과 FX 반환음을 합친 공통 출력 채널. 16개 트랙 수에 포함하지 않는다.
- 선택: 편집/볼륨 조절 대상을 지정하는 동작. 재생·정지와 구분한다.
- 프리롤: 녹음 전에 제공하는 카운트 구간. 해당 테이크에 녹음하지 않는다.
- 상태: EMPTY / ARMED / COUNT-IN / REC / PLAY / STOP. OVERDUB 지원 시 별도 DUB 상태를 사용한다.
- 녹음 종료 요청은 실제 녹음 종료와 구분해 표시한다.

## 3. 트랙과 소리 생성

| ID | 요구사항 | 상태 |
|---|---|---|
| TRK-01 | 전체 16트랙을 AUDIO/SYNTH/DRUM으로 자유롭게 구성한다. | 확정 |
| TRK-02 | 서로 다른 길이의 트랙을 동시에 반복한다. 1·2·3·4마디 등 비정규 길이도 허용한다. | 확정 방향의 구체화 |
| TRK-03 | AUDIO는 선택된 마이크/라인/USB 등 입력의 실제 소리를 녹음한다. 지원 소스와 스테레오 경로는 기기에서 확인한다. | 확정 / 실기 검증 필요 |
| TRK-04 | SYNTH는 MIDI 노트의 음높이·시작 시점·길이·벨로시티를 저장하고 내장 엔진으로 재생한다. | 확정 |
| TRK-05 | DRUM은 한 킷의 여러 악기를 한 트랙의 MIDI 이벤트로 녹음한다. | 확정 |
| TRK-06 | 신스/드럼 음색 변경은 저장된 MIDI 이벤트를 바꾸지 않는다. | 권장안 |
| TRK-07 | 트랙 종류 변경으로 기존 자료를 소리 없이 삭제하지 않는다. 보존/변환/초기화 정책을 먼저 정한다. | 필수 보존 원칙 / 상세 미정 |

내장 신스는 Sample의 CPU에 맞는 가벼운 엔진으로 시작한다. Bass/Keys/Lead/Pad를 출발점으로 하되, 발음 수·합성 방식·프리셋 수는 미정이다. 드럼은 패드당 사운드 하나, 킷당 최대 16개를 설계 기준으로 한다. 샘플형/합성형 여부와 기본 킷은 미정이며 기존 개인용 음원을 자동 포함하지 않는다.

## 4. 녹음·프리롤·타이밍

| ID | 요구사항 | 상태 |
|---|---|---|
| REC-01 | 녹음과 길이 계산은 현재 BPM 및 공통 박자에 따른다. 기본 박자는 4/4로 설계한다. | BPM 확정 / 4/4 권장안 |
| REC-02 | 빈 패드를 누르면 대기/프리롤을 거쳐 녹음한다. 기본 프리롤은 1마디, Off/1/2마디 선택을 제공한다. | 확정 |
| REC-03 | 녹음 시작은 마디 첫 박에 맞춘다. 프리롤 소리는 루프에 포함하지 않는다. | 확정 |
| REC-04 | 녹음 중 같은 패드를 다시 누르면 종료를 예약하고 다음 마디 경계에서 즉시 반복 재생한다. | 확정 |
| REC-05 | 3마디째에 종료를 요청하면 그 마디 끝에 닫아 3마디 루프가 된다. 1/2/4/8마디로 강제하지 않는다. | 확정 예시 |
| REC-06 | 다른 트랙 재생은 프리롤·녹음·새 트랙 추가로 멈추거나 위상이 초기화되지 않는다. | 확정 |
| REC-07 | 녹음 대기/프리롤 중 해당 패드를 다시 누르면 취소한다. | 확정 |
| REC-08 | 화면은 프리롤 카운트, REC 상태, 경과 마디, 종료 예약을 구분한다. | 확정 |
| REC-09 | 루프 타이밍은 공통 오디오 프레임 기준으로 계산하고 녹음 길이는 정수 프레임으로 관리한다. 입출력 지연과 경계 클릭을 처리한다. | 기술 요구사항 |
| REC-10 | 녹음 종료 시 열린 MIDI 노트를 처리하고 재시작·정지·화면 전환에서 stuck note를 만들지 않는다. | 기술 요구사항 |

**미정:** 재생 중 마디 중간에 요청한 프리롤의 시작점. 권장안은 다음 마디 첫 박부터 온전한 지정 마디 수를 카운트하는 것이다. 이는 요청 시점에서 최대 한 마디의 추가 대기가 생길 수 있으므로 승인 후 확정한다.

**미정:** 동시 녹음 트랙 수, 최대 녹음 시간, 메트로놈의 프리롤 후 유지 여부, 재생/녹음 도중 BPM 변경, 타임스트레치, 오버더빙·녹음 Undo 정책. 화면의 UNDO 표시만으로 완성된 Undo를 전제하지 않는다.

## 5. 기본 패드·페이더 조작

- LOOP의 빈 패드: 해당 트랙 녹음 준비.
- LOOP의 녹음 중 패드: 종료 예약.
- LOOP의 재생/정지 트랙 패드: 정지/재생 전환. 전환의 즉시/박/마디 양자화는 미정이다.
- SHIFT+패드: 재생을 바꾸지 않고 트랙 선택.
- PLAY: 패드는 선택한 SYNTH의 건반 또는 DRUM 킷 연주에 사용한다.
- MIDI 입력: 선택/녹음 대상 트랙으로 라우팅한다. 외부 건반의 채널 처리와 패드 스케일은 미정이다.
- 왼쪽 페이더: 선택한 채널의 볼륨. MASTER를 명시적으로 선택했을 때만 마스터 볼륨이다.
- 선택을 바꾸면 저장된 볼륨을 보존하고, 페이더가 그 위치에 도달/통과한 후 조절되는 픽업을 적용한다.
- 화면에 선택 대상, 저장된 값, 픽업 방향을 표시한다. 볼륨 변화는 스무딩한다.
- STOP은 자료를 삭제하지 않고 재생을 멈춘다. 음원/FX 꼬리의 정확한 정지 방식은 미정이다.

각 화면의 패드 역할은 항상 표시한다. 누른 상태에서 화면이 바뀌어도 원래 소유자에게 release를 전달하고 순정 앱의 샘플이 중복 발음하지 않게 한다.

## 6. 화면 구조

| 화면 | 주요 내용 |
|---|---|
| LOOP | 패드와 같은 순서의 4×4 트랙 상태, 종류, 길이, 선택 트랙 볼륨, BPM·녹음 상태 |
| PLAY | 내장 신스/드럼 연주·MIDI 녹음, 선택 트랙과 음색 |
| GRID | 선택 MIDI 루프의 노트와 타이밍·벨로시티 편집 |
| MIX | 16트랙 전체 볼륨·미터·Mute/Solo·FX 전송량 |
| KIT MIX | 선택 DRUM 킷의 최대 16개 사운드 믹싱 |
| TRACK FX | 선택 트랙의 인서트/퍼포먼스 FX 선택·조절 |
| MASTER FX | 전체 믹스 및 그룹 대상 퍼포먼스 FX와 마스터 처리 |
| SETUP / PROJECT | 프로젝트 저장/불러오기, 프리롤·입력·시스템 설정 |

LOOP/PLAY/GRID 등 주요 화면의 최종 진입 버튼은 미정이다. SHIFT+DATA 공통 화면 메뉴는 제안이었으며, FX 세 화면 사이의 빠른 전환을 이 메뉴로 대체하지 않는다.

## 7. GRID 편집

- SYNTH: 가로 시간, 세로 음높이인 피아노 롤.
- DRUM: 가로 시간, 세로 킥/스네어/하이햇 등 사운드 행.
- 녹음한 MIDI 노트가 그리드에 표시되고 재생 위치가 보인다.
- DATA로 노트/커서를 선택하고 K1 음높이/악기, K2 길이, K3 벨로시티를 조절하는 것을 기본안으로 한다.
- 여러 마디는 마디/페이지 이동으로 편집한다. 현재 트랙·마디·범위를 표시한다.
- 노트 생성/삭제/시작 시점 이동, 확대·분해능, 녹음 양자화, 편집 Undo의 최종 조작은 미정이다.
- AUDIO의 파형·트림 편집은 별도 검토 항목이며 MIDI 그리드와 같은 기능으로 취급하지 않는다.

## 8. 전체 믹서와 드럼 믹서

**확정:** 두 믹서 모두 16개를 4×4로 한 화면에 표시한다. 사운드가 16개 미만이면 빈 칸을 표시한다. 예전 4채널씩 페이지 전환 제안은 폐기한다.

- 각 칸: 번호/이름, 볼륨 값, 볼륨 막대·출력 미터, Mute/Solo 상태.
- 선택 채널은 금색 계열 테두리로 강조하고 아래 상세 영역에 전체 이름·볼륨·Pan/Send 값을 표시한다.
- MIX의 패드는 트랙 선택만 하며 재생을 바꾸지 않는다.
- KIT MIX의 패드는 해당 드럼 선택·미리 듣기, SHIFT+패드는 무음 선택이다.
- 왼쪽 페이더는 선택 채널 볼륨, K1 Pan, K2 Delay Send, K3 Reverb Send다.
- DRUM 트랙의 전체 믹서 페이더는 킷 합산 레벨, KIT MIX 페이더는 개별 악기 레벨이다.
- 벨로시티는 개별 MIDI 이벤트 강약이며 믹서 볼륨과 구분한다.
- 킥·스네어 등을 다른 DRUM 트랙으로 분리하는 구성을 지원한다. 기존 패턴의 자동 분리 도구는 미정이다.
- KIT MIX 진입/복귀, Mute/Solo 토글, 마스터 선택의 물리 조작은 최종 검토가 필요하다. 초기 시안의 MUTE/SOLO/BACK 버튼은 이후 FX 전환 정책과 충돌하지 않게 수정한다.

## 9. FX 구성과 오디오 라우팅

**확정한 기본 방향:** 트랙 INSERT 2슬롯, 공유 Delay/Reverb SEND, 별도 마스터 처리를 제공한다. Drive/필터/컴프레서/Lo-Fi를 트랙 FX 후보로 한다. 실제 효과 목록/동시 사용 상한은 기기 성능 측정 후 확정한다.

- 신스 기본 필터/ADSR은 악기 설정이며 FX 슬롯을 차지하지 않는다.
- DRUM 트랙 INSERT는 킷 전체에 적용한다. 개별 드럼 INSERT는 현재 확정 범위가 아니다.
- 트랙·드럼 Send는 볼륨을 내리면 효과음도 함께 줄어드는 post-fader 동작을 기준으로 한다.
- 개별 드럼 Send도 상위 DRUM 트랙 볼륨과 Mute/Solo에 연동한다. 직접 신호와 Send가 이중으로 합산되거나 레벨이 두 번 곱해지지 않게 한다.
- 공유 Delay/Reverb와 마스터 FX를 구분한다. 공유 FX의 반환음도 최종 마스터 출력으로 합친다.
- 원본 AUDIO 녹음은 기본적으로 FX를 나중에 바꿀 수 있는 원음 저장이다.

**퍼포먼스 라우팅 권장안:** 선택 트랙들을 FX BUS A/B로 묶거나 MASTER 전체를 대상으로 효과를 연주한다. A/B의 serial/parallel 구성, Dry 경로, Send 반환음이 받는 효과 범위, 슬롯 수/동시 효과 수는 미정이다. 모든 합산·분기는 실제 신호 흐름을 확정한 뒤 구현한다.

## 10. 빠른 FX 전환 — 최신 사용자 결정

FX 섹션 안에서는 화면 위 물리 버튼을 다음 역할로 고정한다.

| 버튼 | 즉시 이동할 화면 | 해당 화면의 패드 |
|---|---|---|
| B1 | MIX | 조절할 트랙 선택 |
| B2 | TRACK FX | 선택 트랙의 효과 선택/연주 |
| B3 | MASTER FX | 전체 믹스의 효과 선택/연주 |

- K1/K2/K3는 회전 노브이며 화면 전환 버튼으로 사용하지 않는다.
- B1→트랙 패드→B2→효과 패드→B3→마스터 효과 패드의 흐름에 메뉴/확인창을 요구하지 않는다.
- 전환은 재생·녹음·박자·FX 상태를 초기화하지 않는다. 각 화면은 마지막 선택/편집 대상을 기억한다.
- TRACK FX에서 MASTER FX로 이동해도 켜둔 트랙 효과는 계속 동작한다. 패드 release가 필요한 효과는 화면 전환과 무관하게 올바르게 해제한다.
- 회전 노브는 현재 화면의 파라미터를 조절한다. 왼쪽 페이더는 명시된 선택 채널 볼륨을 유지한다.
- 초기 시안의 SAVE/UNDO/SETUP 또는 MUTE/SOLO/DETAIL 상단 고정 배치는 이 세 화면의 확정 배치가 아니다. 이 화면들은 새 B1/B2/B3 정책으로 재설계해야 한다.
- FX 섹션 자체 진입 버튼은 미정이다. PAD FX/KNOB FX 물리 버튼 활용이 권장안이며 실제 입력을 검증한다.

## 11. 마스터와 퍼포먼스

- 마스터는 16개 일반 트랙 외의 공통 채널이다. 전체 출력 볼륨과 큰 스테레오 미터, 피크, 압축량을 표시한다.
- 마스터 선택/볼륨을 명확히 표시하고 페이더 픽업을 적용한다.
- 기본 마스터 처리는 가벼운 컴프레서 및 출력 피크 보호를 설계 기준으로 한다. True-peak/brickwall 보장은 별도 구현·측정 없이 주장하지 않는다.
- 퍼포먼스 효과 후보: Filter, Isolator, Repeat, Reverse, Tape Stop, Dub Echo, Crush, Wobble.
- HOLD는 누르는 동안 적용, LATCH는 놓아도 유지한다. 정확한 latch 토글 조작은 미정이다.
- 적용 대상 MASTER/BUS A/B 또는 선택 TRACK을 항상 표시한다. 효과를 바꾸는 동작이 트랙 선택으로 오해되지 않게 한다.
- FX OFF는 퍼포먼스 효과를 해제하고 루프 자료·기본 재생을 유지한다. 기본 믹스용 인서트까지 해제하는지는 미정이다.
- 노브 변경은 부드럽게 처리하고, 파라미터·선택 효과·대상은 화면에서 확인할 수 있어야 한다.
- 동일 시점 복수 효과의 순서·조합 상한·모순 효과 우선순위는 CPU/음악적 검토 후 확정한다.

## 12. RESAMPLE·프로젝트 저장

- 퍼포먼스와 FX가 포함된 결과를 빈 AUDIO 트랙으로 녹음하는 RESAMPLE을 제공하는 방향이다.
- 원래 AUDIO/MIDI 루프는 보존한다. 녹음 대상 트랙을 자기 자신의 입력에 다시 포함해 피드백을 만들지 않는다.
- RESAMPLE에도 현재 BPM/녹음 경계 규칙을 적용한다. 프리롤·Send/마스터 처리 포함 지점은 최종 라우팅과 함께 확정한다.
- 프로젝트 저장 항목: 트랙 종류·길이·오디오/MIDI, 신스/킷 설정, 믹서 값, FX·Send·라우팅, BPM·프리롤 등.
- 사용자 오디오는 프로젝트 파일과 함께 보존하고 누락 파일을 알려야 한다. 저장 도중 실패해도 이전 정상 저장본을 유지한다.
- 로드 시 임시 녹음 대기·HOLD 입력은 해제한다. 자동 재생과 LATCH 활성 상태의 복원 여부는 미정이다.
- SD 저장 형식, 오디오 내보내기/외부 MIDI 파일 내보내기, 녹음 Undo 횟수, 자동 저장은 미정이다.

## 13. UI·성능·안정성

- 320×240 비터치 화면에 맞춘다. 4×4 배열은 물리 패드 순서와 일치하며 위13–16, 아래1–4다.
- 빈티지 계측기 방향의 절제된 서체·짙은 녹색·크림·금색을 시안 기준으로 한다. 레이아웃 최종 승인 전 제품 스킨 확정으로 취급하지 않는다.
- 재생/녹음/대기/Mute/Solo/선택은 글자·모양을 병행해 색만으로 구분하지 않는다.
- 빈 칸에 가짜 미터/파형을 표시하지 않는다. 구현된 미터는 실제 오디오를 측정한다.
- 화면 전환 응답은 100ms 이내를 권장 목표로 두고 실기에서 측정한다. 녹음/루프 경계의 음악적 대기는 별도로 표시한다.
- 오디오 콜백에서 할당·락·파일 I/O·화면 그리기·로그 출력을 하지 않는다. 초기 할당과 저장은 별도 경로를 사용한다.
- 입력/출력/조작/UI 간 명령과 스냅샷을 안전하게 전달한다. 입력 소유권·해제·지연 보정은 실제 하드웨어에서 검증한다.
- 화면은 변경 영역 중심으로 갱신한다. 큰 화면 리페인트가 패드/오디오 지연을 만들지 않게 한다.
- 16트랙과 지원 FX 조합에서 오디오 블록 마감시간을 지켜야 한다. 메모리·최대 길이·발음 수·FX 상한은 실측해 문서화한다.
- CPU 초과·버퍼 부족·지원하지 않는 펌웨어·입력 단절·SD 오류를 무음 실패나 자료 삭제로 처리하지 않는다.

## 14. 플랫폼·배포·프로젝트 분리

- 위치: `mpc-software/mpc-sample/mods/public/<Project>/` 또는 `private/<Project>/`.
- 기존 Donut Theory/Groove Dice/Crate Press/Felt Club과 SDK·상태·제품 버전·패키지를 분리한다. 실제 필요가 생긴 DSP만 재사용한다.
- 현재 개발 기준: SDK0.7.3 고정 커밋, SampleBoot1.0.2 고정 커밋, Rust1.97.0. 상세는 `sample-sdk.lock.json`을 따른다.
- 펌웨어 대상1.3.0.12. 버전 문자열뿐 아니라 앱 fingerprint를 확인한 뒤 hook을 로드한다.
- SD autorun 패키지와 SampleBoot 메뉴 항목을 계획한다. 기존 autorun/프로젝트를 보존하고 내부 펌웨어를 덮어쓰지 않는다.
- SampleBoot는 선택 실행 도구이며 여러 모드 동시 실행을 보장하지 않는다.
- 기기 설치·재시작·SD 변경·공개 배포는 이번 SRS 작성에 포함하지 않는다. 제품명/제조사 확정 후 정식 프로젝트를 생성한다.
- 개인 음원·펌웨어·제한된 라이브러리는 공개 소스/패키지에서 제외한다.

## 15. 주요 인수 시험

1. 서로 다른 종류·길이의 16트랙을 공통 박자로 재생하고 장시간 위상/길이 누적 오차를 검사한다.
2. Off/1/2마디 프리롤, 대기 취소, 녹음 시작/종료 경계를 오디오와 MIDI로 검사한다.
3. 다른 루프 재생 중 새 녹음이 기존 루프를 멈추거나 위상을 초기화하지 않는지 확인한다.
4. MIDI 노트의 시작·길이·벨로시티가 실제 녹음/재생 및 GRID 편집과 일치하는지 확인한다.
5. 트랙/드럼 사운드 선택 변경 시 페이더 픽업 전 값 유지, 픽업 후 해당 대상만 변경되는지 확인한다.
6. DRUM이 한 트랙으로 동작하고 킷 개별 볼륨·Pan·Send·Mute/Solo가 독립적으로 반영되는지 검사한다.
7. B1/B2/B3 전환 시 연주·녹음·효과가 유지되고 패드/노브 소유권과 release가 올바른지 검사한다.
8. HOLD/LATCH/FX OFF/여러 효과 조합 및 마스터·버스 라우팅을 실제 오디오로 검사한다.
9. DRUM 트랙 볼륨을 내렸을 때 드럼별 Send도 함께 줄어드는지 확인한다.
10. RESAMPLE 결과에 지정된 FX가 포함되고 원본 보존·자기 피드백 차단이 동작하는지 검사한다.
11. 저장/로드·누락 파일·저장 실패를 검사하고 원래 프로젝트가 보존되는지 확인한다.
12. 정지/삭제/모드 변경/입력 단절/메모리 한계에서 stuck note, 무한 FX, 잘못된 데이터 접근이 없는지 확인한다.
13. 실제 기기에서 CPU·메모리·녹음 지연·화면 전환 속도·패드/페이더/노브·복귀를 검증한다.

## 16. 현재 구현·검증 상태

완료: 디렉터리 분리, 비공개 제외, SDK/부팅 도구 소스 고정, 별도 Rust 환경, 읽기 전용 기기 버전 조회 도구. 선택 SDK0.7.3은 126개 단위 시험과1개 문서 시험, strict Clippy, ARM64 cargo check를 통과했다.0.7.4는 자체 시험1건/Clippy4건 실패로 채택하지 않았다.

실기 확인: 연결된 장치의 자체 조회 API가1.3.0.12를 반환했다. 앱 fingerprint·모드 로딩·SampleBoot·물리 오디오/조작은 아직 검증하지 않았다.

개발 중: [Loop Club](../mods/public/LoopClub/README.md)에 네이티브 Rust 코어와 회귀 시험을 작성했다. 오디오/MIDI 녹음·재생, 신스/드럼, 믹스·FX, 저장 형식 및 컨트롤러 상태의 초기 구현이며 완성된 기기 제품이 아니다. SDK 연결, 공통320×240 렌더러8개 화면, 프로젝트 조작 UI와 로컬 개발용 SD 패키지를 작성했다. 물리 입력·오디오·지연·CPU 및 실제 부팅·복구 검증, 전체 GRID 편집·Undo 등 남은 범위는 제품 README/매뉴얼에 명시한다. 아직 기기에 설치하지 않았다. 공개된 화면은 디자인 시안이며 실제 기기 화면이 아니다.

## 17. 구현 전 주요 미정 항목

1. 제품명 Loop Club·제조사 kim 확정. 공개 소스 프로젝트로 생성했으며 공개 배포 전 별도 검증이 필요하다.
2. 이미 재생 중일 때 프리롤이 시작되는 경계.
3. 최대 루프 길이·동시 녹음 수·스테레오·오버더빙/Undo.
4. BPM 변경·타임스트레치·클립 시작/정지 양자화.
5. 신스 상세 방식·발음 수·프리셋, 기본 드럼 킷과 소스.
6. 주요 화면 진입·GRID 편집·Mute/Solo·KIT MIX 및 마스터 선택의 최종 버튼 배치.
7. BUS A/B/MASTER 라우팅·FX 정확한 목록/순서/동시 사용 상한과 LATCH 조작.
8. 저장 형식·로드 시 재생/FX 상태·원래 앱 화면 복귀 시 동작.

이 항목들은 임의로 구현 완료/확정으로 표시하지 않는다.

## 18. 최신 확정: 반복 녹음·연주·소스 편집 (2026-11-22)

이 절은 앞의 오버더빙 미정/일회 녹음 종료 설명보다 우선한다. 트랙마다 REPLACE(기본)/OVERDUB를 저장하며 LOOP·PLAY의 B3에서 전환한다. GRID는 물리 SEQ로 진입한다. 첫 녹음 중 REC는 다음 마디 끝에서 길이를 정하고 반복 녹음을 계속한다. 다시 REC를 누르면 현재 회전 끝에서 녹음만 종료하고 마지막 완성본의 루프 재생은 유지한다. 다른 트랙의 재생은 멈추지 않는다. 기존 루프에 REC로 재진입할 수 있다. 녹음 중 모드 변경은 다음 루프 시작부터 적용한다.

REPLACE: AUDIO/SYNTH는 회전 전체를 교체. DRUM은 회전 중 처음 친 악기 파트만 이전 내용을 제거하고 새 타격을 기록하며, 치지 않은 악기는 보존한다. OVERDUB: 기존 오디오/노트/드럼 타격에 새 연주를 더한다. 취소·입력 오류·이벤트 용량 초과 시 미완성 회전만 버리고 마지막 완성본을 보존한다. RESAMPLE은 자기 피드백을 피하기 위한 별도의 일회 바운스다.

SYNTH PLAY B2는 NOTES↔CHORD로 확정. DRUM 모드에서 SHIFT+악기 패드로 해당 악기 패턴만 생성한다(기존 Groove Dice의 음악적 규칙 참조). MAIN SHIFT+패드는 트랙 무음 선택을 유지한다. AUDIO는 녹음 또는 SD WAV 불러오기로 시작한다. CHOP은 Equal/Transient/BPM/Manual, 원본 파형·마커, 사용자 Start/End/Level, 분할/병합/Undo를 제공한다. 위 구현의 기기 오디오·CPU·물리 입력·부팅 검증은 아직 미완료다.

## 19. 신스 확장 확정 (2026-11-22)

OSC1/OSC2는 각각 Sine/Saw/Square/Triangle, Tune±24반음, Fine±100센트, Level을 제공한다. 필터는 LP/BP/HP·Cutoff·Resonance·Env Amount, AMP ADSR와 FILTER ADSR는 독립이다. SYNTH PLAY에서 +/-로 PLAY/OSC1/OSC2/FILTER/AMP ADSR/FILTER ADSR를 순환한다. 각 편집 중 패드는 계속 연주용이며 B2 NOTES↔CHORD, B3 REC MODE는 유지한다. 세 노브에서 부족한 Fine·Env Amount·Release는 SHIFT로 조절하고 모든 관련 값은 화면에 표시한다. 기기 검증은 별도다.

## 20. LFO 확장 (2026-11-22)

사용자 요청: 필터/피치/볼륨 타깃과 Saw 등을 포함한 여러 파형. 구현은 Sine/Triangle/Saw Up/Saw Down/Square/Random Hold의 LFO 한 개와 세 타깃의 독립 깊이로 구성한다. 모두 동시에 사용 가능하며 기본 깊이는0이다. SYNTH 편집7/7의 K1Rate/K2Pitch/K3Filter, SHIFT+K1Shape/SHIFT+K3Volume으로 조절하고 다섯 값을 모두 표시한다. 노트마다 위상을 재시작하며 Hz 속도다. BPM Sync는 미구현이다. 기존 음색·코드·녹음 모드와 드럼 엔진을 보존한다. 선택한 전체 트랙 볼륨은 기존 왼쪽 페이더다.


## Latest Loop Club workflow — 2026-11-23, local 0.3.0

This supersedes the earlier eight-track and REC auto-navigation decisions. All 16 loop tracks are active. A LOOP pad starts capture without changing the screen; only the pads become instrument performers. The initial trigger never also plays a note. Synth uses the CHROMATIC/CHORD setting from SOUND. REC requests bar-end capture completion and restores loop pad roles. External MIDI targets the active Synth/Drum recorder. REPLACE creates a new whole-bar length; OVERDUB preserves the existing length and untouched tail.

On LOOP, SHIFT+B1/B2/B3 directly select LOOP/MUTE/CHAIN. SAMPLE returns to normal LOOP. Unshifted B1/B2/B3 always remain SOURCE/SOUND/REPLACE–OVERDUB. Recording protects the pad role against mode changes. MUTE preserves phase. CHAIN has up to 16 ordered steps, duplicate loops, 1–16 repeats and optional whole-chain repetition. Only chain members switch; independent tracks continue. DATA rotation selects a step and its sound; DATA push starts/stops the chain. K1 sets repeats, K2 whole-chain repeat, ERASE+pad removes a matching step. Current cached repeats finish before step edits apply. Save configuration; load stopped.

186 Debug and 186 Release tests, host/ARM64 strict Clippy and ARM64 build passed. Isolated real Sample QA verified remote recording/mode/chain controls, actual CoreMIDI input and USB audio, then restored all 16 owner track slots. Actual DRM captures were inspected. This does not certify physical pads/buttons or latest power-cycle boot. See LoopClub/docs/validation-0.3.0.json. Public release remains 0.1.0; no new publication is implied.


## FX pad performance — local 0.3.1, owner approved 2026-11-23

FX pages contain only effect-name/state pads and the fixed top navigation. Remove utility pads (slot/HOLD selector/routing/setup/all-off), parameter cards and footer. Unassigned positions are inert blanks, not invented effects. Track/master-insert boards expose the 11 existing effects directly, with no SHIFT effect bank. Main master board keeps its five texture stages, Duck and two named custom effects.

Ordinary press is immediate momentary HOLD; release disables that effect even after page/track navigation. SHIFT+press toggles per-effect LATCH; SHIFT is not an alternate-effect selector. Converting a held effect to LATCH cancels its old release owner. Render OFF/HOLD/LATCH; refused extra insert shows FULL on that pad. Keep existing DSP, saved parameters, two track-insert slots and master independent processing; do not silently replace another active effect. This entry supersedes previous default tap-toggle/optional-HOLD policy. Local build/device verification must be recorded separately from physical input acceptance.


## Modulation FX — local 0.4.0, owner approved 2026-11-23

TREMOLO, AUTO PAN and AUTO FLT are appended effect types (12–14); existing effect IDs remain unchanged. They operate on audio, synth and drums through the existing insert processing. WOBBLE retains pitch modulation and gains Sync/shape control. All use ordinary press/release HOLD and SHIFT+press LATCH. The TRACK FX board adds pads12/13/14. The main MASTER board adds pads9/10/11 using its existing two custom insert slots; do not silently expand capacity or replace another active effect. Old custom tiles occupied by dedicated modulation types become inert blanks.

DATA rotation selects a visible effect without enabling it. DATA push opens FX SETTINGS; B1 or DATA returns to the same board and selected slot. K1 Rate/K2 Depth/K3 Mix; B2 Shape cycles six waveforms and B3 Sync toggles BPM divisions. Straight/dotted/triplet divisions include1bar through1/32T. Free rate values are retained when Sync changes; rate follows BPM but is not promised transport-phase locked. Settings do not turn an inactive effect on. With both slots used, an additional effect is refused visibly.

SOUND retains pitch/filter LFO. New projects have seven synth pages, with no dedicated Pan/volume LFO controls. Old nonzero volume/pan LFO values remain acoustically intact and expose an eighth LEGACY MOD page (K2 Pan, SHIFT+K2 Volume) so they can be adjusted/zeroed. Do not silently rewrite saved sounds. New FX metadata has serde defaults for old projects. The MIDI/loop/record button policy and missing recording UNDO are unchanged. Physical inputs, actual USB audio, runtime/SD persistence and public release are separate verification claims.


0.4.0 verification completed: 203 Debug + 203 Release tests, strict host/ARM64 Clippy, ARM64 normal/isolated builds. Actual remote Sample QA verifies all16sync divisions, six shapes, track/master HOLD/LATCH, fader targeting, temporary-state suppression and LATCH retention on project save/load, actual stereo USB Auto Pan and existing Wobble. Owner16trackslots restored. Runtime and SD SHA256 3252d9c511dcbbadac2a049ed8239fdeb4606e1bc75f0f66b86fe1e323dd7910 match;20boot files read back and20previous files preserved. Actual DRM screens inspected. Project v4 preserves LATCH; v1–3 remain readable, but older apps reject v4 saves. Save requires stopped transport and completed release tails. Physical controls, power-cycle and worst-case CPU remain separate: see docs/validation-0.4.0.json. No new public release/site changes.


## Latest clip-history decision — 2026-11-23, local 0.5.0

The owner approved implementing physical SHIFT+− UNDO / SHIFT++ REDO. Plain +/− keep their page/selection behavior. One shared last clip edit covers completed Audio/Synth/Drum REPLACE/OVERDUB, continuous recording as one gesture, drum generation, MIDI-grid edit, clear and applied/manual chops. It restores clip data, not instrument/mix/FX settings. Cancel an unfinished take without consuming older history; no partial-take REDO. New committed edits clear REDO. History is volatile and clears on project load/restart. Preserve current16track LOOP/MUTE/CHAIN and FX pad semantics. See LoopClub/docs/validation-0.5.0.json for native, real remote-controller/USB-audio and SD verification; physical SHIFT+key acceptance remains separate.


## Latest always-play pad workflow — local 0.6.0, 2026-11-23

Normal LOOP pads always perform the selected instrument. SHIFT+pad selects and starts recording that numbered track; SHIFT+same or REC finishes at the next bar. SHIFT+different queues the next track: current sound remains active until the boundary, then commits/loops and the next capture/performance target starts without extra count-in. Display current REC > next NEXT. Retarget initial count-in without changing its start. STOP/UNDO clear pending targets. DATA chooses the instrument while not recording; MUTE/CHAIN keep their specialized pad operations. Existing configured Synth chord/chromatic mode is preserved. Supersedes previous pad-start/temporary-performance and shifted playback-toggle behavior. See product manual and validation-0.6.0.json.


### 2026-11-23 approved Audio record targets (local0.7.0)

Audio tracks support both live LINE/MIC/USB capture and existing WAV import. Choose REC: INPUT / CHROMATIC / CHOP explicitly; never automatically infer or switch the target after capture. SHIFT+track-pad starts/counts in, SHIFT+same-track-pad requests bar-end finish; LOOP remains visible and ordinary pads perform. INPUT stores PCM, CHROMATIC stores pitched hold-loop performance and CHOP stores slice hold-loop performance, preserving the original PCM in both performance modes. Existing WAV import copies from the file into an empty Audio track without modifying it. SOURCE pads5/6/7 select the recording target; loaded Audio SOUND B1 selects REC target, B2 opens CHOP EDIT, K1Root/K2Tune/K3Warp. Save original source musical duration independently from the newly recorded performance duration, and tag chop events so changing target or overdubbing pitches cannot reinterpret old events. New format5 embeds all source/performance/settings; old formats1–4 remain readable. Bounded grain-based Warp is an initial implementation with transient/extreme-ratio limits, not a claim of commercial stretch quality.


2026-11-23 approved/local0.8.0: Drum SOURCE pad5 or SHIFT+BROWSE imports an XPM Drum Program after explicit populated A–H bank selection, preserving16padpositions. Worker prepares up to4samplezones/velocity/cycle/random/mute/gate, retained underL1; project6embedsPCM. Keygroup/XPN/nativeMPCFX/filterenvelopes/Warp are unsupported, original files and failed imports preserve content. Local verification and limits: mods/public/LoopClub/docs/qa-0.8.0.md; no new public release.


Latest0.9.2 user-approved FX UX: pressing a physical/remote FX pad immediately selects/activates its effect and shows contextual K1/K2/K3 values above the4×4board. No DATA focus step for basic controls. Header names the selected target/effect. Released settings stay editable while OFF. Two active effects preserve independent slot edits; refused third effects do not mutate occupied slots. Track/Perform/Texture boards all follow this. Format7 stores stopped clock and loop origins for relative alignment on save/load and copy. Release0.9.2 is a public preview with stated physical/CPU/USB limits; user-approved demo videos do not authorize bundling their commercial source banks.
