# Private plugins

개인용 플러그인은 `mpc-software/plugins/private/<PluginName>/`에 둡니다. 이 디렉터리의 프로젝트·자료·바이너리는 Git에서 기본 제외합니다. 이 README와 AGENTS.md만 공개 저장소에 기록합니다.

- 공개 사이트, Pages 산출물, GitHub Releases에 포함하지 않습니다.
- `git add -f`로 제외 규칙을 우회하지 않습니다. 비공개 원격 백업은 별도 비공개 저장소로 관리합니다.
- 샘플과 외부 자료의 개인 사용·변환·재배포 권한은 각각 확인합니다. 개인용이라는 이유로 라이선스 제한이 없어지지는 않습니다.
- `_research/`는 프로젝트 생성 전 로컬 분석 자료 공간이며 빌드할 플러그인이 아닙니다.
- 실제 프로젝트 생성 전 이름·vendor·UID를 결정하고 프로젝트 README에 의존성과 사용권을 기록합니다.
