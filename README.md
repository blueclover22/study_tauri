# Tauri Test Project


## 🛠 기술 스택

### 프론트엔드
- **언어**: TypeScript
- **프레임워크**: React 19
- **빌드 도구**: Vite
- **패키지 매니저**: pnpm

### 백엔드 (네이티브)
- **언어**: Rust
- **프레임워크**: Tauri v2

### 주요 라이브러리
- `@tauri-apps/api`: Tauri 코어 API와의 통신을 위한 라이브러리
- `@tauri-apps/plugin-opener`: 시스템 기본 브라우저나 파일 탐색기를 열기 위한 플러그인
- `serde` & `serde_json`: Rust 데이터 직렬화/역직렬화

---

## 📂 프로젝트 구조

```text
.
├── src/                # React 프론트엔드 소스 코드
│   ├── main.tsx        # 프론트엔드 진입점
│   └── ...
├── src-tauri/          # Tauri 백엔드 (Rust) 소스 코드
│   ├── src/            # Rust 로직
│   ├── Cargo.toml      # Rust 의존성 관리
│   └── tauri.conf.json # Tauri 설정 파일
├── public/             # 정적 자산
├── index.html          # 메인 HTML 파일
├── package.json        # Node.js 의존성 및 스크립트
├── vite.config.ts      # Vite 설정
└── tsconfig.json       # TypeScript 설정
```

---

## 🚀 실행 및 빌드 명령어

이 프로젝트는 `pnpm`을 사용하여 관리됩니다.

### 프로젝트 초기화 (의존성 설치)
프로젝트를 처음 클론하거나 의존성을 업데이트할 때 실행합니다.
```bash
pnpm install
```

### 개발 모드 실행
프론트엔드 개발 서버와 Tauri 데스크톱 창을 동시에 실행합니다.
```bash
pnpm tauri dev
```

### 프로젝트 빌드
애플리케이션을 배포 가능한 형태로 빌드합니다.
```bash
pnpm tauri build
```

### 기타 명령어
- **프론트엔드만 실행**: `pnpm dev`
- **프론트엔드 빌드 확인**: `pnpm build`

---

## 🛠 트러블슈팅 (Troubleshooting)

### 빌드 오류 발생 시 해결 방법

#### 1. Rust 빌드 캐시 삭제 (가장 권장)
`src-tauri` 폴더 안에 있는 `target` 폴더를 삭제해야 합니다. 터미널에서 다음 명령어를 실행하세요:

```bash
# src-tauri 디렉토리로 이동
cd src-tauri
# 빌드 캐시 삭제
cargo clean
# 다시 프로젝트 루트로 이동
cd ..
```
또는 수동으로 `C:\JMW\src\pos_test\study_tauri\src-tauri\target` 폴더를 삭제하셔도 됩니다.

#### 2. 다시 실행
캐시를 삭제한 후 다시 실행합니다:
```bash
pnpm tauri dev
```

#### 3. 추가 확인 사항 (만약 위 방법으로 안 될 경우)
만약 `cargo clean` 후에도 같은 오류가 발생한다면, 다음 파일들에 이전 경로(`tauri_test`)가 하드코딩되어 있는지 확인해 보세요:
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `package.json`

특히 `Cargo.toml`의 `[package]` 섹션에 있는 `name` 필드나 다른 설정들이 현재 폴더 구조와 맞는지 확인이 필요할 수 있습니다.

---

## ⚙️ 개발 환경 세팅 (Setup Guide)

### 1. IDE 세팅 (VS Code / Cursor)
**필수 확장:**
- `rust-analyzer`: Rust 자동완성 및 오류 표시
- `Tauri`: 공식 확장 (명령어 실행 및 설정 파일 유효성 검사)

**권장 설정 (`settings.json`):**
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all"
}
```

### 2. OS별 필수 도구 설치

#### Windows
1. **Visual Studio Community** 설치 ("C++를 사용한 데스크톱 개발" 워크로드 필수)
2. **Rust 설치**: [rustup.rs](https://rustup.rs/)에서 설치 (MSVC 툴체인 사용)
3. **Node.js & pnpm 설치**

#### macOS
1. **Xcode Command Line Tools**: `xcode-select --install`
2. **Rust 설치**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. **Node.js & pnpm 설치**

### 3. Tauri CLI 설치
```bash
pnpm add -g @tauri-apps/cli
```

---

## 📦 빌드 및 배포 상세

### 빌드 결과물 경로
- `src-tauri/target/release/bundle/`
- **Windows**: `.msi`, `.exe`
- **macOS**: `.app`, `.dmg`

### 주요 빌드 타겟 옵션
- **Windows 32비트**: `pnpm tauri build -- --target i686-pc-windows-msvc`
- **macOS Universal**: `pnpm tauri build -- --target universal-apple-darwin`

> **참고**: 각 플랫폼 빌드는 해당 OS 환경에서만 가능합니다. 크로스 플랫폼 빌드가 필요한 경우 GitHub Actions 사용을 권장합니다.
