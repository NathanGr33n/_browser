use reqwest::blocking::Client;
use std::time::Duration;
use url::Url;

/// HTTP client for fetching web resources
pub struct HttpClient {
    client: Client,
}

/// Response from an HTTP request
#[derive(Debug)]
pub struct Response {
    pub url: Url,
    pub status: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

/// Network errors
#[derive(Debug)]
pub enum NetError {
    InvalidUrl(String),
    RequestFailed(String),
    Timeout,
    NetworkError(String),
}

impl std::fmt::Display for NetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            NetError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
            NetError::Timeout => write!(f, "Request timed out"),
            NetError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for NetError {}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Result<Self, NetError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("BrowserEngine/0.1.0")
            .build()
            .map_err(|e| NetError::NetworkError(e.to_string()))?;

        Ok(Self { client })
    }

    /// Fetch a resource from a URL
    pub fn fetch(&self, url: &str) -> Result<Response, NetError> {
        // Parse URL
        let parsed_url = Url::parse(url)
            .map_err(|e| NetError::InvalidUrl(e.to_string()))?;

        // Make request
        let response = self.client
            .get(parsed_url.clone())
            .send()
            .map_err(|e| NetError::RequestFailed(e.to_string()))?;

        // Get status and content type
        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Read body
        let body = response
            .bytes()
            .map_err(|e| NetError::RequestFailed(e.to_string()))?
            .to_vec();

        Ok(Response {
            url: parsed_url,
            status,
            content_type,
            body,
        })
    }

    /// Fetch and return as UTF-8 string (for HTML/CSS)
    pub fn fetch_text(&self, url: &str) -> Result<String, NetError> {
        let response = self.fetch(url)?;
        
        String::from_utf8(response.body)
            .map_err(|e| NetError::NetworkError(format!("Invalid UTF-8: {}", e)))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

/// Navigation history manager
pub struct Navigator {
    history: Vec<Url>,
    current_index: usize,
}

impl Navigator {
    /// Create a new navigator
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_index: 0,
        }
    }

    /// Navigate to a URL
    pub fn navigate_to(&mut self, url: Url) {
        // Remove any forward history
        self.history.truncate(self.current_index + 1);
        
        // Add new URL
        self.history.push(url);
        self.current_index = self.history.len() - 1;
    }

    /// Go back in history
    pub fn back(&mut self) -> Option<&Url> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.history[self.current_index])
        } else {
            None
        }
    }

    /// Go forward in history
    pub fn forward(&mut self) -> Option<&Url> {
        if self.current_index + 1 < self.history.len() {
            self.current_index += 1;
            Some(&self.history[self.current_index])
        } else {
            None
        }
    }

    /// Get current URL
    pub fn current(&self) -> Option<&Url> {
        self.history.get(self.current_index)
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        self.current_index + 1 < self.history.len()
    }
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigator_basic() {
        let mut nav = Navigator::new();
        assert!(nav.current().is_none());
        assert!(!nav.can_go_back());
        assert!(!nav.can_go_forward());
    }

    #[test]
    fn test_navigator_navigation() {
        let mut nav = Navigator::new();
        
        let url1 = Url::parse("http://example.com").unwrap();
        let url2 = Url::parse("http://example.com/page2").unwrap();
        
        nav.navigate_to(url1.clone());
        assert_eq!(nav.current().unwrap(), &url1);
        assert!(!nav.can_go_back());
        
        nav.navigate_to(url2.clone());
        assert_eq!(nav.current().unwrap(), &url2);
        assert!(nav.can_go_back());
        
        nav.back();
        assert_eq!(nav.current().unwrap(), &url1);
        assert!(nav.can_go_forward());
        
        nav.forward();
        assert_eq!(nav.current().unwrap(), &url2);
    }

    #[test]
    fn test_navigator_truncate_on_new_navigation() {
        let mut nav = Navigator::new();
        
        let url1 = Url::parse("http://example.com/1").unwrap();
        let url2 = Url::parse("http://example.com/2").unwrap();
        let url3 = Url::parse("http://example.com/3").unwrap();
        
        nav.navigate_to(url1);
        nav.navigate_to(url2);
        nav.back();
        
        // Navigate to new URL should clear forward history
        nav.navigate_to(url3.clone());
        assert_eq!(nav.current().unwrap(), &url3);
        assert!(!nav.can_go_forward());
    }
}
