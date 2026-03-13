import { useState } from "react";
import { authService } from "../api/authService";
import { LoginRequest, LoginResponse } from "../types/auth";

/// 인증 상태 관리 훅.
/// Electron의 `useAuth.ts` 와 동일한 인터페이스를 제공합니다.
export const useAuth = () => {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");

  const login = async (
    credentials: LoginRequest
  ): Promise<LoginResponse | null> => {
    setIsLoading(true);
    setError("");
    try {
      const userInfo = await authService.login(credentials);
      return userInfo;
    } catch (err) {
      let errorMessage = "로그인 중 오류가 발생했습니다";
      
      if (err instanceof Error) {
        errorMessage = err.message;
      } else if (typeof err === "string" && err.length > 0) {
        errorMessage = err;
      }

      setError(errorMessage);
      return null;
    } finally {
      setIsLoading(false);
    }
  };

  const clearError = () => setError("");

  return { login, isLoading, error, setError, clearError };
};
