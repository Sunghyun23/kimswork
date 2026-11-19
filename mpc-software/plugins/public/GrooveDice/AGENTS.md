# Groove Dice

루트/public 지침과 README.md, SOUND_BANK.md, VALIDATION.md를 먼저 읽는다.

- vendor kim, UID KpGy, 정식1.0.0. 공개 소스 및 자체 제작 코어 배포판이다. 자체 제작 코어는 공개 가능하며 Splat 자료는 expansions/groove-dice/private/ 밖으로 복사하거나 Git에 추가하지 않는다.
- 사용자 승인 시안은 design/main-concept-v3-performance.png 및 fill-editor-concept-v1.png. 실제 영역1280x628, runtime배경1280x628 및 knob64x(64*127) 크기를 지킨다. 원본 이미지를 호스트가 자동 축소할 것으로 가정하지 않는다.
- MAIN16 Q-Link는 엔진 SCREEN relay에 연결한다. EDIT/REC를 누르면 실제 녹음/노트 파라미터를 조절한다. 편집C/D는 미할당. Program/Track MAIN 실제 매핑은 보존한다. SDK에 없는 탭 핸들러를 만들지 않는다.
- UI가 읽는 변경 알림은 render thread token 검사 후 UI idle/value-read에서만 배출한다. process()에서 host알림·I/O·할당·잠금을 하지 않는다.
- 필인 입력은 MIDI/직접 화면 패드만이며 자동 출력은 다시 녹음하지 않는다. 카운트인/오버더빙/녹음 중 선택 슬롯 고정/유한 녹음/loop/seek/stop을 함께 검사한다.
- sample/외부 MIDI는 Content/Banks/default.gdbank에만 둔다. src에 파생 오디오나 비트를 굽지 않는다. 공개 전환 전 외부 자료의 사용권과 자체 음원 교체를 확인한다.
- 실제 구현과 제안/미구현을 구분한다. 외부 사이드체인·전체 MIDI 라이브러리·물리 Q-Link 전체·라우팅을 확인 없이 완료로 보고하지 않는다.

- 2026-11-16: 승인 v3의 재질·입체감·카드 구조를 복원한다. 노브는64px native frame, 크림 패널138px, 기존214개 파라미터 ID/순서는 보존한다. 전체 라이브러리는 단일 뱅크 확장에 넣고 제한된 비동기 캐시만 연주에 사용한다. process()는 파일 접근·디코드·할당·잠금을 하지 않는다. 캐시 수명은 pending/current/voice 참조를 분리한다.

- 0.3 공장 킷96개는 원본 프리셋 복제가 아닌 추출 WAV의 역할별 재조합이다. bank-work/factory-kits.json과 뱅크 GDKITS03 suffix가 출처/샘플SHA/설정을 보유한다. first254 parameter IDs/order 유지,7개 append(261개).
- DRUMS A=FactoryKit/KitDiceMode/RandomAmt/KitLock, B/C/D=Kick/Snare/ClosedHat Sample/Level/Tune/Decay. MAIN/FILL SCREEN 매핑은 유지한다.
- Native PRESET Init+96 programs와 내부 FactoryKit 선택을 동기화한다. 킷 선택은 샘플/파트음색/FX만 적용하고 master/pad/routing/groove/fill/user slots를 보존한다. ROLL은 모든 Scope에서 킷도 바꾸되 KitLock/Amount0는 보존하며 원인을 표시한다. Undo는 변경 전 샘플/음색/FX도 복원한다.

- 0.4 MAIN C=SoundKick/SoundSnare/SoundHat/SoundPerc, Screen relay Q9–12 및 Program 고정 매핑을 함께 맞춘다. FillSlot/Density/Every/Variation은 GROOVE D에 유지한다. RULES 전환/파트별 제약 프로필은 아직 제안이며 확정 구현으로 보고하지 않는다.
- 원본 기준 패턴 변형, downbeat/backbeat 보호, 역할 cap/최소 간격, Amount 비례 그루브 변화, NEXT BAR 큐/stop·seek 취소를 유지한다. 기준 패턴·그루브를 extra state에 저장하며 구형extra크기도 허용한다. 기존261개 parameterID/순서 유지,26개 append(287).
- 역할 선택1606개는 single bank GDSND004 확장으로 저장한다. SoundRequests로 동시 UI입력을 보존하고 오디오의 선택값 readback은 CAS로 다른 UI쓰기를 덮지 않는다. Hat pair를 함께 전환한다. 비동기 로딩 중 직전 소스를 보호하며 voice cache 수명규칙을 유지한다.

- MPC .xpl의chunk후개별파라미터 재적용을 고려해 로드 후0.5audio-second 동안 command/parameter side effect를 억제한다. 마지막 보호 블록에서 proxySeenTarget을 무효화해 구형Q9–12저장값을 실제새Sound선택값으로 다시 동기화한다. 일반오디오/실제MIDI 입력은 유지한다.

- 0.5사용자확정: MAIN B는RULES(Kick/Snare/Hat/Perc), C는기존SOUND4개유지. 모드전환제안아님. Density/Ghost/VelSpread/RandomAmt는GROOVE B유지. first287IDs/order고정,Rule4append=291.
- RULES는자동재생에도적용하며직접MIDI/터치는제약하지않는다. LOCK의빈파트/실제스케줄불변,POCKET노트증감없음,킷/필인만ROLL시메인seed유지검증. 수동A노브는GrooveROLL로덮지않고파트별offset/seed사용,Undo와extraGRP5에저장한다.
- hideQLinkBounds(version4)는disableCoarseDataWheel/repeats뒤,componentsData앞직렬화순서를지킨다. MAINguide=panelbounds좌우4px상하2px외부여백. SDK가이드숨김은실제로포커스를잡고Kit/Pattern등팝업을열어확인한다.

- 0.6 사용자 확정: MAIN action은 하단 BROWSE/CREATE/PLAY3줄(60px). 위쪽은 읽기용 요약. 필인 명령도 하단, 슬롯/마디 선택기는 그리드 근처다. 명령버튼은 Local press gate+연속Slider ResetOnGestureEnd로 누름 색을 제어하고 실제 엔진 command는 Mouse Down1회만 보낸다. release로 엔진 command를 다시 보내지 않는다. 지속 스위치는 실제 On/Off 상태를 표시한다.
- Perc는 단일role7이 아니라 Clap/Tom/Cymbal/Perc4파트 세트다.244기존Perc앵커+같은팩 bank factory catalogue3파트로 구성하며 MAIN C/역할목록/MixParts가 함께교체한다. 수동부분조합은 Custom이며 기존샘플을 덮지않는다. 킥/스네어/하이햇/라우팅/FX/필인보존을 검사한다. first291IDs/order유지,ActionInfo append=292.
- MIDI95개 runtime bank_path는 전부 새이름. 원본path와SHA/바이트는 provenance로보존한다. 이름 변경은 tools/midi_names.py로 재현하며 bank-work/midi-name-map.json 및 midi-renamed95개를 관리한다. beats2/grooves93 구분을 유지한다. 공개자료로 보내지 않는다.

- 최신사용자결정: 하단화면드럼8pads는제거한다. 물리패드로연주하며 기존Pad파라미터/MIDI/녹음은보존한다. MAIN 하단3줄 높이60px, 필인 하단은명령+결과표시다. 화면연주패드를다시좁게끼워넣지않는다.

- 0.6.1 Q-Link가이드최신규칙: 패널전체외곽을감싸는0.5/0.6규칙은폐기한다. 제목/A–D배지는가이드밖, 네컨트롤이름·노브·값은안쪽이다.614×138패널에서guide=(x+4,y+26,606,110), 컨트롤=(x+44+j*140,y+28,138,108). Q배지40px용왼쪽여백을확보하며LCD116×24/숫자크기는유지한다. MAIN및상세페이지에동일적용하고팝업가이드숨김/실제매핑은보존한다. 회귀검사는제목비교차·네컨트롤포함·Q여백을검사한다.

- 0.7사용자확정: Pattern/AllROLL은리듬형→파트규칙→4마디문장→상관된강약/타이밍으로구성한다. 스네어2/4고정, 킥2/4정박제외, HatGrid1/4·1/8·1/16·1/32Roll(연속32타아님), Perc는빈자리응답. RULES4Q-Link는변형강도로유지하며 HatGrid는GROOVE하단터치,다음패턴ROLL에적용한다.
- first292IDs/order고정,HatGrid292/PhraseState293 append=294. PhraseState flags3bits+actualhatgrids9bits(4=legacy)+origins6bits 저장. 기존notes128/extra state형식유지. Undo는PhraseState포함. 신규Scope기본1Kit+Pattern,기존저장Scope는보존한다. KitOnly/GrooveOnly/PatternLock/allLOCK은기존리듬보존.
- LOCK이구조규칙보다우선. legacy부분LOCK은loop길이/위상유지. 전체언락은4bar. 과밀LOCK/큰외부MIDI부분LOCK은자르지말고RHYTHMLOCKLIMIT. 독립녹음Fill은PatternROLL로덮지않고Fill/All요청에만기존변형을적용한다.
- 생성패턴의livebudget은HatGrid를지원하고, 고스트는프레이즈에기록해중복autoGhost를추가하지않는다. 8thSwing/32nd보간, 반복파트타이밍, Density0, backbeat실제스케줄검사. 낮은Amount재ROLL은기본킥모티프를보존한다. 저장값만보고완료라하지말고actualevent·WAV·실기기결과를구분한다.

- 0.8 사용자확정: MAIN D4 Duck→HatRhythm. Duck은 FX상세/기존저장값 유지. SCREEN Q16 relay/Program고정매핑 함께 변경.5간격(quarter/eighth/sixteenth/eighthDOT/sixteenthDOT)×3(plain/FILL/ROLL)=15. HatRule4프로필 유지. 전면LCD터치/왼쪽8행2페이지목록/모달가이드숨김 유지. GROOVE구HatGrid버튼은새선택기로대체.
- first294 parameter정의고정; HatRhythm294/HatPhraseState295/HatRhythmPage296 append=297. HatPhraseState는bank4..6별4bit(rhythm+1;0legacy). Undo/hoststate에 실제리듬보존. 구294state는HatGrid선택만마이그레이션하고 노트/위상은유지. Dotstride720/360ticks는마디경계연속,4barloop에서reset. Dot에straightSwing강제하지않음.
- FILL은응답마디2/4의짧은pickup,ROLL은마지막eighth내32nd4연타. 기본16th FILL도차이가있어야함. 다음Pattern/AllROLL적용,선택만으로현재노트불변,HatRuleLOCK우선/녹음Fill보존. 기존128노트/RT고정배열/오디오thread무할당규칙준수.15×20생성과actualscheduler/상태/Undo/legacy/Q-Link/Duck보존검사필수. 하드웨어노브검증은화면조작/자동검사와구분.

- 0.9확정: 자체제작코어128WAV/95새MIDI/96kit기본. Splat추출11039WAV/95MIDI/96kit는독립PRIVATE `.gdexp`. 확장팩payload를빌드때병합하지않음. 별도bank파일+registry.tsv를시작시병합색인. 설치/삭제는로컬expansions.py, 재인스턴스화로반영. NativeXPN등록으로보고하지말것.
- 기존297parameter정의고정.45append=342. Legacyfixedselector는alias, 새global카탈로그와normalizedQrelay를사용. C/DRUMS/KITS화면+Program매핑함께맞춤.8row동적KitBrowser/Packfilter/ROLE/LIBRARY공통필터. ROLL은현재필터안에서만Kit/parts선택. 128pack/65535kit와rolechoices/1048575entries한계는성능인증아님.
- constructor에서만팩merge, entries마다sourcepath. Cache64/voice수명유지, 오디오thread할당/잠금/I/O/host콜백금지. 누락팩이면다른소리로교체하지말고MISSING표시/해당voice무음. missing상태재저장→재설치후에도sample/MIDI/kit/pack식별자를보존. unresolved문자열은state로드에서만쓰기, render는atomicref만변경. 신규GDXP0001 200byte trailer, legacyextra허용.
- sourceContent는자체코어만. 개인확장팩은bank-work/personal/Content에설치. publicassembly가private확장팩을복사하지않는지실제ZIPpayload검증. core/expansion SHA검사,경로탈출/중복ID/암묵적덮어쓰기금지. 공개용artifact작성과외부publish구분. SDK/기존샘플공유조건을확정법률보증으로말하지않음.

## 2026-11-16 repository structure

- Public project: `mpc-software/plugins/public/GrooveDice/`. Root is `kimswork/`.
- Original authoring: `mpc-software/expansions/groove-dice/public/original-workshop/`.
- Restricted source/import scripts/work/personal packs: `mpc-software/expansions/groove-dice/private/splat-to-clat/`; ignored by Git.
- Shared standalone expansion implementation: `mpc-software/shared/groove-dice-expansion/expansions.py`. Plugin entry point delegates to it. Packaged CLI must include the standalone implementation.
- Public build and default packaging must work without the private tree. `tools/package_profiles.py --personal` explicitly adds the local optional private assembly.
- Historical bank-work references below describe pre-migration artifacts. Current private work is at the expansion path above. Preserve legacy parameter IDs/order/captions using legacy-catalogue.json; never regenerate them from a different bank.

- 1.0.0 release preserves342 parameter identities and UID KpGy. App-authored packs use GDEXP001 with optional target=groove-dice/plugin_uid=KpGy metadata; old manifests remain readable. Pack Builder UI hides its auto-generated Pack ID under Advanced; it is not an account identifier.

## 1.2.2: Sound layers and live rules (user approved)

- MAIN C=DRUM MIX (Kick/Snare/Hat/Perc levels), former DRUMS becomes SOUND. Kick/Snare maximum4 layers, each SCREEN group=Sample/Level/Tune/Shift; Hat remains Closed/Open and Perc four existing roles. Hat C/D are Closed/Open details. ON/SOLO/LOCK and selected-row DETAILS retained. See ../../../docs/plugins/groove-dice/layers.md.
- First342 parameters unchanged; layers extend through index520, rules521–524, sample commands525–532 (533 total). Do not reorder. GDLY0001 extends old state and eight saved user kits; missing sample SHA survives resave/reinstall. Private bank payloads remain separate.
- Shift ±30ms is actual onset timing, not trimming. All roles report30ms latency; do not claim zero-latency pads or hardware compensation without measurement. Same-hit layers share choke ownership; pending voices/cache refs must remain safe.
- The user authorized public release and website publication on 2026-11-17. Publish only original core payloads; optional personal expansions remain excluded.
- MPC 실기 ValueSlider 값 글씨는 샘플 플레이트 아래로 밀려 잘렸다. SOUND Sample은 중앙 Label+투명 FilmStrip Knob 터치층을 유지한다. Sample 부모에 Mouse Down / Q-Link / 해당 SoundQ(row*4)를 명시해 일반 노브처럼 가이드도 따라오게 한다. Details의 Hat에는 Closed/Open 두 선택기만 표시한다.
- 새 인스턴스에서 호스트가 effSetProgram(0)을 생략해도 Init 효과가 꺼지도록 constructor에서 실제 FX값을 초기화한다. 기존342 metadata fixture/ID/순서는 바꾸지 않는다.

- MAIN repeats one bar. RULE/Hat Rhythm changes are live; preserve bounded, deduplicated host notifications (no full layer/audio fanout). ROOM uses per-output stereo room reverbs. Sample arrows and SCREEN Q-Link share selection; retain the first342 fixture and fixed GDLY trailer boundary.

## 1.3.0 local development — HATS / CYMBALS

- MAIN D=Hat Rhythm/Open Hat/Ride/Crash; Drive/Dust/Room stay on FX. First533 declarations unchanged; three selectors append=536. Existing GDLY trailer boundary and bank format stay fixed.
- Dice preserves these rules. OpenHat replaces an eligible closed hit; Ride replaces generated MAIN hats, pauses for recorded fills; Crash is structural. Respect count-in gate, existing user choke groups and finite/allocation-free rendering.
- Ride/Crash/Bell have separate cache lanes/voices and same-pack classified samples or original procedural fallbacks. They share the existing Cymbal output; Ride/Bell follow Hat Mix, Crash Perc Mix. See ../../../docs/plugins/groove-dice/hats-and-cymbals.md.
- This local update does not publish a new release/site. Preserve personal expansions and install on current MPC internal storage. Record actual-device versus automated verification honestly.

## 1.3.3 current release

- User confirmed installed locks/Dice/RULE operation and authorized release/site updates with direct MPC screenshots. Earlier local-only publication notes above are historical.
- MAIN16 DiceLock0–15(623–638) are persistent non-automatable controls. Roll changes eligible visible MAIN values within musical bounds. Extra controls supplementary changes; Kit Fixed also protects MAIN mix. Seven-style RulePickKick/Snare/Hat/Perc append639–642; no LOCK in current knob/list/Screen/Program range. Preserve legacy0–638 IDs and saved legacy LOCK migration.
- SOUND and CYMBALS rows use Sample/Level/Decay/Shift. Details use Start/Tune/LowCut/HighCut. Tap sample names for a left browser; arrows/Q-Link share the catalogue. PREVIEW is isolated to the selected layer and cannot record a fill or choke performance voices. Preserve eight-role fill access and selected-kit pool identities.
- Captionless targets must not create empty fixed-Text widgets: they blanked the entire MPC UI. Conditional label bounds need version2; otherwise LOCK KIT and KIT FIXED overlap. Keep native ordered/duplicate-key serialization and modal guide suppression.
- Public ZIP contains only original default.gdbank; owner expansion registries/banks must be preserved and excluded from Git/Release. Device images must be direct MPC captures, with model/version provenance. A build or rendered mockup is not hardware QA.

## 1.3.4 reverse controls

- All17sound sources have independent44px REV switches beside ON/SOLO/LOCK. REV means reverse playback, separate from polarity inversion. It latches direction per new voice; Start trims from the playback end, Decay/Shift remain independent. Do not reuse Invert or change LayerFields/old GDLY/GDCY layouts.
- First643parameter declarations remain fixed. Reverse0–16 append643–659; SoundRev0–3 append660–663. GDRV0001 stores eight user-kit reverse sets; old states default forward. Factory selection resets reverse; sound browsing/roll preserve direction.
- Automatic engine/build/skin verification and internal-NVMe file installation do not prove physical REV button operation. Record captures and device-touch checks separately.

- Per-row Tune is visible on SOUND/CYMBALS as a fifth touch control beside Sample. Keep16fixed Q-Link relays Sample/Level/Decay/Shift; SoundTune0–3 append664–667 as non-automatable editing relays for existing Tune targets. Do not remove Shift/Decay or add a fifth hardware Q-Link.

- Latest approved SOUND mapping supersedes the prior Decay-first and five-column/Tune proposals: page1 Sample/Level/Tune/Shift; page2 Start/Decay/LowCut/HighCut for every row. Existing SoundDetails is the two-page selector, not a modal. Preserve page across categories/cymbals. Footer previous/next each48×48 and centered SOUND1/2 indicator. REV stays in each row; page2 adds approved Ø using SoundPolarity0–3 at668–671. Inactive HatC/D and CymbalD targets return -1; never write another sound through them.

- 1.3.4 actual MPC correction: dynamic command captions must be children of the command wrapper before its final transparent continuous ResetOnGestureEnd press surface. External labels above a command swallowed sample/list center taps. Legacy library/role/kit rows need one final caption-safe command target; do not leave duplicate parent actions. Verify text-center activation and single-command behavior, not only border clicks.

- 1.4.0local RULES expansion: first672full declarations/ranges stable; RulePickFullKick/Snare/Hat/Perc672–675 append,676total. First7choices preserve meanings;9new per part,16each. Legacy8style/7picker/4intensity aliases retain ranges; full canonical values drive generation/Dice/Undo/Screen/Program. Old state seeds full values. MAIN drawers page16rows as8+8with local page handles; hideQLinkBounds retained. Test36distinct raw+scheduled profiles, backbeats2/4, kick2/4exclusion, drag spacing/caps, other-part/fill preservation, old/new state and bounded physical fine updates. Preserve all installed personal banks/registry and keep them out of public ZIP/Git. Public stable remains1.3.4until new authorization.

- 2026-11-18: user approved official1.4.0 release and website publication. Earlier local-only statements above describe candidate history. Retain actual QA limits and explicit product tags.


## 2026-11-18 공통 Dice 잠금 — 최신 구현 후보

이 결정은 과거 Donut Theory의 5개 헤더 잠금 후보를 대체한다. Donut Theory1.13.0 / Groove Dice1.5.0 로컬 후보이며, 별도 공개 승인 전 정식 버전은 각각1.12.0 /1.4.0이다.

- Dice가 변경하는 값에 개별 자물쇠를 제공한다. 같은 실제 값을 표시하는 페이지/릴레이는 잠금을 공유한다. 수동 터치·Q-Link·명시적 공장 프리셋 선택은 허용한다.
- Donut MAIN16개는 각 노브 왼쪽 위에44px 터치 잠금을 둔다. 다른 페이지의 실제 랜덤 대상도 잠근다. OSC Pair/Wave1/Wave2와 각 LFO의 Rate/Sync/Division은 각각 하나의 잠금을 공유한다. ENV와 PLAY의 Amp Release도 공유한다.
- Groove Dice는 기존 MAIN16개 잠금을 유지하며 SOUND는 실제 역할·레이어·페이지에 따라 잠금 대상을 연결한다. FX의 랜덤 대상과 필인 A–D의 실제 데이터 보호도 제공한다. 랜덤화하지 않는 사운드 파라미터에는 의미 없는 잠금을 추가하지 않는다.
- 기존 킷·패턴·파트·레이어 잠금은 개별 잠금보다 우선한다. UNLOCK ALL은 전체 페이지의 개별 잠금과 상위 보호를 해제하고, Dice 실행·음색/샘플/노트 변경은 하지 않는다.
- master/MIDI/routing 및 항상 보존하는 Donut 코드·키·패드·Pitch Drop에는 Dice 잠금을 만들지 않는다. Q-Link 할당은 바꾸지 않는다. 원래 미술과 글꼴을 보존하며 Crate Press는 변경하지 않는다.
- Donut first293 / Groove first676 파라미터 선언은 유지한다. 새 파라미터는 append하며 총353 /777개다. 공개 ZIP에는 개인 확장팩을 넣지 않는다.
- 자동 검사와 실제 MPC 터치/화면 검증은 분리해서 기록한다. 물리 Q-Link 회전·오디오·최악 CPU는 새 검증 없이 완료로 주장하지 않는다.


## 2026-11-18 Groove Dice1.5.1 — predictable Dice policy (supersedes1.5.0 common-lock candidate)

- Global ROLL DICE changes the MAIN16controls and regenerates the repeating main pattern, respecting MAIN locks and Pattern Lock. If Kit Lock is off it also chooses sounds within the selected pack/kit pool; Kit Lock preserves the kit. Random Amount0 leaves everything unchanged.
- All four recorded/edited fill banks and all seven master/kit FX parameter values are preserved by global Dice, including Whole Kit replacement. The previous Extra scope selector is removed from the interface; saved Scope values remain compatible but no longer choose global behavior.
- ROLL FILL is a separate bottom action in the fill editor: it varies only the selected A–D bank, uses Random Amount, respects legacy locked rhythm roles, and is blocked during active recording. ROLL FX is a separate bottom action on FX: it randomizes Drive/Dust/Room/Duck only. Both support UNDO and do not regenerate the main pattern or choose a new kit.
- Individual Dice padlocks are visible only beside MAIN16controls. Remove the1.5.0-added SOUND/FX/choke/fill locks and duplicated GROOVE timing locks; existing Kit/Pattern/Role/Layer protections and global UNLOCK ALL remain. Appended1.5.0 lock parameters are retained for state/ID compatibility and no longer restrict non-MAIN randomization.
- GROOVE remains the detailed timing/density/ghost/velocity/random-amount/fill-frequency page; this change does not redesign or delete it. Donut Theory remains unchanged with its approved all-page shared locks.
- First777declarations are unchanged; append RollFill777/RollFx778,779total, same kim/KpGy identity and existing Q-Link maps. This is a local installed candidate; public stable stays1.4.0 until a separate release request. Preserve all personal expansions and keep them out of public packages.


2026-11-19: User authorized publishing all current fixes and updating the website: Donut Theory1.13.14 and Groove Dice1.5.1; Crate Press0.2.4 is already current and unchanged. Prior candidate-only statements are historical. User separately requested installing all three on an SD card for another device; SDK currently sees only internal nvme and removable SSD, so card identity/insertion is pending. Do not silently install to SSD or remove the internal copies until the card destination is confirmed and verified. Personal expansions must remain excluded from public packages.

2026-11-19 publication complete: Donut Theory1.13.14 and Groove Dice1.5.1 are official stable Releases from sourced23da9a. ZIP/checksum digests verified anonymously; public Pages run44419559175 succeeded;77responsive route-width checks and current actual MPC screenshots published. Crate Press0.2.4 remains the unchanged public preview. See each device-version/publication.json. Earlier candidate/public1.12.0/1.4.0 statements are historical. SD migration remains pending: the user requested a portable SD install, but SDK drive discovery twice showed only internal nvme and removable SSD. Do not claim card installation or choose SSD without identification. Preserve owner expansions and existing internal copies until the SD destination is confirmed and verified.

## 1.6.0 Triplet hats

- First779 declarations/ranges stay exact. Append HatRhythmFull779 (24 choices), HatPhraseFullState780 (three5-bit bank codes) and HatRhythmFullPage781 (three pages);782 total. Legacy HatRhythm294/HatPhraseState295/HatRhythmPage296 keep their fifteen-choice/four-bit semantics. Migrate old states without regenerating notes; never pack triplet codes into four-bit fields.
- MAIN D1/GROOVE/touch/Screen/Program/Dice/Undo share the full selector. The existing D1 Dice lock is reused; no extra lock button. Triplets are640/320/160 ticks at960PPQ. Keep straight swing off dotted/triplet notes, preserve explicit offsets and bounded nudge. Scheduler budgets must deliver all24 plain sixteenth-triplet hits and ending ornaments.
- Direct MPC captures and remote interactions are device QA; physical encoder actions, hardware audio and worst-case CPU require separate evidence. Retain personal expansions on install; publish only original core payloads. Donut Theory local changes are outside this release.

2026-11-19: User-authorized Groove Dice1.6.0 stable publication completed from source8b37090. Official GitHub tag/ZIP/checksum, kimswork.app/Pages run44450794354, updated manual and direct MPC captures are public. Anonymous asset and website-link digests verified;77responsive and10public checks pass. Final boundary-fix engine was reinstalled on NVMe and SD36C7-1ACA (167files each), app restarted and remote Triplet playback-state/selection/Dice-lock/Undo retested; owner expansions retained. SD-specific execution, physical encoders, hardware audio and worst-case CPU remain separately uncertified. Donut Theory local1.14.x work was excluded; public Donut1.13.14/Crate0.2.4 remain unchanged. See design/device-1.6.0/publication.json.

2026-11-19 user approved Groove Dice Open MPC release with Pack Builder information. Release1.6.1is packaging/docs/licensing only: preserve all782declarations/UI/Q-Link maps/default bank, increment native metadata only. Explicit project/original-core MIT notices do not relicense private expansions or Pack Builder dependencies. Include PACK_BUILDER.md with optional macOS Apple Silicon/Windows x64 app links, folder→export→installation→selection/removal workflow. No UI redesign or hardware certification implied. Publish public profile only; do not change installed owner packs or other plugins.

2026-11-19 Groove Dice1.6.1release/site complete: release source4a89f6e, Pages44527014146success,91responsive and12public page/download checks passed. SDK and per-CPU archives include optional Pack Builder workflow/downloads and explicit MIT notices; only original core bank, empty expansion registry, no personal payloads. Pack Builder binaries remain1.0.0and separate; no companion-app catalog entry or dependency relicense was added. Open MPC PR1035 submitted (review/merge pending); integration records in mpc-software/catalog/open-mpc/. Device owner installations remain1.6.0with their own packs: this docs/licensing-only release did not modify the MPC or claim new hardware certification. Other plugins unchanged.

2026-11-19 Open MPC PR1035review correction: all Groove Dice portable releases must declare user_data=[Content/Expansions] via --user-data Content/Expansions; never publish with an empty declaration. Use tools/package_open_mpc.py (public ZIP only), which validates both metadata copies/installers. Linux actual-installer regression must preserve nested packs/registry on upgrade/reinstall/uninstall and reproduce old1.6.1loss.1.6.2is packaging-only; keep782IDs/UI/corebank. Catalog/latest website screenshots must show only public128WAV/95MIDI; no owner-pack totals. Root README explains scoped MIT; private expansions and app dependencies remain separate. Preserve owner device installs by verified backup/restoration; do not execute Open MPC scripts on the actual device.
