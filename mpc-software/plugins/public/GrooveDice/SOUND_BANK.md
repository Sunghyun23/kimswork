# Current0.9layout

Defaultbank is now the original procedural core. Restricted archived content is an optional private expansion. See [EXPANSIONS.md](EXPANSIONS.md). The following is historical format/provenance documentation for the earlier restricted bank.

# Groove Dice sound bank — 0.4.0

런타임 음원은 `Content/Banks/default.gdbank` **한 파일**이다.1,482,284,328bytes(약1.48GB/1.38GiB). Git·공개 사이트·Release에서 제외한다. 압축·암호화·DRM은 아니다. 엔진/스킨은 샘플 및 외부 MIDI와 분리된다.

## 포함 범위

- WAV11,039개: Kits2,411 / Chops8,043 / Loops585.
- MIDI95개: 정상 export 실제 비트2개 / 앱에 포함된 그루브 템플릿93개.
- 원본 WAV·MIDI의 바이트·경로·SHA256을 모두 보존한다. 중복 SHA payload는 공유하되 각 경로의 디렉터리 항목은 유지한다.
- 원본 앱의 모든 공장 비트가 추출됐다는 뜻이 아니다. Clicks264개는 샘플 본체 미설치로 원본 컬렉션에도 없다. 추출된 전체 파일의 반영 범위와 원본 앱 전체 보유 범위를 구분한다.

## 구조

기존 `GDBANK01` version1 헤더/64개96byte 레코드/초기 WAV/`GDPATT01` 패턴을 호환 prefix로 유지한다. 헤더의 payload 길이는 이 prefix만 가리킨다. 구버전의 `parse(vector)`와 별개로 새 `load(path)`는 prefix 길이만 읽어 전체 뱅크를 RAM에 복사하지 않는다. 구형0.1 바이너리와 새 대용량 뱅크를 혼용하는 것은 지원하지 않는다. 호환성은 기존0.1 저장 상태를 새 엔진에서 읽는 방향이다.

그 뒤 `GDLIB002` version2 확장을 추가한다.32byte 헤더: magic8,version u32,count u32,entrySize384 u32,reserved3*u32.11,134개 디렉터리 항목 각각384bytes: SHA256 ASCII64,UTF8 경로256(0종단),absoluteOffset u64, byteLength/CRC32/kind/role/sampleRate/channels/frames/reserved3 각각u32, padding16. little-endian. kind0=Kits/1=Chops/2=Loops/3=Beats/4=Groove templates. 파일 끝16bytes는 `GDLEND02` magic8+확장 헤더 absoluteOffset u64다.

색인 전체 범위·길이·이름·식별자를 먼저 검증한다. 선택 파일마다 CRC32·WAV 형식/샘플레이트/채널/프레임 수를 검사한다. MIDI95개는 생성 때 원본 SHA 검증, 로딩 때 CRC와 SMF 범위 검사를 거쳐 immutable note 배열로 보관한다. 시간은PPQ960으로 정규화한다. 실제 비트는 현재8역할로 재배치, groove는 timing offset으로 적용한다. 원본 앱 악기 배치/FX의 정확한 복원은 아니다. 원본 MIDI 파일은 별도로 손실 없이 들어 있다.

## 메모리·실시간 경계

전체 PCM 디코드는2,222,030,848bytes이므로 동시에 올리지 않는다. 캐시64slot이며 하나의 WAV는 현재컬렉션 최대4,390,912PCM bytes. 이론적 캐시 최대281,018,368bytes+legacy64샘플/색인/일시적 입출력 버퍼다. 실제 선택·활성voice에 따라 작다. 교체 시 current8개+최대48voice의 이전샘플을 참조로 보호하며, process()가 사용을 끝낸 slot만 로더가 재사용한다. pending/current/voice 수명을 분리한다. 빠른 해제→동일파일 재선택은 요청 세대 번호로 처리한다.

constructor가 색인/MIDI를 읽고 로더를 시작한다. 대규모 실시간 브라우징을 위해 SDK 예제의 constructor/prepare 일괄 로딩에서 **별도 비실시간 로더로 확장**했다. SDK 소스를 수정하지 않았다. process()에서는 원자 요청/공개된 PCM 참조만 다루며 파일 I/O·디코드·메모리 할당·잠금·로그·host 알림을 하지 않는다. 폴더 탐색이나 UI 값 조회에서도 오디오 로딩을 하지 않는다. 종료 시 로더를 join한 후 소유 데이터를 해제한다.

## 상태·개인 자료 분리

초기214개 파라미터 ID/순서를 보존하고40개를 append했다(총254개). 상태의 기존 extra prefix와 optional stable-ID extension을 구분해0.1 QA 상태도 읽는다. 선택8샘플·8사용자킷×8샘플·선택 MIDI의SHA를 저장한다. 원본 음원은 프로젝트에 중복 저장하지 않는다. 미래 자체 제작 음원으로의 공개 전환 시 이 뱅크와 private bank-work를 교체하고 권한을 점검한다.

`bank-work/full-library.manifest.json`은 모든 원본 경로·SHA 및 색인 해시, `default.manifest.json`은 호환 prefix 정보를 기록한다. 두 파일과 `Content/Banks`는 private/Git 제외, 설치에는 뱅크만 넣는다. 원본 research는 변경하지 않는다.

생성: `python3 tools/build_bank.py`(전체). `--legacy-only`는 진단용64샘플 prefix만 생성하므로 실사용에는 지정하지 않는다. 기존 prefix에 전체확장을 다시 붙일 때 `python3 tools/build_library.py`. `python3 tools/verify_library.py`가11,134개 전부의 SHA256/CRC32/메타데이터를 검사한다. 원본 사용권 제한은 개인용 여부와 별개이며 공개 배포 가능성을 의미하지 않는다.

## 0.3 factory kit catalogue

원본 WAV/MIDI 디렉터리·payload·offset을 변경하지 않고 끝 GDLEND02 앞에 GDKITS03 suffix를 추가한다. header=magic8,count u32=96,recordSize u32=800. 각 record는 name64/NUL,8 SHA256 ASCII64,8×5 float32(Level/Tune/Decay/Pan/Choke),8 float32(FX),reserved28,CRC32(first796bytes)이다. suffix 끝 GDKEND03 magic8+catalogue absoluteOffset u64(16bytes), 그 뒤 기존 GDLEND02 footer를 유지한다. little-endian. 없는 suffix는 구형 뱅크로 허용한다. 범위/CRC/SHA→Kits 파일 연결/float 유효성은 constructor에서 검사한다. process는 immutable catalogue만 읽는다.

6개 팩의2233개 비무음 역할 분류 WAV에서 팩별16조합=96킷을 재구성했다. 이름/샘플SHA/분류·게인 근거는 bank-work/factory-kits.json 및 kit-audio-metrics.json에 기록한다. 하이햇은 공통초크1, 나머지는Off. 피크 기반 파트 게인은 킥/스네어/하이햇 등의 역할별 목표로 보정한다. 원본 vendor의 프리셋/FX/보이싱을 복원했다는 뜻이 아니다. 전체 WAV/MIDI 원본 바이트는 그대로 보존한다.

새 parameters는FactoryKit/KitDiceMode/KitBrowserPage/KitPagePrev/KitPageNext/DiceInfo/FactoryReload, 기존254개 뒤에 추가하여261개다.0.2 상태 로드 시 새 값은 기본으로 초기화하며 저장된 실제 샘플SHA·필인·노트 매핑을 보존한다. tools/build_factory_kits.py는 기존 full bank의 suffix만 재구축하며 build_bank/build_library의 전체 생성에도 연결된다.

## 0.4 role sound catalogue

GDKITS03 앞에 GDSND004 header(magic8/count u32=1606/width u32=200),1606records 및 GDSEND04(magic8/start u64) footer를 추가한다. 각record=group u32(0Kick/1Snare/2Hat/3Perc),2 SHAASCII64(두번째는Hat만사용),2×5float(Level/Tune/Decay/Pan/Choke),reserved24,CRC32(first196bytes). 수량440/546/376/244. 원본payload·directoryoffset은 변경하지 않는다. 생성 근거·원본SHA는 bank-work/role-sounds.json, 런타임 데이터는gdbank 한파일에만 있다. 판독은 길이/CRC/SHA/role/float범위를 검사한다. 약1.482GB, 기존11039WAV+95MIDI byte동일.

0.4는 기존261개 뒤26개 파라미터를 append하여287개다. optional GDB4(0x47444234)extra는7개 원본 패턴의 valid/length/128data+gate 및 그루브기준 valid/4float를 보존한다. 기존extra길이도 계속 허용한다. 0.4저장상태를구버전으로되돌리는것은보장하지않는다.

## 0.5 state extension

음원bank는0.4와byte동일하다. 기존287개파라미터뒤RuleKick/RuleSnare/RuleHat/RulePerc추가(291개). GDB4 baseline뒤optional u32magic0x47525035와4×(seed u32+offset float32[4])=총84bytes 추가. oldextra크기도수용하며새값은기본화한다. 변경된파트offset·seed와랜덤seed/KEEP는Undo에도포함한다.

## 0.6 이름·Perc 세트 호환

GDLIB002 디렉터리의 MIDI95개 name필드를 새 bank_path로 교체했다. 원본 path는 full-library.manifest.json과 midi-name-map.json에 보존하며 payload·오프셋·SHA·CRC32·순서는 바뀌지 않는다. WAV 이름/내용도 그대로다. 재빌드 시 build_library.py가 midi_names.bank_name을 적용한다. tools/midi_names.py는 기존 뱅크를 atomic copy로 바꾸고 private midi-renamed에 새파일명으로95개를 export한다. 원본 추출 폴더는 provenance이므로 별도로 보존한다.

GDSND004와GDKITS03의 바이트 형식은 유지한다. 엔진은 초기화 시 각244Perc앵커와 같은 팩의 공장 킷을 찾아 Clap/Tom/Cymbal3개를 연결한다(같은 팩 후보의 catalogue순서 modulo). SoundChoice 메모리 구조만4개로 확장되며 생성/할당은 로드 중에만 한다.4개 샘플/설정은 기존8-role SHA state로 저장되므로 새 추가 오디오 payload나 파일은 필요 없다. 원본 vendor의 Perc preset을 복원한 것은 아니다.
