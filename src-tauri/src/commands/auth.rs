use crate::config;
use crate::services::HttpClient;
use crate::types::IpcResult;
use crate::utils::handle_reqwest_error;
use serde::Deserialize;
use tauri::State;

/// 로그인 요청 페이로드 (프론트엔드 → Rust).
/// Electron의 `LoginRequest` 타입과 동일 구조입니다.
#[derive(Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userPwd")]
    pub user_pwd: String,
}

/// 로그인 Tauri Command.
/// Electron의 `ipcMain.handle('auth:login', ...)` → reqwest HTTP 호출 로직을 대체합니다.
#[tauri::command]
pub async fn auth_login(
    credentials: LoginRequest,
    state: State<'_, HttpClient>,
) -> Result<IpcResult, String> {
    let url = format!(
        "{}{}{}",
        config::BASE_URL,
        config::LOGIN_PATH,
        config::DEVICE_ID
    );
    let body = serde_json::json!({
        "userId": credentials.user_id,
        "userPwd": credentials.user_pwd,
    });

    let response = state.client.post(&url).json(&body).send().await;

    match response {
        Err(e) => {
            let error_msg = handle_reqwest_error(e, "auth:login");
            Ok(IpcResult::err(error_msg))
        }
        Ok(resp) => {
            let is_success = resp.status().is_success();
            let status_code = resp.status().as_u16();

            match resp.json::<serde_json::Value>().await {
                Err(_) => {
                    Ok(IpcResult::err("서버 응답 형식이 올바르지 않습니다."))
                }
                Ok(data) => {
                    // 1) HTTP 레벨 오류 처리
                    if !is_success {
                        let server_msg = data["status"]["message"]
                            .as_str()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                format!("서버 오류가 발생했습니다. (HTTP {})", status_code)
                            });
                        return Ok(IpcResult::err(server_msg));
                    }

                    // 2) 비즈니스 로직 상태 코드 검사 (status.code != "0000")
                    let biz_code = data["status"]["code"].as_str().unwrap_or("");
                    if biz_code != "0000" {
                        let server_msg = data["status"]["message"]
                            .as_str()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                format!("로그인 실패 (코드: {})", biz_code)
                            });
                        return Ok(IpcResult::err(server_msg));
                    }

                    // 3) info 존재 여부 검사
                    let info = &data["info"];
                    let is_info_empty = info.is_null()
                        || (info.is_string() && info.as_str().unwrap_or("").is_empty())
                        || info
                            .as_object()
                            .map(|m| m.is_empty())
                            .unwrap_or(false);

                    if is_info_empty {
                        return Ok(IpcResult::err("로그인 정보가 올바르지 않습니다."));
                    }

                    // 4) 인증 토큰 저장 (명확한 성공 경로에서만)
                    if let Some(token) = data["info"]["tokenValue"].as_str() {
                        state.set_auth_token(Some(token.to_string()));
                    }

                    // 성공 경로: info 객체만 전달
                    Ok(IpcResult::ok(data["info"].clone()))
                }
            }
        }
    }
}
