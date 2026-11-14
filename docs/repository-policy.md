# 모노레포 및 배포 정책

## 프로젝트 구조

- 소스: `plugins/<PluginName>/` (첫 프로젝트 `plugins/DonutTheory/`).
- 공개 slug: 소문자 kebab-case (`donut-theory`). 표시 이름: `Donut Theory`.
- 제조사와 UID는 생성 전에 결정한다. manifest·코드·패키지의 이름/제조사/UID가 일치해야 한다.
- 프로젝트는 `plugin.json`, `src/`, `ui.json` 또는 커스텀 `skins/`, README를 가진다. 리소스는 SDK 규칙에 맞춰 둔다.
- README에 기능, 파라미터, 프리셋, 빌드·검증, 설치, 제한 사항, 외부 코드·자산 라이선스를 기록한다.
- 플러그인별 독립 빌드·버전·배포가 가능해야 한다.

## 공유 SDK

공식 저장소: https://github.com/inmusic-software/mpc-plugin-builder-sdk

`third_party/mpc-plugin-builder-sdk/`에 한 벌을 두고 커밋을 고정한다. `third_party/sdk.lock.json`과 `scripts/bootstrap_sdk.py`로 고정 커밋을 설치하고 재현 절차를 기록한다.

초기 분석 출처는 `febd2ca5bd9eb16c4a412837196e145566bc981b`다. 현재 설치된 공유 SDK도 이 커밋으로 고정한다. 업데이트 시 API·스킨·호스트 계약 변경을 확인한다.

## GitHub Pages

- 소스는 `site/`. Astro와 .github/workflows/pages.yml로 빌드·배포한다.
- 홈은 플러그인 카탈로그: 이름, 이미지, 악기/이펙트 분류, 짧은 설명, 출시 상태, 상세 링크.
- 각 플러그인은 `/plugins/<slug>/` 상세 페이지를 가진다. GitHub 프로젝트 Pages의 저장소 base path를 고려한다.
- 상세 페이지: 실제 화면, 핵심 기능, 사용/연주 방법, 지원 조건, 설치법, 버전, Release 및 다운로드 링크.
- 새 플러그인이 추가될 때 카탈로그와 상세 페이지를 함께 갱신한다. 미출시 항목은 미출시로 표시하고 가짜 다운로드를 만들지 않는다.
- 실제 구현 시 메타데이터를 한곳에서 관리해 카탈로그·상세 페이지·릴리스 링크가 일치하게 한다.
- 배포 전 링크, base path, 모바일 레이아웃, 접근성, 이미지를 확인한다.
- 공개 URL·도메인·Pages 활성화와 권한은 실제 설정 시 확인한다. 이미 활성화되었다고 가정하지 않는다.

## GitHub Releases

각 플러그인은 독립적인 Semantic Versioning을 사용한다. 모노레포 태그 충돌 방지 규칙:

| 항목 | 형식 | 예시 (아직 발행하지 않음) |
|---|---|---|
| 태그 | `<slug>/v<version>` | `donut-theory/v0.1.0` |
| 제목 | `<Display Name> <version>` | `Donut Theory 0.1.0` |
| 파일 | `<slug>-<version>-mpc.zip` | `donut-theory-0.1.0-mpc.zip` |

- SDK 패키징 도구를 우선 사용한다. 패키지는 `<Manufacturer> - VST - <Name>/`와 manifest/version, ARM32·ARM64 바이너리, 스킨, 필요한 리소스를 포함한다.
- 릴리스 노트에는 변경 사항, 호환성, 최소 OS, 설치법, 검증 환경, 알려진 제한을 쓴다.
- Pages는 해당 플러그인의 명시적인 태그·asset URL로 연결한다. 저장소 공통 `/releases/latest`를 사용하지 않는다.
- 빌드 결과·캐시·ZIP은 소스 Git이나 Pages에 중복 보관하지 않는다. 배포 패키지는 Releases에 둔다.

## 출시 검증 순서

1. 버전, UID, 이름, 제조사 정합성을 확인한다.
2. native, ARM32, ARM64 빌드 및 SDK 검증을 통과한다.
3. host simulator의 저장/복원·파라미터 변화·출력 안정성을 확인하고 WAV를 청음한다.
4. 실제 MPC 모델/OS, CPU, Q-Link, 프리셋, 저장/복원 결과를 기록한다. 실기기 미검증이면 명시한다.
5. 패키지와 릴리스 노트를 준비한다.
6. 사용자에게 허용된 배포 범위에서 Release를 게시하고 Pages 링크를 갱신한다.

초기 SDK 문서 기준 조건은 MPC OS 3.10.1 이상 및 Enable Enhancements / External Plugin Support 활성화다. 출시 시 사용하는 SDK와 실제 기기 조건을 다시 확인한다.

## 현재 상태

이 문서는 운영 정책이다. 공유 SDK는 `third_party/sdk.lock.json`과 `scripts/bootstrap_sdk.py` 방식으로 도입했다. 사이트와 Pages 워크플로는 구현됐으며 공개 도메인은 https://kimswork.app/ 이다. 릴리스 상태는 제품 메타데이터와 GitHub Releases에서 확인한다.

## Pages 구현 확정 (2026-11-14)

사용자가 Pages 구현·서빙을 승인했다. `site/`의 Astro 정적 사이트와 `.github/workflows/pages.yml`로 구현한다. 제품 데이터는 `site/src/data/plugins.ts`, 공개 주소는 https://kimswork.app/ 이다. PC·일반 모바일·폴드 외부/내부 화면에 대응하며 세부 운영 규칙과 검증 결과는 [pages.md](pages.md)를 따른다. 이전의 사이트 미구현 상태 설명은 이 항목으로 대체한다. 플러그인 Release 발행은 사이트 배포와 별개다.
