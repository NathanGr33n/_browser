use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use url::Url;

use super::{HttpClient, NetError};

/// Represents a resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Html,
    Css,
    Image,
    Font,
    Other,
}

impl ResourceType {
    /// Determine resource type from content-type header or URL extension
    pub fn from_content_type(content_type: &str) -> Self {
        let lower = content_type.to_lowercase();
        if lower.contains("text/html") {
            ResourceType::Html
        } else if lower.contains("text/css") {
            ResourceType::Css
        } else if lower.contains("image/") {
            ResourceType::Image
        } else if lower.contains("font/") || lower.contains("application/font") {
            ResourceType::Font
        } else {
            ResourceType::Other
        }
    }

    /// Determine resource type from file extension
    pub fn from_extension(url: &Url) -> Self {
        if let Some(ext) = url.path().split('.').last() {
            match ext.to_lowercase().as_str() {
                "html" | "htm" => ResourceType::Html,
                "css" => ResourceType::Css,
                "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => ResourceType::Image,
                "woff" | "woff2" | "ttf" | "otf" => ResourceType::Font,
                _ => ResourceType::Other,
            }
        } else {
            ResourceType::Other
        }
    }
}

/// Cached resource data
#[derive(Debug, Clone)]
pub struct CachedResource {
    pub url: Url,
    pub resource_type: ResourceType,
    pub content_type: String,
    pub data: Vec<u8>,
    /// Timestamp for LRU eviction (system time in seconds)
    pub last_accessed: u64,
}

impl CachedResource {
    /// Get the resource as a UTF-8 string
    pub fn as_text(&self) -> Result<String, NetError> {
        String::from_utf8(self.data.clone())
            .map_err(|e| NetError::ParseError(format!("Invalid UTF-8: {}", e)))
    }
}

/// Resource loader with LRU caching
pub struct ResourceLoader {
    client: HttpClient,
    cache: Arc<Mutex<ResourceCache>>,
}

impl ResourceLoader {
    /// Create a new resource loader with specified cache capacity (in bytes)
    pub fn new(cache_size: usize) -> Self {
        Self {
            client: HttpClient::new(),
            cache: Arc::new(Mutex::new(ResourceCache::new(cache_size))),
        }
    }

    /// Create a resource loader with default 50MB cache
    pub fn with_default_cache() -> Self {
        Self::new(50 * 1024 * 1024) // 50 MB
    }

    /// Load a resource, using cache if available
    pub fn load(&self, url: &Url) -> Result<CachedResource, NetError> {
        // Check cache first
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(resource) = cache.get(url) {
                return Ok(resource);
            }
        }

        // Fetch from network
        let response = self.client.fetch(url)?;
        
        // Determine resource type
        let resource_type = if !response.content_type.is_empty() {
            ResourceType::from_content_type(&response.content_type)
        } else {
            ResourceType::from_extension(&response.url)
        };

        // Create cached resource
        let resource = CachedResource {
            url: response.url.clone(),
            resource_type,
            content_type: response.content_type,
            data: response.body,
            last_accessed: current_timestamp(),
        };

        // Store in cache
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(response.url, resource.clone());
        }

        Ok(resource)
    }

    /// Load a resource and return as UTF-8 text
    pub fn load_text(&self, url: &Url) -> Result<String, NetError> {
        let resource = self.load(url)?;
        resource.as_text()
    }

    /// Clear the cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get current cache size in bytes
    pub fn cache_size(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.current_size
    }

    /// Get number of cached resources
    pub fn cache_count(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.entries.len()
    }
}

/// Internal LRU cache implementation
struct ResourceCache {
    /// Maximum cache size in bytes
    max_size: usize,
    /// Current cache size in bytes
    current_size: usize,
    /// Cached resources
    entries: HashMap<Url, CachedResource>,
}

impl ResourceCache {
    fn new(max_size: usize) -> Self {
        Self {
            max_size,
            current_size: 0,
            entries: HashMap::new(),
        }
    }

    fn get(&mut self, url: &Url) -> Option<CachedResource> {
        if let Some(resource) = self.entries.get_mut(url) {
            // Update last accessed time
            resource.last_accessed = current_timestamp();
            Some(resource.clone())
        } else {
            None
        }
    }

    fn put(&mut self, url: Url, resource: CachedResource) {
        let resource_size = resource.data.len();

        // Evict old entries if needed
        while self.current_size + resource_size > self.max_size && !self.entries.is_empty() {
            self.evict_lru();
        }

        // Don't cache if resource is larger than max cache size
        if resource_size > self.max_size {
            return;
        }

        // Remove old entry if exists
        if let Some(old) = self.entries.remove(&url) {
            self.current_size -= old.data.len();
        }

        // Add new entry
        self.current_size += resource_size;
        self.entries.insert(url, resource);
    }

    fn evict_lru(&mut self) {
        // Find the least recently used entry
        if let Some((url, _)) = self
            .entries
            .iter()
            .min_by_key(|(_, resource)| resource.last_accessed)
        {
            let url = url.clone();
            if let Some(removed) = self.entries.remove(&url) {
                self.current_size -= removed.data.len();
            }
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.current_size = 0;
    }
}

/// Get current timestamp in seconds
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_from_content_type() {
        assert_eq!(
            ResourceType::from_content_type("text/html; charset=utf-8"),
            ResourceType::Html
        );
        assert_eq!(
            ResourceType::from_content_type("text/css"),
            ResourceType::Css
        );
        assert_eq!(
            ResourceType::from_content_type("image/png"),
            ResourceType::Image
        );
        assert_eq!(
            ResourceType::from_content_type("font/woff2"),
            ResourceType::Font
        );
        assert_eq!(
            ResourceType::from_content_type("application/json"),
            ResourceType::Other
        );
    }

    #[test]
    fn test_resource_type_from_extension() {
        let html_url = Url::parse("https://example.com/page.html").unwrap();
        assert_eq!(ResourceType::from_extension(&html_url), ResourceType::Html);

        let css_url = Url::parse("https://example.com/style.css").unwrap();
        assert_eq!(ResourceType::from_extension(&css_url), ResourceType::Css);

        let img_url = Url::parse("https://example.com/image.png").unwrap();
        assert_eq!(ResourceType::from_extension(&img_url), ResourceType::Image);

        let font_url = Url::parse("https://example.com/font.woff2").unwrap();
        assert_eq!(ResourceType::from_extension(&font_url), ResourceType::Font);
    }

    #[test]
    fn test_resource_cache_basic() {
        let mut cache = ResourceCache::new(1024);
        let url = Url::parse("https://example.com/test").unwrap();
        let resource = CachedResource {
            url: url.clone(),
            resource_type: ResourceType::Html,
            content_type: "text/html".to_string(),
            data: vec![1, 2, 3, 4],
            last_accessed: current_timestamp(),
        };

        cache.put(url.clone(), resource.clone());
        assert_eq!(cache.current_size, 4);
        assert_eq!(cache.entries.len(), 1);

        let retrieved = cache.get(&url).unwrap();
        assert_eq!(retrieved.data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_resource_cache_eviction() {
        let mut cache = ResourceCache::new(10); // Very small cache

        let url1 = Url::parse("https://example.com/1").unwrap();
        let url2 = Url::parse("https://example.com/2").unwrap();
        let url3 = Url::parse("https://example.com/3").unwrap();

        // Add first resource
        cache.put(
            url1.clone(),
            CachedResource {
                url: url1.clone(),
                resource_type: ResourceType::Html,
                content_type: "text/html".to_string(),
                data: vec![1, 2, 3, 4], // 4 bytes
                last_accessed: 100,
            },
        );

        // Add second resource
        cache.put(
            url2.clone(),
            CachedResource {
                url: url2.clone(),
                resource_type: ResourceType::Html,
                content_type: "text/html".to_string(),
                data: vec![5, 6], // 2 bytes
                last_accessed: 200,
            },
        );

        // Both should fit (4 + 2 = 6 bytes)
        assert_eq!(cache.entries.len(), 2);

        // Add third resource that requires eviction
        cache.put(
            url3.clone(),
            CachedResource {
                url: url3.clone(),
                resource_type: ResourceType::Html,
                content_type: "text/html".to_string(),
                data: vec![7, 8, 9, 10, 11], // 5 bytes
                last_accessed: 300,
            },
        );

        // Should have evicted url1 (oldest) to make room
        assert!(cache.get(&url1).is_none());
        assert!(cache.get(&url2).is_some());
        assert!(cache.get(&url3).is_some());
    }

    #[test]
    fn test_resource_cache_clear() {
        let mut cache = ResourceCache::new(1024);
        let url = Url::parse("https://example.com/test").unwrap();
        
        cache.put(
            url.clone(),
            CachedResource {
                url: url.clone(),
                resource_type: ResourceType::Html,
                content_type: "text/html".to_string(),
                data: vec![1, 2, 3],
                last_accessed: current_timestamp(),
            },
        );

        assert_eq!(cache.entries.len(), 1);
        assert_eq!(cache.current_size, 3);

        cache.clear();
        assert_eq!(cache.entries.len(), 0);
        assert_eq!(cache.current_size, 0);
    }
}
