# Groove Dice

`kim`의 공개 소스 MPC 드럼·그루브 플러그인. UID `KpGy`, 정식판 `1.3.3`. 검증 범위와 남은 제한은 VALIDATION.md를 따른다. source는 public이며 자체 제작 코어만 기본 포함한다. 제한된 외부 확장팩은 Git 제외이고 Donut Theory의 소리·UID·기존 릴리스는 변경하지 않는다.

승인 디자인은 `design/main-concept-v3-performance.png`, `design/fill-editor-concept-v1.png`다. 실제 MPC 사용 영역은1280×628이며 상단 호스트 바/하단 함수 키는 호스트 소유다. 이미지 시안을 작동하는 스킨이나 실기기 캡처와 혼동하지 않는다.

## 1.2.2 — 레이어와 즉시 편집

정식판 1.2.2에 포함한다. MAIN C는 킥·스네어·하이햇·Perc 볼륨, 새 SOUND 탭은 킥·스네어 최대 4레이어의 Sample/Level/Tune/Shift를 SCREEN Q-Link로 편집한다. ±30ms Shift를 위해 전체 30ms 보고 지연이 추가된다. 저장 호환성·상세 조작·제한은 [레이어 명세](../../../docs/plugins/groove-dice/layers.md)를 따른다.

## 읽기 쉬운 이름

기본96킷은 Velvet Basement / Amber Room / Dusty Sunday 같은 이름으로, 샘플은 Dry Kick01 / Round Snare03 / Crate Kick001 같은 이름으로 표시한다. 상단 공장 PRESET과 내부 킷 브라우저는 같은 이름을 사용한다. 원본 경로·SHA·킷ID·음원·파라미터는 유지한다. [이름 정책과 호환성](../../../docs/plugins/groove-dice/naming.md).

## 연주

- MAIN의 PREVIEW: 호스트가 정지해 있을 때 패턴 미리듣기. 다시 눌러 멈춘다.
- FOLLOW MPC: MPC PLAY/STOP·위치·BPM에 맞춘 자동연주. 현재 엔진의 마디 단위 동작은4/4 기준이다.
- 드럼 연주는 물리 패드를 사용한다. MIDI는 기본24/26/28/29/31/33/35/36(MPC C1 Major의 아래8패드 배열)이며 ROUTING에서 역할별 노트를 지정한다. Pad Perform 변경을 요구하지 않는다.
- 기본 킷: 자체 합성128개 원샷으로 구성한96개 킷과 Init. 킥·스네어·closed/openhat·clap·tom·cymbal·perc8역할이다.96개는 킷 조합이며96개 별도 녹음 세트라는 뜻은 아니다.
- 기본 MIDI: 직접 작성한 비트2개와 timing template93개. 기존 추출 MIDI는 기본 배포판에서 제외하고 개인용 Splat 확장팩에만 보관한다.
- 확장팩: 독립된 `.gdexp`를 추가/제거할 수 있다. 엔진 재빌드나 WAV 병합 없이 플러그인을 다시 열어 목록을 갱신한다. 기본/공개 조립은 자체 제작 팩만, 개인용 조립은 별도 Splat Private 팩을 포함한다. [설치·제거·배포 조립 안내](EXPANSIONS.md).

## MAIN Q-Link

**SCREEN 모드**에서 화면과 일치한다. Program/Track 모드는 실제 MAIN 파라미터를 고정 제어하며 필인 편집 화면을 따라가지 않는다.

| 그룹 | 위에서 아래로4개 물리 Q-Link |
|---|---|
| A · Timing | Swing / Snare Late / Hat Shift / Random Nudge |
| B · Rules | Kick Rule / Snare Rule / Hat Rule / Perc Rule |
| C · Drum Mix | Kick Level / Snare Level / Hat Level / Perc Level |
| D · Character | Drive / Dust / Room / Hat Rhythm |

Swing은16분음표의 홀수 step 지연, Snare Late/Hat Shift는 해당 역할의 고정 이동, Random Nudge는 노트별 타이밍 흔들림 폭이다. Random Amount는 ROLL DICE의 변경 강도다. KEEP은 반복 시 랜덤 패턴을 고정한다. 노트별 Chance는 재생 확률이다. 자동 필인은Off/4/8/16마디이며 마디 말미에 실행한다. 내부 Duck은 킥으로 나머지 드럼을 낮춘다. **다른 MPC 트랙을 트리거하거나 덕킹하는 외부 사이드체인은 미구현/미검증**이다.

## 필인 녹음·수정

1. MAIN의 EDIT / REC를 누른다. A–D중 필인을 선택한다.
2. Q-Link A에서 Fill Select / Length / Replace–Overdub / Count-in을 설정한다. 길이는1beat/2beats/1bar/2bars/4bars다.
3. REC FILL을 누르면 다음 마디 및 설정한 카운트인 후 녹음한다. 호스트 정지 상태에서는 내부 Preview를 시작한다.
4. 물리 패드의 직접 입력만 녹음한다. 자동연주·자동 필인 출력이 다시 녹음되지 않는다. 선택 슬롯과 길이는 시작 시 고정한다. 마디 경계를 건너는 정상 호스트 loop는 내부 연속 위치로 처리하며 일반 seek/stop은 녹음을 취소한다.
5. 정해진 길이 후 RECORDED로 바뀐다. 슬롯당 최대128노트이며 넘치면 FULL을 표시한다. CANCEL은 대기를 취소한다. Replace 녹음이 실제 시작한 뒤의 복구에는 UNDO를 사용한다.
6. BAR1–4로 보고 싶은 마디를 선택하고 grid의 노트를 누른다. 빈 칸을 누르면 노트를 만든다. 한 칸에 여러 노트가 있으면 첫 노트를 선택하며 Note Select로 개별 이벤트를 선택할 수 있다.
7. Q-Link B로 Nudge / Velocity / Gate / Chance를 편집한다. Gate는 Gated Samples가 켜진 경우에만 발음 길이를 제한한다. 현재 Gated Samples는 호스트 파라미터로 제공하며 전용 스킨 스위치는 미배치다. 기본 드럼은 원샷+초크/Decay로 종료한다.
8. AUDITION/TRIGGER FILL은 다음 마디에 선택 필인을1회 실행하고 기본 패턴으로 돌아온다. 필인 편집의 C/D Q-Link는 비워 두고 MAIN 파라미터를 잘못 바꾸지 않는다.

노트·세기·타이밍·확률·Gate·킷 설정은 MPC 프로젝트/호스트 프리셋 state에 저장한다. 화면의 SAVE KIT는 선택한 사용자 킷 슬롯에 설정을 복사하는 기능이며, 디스크 파일 저장이 아니다. **MPC 상단 저장으로 프로젝트/호스트 프리셋을 저장해야 영구 보존된다.** 플러그인 생성 MIDI를 MPC 시퀀서로 직접 기록하지 않는다(호스트가 MIDI 출력을 폐기한다).

## 킷·랜덤·FX

- KITS에서 사용자 슬롯1–8을 선택하고 SAVE KIT/LOAD KIT를 사용한다. 샘플·레벨·튜닝·Decay·Pan·초크·출력·Pad Note와 FX 설정을 저장한다. 현재 킷 이름 변경/삭제/파일 브라우저/외부 WAV 실시간 import는 미구현이다.
- 1.1 개발판 SOUND의 Q-Link A–D는 각 레이어의 Sample/Level/Tune/Shift다. Decay는 DETAILS에 유지한다. 킥·스네어만4레이어이며 하이햇/Perc는 기존 역할을 유지한다. Kit Dice Mode는 KITS 아래에 유지하고 Random Amount는 GROOVE에서 조절한다.
- ROLL DICE는 모든 Scope에서 킷도 변경한다(Kit Lock 또는 Amount0 제외). Whole Kit은96개 중 현재와 다른 킷, Mix Parts는 역할별 샘플 조합이며 최소 한 역할을 바꾼다. Scope는 Kit Only/Kit+Pattern/Kit+Groove/Kit+Fill/All/Kit+FX로 추가 변경 범위를 지정한다. 횟수와 KIT LOCKED/AMOUNT ZERO 상태를 표시한다. UNDO는 샘플 선택·레벨·튜닝·FX 및 변경한 패턴/필인을 직전 snapshot으로 복원한다. 현재1단계이며 redo는 없다.
- Kit/Pattern Lock을 보존한다. 역할 범위를 벗어나 샘플을 섞지 않는다. 기본 하이햇 그룹1과 킥/스네어 독립성을 적용한다. 세부 사용자 초크 그룹별 잠금은 미구현이다.
- 초크1–16/Off, 클릭 방지 Choke Time. 모든 MIDI·화면·자동 입력에 같은 초크 처리를 적용한다.
- Drive/Dust/Room/Duck/Low-cut 및 저속 볼륨 모듈레이션. 사용자 킷과 같이 저장한다. 별도 파트 insert FX 체인·별도 마스터 FX 체인의 완전 분리는 후속 작업이다. 현재 Master는 최종 출력 게인이다.

## 개별 출력

16mono channels=8stereo pairs. ROUTING에서 역할별 출력과 SEPARATE OUT을 설정한다. 기본은 전체 드럼을 첫 stereo pair로 합친다. Separate On일 때 첫 pair는 Main/Kick 역할이며, 나머지는Snare/Closed Hat/Open Hat/Clap/Tom/Cymbal/Perc로 기본 분리한다. 킥 전용과 전체 믹스의 동시9번째 stereo 출력은 제공하지 않는다. 목적지는 MPC 믹서에서 지정하며 플러그인이 MPC 시퀀서 트랙을 자동 생성하지 않는다. 실제 MPC 수신 트랙/믹서 라우팅 전체는 추가 검증이 필요하다.

## 사운드 뱅크와 확장팩

기본 `Content/Banks/default.gdbank`는 자체 제작 Original Workshop(약19.7MB)이다. 추가 팩은 `Content/Expansions/<id>/bank.gdbank`에 각각 보관하며 런타임 목록만 합친다. 기존1.48GB추출 뱅크는 `mpc-software/expansions/groove-dice/private/splat-to-clat/work/legacy-reference/`에 보존했고 별도 Splat Private 확장팩으로 패키징했다. 공개용 조립에서 이 팩은 제외한다. [현재 형식과 사용법](EXPANSIONS.md), [과거 포맷·출처 기록](SOUND_BANK.md)을 구분한다.

## 전체 라이브러리 사용 (0.2.0부터)

1.1 개발판은 KITS/GROOVE의 LIBRARY를 사용한다. SOUND는 각 레이어의 역할별 Sample을 직접 선택한다. 아래의 DRUMS 명칭은 이전판 기록이다.

DRUMS/KITS/GROOVE의 **LIBRARY**를 누른다. KITS/CHOPS/LOOPS→폴더→파일 순서로 탐색하고, 위의 대상 역할(Kick..Perc)을 선택한 뒤 파일을 누르면 그 역할에 배정된다. PREV/NEXT로8개씩 넘기고 FOLDERS로 돌아간다. MPC 호스트 표시의31바이트 제한 때문에 폴더는 팩번호/말단폴더명으로, 긴 파일명은 앞부분과 식별 가능한 끝부분을 함께 보여 준다. 원본 전체 이름은 뱅크에 그대로 보존한다. 샘플 준비 중에는 새 입력이 무음일 수 있으며 준비된 뒤부터 발음한다. 이전 샘플의 재생 중인 voice는 수명 종료까지 보존한다. OLD8은 선택한 역할을 기존8개 변형으로 되돌린다. 파트의 Sample1..8 노브는 OLD8 모드의 변형을 제어하며 전체 라이브러리 파일 선택은 LIBRARY에서 한다. 라이브러리 선택 상태에서 Sample노브를 바꾸면 그 역할의 기존8변형으로 돌아온다. 상세 화면 아래에 선택한 파일과 현재 비트/그루브 이름을 표시한다.

BEATS는 자동연주의 실제 비트, GROOVE는 기본 비트에 적용할 timing template다. BEAT OFF/GRV OFF로 해제한다. MAIN의 PATTERN 좌우/목록 선택은 라이브러리 비트를 해제하고 기본 패턴으로 돌아온다. ROLL Pattern은 선택 비트를 편집용 패턴으로 복사한 후 변형한다(편집 한도128노트/4bar; 원본 MIDI는 뱅크에 그대로 보존). 템플릿과 MAIN Swing/Nudge는 합산된다. 전체 WAV를 어느 역할에든 수동 배정할 수 있으나 Kit 랜덤은 Kits의 역할 분류만 사용한다. Chops/Loops는 원샷이며 자동 BPM stretch/slice는 하지 않는다.

로드 시 약4.3MB 색인과 기존64샘플만 읽는다. 선택 WAV는 별도 로더가 검사·디코딩해64개 제한 캐시에 전달한다. 전체 디코드2.22GB를 요구하지 않는다. 샘플 ID는 SHA256이고 사용자 킷/프로젝트와 함께 저장·재연결한다. 새로 저장한0.2 상태를0.1로 되돌리는 하위 호환은 보장하지 않는다. 기존0.1 QA 상태는 읽는다.

## 빌드·테스트

저장소 루트에서:

```sh
python3 mpc-software/plugins/public/GrooveDice/tools/build.py all
python3 mpc-software/plugins/public/GrooveDice/tools/test.py
```

SDK는 루트의 고정 버전 한 벌을 사용한다. Zig cache 경로는 환경변수 `ZIG_GLOBAL_CACHE_DIR`/`ZIG_LOCAL_CACHE_DIR`로 지정할 수 있다. `tools/build.py`는 빌드 후 미참조 런타임 이미지를 제거한다. `build/kim - VST - Groove Dice/`가 설치 폴더다. MPC OS3.10.1이상 및 Enable Enhancements/External Plugin Support가 필요하다. 현재 실기기는MPC Live III3.10.1.12이며 Donut Theory와 같은 SD 위치에 설치했다.

외부 코드·자산 출처는 [THIRD_PARTY.md](THIRD_PARTY.md)를 따른다.

최신 결과·한계는 [VALIDATION.md](VALIDATION.md)를 확인한다. 시작 시 새 인스턴스는 MAIN이다. 저장된 프로젝트의 편집 컨텍스트는 복원될 수 있다. UID·파라미터 ID는 고정하고 이후 추가만 한다.

## 이전 버전 설계·구현 기록

아래는 과거 버전별 변경 이력이다. 1.1 개발판의 MAIN C·SOUND·레이어 동작은 위 설명과 layers.md를 우선한다.

## 공장 킷 선택 (0.3.0)

MPC 상단 PRESET에서 Init / Pocket 및 Kits / V01..V06의96개 킷을 선택한다. 플러그인 안에서는 MAIN의 KIT 좌우/목록, DRUMS A의 Factory Kit 노브·값 터치, KITS의 Factory Kit로 같은 선택을 제어한다. 내부 목록은8행/13페이지이며 마지막 항목은V06 / Dust16이다. Custom / Legacy는 직접 수정하거나 Mix Parts로 조합한 상태를 나타낸다.

킷 선택은8샘플·파트 레벨/튜닝/Decay/Pan/Choke·킷 FX를 적용한다. 마스터 음량·패드 MIDI 노트·출력 라우팅·Swing 등 연주 설정·녹음한 필인과 사용자 킷 슬롯은 보존한다. 같은 킷 재선택은 음색을 다시 적용한다. Init은 기존 기본 샘플 조합과 FX Off로 돌아간다. Factory Kit 선택은 파라미터/호스트 program 양쪽에서 연결하며 현재 프로그램명도 갱신한다. Mix Parts/수동 조합은 Custom / Legacy로 표시하며 호스트의0번 program(Init / Pocket)을 사용하므로 상단에는 Init / Pocket이 보일 수 있다. 실제 샘플을 Init으로 되돌린 것은 아니다. 호스트 프리셋 파일(.xpl) 저장/로드는 전체 state 저장 기능으로 별개다.

96개 이름·샘플SHA·설정·출처는 bank-work/factory-kits.json에 기록한다. 실제 런타임 카탈로그도 전용 gdbank 안에 들어 있으며 별도 음원 파일은 필요 없다. 최초254개 파라미터의 ID/순서를 유지하고7개를 append했다(총261개). 기존0.1/0.2 저장 상태는 Custom / Legacy로 읽고 실제 샘플/필인을 보존한다.

## 0.4 그루브 보존과 전면 소리 선택

MAIN C의 Kick/Snare/Hat/Perc는 전체 역할별 후보440/546/376쌍/244개를 탐색한다. 값 터치는8행 역할 목록을 열며 종류 탭·PREV/NEXT·X·바깥 터치로 탐색/닫는다. Hat은 같은 팩의 closed/open pair와초크1을 적용한다. 파일의 역할·팩을 기준으로 묶은 조합이며 원본 vendor preset의 pairing을 복원한 것은 아니다. 소리 선택은 해당 역할의 레벨 보정·튠·Decay·Pan·Choke를 적용하고 다른 파트·패드 노트·출력·FX·녹음은 보존한다. 선택0은 현재/Legacy 상태이며 기존8변형으로 강제 복귀시키지 않는다. 기존 레거시 Sample 컨트롤은 DRUMS에 유지한다.

Fill Select/Density/Every/Variation은 GROOVE의 D그룹에 유지한다. MAIN의 필인 선택·TRIGGER FILL·EDIT/REC 및 기존 편집16/8 relay는 유지한다. Screen/Program의 MAIN C를 함께 갱신했으며 과거 자동화의 Q9–12 의미가 달라진다. 실제 Fill 파라미터 ID/순서는 유지한다.

ROLL의 그루브는 현재 타이밍을 원본으로 보존하고 Swing/Snare/Hat/Nudge에Amount비례 ±3%p/±5ms/±3ms/±2ms 이내 변화를 준다. 반복 ROLL이 타이밍을 계속 밀지 않는다. 수동 타이밍 변경 시 새 기준을 잡는다. groove template 선택 중에는 Swing을 추가 랜덤화하지 않는다. 기존 수동 Swing/Variation 등은 별도 연주 설정으로 유지한다.

패턴/필인 ROLL은 최초 후보의 원본에서 다시 시작한다. 킥 첫박과 스네어2·4박의 기존 노트를 보호한다. 낮은Amount(≤50%)는 작은 타이밍/세기 변경만, 그 이상은 제한된 보조 타격 추가·삭제까지 허용한다. 마디별 역할 타격수와 같은 역할/하이햇 최소 간격·루프 경계 간격을 검사한다. 모든 원본에 없는 백비트를 강제 삽입하지 않는다. 필인의 수동 편집·녹음·길이 변경은 새 원본으로 취급한다. 원본 기준이 없는 구형 저장 상태의 기본 패턴은 공장 비트 원본을 기준으로 ROLL하여, 이미 과밀해진 구형 결과가 새 기준으로 굳지 않게 한다. 불러온 직후에는 저장된 재생 결과를 보존하고 다음 ROLL부터 적용한다. 원본과 그루브 기준도 새 state의 optional extension으로 저장한다. 기존0.1~0.3state도 읽는다.

재생 중 ROLL은 NEXT BAR를 표시하고 다음 마디 시작 블록에 적용한다. 대기 중 다시 누르면 다음 적용 요청을 갱신하며, 정지/seek/state load/Undo 시 대기를 취소한다. 정지 중에는 즉시 적용한다. 샘플 로딩 중에는 기존 소스를 유지해 타격이 빠지지 않게 하고 준비 후 새 소스로 발음한다. 실제 파일 오류/누락은 기존 오류 처리로 표시한다. 녹음 중 필인 자체의 랜덤 변형은 건너뛴다.

0.4 당시 C의 SOUND/RULES 전환은 제안 상태였다. 현재0.5는 사용자 확정에 따라 B를 RULES로 교체한다(아래 참조).

MPC 호스트 파일 복원은 chunk 뒤에 개별 파라미터를 다시 적용할 수 있으므로 로드 후0.5초의 오디오 처리 동안 명령 변화와 파라미터 부작용을 억제한다. 저장된 ROLL/REC/좌우명령이 로드 동작만으로 실행되는 것을 방지한다. 이 짧은 복원 구간에는 명령 입력도 적용하지 않는다. 오디오 파라미터 값과 실제 MIDI 연주는 계속 처리한다.

## 0.5 확정 — MAIN B RULES / Q-Link 가이드

사용자가PATTERN그룹을RULES로교체하도록확정했다. MAIN A=Timing, B=Rules, C=DrumSounds, D=Character다. C의SOUND/RULES모드전환제안은채택하지않는다. Density/Ghost/VelocitySpread/RandomAmount는GROOVE B 및기존실제파라미터로유지한다. 새4프로필을기존287개뒤append하여총291개이며기존IDs/순서는유지한다. B의Screen relayQ5–8과Program/Track고정매핑을함께변경한다.

Kick/Snare/Hat/Perc별LOCK/POCKET/LOOSE/BUSY. 기본1/1/2/2. LOCK은해당리듬의기존노트·세기·미세타이밍을ROLL이바꾸지않는다(빈파트도보존). POCKET은기존기본박자를지키며작은타이밍/세기만변경한다. LOOSE는절제된보조타격·고스트, BUSY는상한안에서더많은변형을허용한다. Perc규칙은Clap/Tom/Cymbal/Perc보조파트전체에적용하고, Hat은closed/open에함께적용한다. 샘플교체는별도KitLock/Sound정책이며RhythmLOCK만으로킷이잠기지는않는다.

자동연주에도프로필별타격수·간격·velocity/jitter/variation/ghost상한을적용해구형과밀패턴과강한저장값을제약한다. 기본킥과스네어2·4박은우선한다. 물리MIDI/직접터치는제약하지않는다. 4프로필중LOCK은새자동Ghost/jitter를허용하지않고, POCKET도자동ghost추가를하지않는다. 기존노트로기록된ghost는보존한다. GROOVE의원래값은보존되며RULES가최종허용범위를제한한다.

GrooveROLL은수동A노브를바꾸는대신파트별작은offset/seed를변경한다. LOCK파트는제외한다. 킷만교체하거나Fill만변형할때메인타이밍seed를바꾸지않는다. Undo는노트·킷외에이offset/seed와랜덤seed/KEEP도복원한다. 새추가state84bytes(GRP5)는파트seed+4offset을저장하고구형state도읽는다.

MAIN가이드는실제614×138패널바깥에좌우4px/상하2px여백을두어622×142로맞춘다. 팝업의version4 hideQLinkBounds는SDK키순서에맞게componentsData바로앞에둔다. 구형코드의actions뒤삽입을고쳤으며Kit/Pattern/Role/Rule/Library팝업에공통적용한다. 실제기기확인결과는VALIDATION을따른다.

## 0.6 — 하단 조작부 / Perc 세트 / 새 MIDI 이름

MAIN 상단은 킷·패턴·위치·녹음 상태·BPM의 읽기용 요약이다. 모든 MAIN 버튼은 네 Q-Link 패널 아래로 모았다. BROWSE는 킷/패턴 목록·좌우·잠금·사용자 킷 저장, CREATE는 ROLL/범위/UNDO/KEEP, PLAY는 FOLLOW/PREVIEW/필인 선택·트리거·편집이다. 사용자 재검토에 따라 화면 하단8개 드럼 패드는 삭제했다. 물리 패드로8역할을 연주하고, 확보한 공간을 세 줄의60px높이 터치 버튼에 쓴다. 필인 편집의 MAIN 복귀·녹음·Audition·Cancel·Delete·Undo도 아래 도구막대에 모았다. 슬롯/마디 선택기는 편집 그리드 바로 위에 유지한다.

명령 버튼은 엔진 명령과 독립된 UI Local press gate 및 투명 연속 Slider의 ResetOnGestureEnd를 사용한다. 누르는 동안 밝은 색, 뗄 때 원래 색으로 복귀하며 실제 명령은 Mouse Down에서 한 번만 실행한다. Follow/Preview/Lock 등 지속 스위치는 실제 On/Off 색을 유지하고 목록은 현재 선택 항목을 강조한다. 마지막 실행 명령과 증가하는 번호를 PLAY줄 오른쪽에 표시한다. 엔진 명령 자체를 손 뗌 초기화하여 두 번 실행하는 방식은 사용하지 않는다.

**Perc Set은 Clap/Tom/Cymbal/Perc 네 악기의 묶음**이다. 기존244개 Perc 후보를 앵커로 같은 소스 팩의 공장 킷에서 나머지 세 악기·레벨·Tune·Decay·Pan·Choke를 완성한다. 원본 앱의 세트를 복제한 것은 아니다. 선택/MAIN C의 네 번째 Q-Link/Mix Parts 랜덤이 네 파트를 함께 교체한다. 킥·스네어·하이햇, 패드 노트·출력·FX·녹음은 유지한다. 네 파트는 독립적인 소리·패드·출력이며 동시에 한 번에 모두 재생하는 레이어가 아니다. 구형 사용자 조합이 새 세트와 정확히 일치하지 않으면 Custom으로 표시하고 소리는 보존한다. Whole Kit은 여전히8파트 공장 킷 전체를 바꾼다.

모든95개 MIDI의 뱅크 안 파일명 및 표시 이름을 새로 지었다. 실제 비트는 Velvet Backbeat84 / Basement Pocket90, timing template는 Velvet Stoop·Basement Bounce·Honey Crumbs 등의 고유 이름이다.93개 템플릿을 새 드럼 비트로 만든 것은 아니다. 원본 MIDI 바이트·SHA·인덱스는 그대로이므로 저장 연결과 연주 내용은 바뀌지 않는다. 원본 경로와 새 경로의 대응은 private bank-work/midi-name-map.json, 새 이름으로 내보낸95파일은 bank-work/midi-renamed에 보관한다. 원본 추출 연구 폴더는 출처 증거로 보존한다.

기존291개 파라미터의 ID/순서를 유지하고 읽기용 ActionInfo만 append했다(292개). SoundPerc의 범위0..244와 저장된 샘플SHA는 유지한다. GUIDE는 이동한 실제 패널 바깥으로 함께 이동했으며 팝업의 ordered hideQLinkBounds를 유지한다. 실기기 검증 상태는 VALIDATION.md를 따른다.

2026-11-16 최신 결정: 화면에서 직접 두드리는 드럼 패드는 MAIN/필인 편집에서 표시하지 않는다. 관련 기존 Pad 파라미터와 MIDI 동작은 호환용으로 보존하며, 필인 입력은 물리 패드를 사용한다. UI 공간은 하단60px 조작 줄과 편집 결과 표시로 돌린다.

## 0.6.1 — Q-Link 가이드 겹침 수정

가이드는 이제 A–D/제목 아래의 실제 네 컨트롤만 감싼다. Q 배지용 왼쪽 여백을 두고 LCD116×24와 글자 크기는 유지했다. MAIN과 GROOVE/DRUMS/KITS/FX/ROUTING에 같은 기준을 적용한다. 기존16개 매핑·파라미터292개·음원은 그대로다. 이전의 패널 전체를 감싸는 외곽 검사로는 제목을 가리는 문제를 놓쳤으므로, 제목/배지와의 비교차 및 컨트롤 포함을 검사하도록 바꿨다. 실기기 결과는 VALIDATION.md를 따른다.

## 0.7 — 음악적인 패턴 생성 규칙

패턴을 바꾸는 **Kit + Pattern / All ROLL**은 이제 기본 리듬형을 선택해4마디 문장으로 만든다. 킷·그루브만 바꾸는 Scope는 기존처럼 리듬을 보존한다. 새 인스턴스의 기본 Scope는 Kit + Pattern이다. 기존 프로젝트의 Scope와 저장된 원본/녹음은 로드만으로 바뀌지 않는다.

- **스네어:** 매 마디2·4박의 본 타격을 보장한다. POCKET에는 추가 고스트가 없다. LOOSE/BUSY는 Ghost와 Random Amount에 따라 최대1/2개 고스트를 허용하며 세기는24–36으로 본 타격100보다 낮다. 연주용 Snare Late는 그대로 적용된다. 생성한 패턴에서 Density0은 자동 타격을 무음으로 한다.
- **킥:** 직접 설계한 리듬형 또는 유효한 원본 MIDI의 첫마디 리듬을 후보로 삼는다.2·4박 정박은 제외하며, 그 앞뒤의 엇박은 허용한다. POCKET은 마디당 최대4타, LOOSE/BUSY는 최대5타를 구성한다. 첫 마디와 세 번째 마디는 같은 기본 리듬을 공유하며 두 번째는 약한 응답, 네 번째는 허용 강도 안에서 짧은 뒷부분 변형을 준다. 낮은 Amount(≤50%)로 다시 ROLL하면 이미 고른 기본 킥 리듬을 유지해 누적 변형을 피한다.
- **하이햇:** GROOVE 하단의 **HAT GRID**에서1/4·1/8·1/16·1/32 ROLL을 선택하고 다음 패턴 ROLL에 적용한다. 기본1/8. 강박은 강하게, 사이 타격은 약하게 연주한다. LOOSE/BUSY는 약한 타격의 일부 생략·간헐적인 열린 하이햇·마지막 마디의 짧은 장식만 허용한다.1/32는 계속32타로 채우는 모드가 아니라1/16 기본 박자 위에 짧은32분음표 롤을 넣는 방식이다. 기존 타격수 제한이1/16/1/32를 지우지 않도록 간격별 예산을 적용했다.
- **Perc:** 킥/스네어와 겹치지 않는 자리에 작고 드문 응답을 배치한다. Pocket은2·4마디 중심, Loose/Busy는 더 많은 보조 응답을 허용한다. 단일 파일이 아닌 기존4악기 Perc Set을 사용한다.
- **타이밍:** 생성한 패턴은 매 반복마다 노트별로 새 타이밍을 뽑지 않는다. 같은 파트/위치의 강약·미세타이밍이 반복된다. Variation은 비고정 타격에 파트·응답 마디별 최대2ms의 상관된 변화를 주며 기존 Swing/Snare Late/Hat Shift/Random Nudge와 함께 쓴다.1/8 Swing도 실제로 적용하고, 짧은32분음표 롤에서는 보간해 타격 순서가 뒤집히지 않게 한다.

기존 MAIN B의 LOCK/POCKET/LOOSE/BUSY4개 Q-Link는 유지한다. 새 Hat Grid는 터치 선택이며16개 기본 매핑을 바꾸지 않는다. **LOCK은 구조 규칙보다 우선하는 사용자 보존 명령**이다. 잠근 기존 파트가 있는 구형1/2마디 패턴은 길이와 위상을 유지한다. 잠금을 푼 패턴은4마디로 생성하며, 이미4마디인 패턴의 부분 잠금은 그대로4마디다. 표현 가능한128노트를 넘어 잠금을 보존할 수 없으면 RHYTHM LOCK LIMIT를 표시하고 원본을 잘라내지 않는다. 큰 외부 MIDI를 부분 잠금한 경우에도 같은 보호를 적용한다.

직접 연주·녹음한 필인은 독립된 사용자 데이터다. 패턴 생성으로 덮지 않는다. **Kit + Fill / All에서 필인 변형을 요청했을 때만** 기존 필인 변형 규칙을 적용한다. 수동 필인·원본 MIDI·LOCK 파트는 새 기본 구조를 강제로 재작성하지 않으므로 의도한 예외 연주가 가능하다.

기존292개 파라미터의 ID/순서는 그대로다. HatGrid(292)와 비자동화 내부 PhraseState(293)만 append했다(총294). 실제 생성 간격·4마디 위상·생성 여부는 패턴별로 저장되며 Hat Grid를 눌렀다는 이유만으로 이전 패턴을 즉시 덮지 않는다. Undo는 노트/길이/생성 상태까지 함께 복원한다.0.6이하 상태의 기존 소리는 다음 패턴 ROLL 전까지 보존한다.

## 0.8 — 전면 HAT RHYTHM / 15가지 리듬

MAIN D의 네 번째 Q-Link는 DUCK 대신 **Hat Rhythm**이다. DUCK은 FX 상세의 기존 파라미터로 유지되며, 저장된 값과 내부 킥 덕킹 효과는 보존된다. SCREEN relay Q16과 Program/Track의 해당 고정 매핑을 함께 변경했다. 기존 프로젝트의 Q16 자동화 의미는 이제 Hat Rhythm이며 실제 Duck 파라미터 ID는 그대로다.

간격은 **1/4 · 1/8 · 1/16 · 1/8 DOT · 1/16 DOT**, 각각 **기본 / + FILL / + ROLL**로 총15가지다. Q-Link를 돌리거나 MAIN의 값 LCD를 터치해 왼쪽2페이지 목록에서 고른다. GROOVE 하단도 동일 선택기를 제공한다. MPC의 선택값 치환 문자열은 `1/8 DOT + FILL`처럼 전체 이름을 표시하므로 전면 LCD에는18px 글자를 사용한다. 호스트 값 텍스트는 `1/8D FILL`처럼 짧게 표시한다.

- DOT은 기본 간격의1.5배: 점8분720ticks, 점16분360ticks(PPQ960). 마디 경계에서 강제 초기화하지 않는다.4마디 루프 끝에서는 다른 패턴과 함께 시작점으로 돌아간다. 점음표는 straight swing grid로 휘지 않으며 Hat Shift/Nudge는 유지한다.
- FILL은2·4마디 끝에 짧은 픽업·일부 생략을 배치한다. 촘촘한16분에서도 기본과 구분되는 사이 타격을 제공한다.
- ROLL은2·4마디 끝의 마지막8분 구간을 짧은32분4연타로 바꾼다. 연속32분 패턴이 아니다.
- 선택한 리듬은 ROLL DICE가 임의 변경하지 않는다. 다음 **Kit + Pattern / All ROLL**에 적용하며 재생 중이면 기존 NEXT BAR 큐를 따른다. 선택만으로 현재 노트를 덮지 않는다. KitOnly/PatternLock/파트LOCK은 기존 보존 규칙을 따른다.
- MAIN B의 Hat Rule은 LOCK/변주 강도를 유지하고 새 Hat Rhythm은 간격과 장식을 정한다. 직접 녹음한 필인과 슬롯을 하이햇 장식으로 덮지 않는다.

기존294개 파라미터 정의는 모두 보존하고 HatRhythm294, 비자동화 HatPhraseState295, 비자동화 HatRhythmPage296을 append했다(총297). 실제 생성 리듬은 패턴별4bit로 저장하며0은 구버전 메타데이터를 뜻한다. Undo/상태 복원은 노트·길이·실제 리듬을 함께 복원한다. 구형 HatGrid는 호환용으로 보존하고 UI에서 제거했다. 구형 상태를 불러오면 선택값만 대응시켜 표시하며 저장 노트는 다시 만들지 않는다. 새0.8 상태를0.7에서 읽는 하위 호환은 보장하지 않는다.

## 0.9 — 독립 확장팩과 자체 제작 코어

최신 기준은 [EXPANSIONS.md](EXPANSIONS.md)다. 아래0.2–0.8항목은 당시 구현 기록이며, 현재 기본 배포판이 상용 추출물을 포함한다는 뜻이 아니다. 기본 코어128WAV/95MIDI/96kits는 합성식·seedednoise·직접 작성 리듬으로 생성했다. 샘플에 상용 원본/모델 출력/MIDI를 입력하지 않았다. `tools/build_original_bank.py`와 SHAmanifest로 재현한다.

기존297개 parameter정의를 유지하고 확장 카탈로그/정규화 Q-Link/8행 동적브라우저용45개를append(342)했다. 이전 고정범위 항목은 alias로 남기고, 새 카탈로그는 설치된 실제 항목을 사용한다. 기존16개MAIN역할/리듬그룹은 보존하며 C와DRUMS/KITS의 소리선택 대상은 확장 카탈로그 relay로 이동한다. Native PRESET은 설치된 모든 킷을 나열한다. 프로젝트/사용자킷의 샘플·MIDI는 SHA, 킷과필터는 packID+kitID로 복원한다. 누락 팩 상태에서 재저장해도 식별자를 보존한다. 신규 identity trailer200bytes를 append하며0.8이하로의 하위 호환은 보장하지 않는다.

공통 테스트는 자체 코어만으로 실행한다. `python3 tools/test.py --with-private`는 별도 보관한 추출뱅크의 역호환/무손실 검사도 실행한다. 확장팩 합성/패키지 도구는 공개·비공개 source정책과 별개다. 공개용 ZIP작성은 외부 게시가 아니며 명시적 요청 없이GitHub/Pages에 올리지 않는다.

## 2026-11-16: 예박·킷 보호 및 내장 설치

FOLLOW가 켜진 음수 PPQ 예박에서 자동 패턴이 반복되던 문제를 수정했다. 첫 마디가 시작되는 블록의 원래 샘플 위치를 유지하며, PREVIEW가 켜져 있어도 예박 자동 재생은 막는다. 물리 패드/MIDI 직접 연주는 유지한다. MPC XL에서 사용자가 예박 무음→실제 녹음부터 정상 재생을 확인했다.

MAIN의 LOCK KIT 버튼을 넓히고 활성 상태를 KIT FIXED로 표시한다. 잠금 시 Scope와 사이드 목록은 Pattern Only/Groove Only/Fill Only/Rhythm + FX/FX Only로 유효 대상을 표시하며 Kit Only는 No Changes다. 기존 Scope 저장값과 파라미터는 유지한다. FX 포함 범위는 FX를 바꿀 수 있지만 샘플·레이어·믹스는 보존한다.

현재 소유자 기기는 MPC XL이며 기본 설치 대상은 SD가 아닌 내장 NVMe다. `/nvme/VST2 Plugins/kim - VST - Groove Dice`에 추가 뱅크까지 설치·검증했다. [검증 범위](VALIDATION.md)를 참고한다.

## 1.1.1 개발 — 실제 MPC 양수 예박 자동 감지

앞선 음수 PPQ 차단만으로 해결되지 않았음을 실기 로그에서 확인했다. MPC XL은 예박 때도 Playing/Recording을 켜고 임의의 양수 위치를 진행한 뒤 실제 녹음 시작점으로 돌아왔다. 이 정지→녹음 진입 패턴을 자동 감지하는 HostRecordGate를 적용했다. 설정 항목은 추가하지 않았다. 실제 예박 On/Off CSV를 재생하는 회귀 검사에서 예박 드럼14회→0회, 실제 녹음 첫 블록부터 재생을 확인했다. 원본 신호에 전용 예박 플래그는 관측되지 않았으므로 보편적인 호스트 API라고 설명하지 않는다. [검증 범위와 중간 박 진입의 한계](VALIDATION.md)를 따른다.

## 1.2.0 개발 — 1마디 반복·즉시 편집·파트별 RULES

MAIN을 같은1마디 루프의 반복으로 변경했다. Dice와 Division/룰 변경은 다음 오디오 블록부터 적용하며 마디마다 다른 응답 패턴을 자동 생성하지 않는다. B RULES는 파트별8종(총32선택)이며 기존 파라미터521개를 보존하고4개 스타일만 추가했다. 하단 필인 A–D는 독립 길이/녹음을 유지하고 수동 FILL은 다음 마디에1회, 자동 필인은 간격 경계에 끝나도록 전체 길이를 연주한 후 같은 메인 루프로 복귀한다.

[상세 동작·RULES 목록·호환성](../../../docs/plugins/groove-dice/one-bar-loops.md). 실제 검증 결과는 [VALIDATION](VALIDATION.md)의 최신 항목을 따른다.

## 1.2.2 배포

MAIN은 1마디를 반복하며 Dice/Rule/Hat Rhythm 변경을 즉시 반영한다. ROOM은 스테레오 룸 리버브다. SOUND 샘플 이름·좌우 화살표·Q-Link가 연결된다. [릴리스 상세](../../../docs/plugins/groove-dice/releases/1.2.2.md)와 [사용 설명서](../../../docs/plugins/groove-dice/manual.md)를 참고한다.

## 1.2.3 로컬 변경 — 박자별 강약

자동 패턴 하이햇을 강·약·중·약(90/45/68/40), 킥을 중심박/연결음으로 구분한다. Vel Spread는 구조를 유지하는 작은 편차로 제한한다. 직접 패드·녹음 필인의 입력 강약과 저장 노트는 보존한다. 새로 생성하는 패턴부터 적용하며 [상세 명세](../../../docs/plugins/groove-dice/velocity-patterns.md)를 따른다. MPC XL 내장 설치와 자동 검증을 완료했고 공개 Release/웹사이트는 1.2.2 그대로다. 개인용 팩과 그 변환물은 Git 제외 경로에만 보관한다.

## 1.3.0 로컬 변경 — HATS / CYMBALS

MAIN D=Hat Rhythm/Open Hat/Ride/Crash. Drive/Dust/Room은 기존 FX 탭에서 조절한다. 오픈햇은 CH 대체·기본 초크, 라이드는 자동 햇 대체, 크래시는 시작/필인 뒤/4·8·16마디 경계다. Dice는 이 선택을 유지한다. Ride/Bell은 Hat Mix, Crash는 Perc Mix를 따르며 출력은 기존 Cymbal 경로를 공유한다. [전체 동작·호환성·한계](../../../docs/plugins/groove-dice/hats-and-cymbals.md). 공개 릴리스와 웹사이트는 별도 요청 전까지 그대로다.

## 1.3.1 local QA: sound selection and kit range

Tap-to-list sample selection, synchronized arrows/Q-Link, isolated PREVIEW, independent Ride/Crash/Bell sound controls, selected-kit Dice pools and all-eight-role fill editing. See [workflow details](../../../docs/plugins/groove-dice/sound-browser-and-kit-pool.md). Released in1.3.3 following owner confirmation.

## 1.3.2 local QA: visible Dice and individual locks

MAIN now has 16 persistent padlocks. Roll Dice updates eligible visible Timing / Rules / Mix / Hats-Cymbals controls, with musical bounds; locked values stay fixed and remain manually editable. Scope is labelled Extra changes for supplementary pattern/fill/FX operations. See [workflow details](../../../docs/plugins/groove-dice/sound-browser-and-kit-pool.md). Included in1.3.3.

## 1.3.3 local QA: one lock mechanism in RULES

RULES exposes seven playable styles with no LOCK choice, via touch and Q-Link. Use the adjacent padlock for Dice protection. Old LOCK projects retain their held rhythm and display BASIC with a closed padlock until released.

## 1.3.3 공개 배포

16개 전면 자물쇠와 실제 표시값 Dice, LOCK을 제외한7개 RULE 스타일, 모든 사운드의 Sample/Level/Decay/Shift, 왼쪽 샘플 브라우저·개별 미리듣기, 독립 Ride/Crash/Bell, 최대64개 선택 킷 풀을 포함한다.643개 파라미터와 구버전 상태를 유지한다. 사용자가 MPC XL에서 정상 동작을 확인하고 공개 배포를 승인했다. 과거0.x–1.3.2 단락은 이력이며 최신 동작은 [매뉴얼](../../../docs/plugins/groove-dice/manual.md)과 [1.3.3 릴리스](../../../docs/plugins/groove-dice/releases/1.3.3.md)를 따른다. 실제 MPC 캡처로 웹 소개 이미지를 교체한다.
