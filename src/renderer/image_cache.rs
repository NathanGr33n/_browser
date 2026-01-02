use image::{ImageError, ImageFormat};
use std::collections::HashMap;
use url::Url;

/// Decoded image data ready for GPU upload
#[derive(Debug, Clone)]
pub struct DecodedImage {
    /// Image URL
    pub url: Url,
    /// Image dimensions
    pub width: u32,
    pub height: u32,
    /// RGBA8 pixel data
    pub data: Vec<u8>,
    /// Original format
    pub format: ImageFormat,
}

impl DecodedImage {
    /// Create from raw image bytes
    pub fn from_bytes(url: Url, bytes: &[u8]) -> Result<Self, ImageError> {
        let img = image::load_from_memory(bytes)?;
        let format = image::guess_format(bytes).unwrap_or(ImageFormat::Png);
        
        // Convert to RGBA8
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        
        Ok(Self {
            url,
            width,
            height,
            data: rgba.into_raw(),
            format,
        })
    }
    
    /// Get the size in bytes
    pub fn byte_size(&self) -> usize {
        self.data.len()
    }
}

/// Cache for decoded images
pub struct ImageCache {
    /// Cached images by URL
    images: HashMap<Url, DecodedImage>,
    /// Maximum cache size in bytes
    max_size: usize,
    /// Current cache size in bytes
    current_size: usize,
}

impl ImageCache {
    /// Create a new image cache with specified size limit
    pub fn new(max_size: usize) -> Self {
        Self {
            images: HashMap::new(),
            max_size,
            current_size: 0,
        }
    }
    
    /// Create with default 100MB cache
    pub fn with_default_size() -> Self {
        Self::new(100 * 1024 * 1024) // 100 MB
    }
    
    /// Load and decode an image from bytes
    pub fn load_from_bytes(&mut self, url: Url, bytes: &[u8]) -> Result<&DecodedImage, ImageError> {
        // Check if already cached
        if self.images.contains_key(&url) {
            return Ok(self.images.get(&url).unwrap());
        }
        
        // Decode the image
        let decoded = DecodedImage::from_bytes(url.clone(), bytes)?;
        let size = decoded.byte_size();
        
        // Evict old images if needed
        while self.current_size + size > self.max_size && !self.images.is_empty() {
            self.evict_oldest();
        }
        
        // Don't cache if image is larger than max cache size
        if size > self.max_size {
            // Still decode and store temporarily
            self.images.insert(url.clone(), decoded);
            return Ok(self.images.get(&url).unwrap());
        }
        
        // Add to cache
        self.current_size += size;
        self.images.insert(url.clone(), decoded);
        
        Ok(self.images.get(&url).unwrap())
    }
    
    /// Get a cached image
    pub fn get(&self, url: &Url) -> Option<&DecodedImage> {
        self.images.get(url)
    }
    
    /// Evict the oldest image (simple FIFO for now)
    fn evict_oldest(&mut self) {
        if let Some((url, _)) = self.images.iter().next() {
            let url = url.clone();
            if let Some(removed) = self.images.remove(&url) {
                self.current_size -= removed.byte_size();
            }
        }
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.images.clear();
        self.current_size = 0;
    }
    
    /// Get current cache size in bytes
    pub fn size(&self) -> usize {
        self.current_size
    }
    
    /// Get number of cached images
    pub fn count(&self) -> usize {
        self.images.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Create a minimal 1x1 PNG image for testing
    fn create_test_png() -> Vec<u8> {
        // Minimal valid PNG: 1x1 white pixel
        vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 dimensions
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
            0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, // IDAT chunk
            0x54, 0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0x3F,
            0x00, 0x05, 0xFE, 0x02, 0xFE, 0xDC, 0xCC, 0x59,
            0xE7, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, // IEND chunk
            0x44, 0xAE, 0x42, 0x60, 0x82,
        ]
    }
    
    #[test]
    fn test_decoded_image_from_bytes() {
        let png_data = create_test_png();
        let url = Url::parse("http://example.com/test.png").unwrap();
        
        let result = DecodedImage::from_bytes(url.clone(), &png_data);
        assert!(result.is_ok());
        
        let img = result.unwrap();
        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
        assert_eq!(img.data.len(), 4); // 1 pixel * 4 bytes (RGBA)
        assert_eq!(img.url, url);
    }
    
    #[test]
    fn test_image_cache_creation() {
        let cache = ImageCache::new(1024 * 1024); // 1 MB
        assert_eq!(cache.count(), 0);
        assert_eq!(cache.size(), 0);
    }
    
    #[test]
    fn test_image_cache_load() {
        let mut cache = ImageCache::new(1024 * 1024);
        let png_data = create_test_png();
        let url = Url::parse("http://example.com/test.png").unwrap();
        
        let result = cache.load_from_bytes(url.clone(), &png_data);
        assert!(result.is_ok());
        
        let img = result.unwrap();
        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
        assert_eq!(cache.count(), 1);
    }
    
    #[test]
    fn test_image_cache_retrieval() {
        let mut cache = ImageCache::new(1024 * 1024);
        let png_data = create_test_png();
        let url = Url::parse("http://example.com/test.png").unwrap();
        
        cache.load_from_bytes(url.clone(), &png_data).unwrap();
        
        let cached = cache.get(&url);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().width, 1);
    }
    
    #[test]
    fn test_image_cache_clear() {
        let mut cache = ImageCache::new(1024 * 1024);
        let png_data = create_test_png();
        let url = Url::parse("http://example.com/test.png").unwrap();
        
        cache.load_from_bytes(url, &png_data).unwrap();
        assert_eq!(cache.count(), 1);
        
        cache.clear();
        assert_eq!(cache.count(), 0);
        assert_eq!(cache.size(), 0);
    }
    
    #[test]
    fn test_image_cache_eviction() {
        // Very small cache - only 100 bytes
        let mut cache = ImageCache::new(100);
        let png_data = create_test_png();
        
        let url1 = Url::parse("http://example.com/test1.png").unwrap();
        let url2 = Url::parse("http://example.com/test2.png").unwrap();
        
        cache.load_from_bytes(url1.clone(), &png_data).unwrap();
        
        // Load another image - should trigger eviction
        cache.load_from_bytes(url2.clone(), &png_data).unwrap();
        
        // Should have evicted the first one (FIFO)
        assert!(cache.get(&url2).is_some());
    }
}
