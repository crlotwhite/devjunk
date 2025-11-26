# DevJunk

**크로스플랫폼 개발용 빌드/캐시 디렉터리 정리 툴**

Windows, macOS, Linux에서 개발 프로젝트의 불필요한 빌드 아티팩트와 캐시 디렉터리를 스캔하고 정리하는 도구입니다.

## 지원하는 정리 대상

| 종류 | 디렉터리 패턴 |
|------|--------------|
| Python Venv | `.venv`, `venv` |
| Python Tox | `.tox` |
| Python Cache | `__pycache__` |
| Mypy Cache | `.mypy_cache` |
| Pytest Cache | `.pytest_cache` |
| Node Modules | `node_modules` |
| Rust Target | `target` |
| Build Dir | `build` |
| Dist Dir | `dist` |
| Out Dir | `out` |
| Go Vendor | `vendor` |
| Next.js | `.next` |
| Nuxt.js | `.nuxt` |

## 프로젝트 구조

```
devjunk/
├── Cargo.toml                 # Workspace 루트
├── devjunk-core/              # 핵심 라이브러리 (스캔/삭제 로직)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs             # 라이브러리 엔트리포인트
│       ├── types.rs           # 도메인 타입 정의
│       ├── error.rs           # 에러 타입
│       ├── scanner.rs         # 디렉터리 스캔 로직
│       └── cleaner.rs         # 정리/삭제 로직
├── devjunk-cli/               # CLI 바이너리
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── devjunk-gui/               # Tauri GUI 앱
    ├── src-tauri/             # Tauri Rust 백엔드
    │   ├── Cargo.toml
    │   ├── tauri.conf.json
    │   └── src/
    │       ├── main.rs
    │       ├── commands.rs    # Tauri 커맨드
    │       └── dto.rs         # DTO 타입
    ├── package.json
    └── src/                   # React 프론트엔드
        ├── main.tsx
        ├── App.tsx
        ├── store/
        │   └── scanStore.ts   # Zustand 상태 관리
        └── components/
            ├── PathInput.tsx
            ├── ScanTable.tsx
            └── ActionBar.tsx
```

## 기술 스택

- **Core**: Rust 2021 Edition
- **GUI**: Tauri 2.0 + React 18 + TypeScript 5
- **상태 관리**: Zustand (가벼운 상태 관리 라이브러리, Redux 대비 보일러플레이트 최소화)
- **병렬 처리**: Rayon (디렉터리 스캔 병렬화)

## 사전 요구사항

### 공통
- [Rust](https://rustup.rs/) (1.70 이상)
- [Node.js](https://nodejs.org/) (18.x 이상)
- npm 또는 pnpm

### OS별 추가 요구사항

#### Windows
- WebView2 (Windows 10/11에 기본 설치됨)
- Visual Studio Build Tools (C++ 빌드 도구)

#### macOS
- Xcode Command Line Tools
```bash
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## 빌드 및 실행

### 1. 저장소 클론 및 디렉터리 이동
```bash
cd devjunk
```

### 2. CLI 빌드 및 실행

```bash
# 빌드
cargo build -p devjunk-cli

# 또는 릴리즈 빌드
cargo build -p devjunk-cli --release

# 실행 (현재 디렉터리 스캔)
cargo run -p devjunk-cli -- scan .

# 특정 디렉터리 스캔
cargo run -p devjunk-cli -- scan /path/to/project1 /path/to/project2

# Dry-run 삭제 (실제 삭제 없이 미리보기)
cargo run -p devjunk-cli -- clean . --dry-run

# 실제 삭제 (확인 프롬프트 표시)
cargo run -p devjunk-cli -- clean .

# 확인 없이 삭제
cargo run -p devjunk-cli -- clean . -y

# 지원하는 정크 타입 목록
cargo run -p devjunk-cli -- types

# JSON 출력
cargo run -p devjunk-cli -- scan . --json
```

### 3. GUI 앱 개발 모드

```bash
# GUI 디렉터리로 이동
cd devjunk-gui

# npm 의존성 설치
npm install

# 기본 아이콘 생성 (최초 1회 필요)
npm run tauri icon

# 개발 모드 실행 (핫 리로드 지원)
npm run tauri dev
```

> **Note**: Windows에서 처음 빌드할 때 `npm run tauri icon` 명령으로 기본 아이콘을 생성해야 합니다.
> 또는 `src-tauri/icons/` 폴더에 직접 아이콘 파일을 추가할 수 있습니다.

### 4. GUI 앱 프로덕션 빌드

```bash
cd devjunk-gui

# 프로덕션 빌드 (인스톨러 생성)
npm run tauri build
```

빌드된 앱은 다음 위치에 생성됩니다:
- **Windows**: `devjunk-gui/src-tauri/target/release/bundle/msi/`
- **macOS**: `devjunk-gui/src-tauri/target/release/bundle/dmg/`
- **Linux**: `devjunk-gui/src-tauri/target/release/bundle/deb/` 또는 `appimage/`

## CLI 사용법

```
devjunk - A tool for scanning and cleaning development build/cache directories

Usage: devjunk <COMMAND>

Commands:
  scan   Scan directories for development junk
  clean  Clean (delete) development junk directories
  types  List supported junk types
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Scan 명령
```bash
devjunk scan [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  Paths to scan [default: .]

Options:
  -d, --max-depth <MAX_DEPTH>  Maximum depth to scan
      --include-hidden         Include hidden directories in scan
      --json                   Output in JSON format
  -h, --help                   Print help
```

### Clean 명령
```bash
devjunk clean [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  Paths to scan and clean [default: .]

Options:
      --dry-run                 Perform a dry run (don't actually delete)
  -d, --max-depth <MAX_DEPTH>  Maximum depth to scan
      --kind <KIND>            Filter by junk kind (can be specified multiple times)
  -y, --yes                    Skip confirmation prompt
  -h, --help                   Print help
```

## 테스트

```bash
# 모든 테스트 실행
cargo test --workspace

# 특정 크레이트 테스트
cargo test -p devjunk-core
```

## 아키텍처 설계 결정

### 1. 멀티 크레이트 구조
- `devjunk-core`: 순수 비즈니스 로직 (스캔, 삭제)
- `devjunk-cli`: CLI 인터페이스
- `devjunk-gui`: Tauri GUI 앱

이 구조를 통해:
- 로직 재사용성 극대화
- 테스트 용이성 향상
- 관심사 분리

### 2. 상태 관리 (Zustand 선택 이유)
- **최소한의 보일러플레이트**: Redux 대비 설정 코드가 거의 없음
- **TypeScript 친화적**: 타입 추론이 자연스럽게 동작
- **Provider 불필요**: React Context 래핑 없이 직접 훅 사용
- **비동기 지원 내장**: async 액션을 자연스럽게 지원
- **경량**: gzip 압축 시 ~1KB

### 3. DTO 패턴
Rust 도메인 타입(`ScanResult`, `CleanResult`)과 프론트엔드 직렬화용 DTO(`ScanResultDto`, `CleanResultDto`)를 분리하여:
- API 안정성 보장
- 프론트엔드 친화적인 필드명 (camelCase)
- 추가 표시용 필드 제공 (예: `sizeDisplay`)

### 4. 에러 처리
- `thiserror`를 사용한 타입 안전한 에러 정의
- `anyhow`를 통한 컨텍스트 첨부
- 확장 가능한 에러 계층 구조

## TODO (향후 확장)

- [x] 네이티브 디렉터리 선택 다이얼로그 (Tauri dialog 플러그인)
- [ ] 테이블 정렬 기능
- [ ] 필터링 UI (종류별, 크기별)
- [ ] 설정 저장 (최근 경로, 제외 패턴 등)
- [x] 진행률 표시 (대용량 스캔 시)
- [ ] 시스템 트레이 아이콘
- [ ] 정기 스캔 스케줄링
- [x] 국제화 (i18n) - 한국어/영어 지원, 브라우저 언어 자동 감지

## 라이선스

MIT License
