/// 모든 Tauri Command 공통 응답 인터페이스.
/// Electron의 IpcResult<T> (ipcErrorHandler.ts) 역할을 대체합니다.
/// Rust에서 `Option` 필드는 skip_serializing_if로 undefined가 됩니다.
export interface IpcResponse<T = unknown> {
  success: boolean;
  data?: T;
  error?: string;
}
