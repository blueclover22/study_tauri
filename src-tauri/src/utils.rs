/// reqwest 에러를 사용자 친화적인 한국어 메시지로 변환합니다.
/// Electron의 `ipcErrorHandler.ts` – handleAxiosError() 역할을 대체합니다.
pub fn handle_reqwest_error(error: reqwest::Error, context: &str) -> String {
    eprintln!("[Rust] {} Error: {:?}", context, error);

    if error.is_connect() || error.is_timeout() {
        "서버에 연결할 수 없습니다. 서버 상태를 확인해주세요.".to_string()
    } else if error.is_decode() {
        "서버 응답 형식이 올바르지 않습니다.".to_string()
    } else if error.is_request() {
        "요청을 처리하는 중 오류가 발생했습니다.".to_string()
    } else {
        format!("{} 중 알 수 없는 오류가 발생했습니다.", context)
    }
}
