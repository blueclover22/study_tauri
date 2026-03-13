import { invoke } from "@tauri-apps/api/core";
import { LoginRequest, LoginResponse } from "../types/auth";
import { IpcResponse } from "../types/ipc";

/// 인증 도메인 서비스.
/// Rust(auth.rs)에서 모든 에러 처리를 수행합니다.
/// - 성공: IpcResult::ok(info) → { success: true, data: {...} }
/// - 실패: IpcResult::err(message) → { success: false, error: "메시지" }
export const authService = {
  login: async (credentials: LoginRequest): Promise<LoginResponse> => {
    let result: IpcResponse<LoginResponse>;
    try {
      result = await invoke<IpcResponse<LoginResponse>>(
        "auth_login",
        { credentials }
      );
    } catch (invokeErr) {
      // Tauri invoke 자체가 throw하는 경우 (매우 드문 경우)
      const msg =
        typeof invokeErr === "string"
          ? invokeErr
          : invokeErr instanceof Error
          ? invokeErr.message
          : "로그인 중 오류가 발생했습니다.";
      throw new Error(msg);
    }

    // Rust에서 처리한 결과 확인
    if (!result.success) {
      // Rust에서 설정한 에러 메시지 전달
      const errorMsg = result.error || "로그인 중 오류가 발생했습니다.";
      throw new Error(errorMsg);
    }

    // 성공 경로
    return result.data!;
  },
};
