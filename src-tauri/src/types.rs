use serde::Serialize;

/// 공통 IPC 응답 타입.
/// Electron의 `IpcResult<T>` (ipcErrorHandler.ts) 역할을 대체합니다.
/// 모든 Tauri Command의 반환 타입으로 사용합니다.
#[derive(Serialize)]
pub struct IpcResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl IpcResult {
    pub fn ok(data: serde_json::Value) -> Self {
        IpcResult {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        IpcResult {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}
