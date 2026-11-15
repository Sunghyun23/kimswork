# mpc-plugins

[inMusic MPC Plugin Builder SDK](https://github.com/inmusic-software/mpc-plugin-builder-sdk)를 사용하는 Akai MPC 악기·이펙트 플러그인 모노레포입니다. 여러 플러그인을 독립적으로 개발·버전 관리·배포합니다.

## 소개와 배포

- **GitHub Pages**: 플러그인 카탈로그 및 플러그인별 소개 페이지. 플러그인이 추가될 때마다 소개 페이지와 카탈로그 항목을 추가합니다.
- **GitHub Releases**: 설치 패키지와 릴리스 노트. 소개 페이지에서 해당 플러그인의 Release와 다운로드 파일로 연결합니다.
- 공개 페이지에는 실제 구현·검증된 기능과 배포 상태를 표시합니다.

## 구조

```text
plugins/<PluginName>/               개별 플러그인 소스·UI·프리셋·README
shared/                            실제 공유 코드·리소스
docs/                              운영 규칙과 설계 문서
docs/plugins/<slug>/                개별 사양 및 디자인 기준
site/                              Astro GitHub Pages 소스
third_party/mpc-plugin-builder-sdk/ 공유 SDK, 커밋 고정
```

프로젝트별 빌드 결과는 `build/`에 생성하고 Git에 커밋하지 않습니다. `docs/`는 개발 문서이며 사이트 소스와 구분합니다.

## 구현 전 읽을 문서

1. [AGENTS.md](AGENTS.md): 다음 작업에서도 지킬 구현 규칙
2. [모노레포·Pages·Release 정책](docs/repository-policy.md)
3. [Donut Theory 사양](docs/plugins/donut-theory/spec.md)
4. [Donut Theory 디자인 기준](docs/plugins/donut-theory/design.md)

## 플러그인

| 이름 | 소스 경로 계획 | 상태 |
|---|---|---|
| Donut Theory | `plugins/DonutTheory/` | [v1.2.2 정식 릴리스](https://github.com/Sunghyun23/mpc-plugins/releases/tag/donut-theory/v1.2.2): 64음색·최대16코드 슬롯·뱅크별 진행·통합 편집; 검증 범위는 매뉴얼 참고 |

## 현재 상태

공유 SDK를 고정 커밋으로 설치하고 Donut Theory 1.2.2의 Moog-inspired 엔진과 공장 음색을 구현했습니다. native·ARM32·ARM64 빌드, host simulator 19항목 및 엔진·UBSan·스킨 회귀 검사를 통과했습니다. MPC Live III / OS 3.10.1.12에서 64음색 전환·발음, 화면 hold/릴리스, 필터와 네이티브 프리셋 재시작 복원을 검증했습니다. [제품 소개](https://kimswork.app/plugins/donut-theory/)와 [상세 매뉴얼](https://kimswork.app/plugins/donut-theory/manual/)에서 설치 조건과 알려진 제한을 확인하세요.

빌드: `python3 scripts/build_plugin.py DonutTheory --target all`

검증: `python3 scripts/test_plugin.py`

[Donut Theory 인계·재개 기준](docs/plugins/donut-theory/handoff.md) · [설치·개발 안내](plugins/DonutTheory/README.md) · [구현 및 검증 기록](docs/plugins/donut-theory/implementation.md)

승인 이미지는 디자인 콘셉트이며 실행 가능한 플러그인이나 실제 MPC 캡처가 아닙니다. 구현 시 SDK 좌표·호스트 영역·가독성에 맞춰 변환합니다.

## GitHub Pages 사이트

[플러그인 카탈로그](https://kimswork.app/) · [사이트 운영·반응형 기준](docs/pages.md) · [사이트 개발 안내](site/README.md)

홈·제품 상세·설치 가이드는 `site/`에서 관리합니다. PC·일반 모바일·폴드 외부/내부 화면에 대응합니다. 제품의 공개 Release가 없으면 다운로드 준비 중으로 표시합니다.


## 1.2.0 변경 사항

프리셋 조정에서 엔진 자체 재설계로 전환했습니다. 비선형 오실레이터 믹서·4단 래더Low-pass·VCA를 기본 경로에 적용하고63개 연주 음색을 새 엔진에 맞춰 다시 설계·레벨 보정했습니다.208개 파라미터·UID·UI·Q-Link는 유지하지만 기존 프로젝트의 소리는 달라집니다. 정확한 구버전 소리가 필요하면1.1.2를 보관하고 트랙을 렌더하세요.

[전체 변경 내역과 검증 범위](docs/plugins/donut-theory/releases/1.2.0.md) · [엔진 구조·연구 출처](docs/plugins/donut-theory/engine.md) · [프리셋 설계 기준](docs/plugins/donut-theory/presets.md). 전체 실기기 청음·최악CPU 인증과 자동 수치 검증을 구분합니다.


## 1.2.1 실기기 검증과 UI 수정

선택 사이드바 위로 코드 페이지 화살표가 겹치는 문제를 수정했습니다. 새 엔진의 MPC 실행·오디오·CPU 관찰과 실제1.2.1 스크린샷을 공개했습니다. [검증 결과와 한계](docs/plugins/donut-theory/releases/1.2.1.md)를 참고하세요.


## 1.2.2 — CHORD 경량화

불안정한 수동 패드 위치 표시 기능을 제거했습니다. CHORD는10,889→1,913위젯으로 줄였고 실제 MIDI/LEARN·기존 매핑은 유지합니다. 실기기 원격 반복 전환·편집 결과 및 한계는 [릴리스 기록](docs/plugins/donut-theory/releases/1.2.2.md)을 참고하세요.
