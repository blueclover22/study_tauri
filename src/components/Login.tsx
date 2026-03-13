import React, { useState } from "react";
import { useAuth } from "../hooks/useAuth";
import "./Login.css";

interface LoginProps {
  onLoginSuccess: (username: string) => void;
}

export const Login: React.FC<LoginProps> = ({ onLoginSuccess }) => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const { login, isLoading, error, setError, clearError } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!username.trim() || !password.trim()) {
      setError("사용자명과 비밀번호를 입력해주세요");
      return;
    }
    try {
      const userInfo = await login({ userId: username, userPwd: password });
      console.log("[Login] userInfo:", userInfo);
      if (userInfo) {
        onLoginSuccess(userInfo.userNm || userInfo.userId || username);
      }
    } catch (err) {
      // useAuth가 모든 에러를 잡으므로 이 경로는 타지 않아야 함
      console.error("[Login] Unexpected error:", err);
      setError("예상치 못한 오류가 발생했습니다.");
    }
  };

  return (
    <div className="login-container">
      <div className="login-box">
        <div className="login-header">
          <h1>POS 시스템 (Tauri)</h1>
          <p>관리자 로그인</p>
        </div>

        <form onSubmit={handleSubmit} className="login-form">
          <div className="form-group">
            <label htmlFor="username">사용자명</label>
            <input
              id="username"
              type="text"
              value={username}
              onChange={(e) => {
                setUsername(e.target.value);
                clearError();
              }}
              placeholder="사용자명을 입력하세요"
              disabled={isLoading}
              autoComplete="username"
              className={error ? "input-error" : ""}
            />
          </div>

          <div className="form-group">
            <label htmlFor="password">비밀번호</label>
            <div className="password-input-wrapper">
              <input
                id="password"
                type={showPassword ? "text" : "password"}
                value={password}
                onChange={(e) => {
                  setPassword(e.target.value);
                  clearError();
                }}
                placeholder="비밀번호를 입력하세요"
                disabled={isLoading}
                autoComplete="current-password"
                className={error ? "input-error" : ""}
              />
              <button
                type="button"
                className="toggle-password"
                onClick={() => setShowPassword(!showPassword)}
                disabled={isLoading}
              >
                {showPassword ? "숨기기" : "표시"}
              </button>
            </div>
          </div>

          {error && (
            <div
              className={`error-message ${
                error.includes("서버에 연결할 수 없습니다")
                  ? "error-message--server"
                  : ""
              }`}
            >
              {error.includes("서버에 연결할 수 없습니다") && (
                <span className="error-icon">⚠ </span>
              )}
              {error}
            </div>
          )}

          <button type="submit" className="login-button" disabled={isLoading}>
            {isLoading ? "로그인 중..." : "로그인"}
          </button>
        </form>

        <div className="login-footer">
          <p>POS System v1.0 | Tauri Edition</p>
        </div>
      </div>

      <div className="login-background">
        <div className="gradient-overlay"></div>
      </div>
    </div>
  );
};
