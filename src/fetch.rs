// Fetch API - Phase 8 Advanced JavaScript

use std::collections::HashMap;

/// Fetch a resource from the network
pub async fn fetch(input: RequestInfo, init: Option<RequestInit>) -> Result<Response, FetchError> {
    let request = match input {
        RequestInfo::Url(url) => {
            let mut req = Request::new(url, init.as_ref().and_then(|i| i.method.clone()).unwrap_or(Method::Get));
            if let Some(init) = init {
                req.apply_init(init)?;
            }
            req
        }
        RequestInfo::Request(req) => req,
    };
    
    // In production, would use reqwest or similar
    // For now, simulate successful response
    Ok(Response::new(
        200,
        "OK".to_string(),
        Headers::new(),
        vec![],
        request.url.clone(),
    ))
}

/// Request information for fetch
#[derive(Debug, Clone)]
pub enum RequestInfo {
    Url(String),
    Request(Request),
}

/// Fetch Request
#[derive(Debug, Clone)]
pub struct Request {
    /// URL
    pub url: String,
    /// HTTP method
    pub method: Method,
    /// Headers
    pub headers: Headers,
    /// Body
    pub body: Option<Vec<u8>>,
    /// Mode
    pub mode: RequestMode,
    /// Credentials
    pub credentials: RequestCredentials,
    /// Cache mode
    pub cache: RequestCache,
    /// Redirect mode
    pub redirect: RequestRedirect,
    /// Referrer
    pub referrer: String,
    /// Integrity
    pub integrity: String,
}

impl Request {
    /// Create a new request
    pub fn new(url: String, method: Method) -> Self {
        Self {
            url,
            method,
            headers: Headers::new(),
            body: None,
            mode: RequestMode::Cors,
            credentials: RequestCredentials::SameOrigin,
            cache: RequestCache::Default,
            redirect: RequestRedirect::Follow,
            referrer: "about:client".to_string(),
            integrity: String::new(),
        }
    }
    
    /// Apply request init
    pub fn apply_init(&mut self, init: RequestInit) -> Result<(), FetchError> {
        if let Some(method) = init.method {
            self.method = method;
        }
        if let Some(headers) = init.headers {
            self.headers = headers;
        }
        if let Some(body) = init.body {
            self.body = Some(body);
        }
        if let Some(mode) = init.mode {
            self.mode = mode;
        }
        if let Some(credentials) = init.credentials {
            self.credentials = credentials;
        }
        if let Some(cache) = init.cache {
            self.cache = cache;
        }
        if let Some(redirect) = init.redirect {
            self.redirect = redirect;
        }
        if let Some(referrer) = init.referrer {
            self.referrer = referrer;
        }
        if let Some(integrity) = init.integrity {
            self.integrity = integrity;
        }
        Ok(())
    }
    
    /// Clone the request
    pub fn clone_request(&self) -> Self {
        self.clone()
    }
}

/// Request initialization options
#[derive(Debug, Clone, Default)]
pub struct RequestInit {
    pub method: Option<Method>,
    pub headers: Option<Headers>,
    pub body: Option<Vec<u8>>,
    pub mode: Option<RequestMode>,
    pub credentials: Option<RequestCredentials>,
    pub cache: Option<RequestCache>,
    pub redirect: Option<RequestRedirect>,
    pub referrer: Option<String>,
    pub integrity: Option<String>,
}

/// HTTP method
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

/// Request mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestMode {
    SameOrigin,
    Cors,
    NoCors,
    Navigate,
}

/// Request credentials
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestCredentials {
    Omit,
    SameOrigin,
    Include,
}

/// Request cache mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestCache {
    Default,
    NoStore,
    Reload,
    NoCache,
    ForceCache,
    OnlyIfCached,
}

/// Request redirect mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestRedirect {
    Follow,
    Error,
    Manual,
}

/// Fetch Response
#[derive(Debug, Clone)]
pub struct Response {
    /// Status code
    pub status: u16,
    /// Status text
    pub status_text: String,
    /// Headers
    pub headers: Headers,
    /// Body (simplified - would be ReadableStream in real implementation)
    body: Vec<u8>,
    /// URL
    pub url: String,
    /// Redirected flag
    pub redirected: bool,
    /// Type
    pub response_type: ResponseType,
    /// Body used flag
    body_used: bool,
}

impl Response {
    /// Create a new response
    pub fn new(status: u16, status_text: String, headers: Headers, body: Vec<u8>, url: String) -> Self {
        Self {
            status,
            status_text,
            headers,
            body,
            url,
            redirected: false,
            response_type: ResponseType::Basic,
            body_used: false,
        }
    }
    
    /// Check if response is OK (status 200-299)
    pub fn ok(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
    
    /// Read body as text
    pub async fn text(&mut self) -> Result<String, FetchError> {
        if self.body_used {
            return Err(FetchError::BodyUsed);
        }
        self.body_used = true;
        
        String::from_utf8(self.body.clone())
            .map_err(|_| FetchError::InvalidBody)
    }
    
    /// Read body as JSON (simplified - would parse JSON)
    pub async fn json(&mut self) -> Result<String, FetchError> {
        self.text().await
    }
    
    /// Read body as bytes
    pub async fn bytes(&mut self) -> Result<Vec<u8>, FetchError> {
        if self.body_used {
            return Err(FetchError::BodyUsed);
        }
        self.body_used = true;
        Ok(self.body.clone())
    }
    
    /// Clone the response
    pub fn clone_response(&self) -> Result<Self, FetchError> {
        if self.body_used {
            return Err(FetchError::BodyUsed);
        }
        Ok(self.clone())
    }
    
    /// Create an error response
    pub fn error() -> Self {
        Self {
            status: 0,
            status_text: String::new(),
            headers: Headers::new(),
            body: vec![],
            url: String::new(),
            redirected: false,
            response_type: ResponseType::Error,
            body_used: false,
        }
    }
    
    /// Create a redirect response
    pub fn redirect(url: String, status: u16) -> Result<Self, FetchError> {
        if !matches!(status, 301 | 302 | 303 | 307 | 308) {
            return Err(FetchError::InvalidRedirect);
        }
        
        let mut headers = Headers::new();
        headers.set("Location".to_string(), url.clone());
        
        Ok(Self {
            status,
            status_text: "Redirect".to_string(),
            headers,
            body: vec![],
            url,
            redirected: true,
            response_type: ResponseType::Basic,
            body_used: false,
        })
    }
}

/// Response type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseType {
    Basic,
    Cors,
    Error,
    Opaque,
    OpaqueRedirect,
}

/// HTTP Headers
#[derive(Debug, Clone)]
pub struct Headers {
    /// Header map (case-insensitive keys)
    headers: HashMap<String, String>,
}

impl Headers {
    /// Create new headers
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }
    
    /// Append a header value
    pub fn append(&mut self, name: String, value: String) {
        let key = name.to_lowercase();
        if let Some(existing) = self.headers.get_mut(&key) {
            existing.push_str(", ");
            existing.push_str(&value);
        } else {
            self.headers.insert(key, value);
        }
    }
    
    /// Delete a header
    pub fn delete(&mut self, name: &str) {
        self.headers.remove(&name.to_lowercase());
    }
    
    /// Get a header value
    pub fn get(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_lowercase()).map(|s| s.as_str())
    }
    
    /// Check if header exists
    pub fn has(&self, name: &str) -> bool {
        self.headers.contains_key(&name.to_lowercase())
    }
    
    /// Set a header value (replaces existing)
    pub fn set(&mut self, name: String, value: String) {
        self.headers.insert(name.to_lowercase(), value);
    }
    
    /// Get all header names
    pub fn keys(&self) -> Vec<String> {
        self.headers.keys().cloned().collect()
    }
    
    /// Get all header values
    pub fn values(&self) -> Vec<String> {
        self.headers.values().cloned().collect()
    }
    
    /// Iterate over headers
    pub fn entries(&self) -> Vec<(String, String)> {
        self.headers.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

/// Fetch errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    /// Network error
    NetworkError,
    /// Invalid URL
    InvalidUrl,
    /// CORS error
    CorsError,
    /// Body already used
    BodyUsed,
    /// Invalid body
    InvalidBody,
    /// Invalid redirect
    InvalidRedirect,
    /// Timeout
    Timeout,
    /// Aborted
    Aborted,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FetchError::NetworkError => write!(f, "Network error"),
            FetchError::InvalidUrl => write!(f, "Invalid URL"),
            FetchError::CorsError => write!(f, "CORS error"),
            FetchError::BodyUsed => write!(f, "Body already used"),
            FetchError::InvalidBody => write!(f, "Invalid body"),
            FetchError::InvalidRedirect => write!(f, "Invalid redirect status"),
            FetchError::Timeout => write!(f, "Request timeout"),
            FetchError::Aborted => write!(f, "Request aborted"),
        }
    }
}

impl std::error::Error for FetchError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_request_creation() {
        let req = Request::new("https://example.com".to_string(), Method::Get);
        assert_eq!(req.url, "https://example.com");
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.mode, RequestMode::Cors);
    }
    
    #[test]
    fn test_request_with_init() {
        let mut req = Request::new("https://example.com".to_string(), Method::Get);
        
        let mut init = RequestInit::default();
        init.method = Some(Method::Post);
        init.credentials = Some(RequestCredentials::Include);
        
        req.apply_init(init).unwrap();
        
        assert_eq!(req.method, Method::Post);
        assert_eq!(req.credentials, RequestCredentials::Include);
    }
    
    #[test]
    fn test_response_ok() {
        let resp = Response::new(200, "OK".to_string(), Headers::new(), vec![], "https://example.com".to_string());
        assert!(resp.ok());
        
        let resp_bad = Response::new(404, "Not Found".to_string(), Headers::new(), vec![], "https://example.com".to_string());
        assert!(!resp_bad.ok());
    }
    
    #[tokio::test]
    async fn test_response_text() {
        let mut resp = Response::new(
            200,
            "OK".to_string(),
            Headers::new(),
            b"Hello, World!".to_vec(),
            "https://example.com".to_string(),
        );
        
        let text = resp.text().await.unwrap();
        assert_eq!(text, "Hello, World!");
        
        // Body should be consumed
        assert!(resp.text().await.is_err());
    }
    
    #[tokio::test]
    async fn test_response_bytes() {
        let mut resp = Response::new(
            200,
            "OK".to_string(),
            Headers::new(),
            vec![1, 2, 3, 4],
            "https://example.com".to_string(),
        );
        
        let bytes = resp.bytes().await.unwrap();
        assert_eq!(bytes, vec![1, 2, 3, 4]);
    }
    
    #[test]
    fn test_response_clone() {
        let resp = Response::new(200, "OK".to_string(), Headers::new(), vec![1, 2, 3], "https://example.com".to_string());
        
        let cloned = resp.clone_response().unwrap();
        assert_eq!(cloned.status, 200);
        assert_eq!(cloned.body, vec![1, 2, 3]);
    }
    
    #[test]
    fn test_response_redirect() {
        let resp = Response::redirect("https://example.com/new".to_string(), 302).unwrap();
        
        assert_eq!(resp.status, 302);
        assert!(resp.redirected);
        assert_eq!(resp.headers.get("location"), Some("https://example.com/new"));
    }
    
    #[test]
    fn test_invalid_redirect() {
        let result = Response::redirect("https://example.com".to_string(), 200);
        assert_eq!(result, Err(FetchError::InvalidRedirect));
    }
    
    #[test]
    fn test_headers_basic() {
        let mut headers = Headers::new();
        
        headers.set("Content-Type".to_string(), "application/json".to_string());
        assert_eq!(headers.get("content-type"), Some("application/json"));
        assert!(headers.has("Content-Type"));
        
        headers.delete("content-type");
        assert!(!headers.has("content-type"));
    }
    
    #[test]
    fn test_headers_append() {
        let mut headers = Headers::new();
        
        headers.set("Accept".to_string(), "text/html".to_string());
        headers.append("Accept".to_string(), "application/json".to_string());
        
        assert_eq!(headers.get("accept"), Some("text/html, application/json"));
    }
    
    #[test]
    fn test_headers_case_insensitive() {
        let mut headers = Headers::new();
        
        headers.set("Content-Type".to_string(), "text/plain".to_string());
        
        assert_eq!(headers.get("content-type"), Some("text/plain"));
        assert_eq!(headers.get("CONTENT-TYPE"), Some("text/plain"));
        assert_eq!(headers.get("Content-Type"), Some("text/plain"));
    }
    
    #[test]
    fn test_headers_iteration() {
        let mut headers = Headers::new();
        
        headers.set("Content-Type".to_string(), "text/html".to_string());
        headers.set("Accept".to_string(), "application/json".to_string());
        
        let keys = headers.keys();
        assert_eq!(keys.len(), 2);
        
        let entries = headers.entries();
        assert_eq!(entries.len(), 2);
    }
    
    #[tokio::test]
    async fn test_fetch_basic() {
        let result = fetch(
            RequestInfo::Url("https://example.com".to_string()),
            None,
        ).await;
        
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.status, 200);
    }
    
    #[tokio::test]
    async fn test_fetch_with_options() {
        let mut init = RequestInit::default();
        init.method = Some(Method::Post);
        init.credentials = Some(RequestCredentials::Include);
        
        let result = fetch(
            RequestInfo::Url("https://example.com".to_string()),
            Some(init),
        ).await;
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_request_modes() {
        let req = Request::new("https://example.com".to_string(), Method::Get);
        assert_eq!(req.mode, RequestMode::Cors);
        assert_eq!(req.credentials, RequestCredentials::SameOrigin);
        assert_eq!(req.redirect, RequestRedirect::Follow);
    }
}
