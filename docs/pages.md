# kim GitHub Pages

사이트: https://kimswork.app/

## 구조와 배포

`site/`는 Astro 정적 사이트다. 홈, 제품별 상세(`/plugins/<slug>/`), 설치 가이드, 404 페이지를 생성한다. GitHub Actions가 main의 site 변경 또는 수동 실행 시 빌드·내부 링크 검사 후 Pages에 배포한다. PR은 빌드·검사만 수행한다. 사이트 배포와 플러그인 바이너리 Release 발행은 별도다.

제품의 소개, 이미지, 개발 버전, Q-Link 표, 검증 기기, 알려진 제한, 공개 Release를 `site/src/data/plugins.ts`에서 관리한다. 새 제품은 고유 slug와 이미지를 추가하면 홈과 상세 페이지가 생성된다. release가 null이면 다운로드 버튼을 만들지 않는다. release의 tag는 `<slug>/v<version>`이며 해당 플러그인의 실제 ZIP asset을 지정한다. 저장소 공통 releases/latest 링크는 금지한다.

## 반응형 기준

PC, 일반 모바일, 폴드 외부·내부 화면을 지원한다. 기종 판별 대신 가용 뷰포트 너비를 사용한다. 320px 좁은 화면에서는 메뉴를 두 줄로 배치하고, 560–760px에서는 기능 카드를 두 열로 전환한다. 761–1050px는 간격과 타이포를 줄이고, 큰 화면은 최대 콘텐츠 폭을 유지한다. 터치 환경은 주요 링크의 최소 높이를 확보한다. 긴 사양 표는 표 영역 안에서만 가로 스크롤한다.

브라우저에서 320 / 344 / 390 / 690 / 768 / 1024 / 1440px의 홈·상세·설치 안내 21가지 조합을 검증했다. 페이지 가로 넘침과 이미지 로딩 실패 없음. 이는 반응형 브라우저 검증이며 폴드 실기기 또는 접힘 힌지 검증 완료를 뜻하지 않는다.

## 콘텐츠 원칙

현재 Donut Theory는 개발 알파다. 공개 Release 없음. Screen Q-Link와 화면 코드 hold 문제를 제품 페이지에 표시한다. 승인 이미지는 디자인 시안으로 명시한다. 실제 캡처와 시안을 혼동하지 않는다. 개발 버전과 공개 버전을 별도로 관리하고, 새 Release를 발행하면 링크 실재 여부와 기능·제한 설명도 갱신한다.

개발 명령과 제품 추가 절차: [site README](../site/README.md).

## 언어·테마 확정 (2026-11-14)

사용자 요청으로 공개 사이트의 모든 UI·본문·메타데이터·접근성 라벨을 영어로 통일했다. HTML 언어는 en이며, 기본 테마는 차콜 배경(#171614), 크림 본문(#eee5d5), 앰버 강조(#e6a16c)의 다크 테마다. 제품의 승인 아트워크는 그대로 유지한다. 향후 페이지도 영어·다크 테마 기준을 따른다.

홈·제품 상세·설치 안내·404를 320 / 344 / 390 / 690 / 768 / 1024 / 1440px의 28개 조합으로 재검증했다. 영문 lang 설정, 한글 잔여 문구 없음, 페이지 가로 넘침 없음, 다크 배경 적용을 확인했다. 480px 이하에서는 영문 메뉴를 두 줄로 배치한다.

## 배포 직후 테마 혼재 대응

이전 HTML 캐시로 페이지 이동 시 구 테마가 다시 표시되는 것을 방지하기 위해 내부 페이지 링크에 `navigationVersion` 쿼리를 붙인다. 이미지/CSS는 기존 자산 경로·해시를 유지하고 canonical에는 쿼리를 넣지 않는다. 공통 HTML/body에는 최소 다크 배경을 인라인으로 지정해 스타일 로딩 중 밝은 플래시도 방지한다. 2026-11-14 서버 응답과 실제 홈→상세→설치 이동은 모두 영문·다크임을 확인했다. 사용자 기기의 캐시 상태는 직접 확인하지 못했으므로 원인을 확정하지 않는다.

## 첫 방문 정보 구조 개선 (2026-11-14)

홈의 주 제목은 “Instrument plugins for MPC standalone.”으로 제품 종류와 플랫폼을 명시한다. 첫 제품 이름·한 문장 용도·다운로드 가능 여부·제품 상세 버튼을 첫 화면에 모은다. 출시 전 상태는 “In development — not available to download yet.”로 명확히 쓴다. 시각적 슬로건은 보조 역할로 유지한다. PC는 설명과 제품 아트워크를 나란히, 모바일·폴드 내부의 중간 너비에서는 설명을 먼저 보여준다.

상세 페이지 첫 화면에는 제품 용도·배포 상태·지원 환경·기능/검증 상태 바로가기를 둔다. Q-Link 표는 기능과 배포/검증 안내 뒤에 둔다. 첫 화면에서 알파 버전을 다운로드 가능 제품으로 오해하게 만들지 않는다. 320–1440px의 홈·상세 14개 조합에서 가로 넘침 없이 상태와 주 버튼 노출을 확인했다. 실제 신규 방문자 테스트와는 구분한다.

## 커스텀 도메인

사용자가 구매한 `kimswork.app`을 기본 주소로 사용한다. 브랜드 확정명은 Kim’s Work이며 플러그인 vendor `kim`과 UID는 변경하지 않는다. Cloudflare DNS의 apex A 레코드는 GitHub Pages의 네 IP(185.199.108.153–185.199.111.153), www CNAME은 `sunghyun23.github.io`이며 DNS only로 둔다. GitHub Pages Custom domain은 `kimswork.app`, Astro site는 `https://kimswork.app`, base는 `/`다. 인증서 발급 후 Enforce HTTPS를 활성화한다. 배포는 기존 GitHub Actions로 유지하며 별도 웹 호스팅은 구매하지 않는다.


## Donut Theory 1.0.0 publication
The public brand is Kim’s Work at https://kimswork.app/. Product metadata lives in site/src/data/plugins.ts. Actual MPC Live III screenshots replace concept art. The detailed English manual is imported from docs/plugins/donut-theory/manual.md into /plugins/donut-theory/manual/; changes to that source trigger Pages too. Release metadata must reference an existing plugin-specific tag and asset, never repository-wide latest.
