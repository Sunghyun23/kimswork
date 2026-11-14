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
| Donut Theory | `plugins/DonutTheory/` | [v1.0.1 정식 릴리스](https://github.com/Sunghyun23/mpc-plugins/releases/tag/donut-theory/v1.0.1): 64음색·최대16코드 슬롯·뱅크별 진행·통합 편집; 검증 범위는 매뉴얼 참고 |

## 현재 상태

공유 SDK를 고정 커밋으로 설치하고 Donut Theory 1.0.1을 구현했습니다. native·ARM32·ARM64 빌드, host simulator 19항목 및 엔진·UBSan·스킨 회귀 검사를 통과했습니다. MPC Live III / OS 3.10.1.12에서 실제 UI를 검증했습니다. [제품 소개](https://kimswork.app/plugins/donut-theory/)와 [상세 매뉴얼](https://kimswork.app/plugins/donut-theory/manual/)에서 설치 조건과 알려진 제한을 확인하세요.

빌드: `python3 scripts/build_plugin.py DonutTheory --target all`

검증: `python3 scripts/test_plugin.py`

[설치·개발 안내](plugins/DonutTheory/README.md) · [구현 및 검증 기록](docs/plugins/donut-theory/implementation.md)

승인 이미지는 디자인 콘셉트이며 실행 가능한 플러그인이나 실제 MPC 캡처가 아닙니다. 구현 시 SDK 좌표·호스트 영역·가독성에 맞춰 변환합니다.

## GitHub Pages 사이트

[플러그인 카탈로그](https://kimswork.app/) · [사이트 운영·반응형 기준](docs/pages.md) · [사이트 개발 안내](site/README.md)

홈·제품 상세·설치 가이드는 `site/`에서 관리합니다. PC·일반 모바일·폴드 외부/내부 화면에 대응합니다. 제품의 공개 Release가 없으면 다운로드 준비 중으로 표시합니다.
