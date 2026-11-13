# mpc-plugins

Akai MPC용 악기 및 이펙트 플러그인을 개발하는 모노레포입니다.

## SDK

[inMusic MPC Plugin Builder SDK](https://github.com/inmusic-software/mpc-plugin-builder-sdk)를 사용합니다.
플러그인은 C++ DSP 코드, `plugin.json` 메타데이터, JSON UI로 구성되며 SDK의 Python 도구로 빌드합니다.

## 디렉터리 구성

- `plugins/<PluginName>/`: 각 플러그인의 소스, 메타데이터, UI, 프리셋 및 README
- `shared/`: 여러 플러그인이 함께 사용하는 코드와 리소스
- `docs/`: 프로젝트 설계 및 개발 문서

각 플러그인은 독립적으로 빌드하고 배포할 수 있도록 관리합니다.
공용 코드는 실제로 여러 프로젝트에서 필요한 시점에 `shared/`로 추출합니다.
플러그인별 기능, 파라미터 및 테스트 방법은 해당 프로젝트의 README에 기록합니다.

## 초기 상태

현재는 모노레포의 디렉터리 구조만 준비되어 있습니다.
SDK와 컴파일러는 아직 설치하지 않았으며, 첫 프로젝트를 구성할 때 SDK 버전을 고정하고 빌드 및 테스트 절차를 추가합니다.
