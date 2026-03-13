use crate::types::IpcResult;
use serde::Serialize;

/// 장치 상태 정보 (프론트엔드 → TypeScript `DeviceStatusInfo` 와 대응)
#[derive(Serialize)]
pub struct DeviceStatusInfo {
    pub id: String,
    pub status: String,       // "connected" | "disconnected" | "error"
    pub last_checked: String,
}

/// 모든 주변 장치의 연결 상태를 확인합니다.
/// DeviceManagement 화면의 "상태 새로고침" 버튼에서 호출됩니다.
///
/// 실제 하드웨어 연동 시 이 함수 내부에서 serialport 등 크레이트를 통해
/// 각 장치에 ping 을 보내고 응답 여부로 status를 결정합니다.
#[tauri::command]
pub async fn device_check_status() -> Result<IpcResult, String> {
    // TODO: 실제 시리얼/네트워크 포트 점검 로직으로 교체
    // 현재는 시스템 시간을 기반으로 상태를 갱신하는 스텁(stub) 구현입니다.
    let now = chrono::Local::now().format("%H:%M:%S").to_string();

    let statuses = vec![
        DeviceStatusInfo {
            id: "card-terminal".to_string(),
            status: "connected".to_string(),
            last_checked: now.clone(),
        },
        DeviceStatusInfo {
            id: "receipt-printer".to_string(),
            status: "connected".to_string(),
            last_checked: now.clone(),
        },
        DeviceStatusInfo {
            id: "barcode-scanner".to_string(),
            status: "disconnected".to_string(),
            last_checked: now.clone(),
        },
        DeviceStatusInfo {
            id: "ticket-printer".to_string(),
            status: "error".to_string(),
            last_checked: now.clone(),
        },
    ];

    let data = serde_json::to_value(statuses)
        .map_err(|e| format!("직렬화 오류: {}", e))?;

    Ok(IpcResult::ok(data))
}
