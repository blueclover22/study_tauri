use reqwest::{header, Client};
use std::sync::Mutex;

/// 백엔드 통신용 공유 HTTP 클라이언트 상태.
/// Electron의 `axiosInstance.ts` (axios.create() + setAuthToken()) 역할을 대체합니다.
pub struct HttpClient {
    pub client: Client,
    pub auth_token: Mutex<Option<String>>,
}

impl HttpClient {
    pub fn new() -> Self {
        let mut default_headers = header::HeaderMap::new();
        default_headers.insert(
            header::CONTENT_TYPE,
            "application/json; charset=utf-8".parse().unwrap(),
        );
        default_headers.insert(header::USER_AGENT, "IM-POS".parse().unwrap());
        default_headers.insert(
            header::ACCEPT,
            "application/json; charset=utf-8".parse().unwrap(),
        );

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .default_headers(default_headers)
            .build()
            .expect("Failed to create HTTP client");

        HttpClient {
            client,
            auth_token: Mutex::new(None),
        }
    }

    /// Authorization 헤더 값을 갱신합니다. (Bearer 토큰)
    pub fn set_auth_token(&self, token: Option<String>) {
        if let Ok(mut t) = self.auth_token.lock() {
            *t = token;
        }
    }

    /// 현재 저장된 Bearer 토큰 문자열을 반환합니다.
    pub fn get_auth_header(&self) -> Option<String> {
        self.auth_token
            .lock()
            .ok()?
            .as_ref()
            .map(|t| format!("Bearer {}", t))
    }

    /// 저장된 Bearer 토큰을 Authorization 헤더에 자동 주입한 POST 요청을 반환합니다.
    /// 인증이 필요한 API Command에서 사용합니다.
    #[allow(dead_code)]
    pub fn post_with_auth(&self, url: &str) -> reqwest::RequestBuilder {
        let req = self.client.post(url);
        match self.get_auth_header() {
            Some(header) => req.header(header::AUTHORIZATION, header),
            None => req,
        }
    }

    /// 저장된 Bearer 토큰을 Authorization 헤더에 자동 주입한 GET 요청을 반환합니다.
    /// 인증이 필요한 API Command에서 사용합니다.
    #[allow(dead_code)]
    pub fn get_with_auth(&self, url: &str) -> reqwest::RequestBuilder {
        let req = self.client.get(url);
        match self.get_auth_header() {
            Some(header) => req.header(header::AUTHORIZATION, header),
            None => req,
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
