# kim plugin catalog

Astro 정적 사이트. 제품 데이터: `src/data/plugins.ts`. 페이지는 제품 데이터에서 자동 생성한다.

```sh
cd site
npm ci
npm run dev
npm run build
npm run preview
```

개발/프리뷰 주소는 터미널에 표시된 포트의 `/`다. 커스텀 도메인의 base도 `/`이며 모든 내부 URL은 `url()`로 생성한다.

## 제품 추가

1. `public/images/`에 웹 최적화 이미지 추가. 시안/실제 화면 구분 및 alt 작성.
2. `src/data/plugins.ts`의 제품 배열에 고유 slug, 설명, 기능, 개발 버전 추가.
3. 출시 전 `release: null`. 실제 Release 발행 후 version/tag/asset 지정. tag는 `<slug>/v<version>`, asset은 실제 ZIP 이름.
4. 빌드 및 모바일·데스크톱 화면, 직접 접속, 링크 검증.

상세 소개, Q-Link 표, 검증 기기, 알려진 제한도 제품 데이터에 저장한다. 공통 상세 템플릿에 특정 제품 이름이나 사양을 하드코딩하지 않는다.

## 배포

GitHub repository Settings → Pages → Source: GitHub Actions. `main`의 site 변경 또는 수동 실행으로 배포한다. PR은 빌드만 진행한다. ZIP은 사이트에 넣지 않고 플러그인별 Release에 연결한다. 사이트 배포와 악기 Release 발행은 별도다.

공개 목표: https://kimswork.app/

개발 중 기능을 출시 완료로 소개하지 않는다. 현재 Screen Q-Link와 화면 hold 발음 문제를 공개 상태에 반영한다.

반응형 검증·운영 규칙: [docs/pages.md](../docs/pages.md). 내부 링크 검사: `npm run check:links` (빌드 후).


Current publication: Kim’s Work / https://kimswork.app/. Donut Theory1.1.2 is an official GitHub Release. The product page uses actual device exports and imports the English manual from docs/plugins/donut-theory/manual.md. Never replace these images with conceptual renders labelled as device screenshots.
