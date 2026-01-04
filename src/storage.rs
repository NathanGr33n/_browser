// Storage APIs - Phase 7 Task 4

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Storage quota limit (5MB for localStorage, 5MB for sessionStorage)
const STORAGE_QUOTA: usize = 5 * 1024 * 1024;

/// LocalStorage - persistent key-value storage
pub struct LocalStorage {
    /// Data store
    data: HashMap<String, String>,
    /// Current size in bytes
    size: usize,
    /// Storage quota
    quota: usize,
}

impl LocalStorage {
    /// Create a new LocalStorage
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            size: 0,
            quota: STORAGE_QUOTA,
        }
    }
    
    /// Set an item
    pub fn set_item(&mut self, key: String, value: String) -> Result<(), StorageError> {
        let key_size = key.as_bytes().len();
        let value_size = value.as_bytes().len();
        let new_size = key_size + value_size;
        
        // Check if key exists to calculate size difference
        let existing_size = self.data.get(&key)
            .map(|v| key.as_bytes().len() + v.as_bytes().len())
            .unwrap_or(0);
        
        let size_delta = new_size as i64 - existing_size as i64;
        let new_total_size = (self.size as i64 + size_delta) as usize;
        
        if new_total_size > self.quota {
            return Err(StorageError::QuotaExceeded);
        }
        
        self.data.insert(key, value);
        self.size = new_total_size;
        
        Ok(())
    }
    
    /// Get an item
    pub fn get_item(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
    
    /// Remove an item
    pub fn remove_item(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.data.remove(key) {
            let size = key.as_bytes().len() + value.as_bytes().len();
            self.size = self.size.saturating_sub(size);
            Some(value)
        } else {
            None
        }
    }
    
    /// Clear all items
    pub fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }
    
    /// Get number of items
    pub fn length(&self) -> usize {
        self.data.len()
    }
    
    /// Get key at index
    pub fn key(&self, index: usize) -> Option<String> {
        self.data.keys().nth(index).cloned()
    }
    
    /// Get current size in bytes
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Get remaining quota
    pub fn remaining_quota(&self) -> usize {
        self.quota.saturating_sub(self.size)
    }
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// SessionStorage - per-tab key-value storage (cleared on tab close)
pub struct SessionStorage {
    /// Data store
    data: HashMap<String, String>,
    /// Current size in bytes
    size: usize,
    /// Storage quota
    quota: usize,
}

impl SessionStorage {
    /// Create a new SessionStorage
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            size: 0,
            quota: STORAGE_QUOTA,
        }
    }
    
    /// Set an item
    pub fn set_item(&mut self, key: String, value: String) -> Result<(), StorageError> {
        let key_size = key.as_bytes().len();
        let value_size = value.as_bytes().len();
        let new_size = key_size + value_size;
        
        let existing_size = self.data.get(&key)
            .map(|v| key.as_bytes().len() + v.as_bytes().len())
            .unwrap_or(0);
        
        let size_delta = new_size as i64 - existing_size as i64;
        let new_total_size = (self.size as i64 + size_delta) as usize;
        
        if new_total_size > self.quota {
            return Err(StorageError::QuotaExceeded);
        }
        
        self.data.insert(key, value);
        self.size = new_total_size;
        
        Ok(())
    }
    
    /// Get an item
    pub fn get_item(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
    
    /// Remove an item
    pub fn remove_item(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.data.remove(key) {
            let size = key.as_bytes().len() + value.as_bytes().len();
            self.size = self.size.saturating_sub(size);
            Some(value)
        } else {
            None
        }
    }
    
    /// Clear all items
    pub fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }
    
    /// Get number of items
    pub fn length(&self) -> usize {
        self.data.len()
    }
    
    /// Get key at index
    pub fn key(&self, index: usize) -> Option<String> {
        self.data.keys().nth(index).cloned()
    }
}

impl Default for SessionStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Cookie with attributes
#[derive(Debug, Clone)]
pub struct Cookie {
    /// Cookie name
    pub name: String,
    /// Cookie value
    pub value: String,
    /// Expiration time
    pub expires: Option<SystemTime>,
    /// Max age in seconds
    pub max_age: Option<Duration>,
    /// Domain
    pub domain: Option<String>,
    /// Path
    pub path: String,
    /// Secure flag
    pub secure: bool,
    /// HttpOnly flag
    pub http_only: bool,
    /// SameSite attribute
    pub same_site: SameSite,
}

/// SameSite attribute values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Cookie {
    /// Create a new cookie
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            expires: None,
            max_age: None,
            domain: None,
            path: "/".to_string(),
            secure: false,
            http_only: false,
            same_site: SameSite::Lax,
        }
    }
    
    /// Check if cookie is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            SystemTime::now() > expires
        } else if let Some(max_age) = self.max_age {
            // Would need creation time to check max_age properly
            // For simplicity, we'll just use expires
            false
        } else {
            false
        }
    }
    
    /// Check if cookie matches domain
    pub fn matches_domain(&self, domain: &str) -> bool {
        if let Some(cookie_domain) = &self.domain {
            domain.ends_with(cookie_domain)
        } else {
            true
        }
    }
    
    /// Check if cookie matches path
    pub fn matches_path(&self, path: &str) -> bool {
        path.starts_with(&self.path)
    }
}

/// Cookie jar for managing cookies
pub struct CookieJar {
    /// Cookies storage
    cookies: HashMap<String, Cookie>,
}

impl CookieJar {
    /// Create a new cookie jar
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }
    
    /// Set a cookie
    pub fn set_cookie(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }
    
    /// Get a cookie by name
    pub fn get_cookie(&self, name: &str) -> Option<&Cookie> {
        let cookie = self.cookies.get(name)?;
        if cookie.is_expired() {
            return None;
        }
        Some(cookie)
    }
    
    /// Get all cookies for a domain and path
    pub fn get_cookies_for_request(&self, domain: &str, path: &str, secure: bool) -> Vec<&Cookie> {
        self.cookies
            .values()
            .filter(|c| {
                !c.is_expired() &&
                c.matches_domain(domain) &&
                c.matches_path(path) &&
                (!c.secure || secure)
            })
            .collect()
    }
    
    /// Remove a cookie
    pub fn remove_cookie(&mut self, name: &str) -> Option<Cookie> {
        self.cookies.remove(name)
    }
    
    /// Clear all cookies
    pub fn clear(&mut self) {
        self.cookies.clear();
    }
    
    /// Remove expired cookies
    pub fn purge_expired(&mut self) {
        self.cookies.retain(|_, cookie| !cookie.is_expired());
    }
    
    /// Get all cookie names
    pub fn names(&self) -> Vec<String> {
        self.cookies.keys().cloned().collect()
    }
}

impl Default for CookieJar {
    fn default() -> Self {
        Self::new()
    }
}

/// Storage error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    /// Quota exceeded
    QuotaExceeded,
    /// Invalid key
    InvalidKey,
    /// Invalid value
    InvalidValue,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StorageError::QuotaExceeded => write!(f, "Storage quota exceeded"),
            StorageError::InvalidKey => write!(f, "Invalid storage key"),
            StorageError::InvalidValue => write!(f, "Invalid storage value"),
        }
    }
}

impl std::error::Error for StorageError {}

/// Storage event (fired when storage changes)
#[derive(Debug, Clone)]
pub struct StorageEvent {
    /// Storage key
    pub key: Option<String>,
    /// Old value
    pub old_value: Option<String>,
    /// New value
    pub new_value: Option<String>,
    /// Storage area type
    pub storage_area: StorageArea,
}

/// Storage area type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageArea {
    Local,
    Session,
}

/// Storage manager coordinating all storage types
pub struct StorageManager {
    /// LocalStorage instance
    local_storage: LocalStorage,
    /// SessionStorage instance
    session_storage: SessionStorage,
    /// Cookie jar
    cookie_jar: CookieJar,
    /// Storage event listeners
    event_listeners: Vec<Box<dyn Fn(&StorageEvent)>>,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new() -> Self {
        Self {
            local_storage: LocalStorage::new(),
            session_storage: SessionStorage::new(),
            cookie_jar: CookieJar::new(),
            event_listeners: Vec::new(),
        }
    }
    
    /// Get LocalStorage
    pub fn local_storage(&mut self) -> &mut LocalStorage {
        &mut self.local_storage
    }
    
    /// Get SessionStorage
    pub fn session_storage(&mut self) -> &mut SessionStorage {
        &mut self.session_storage
    }
    
    /// Get CookieJar
    pub fn cookie_jar(&mut self) -> &mut CookieJar {
        &mut self.cookie_jar
    }
    
    /// Add storage event listener
    pub fn add_storage_listener<F>(&mut self, listener: F)
    where
        F: Fn(&StorageEvent) + 'static,
    {
        self.event_listeners.push(Box::new(listener));
    }
    
    /// Fire storage event
    pub fn fire_storage_event(&self, event: StorageEvent) {
        for listener in &self.event_listeners {
            listener(&event);
        }
    }
    
    /// Set item in LocalStorage and fire event
    pub fn set_local_item(&mut self, key: String, value: String) -> Result<(), StorageError> {
        let old_value = self.local_storage.get_item(&key);
        self.local_storage.set_item(key.clone(), value.clone())?;
        
        self.fire_storage_event(StorageEvent {
            key: Some(key),
            old_value,
            new_value: Some(value),
            storage_area: StorageArea::Local,
        });
        
        Ok(())
    }
    
    /// Set item in SessionStorage and fire event
    pub fn set_session_item(&mut self, key: String, value: String) -> Result<(), StorageError> {
        let old_value = self.session_storage.get_item(&key);
        self.session_storage.set_item(key.clone(), value.clone())?;
        
        self.fire_storage_event(StorageEvent {
            key: Some(key),
            old_value,
            new_value: Some(value),
            storage_area: StorageArea::Session,
        });
        
        Ok(())
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_local_storage_basic() {
        let mut storage = LocalStorage::new();
        
        storage.set_item("key1".to_string(), "value1".to_string()).unwrap();
        assert_eq!(storage.get_item("key1"), Some("value1".to_string()));
        assert_eq!(storage.length(), 1);
    }
    
    #[test]
    fn test_local_storage_update() {
        let mut storage = LocalStorage::new();
        
        storage.set_item("key1".to_string(), "value1".to_string()).unwrap();
        storage.set_item("key1".to_string(), "value2".to_string()).unwrap();
        
        assert_eq!(storage.get_item("key1"), Some("value2".to_string()));
        assert_eq!(storage.length(), 1);
    }
    
    #[test]
    fn test_local_storage_remove() {
        let mut storage = LocalStorage::new();
        
        storage.set_item("key1".to_string(), "value1".to_string()).unwrap();
        let removed = storage.remove_item("key1");
        
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(storage.get_item("key1"), None);
        assert_eq!(storage.length(), 0);
    }
    
    #[test]
    fn test_local_storage_clear() {
        let mut storage = LocalStorage::new();
        
        storage.set_item("key1".to_string(), "value1".to_string()).unwrap();
        storage.set_item("key2".to_string(), "value2".to_string()).unwrap();
        storage.clear();
        
        assert_eq!(storage.length(), 0);
        assert_eq!(storage.size(), 0);
    }
    
    #[test]
    fn test_local_storage_quota() {
        let mut storage = LocalStorage::new();
        
        // Try to exceed quota
        let large_value = "x".repeat(6 * 1024 * 1024); // 6MB
        let result = storage.set_item("key".to_string(), large_value);
        
        assert_eq!(result, Err(StorageError::QuotaExceeded));
    }
    
    #[test]
    fn test_session_storage() {
        let mut storage = SessionStorage::new();
        
        storage.set_item("key1".to_string(), "value1".to_string()).unwrap();
        assert_eq!(storage.get_item("key1"), Some("value1".to_string()));
        
        storage.clear();
        assert_eq!(storage.length(), 0);
    }
    
    #[test]
    fn test_cookie_basic() {
        let cookie = Cookie::new("name".to_string(), "value".to_string());
        
        assert_eq!(cookie.name, "name");
        assert_eq!(cookie.value, "value");
        assert!(!cookie.is_expired());
    }
    
    #[test]
    fn test_cookie_jar() {
        let mut jar = CookieJar::new();
        
        let cookie = Cookie::new("session_id".to_string(), "abc123".to_string());
        jar.set_cookie(cookie);
        
        assert!(jar.get_cookie("session_id").is_some());
        assert_eq!(jar.get_cookie("session_id").unwrap().value, "abc123");
    }
    
    #[test]
    fn test_cookie_domain_matching() {
        let mut cookie = Cookie::new("test".to_string(), "value".to_string());
        cookie.domain = Some("example.com".to_string());
        
        assert!(cookie.matches_domain("example.com"));
        assert!(cookie.matches_domain("sub.example.com"));
        assert!(!cookie.matches_domain("other.com"));
    }
    
    #[test]
    fn test_cookie_path_matching() {
        let mut cookie = Cookie::new("test".to_string(), "value".to_string());
        cookie.path = "/api".to_string();
        
        assert!(cookie.matches_path("/api"));
        assert!(cookie.matches_path("/api/users"));
        assert!(!cookie.matches_path("/other"));
    }
    
    #[test]
    fn test_storage_manager() {
        let mut manager = StorageManager::new();
        
        manager.set_local_item("key1".to_string(), "value1".to_string()).unwrap();
        manager.set_session_item("key2".to_string(), "value2".to_string()).unwrap();
        
        assert_eq!(manager.local_storage().get_item("key1"), Some("value1".to_string()));
        assert_eq!(manager.session_storage().get_item("key2"), Some("value2".to_string()));
    }
    
    #[test]
    fn test_cookie_jar_request_filtering() {
        let mut jar = CookieJar::new();
        
        let mut cookie1 = Cookie::new("test1".to_string(), "value1".to_string());
        cookie1.domain = Some("example.com".to_string());
        cookie1.path = "/api".to_string();
        
        let mut cookie2 = Cookie::new("test2".to_string(), "value2".to_string());
        cookie2.domain = Some("other.com".to_string());
        
        jar.set_cookie(cookie1);
        jar.set_cookie(cookie2);
        
        let cookies = jar.get_cookies_for_request("example.com", "/api/users", false);
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies[0].name, "test1");
    }
}
