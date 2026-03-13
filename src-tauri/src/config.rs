/// 백엔드 API Base URL.
///
/// 빌드 프로파일에 따라 **컴파일 타임**에 자동 선택됩니다.
/// 값 변경은 `src-tauri/.cargo/config.toml` 한 곳만 수정하면 됩니다.
///
/// | 프로파일 | 명령어 | 사용 키 |
/// |---|---|---|
/// | debug   | `pnpm tauri dev`   | `API_BASE_URL_DEV`  |
/// | release | `pnpm tauri build` | `API_BASE_URL_PROD` |
///
/// 키가 없으면 빌드 시 `environment variable not defined` 컴파일 오류가 발생합니다.
#[cfg(debug_assertions)]
pub const BASE_URL: &str = env!("API_BASE_URL_DEV");

#[cfg(not(debug_assertions))]
pub const BASE_URL: &str = env!("API_BASE_URL_PROD");

/// 로그인 API 경로
pub const LOGIN_PATH: &str = "/api/air/posuser/login/";

/// POS 단말기 Device ID
/// 값 변경은 `src-tauri/.cargo/config.toml`의 `POS_DEVICE_ID` 키를 수정하세요.
/// 키가 없으면 빌드 시 `environment variable not defined` 컴파일 오류가 발생합니다.
pub const DEVICE_ID: &str = env!("POS_DEVICE_ID");

