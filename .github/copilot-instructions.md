# Tauri POS 프로젝트 — 개발 가이드

## 프로젝트 개요
Tauri v2 기반 POS 데스크톱 앱. React 19 + TypeScript (WebView) / Rust (메인 프로세스) 이중 계층 아키텍처.

## 기술 스택
- **Frontend**: React 19, TypeScript, Vite 7
- **Desktop**: Tauri v2
- **Rust 의존성**: reqwest 0.12, serde/serde_json 1, chrono 0.4
- **Package Manager**: pnpm

---

## 프로젝트 구조

### 프론트엔드 (`src/`)
```
src/
├── api/            # Tauri invoke() 래퍼 — 도메인별 파일
├── components/     # 화면 단위 컴포넌트 + 동명 .css
├── hooks/          # Custom Hooks — 비즈니스 로직 분리
├── types/
│   ├── ipc.ts      # IpcResponse<T> — 모든 Command 공통 응답 타입
│   └── {domain}.ts # 도메인별 요청/응답 타입
├── App.tsx         # AppScreen 상태 기반 화면 라우팅
└── main.tsx        # 진입점
```

### 백엔드 (`src-tauri/src/`)
```
src-tauri/src/
├── commands/       # #[tauri::command] 핸들러 — 도메인별 파일
│   ├── mod.rs      # pub mod 선언
│   └── {domain}.rs # 도메인별 Command 구현
├── lib.rs          # invoke_handler![] 등록
├── services.rs     # HttpClient 싱글톤 (reqwest + Mutex 토큰)
├── config.rs       # API URL / DEVICE_ID (빌드 환경변수)
├── types.rs        # IpcResult 공통 구조체
└── utils.rs        # 에러 변환 유틸
```

---

## 코드 패턴

### Rust — 새 Command 추가
1. `commands/{domain}.rs` 생성 → `#[tauri::command]` 함수 작성
2. 반환 타입: **반드시** `Result<IpcResult, String>`
3. 성공/실패 모두 `Ok(IpcResult::ok(...))` / `Ok(IpcResult::err(...))` 반환
4. `commands/mod.rs`에 `pub mod {domain};` 추가
5. `lib.rs`의 `invoke_handler![]`에 함수 등록
6. Command 이름 규칙: `{도메인}_{동작}`
7. `capabilities/default.json` 권한 확인

### TypeScript — 새 API 서비스
- `src/api/{domain}Service.ts`에 `invoke<IpcResponse<T>>()` 작성
- 타입은 `src/types/{domain}.ts`에 정의

### React — 새 화면
1. `src/components/{Name}.tsx` + `.css` 쌍 생성
2. `App.tsx`의 `AppScreen` 유니온 타입 추가
3. `handleSelectMenu` 분기 추가
4. 비즈니스 로직은 `hooks/use{Domain}.ts`로 분리

### IPC 흐름
```
Frontend: invoke('command_name', payload) → IpcResponse<T>
Rust:     #[tauri::command] async fn command_name(...) -> Result<IpcResult, String>
```

---

## 에러 처리

### Rust — Command 에러 처리
- **HTTP 실패**: `handle_reqwest_error(e, "context")` → `Ok(IpcResult::err(msg))`
- **HTTP 오류**: `resp.status().is_success()` 체크 후 메시지 추출
- **JSON 파싱 실패**: 고정 에러 메시지
- **절대 금지**: `Err()` 직접 반환 (패닉만 허용)

```rust
match response {
    Err(e) => Ok(IpcResult::err(handle_reqwest_error(e, "context"))),
    Ok(resp) => {
        if !resp.status().is_success() {
            return Ok(IpcResult::err("서버 오류가 발생했습니다."));
        }
        match resp.json::<serde_json::Value>().await {
            Err(_) => Ok(IpcResult::err("서버 응답 형식이 올바르지 않습니다.")),
            Ok(data) => Ok(IpcResult::ok(data)),
        }
    }
}
```

### TypeScript — 프론트엔드 에러 처리
```typescript
const result = await invoke<IpcResponse<T>>('command_name', payload);
if (!result.success) {
  setError(result.error ?? '알 수 없는 오류가 발생했습니다.');
  return;
}
return result.data!;
```

### 에러 메시지 규칙
- 모든 Rust 에러는 **한국어**
- 네트워크 오류: `utils.rs`의 `handle_reqwest_error()` 재사용
- 서버 메시지 우선, 없으면 HTTP 상태 코드 포함

---

## 필수 규칙

| 규칙 | 설명 |
|------|------|
| **README.md 업데이트** | 새 패키지, 라이브러리, 모듈 추가/변경 시 필수 |
| **Tauri v2 API** | `@tauri-apps/api/core`에서 `invoke` import |
| **Rust 에러** | **한국어** 작성 |
| **CSS 분리** | 컴포넌트 동명 `.css` (전역: `index.css`, 앱: `App.css`) |
| **React 라우팅** | `AppScreen` 상태 기반 (React Router 미사용) |
| **HTTP 호출** | `services.rs`의 `HttpClient` 사용 |
| **토큰 관리** | Rust의 `Mutex<Option<String>>`만 사용 |

---

## 금지 사항
❌ Rust Command에서 `Err()` 직접 반환
❌ TypeScript에서 직접 HTTP 호출 (fetch, axios)
❌ 클라이언트에서 토큰 저장
❌ React Router 도입
❌ CSS-in-JS 또는 CSS 모듈 사용
❌ 전역 상태 관리 도구 (Redux 등)
❌ 영어 에러 메시지
