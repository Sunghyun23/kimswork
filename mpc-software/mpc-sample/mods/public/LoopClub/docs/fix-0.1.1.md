# Loop Club0.1.1 — 로컬 수정·검증 보고

**판정: 로컬 자동 검사와 ARM64 빌드 통과. 실기 적용·저장·재시작·SD 변경·공개 배포는 하지 않았습니다.** 사용자의 “저장·재시작하지 말고 대기” 및 “세세한 테스트…빌드 후 보고” 지시를 유지했습니다.

## 원인과 수정

| 항목 | 확인한 내용 | 수정 |
|---|---|---|
| 마스터 무음 | 점검 당시 실행 중인 소프트웨어 마스터가0%. 기존 코드는 CC의 현재 원시 값 대신 `low`를 사용해, 원시 값이 올라가도0으로 계산될 수 있었음 | 원시 CC 값으로 정규화. 물리 MAIN VOLUME과 저장되는 프로젝트 마스터를 분리하고 MAIN/USB·카운트인 모두 스무딩 적용 |
| 볼륨 상태를 알기 어려움 | FX 화면에서 소프트웨어 마스터 값이 숨겨졌음 | 마스터 값 표시와 `MASTER MUTED` / `MAIN VOL MUTED` 상태 표시 |
| DATA 커서 | 큰 델타를 목록 길이로 나머지 연산하면 움직이지 않는 경우를 로컬 재현(예:128 %4=0). 실제 기기의 원시 델타는 아직 계측하지 못함 | 하드웨어 회전 이벤트는 부호에 따라 한 항목 이동. 목록 양 끝에서 정지 |
| +/− 두 칸 이동 | 기존6줄 이동을4항목 목록에서 순환하면2칸 이동 | 일반+/−는1항목, SHIFT+/−는6항목. 순환 제거 |
| 첫 회전 누락 | 목록 준비가 완료돼도 UI total 갱신 전 첫 회전이 무시될 수 있음 | 입력 전 목록 갱신 |
| Repeat 추가 무음 | 무음에서 Repeat ON 또는 OFF 후 무음 상태로 재활성화하면 빈 버퍼를 고정하는 실패 테스트를 확보 | 실제 입력부터 한 Repeat 구간을 채운 뒤 고정. 재활성화 시 다시 대기 |

물리 볼륨은 제거하지 않았습니다. 루퍼 신호는 stock 믹스 뒤에 추가되므로, 그 신호에는 물리 볼륨을 별도로 적용하되 프로젝트의 마스터 페이더를 덮어쓰지 않도록 했습니다. 출력 경로는 [SDK 제작자 루퍼 설명](https://github.com/ioma8/mpc-minimod/blob/main/docs/LIVE-LOOPER.md#output-mixed-into-the-device-block)과 [CC 원시 값 전달 코드](https://github.com/ioma8/mpc-sample-sdk/blob/c7d54aaf77a8848437b7e89bd191c0f72689ac5e/src/sites.rs)를 대조했습니다. 이는 Loop Club0.1.1의 실기 검증을 대신하지 않습니다.

이전 원격 입력 테스트는 `low`에 가상 현재 값을 넣어 실제 CC 형식을 제대로 재현하지 못했습니다. 이번에는 `low=0`, `raw=20/64/100/127` 조건과 실제 콜백 경로를 로컬 회귀 검사에 추가했습니다.

## 검증 결과

| 검사 | 결과 |
|---|---|
| Debug 전체 | **158개 통과, 실패0** |
| 최적화 Release 전체 | **158개 통과, 실패0** |
| Clippy `--all-targets -D warnings` | 통과 |
| rustfmt 검사 | 통과 |
| ARM64 Release | 통과 · ELF64 AArch64 · 최대 GLIBC_2.30 |
| 실시간 할당/해제 검사 | Repeat 대기→캡처와 볼륨 변경 포함 통과 |
| 기존 회귀 | 녹음·카운트인·신스·레이어·사이드체인·4×4FX·저장복원·출력 분리 포함 |

추가 경계값: 빈 목록/단일 항목/4항목/6항목/다중 페이지/1001항목, 양방향 끝 처리, 부호·크기가 다른 회전 입력, 네 WAV 중 선택한 파일의 실제 이름·원본1000프레임 로딩,4팩×4킷 중 Bank2/Kit1의 정확한 선택·PCM 로딩을 검사했습니다. 소프트웨어 마스터와 물리 볼륨의 별도 음소거, MAIN/USB 양쪽 복구 및 프로젝트 전환 시 현재 물리 볼륨 보존도 확인했습니다.

## 빌드 산출물

- 버전: **0.1.1 로컬 후보**, 공개0.1.0은 그대로 유지
- 파일: `loop-club.so`, 1,204,360bytes
- SHA-256: `d6970ac82c694e7997ccb4f186cfae1f2b8f1dec2e8ba949b399c4af96e5e9b9`
- 같은 폴더의 `logs/`에 최종 검사 로그와 수정 전 의도적 실패 재현 로그를 보관했습니다.

## 남은 확인

실제 DATA 회전 입력이 어떤 크기로 전달되는지, 물리 노브·오디오가0.1.1에서 정상인지 아직 확인하지 않았습니다. 실기 적용/재시작이 보류됐으므로 **실기 해결 완료로 보고하지 않습니다.** 이미 저장된 프로젝트의 마스터0은 사용자 의도일 수 있어 자동으로 바꾸지 않았습니다. 기존 공개판의 CPU 순간 초과 문제도 이번 로컬 검사로 해결됐다고 주장하지 않습니다.


## Applied BPM100 revision — 2026-11-22

Owner explicitly authorized device installation and reset, superseding the previous hold. New projects default100BPM; PROJECT (SHIFT+DATA) K1 selects integer tempo and DATA moves1BPM. Stop playback first; nonempty audio loops retain the tempo lock. Stopped MIDI loops preserve note ticks and lengths. Debug160/Release160, strictClippy and ARM64 build pass. Owner project was successfully saved before this restart; both existing tracks were checked after restart. Current project set/saved100BPM; actual remote DATA100→101→100 and DRM verified. Runtime/SD SHA256 `c7d5b7b398c5800fa6b47bac0949e2e091e960242bec7bf09e681a2d4cb0a01c`;20boot files read back,20previous files backed up. Physical encoder/knob acceptance and hardware power-cycle were not performed in this pass. Public0.1.0 remains unchanged. Private device evidence stays ignored under build/qa-device/apply-100bpm.


## Physical DATA dispatch and recording views — 2026-11-22

Physical DATA did not increment the high-level encoder counter despite remote encoder tests passing. The mod consumed CC100 before the firmware could dispatch its DATA handler. Both independent and fallback CC paths now forward CC100 once; the high-level encoder hook consumes the resulting turn. Actual owner input moved hardware count1025→1026 and bank row0→1 with delta+1. Whole-kit list movement also passed remotely after entering a bank. This is separate from remote-only encoder acceptance.

REC from LOOP enters performance for Synth/Drum and the waveform view for Audio, then follows the configured count-in. Recording Play/DrumSound views draw current MIDI events; Audio uses current capture peak features rather than the previous committed clip. SEQ is source-aware: MIDI grid or audio waveform. No sound/recording mode parameter is changed by entering the display. Note rows fit the captured pitch range.

164native tests, strictClippy and ARM64 build pass. Isolated actual Sample QA session capture-preview3-20261122 verified one-bar count-in excluding early notes, REC entry, synth/drum notes, audio capture-frame growth and audio SEQ. Actual audio input was silent: the live waveform was a flat line; native nonzero-input peak tests passed. Physical REC/pad acceptance is not claimed. Original owner project restored and track settings/lengths/event counts checked. Final runtime/SD SHA256181927747d24760c2aebca1c4c87fb08daae3e06511d93748700fc2b899b5bb7; previous boot backed up. Actual private captures and reports remain ignored under build/qa-device/data-kit-investigation. Public0.1.0 unchanged.


## Smooth harmony, random lanes, Sync and audio inputs — 2026-11-23

SOUND SHIFT+instrument pad now generates the selected drum lane, including deliberate replacement of recorded content. Other lanes remain; SHIFT+ERASE+samepad restores the last generation only. Active recording blocks generation. Default smooth chord voicing searches inversions/octaves against previous chord voices without changing pitch classes; SHIFT+B2 in CHORD toggles it. Shared LFO/PAN gets free or BPM Sync,16divisions with dotted/triplet variants; SHIFT+B3 on LFO/PAN toggles; Rate selects division when synced. Free rate remains stored. Old JSON has compatible defaults.

SHIFT+SAMPLE opens LINE/MIC/USBinput selection (pads1–3orDATA+B2). A non-audio worker validates the actualcard2boolean controls and names, switches/readbacks source, handles failures with rollback and persists local InputSource.json. Audio callback accepts selectedstream only, using measured128frameADC/24frameUSBgeometry. MIC disables speaker and keeps it off against delayed stock startup changes; previousspeakerstate is retained inpreferences and restored onleavingMIC. Inputselectioncurrentlyrequires stoppedtransport/no recorder.

169native,host/LinuxstrictClippy,ARM64PASS. RealdeviceQAfeatures-20261123 verified generation/Undo, smooth recordedchordnotes,16division metadata control plus straight/dotted/triplet captures, all3inputswitchstates and realUSBtonecapturepeak0.08. Post-restartMICguardandUSBspeakerrestorechecked. RealLINE/MICsignal listening, physicalnewbuttonacceptance and fullpowercycle remain untested. Originalownerproject restored. Exactfinalnormal runtime+SD SHA58f9e384d8a7d88ae222bdf0731e5a7143ec2ad03654e8b5d79fb94003ac8685. Private reports/captures under build/qa-device/smooth-sync-input. No public release. Eight-track redesign is subsequent work, not part of those verified captures.
