// Text rendering module with GPU-accelerated text drawing

use wgpu::{Device, Queue, Texture};

use super::font_manager::FontManager;
use super::glyph_cache::{GlyphCache, GlyphKey};

/// Text renderer with GPU-accelerated text drawing
pub struct TextRenderer {
    /// Font manager for loading fonts
    font_manager: FontManager,
    /// Glyph cache with texture atlas
    glyph_cache: GlyphCache,
    /// GPU texture for glyph atlas
    atlas_texture: Option<Texture>,
}

impl TextRenderer {
    /// Create a new text renderer
    pub fn new() -> Result<Self, String> {
        let font_manager = FontManager::new()
            .map_err(|e| format!("Failed to create font manager: {}", e))?;
        
        // Create glyph cache with 1024x1024 atlas
        let glyph_cache = GlyphCache::new(1024, 1024);
        
        Ok(Self {
            font_manager,
            glyph_cache,
            atlas_texture: None,
        })
    }

    /// Initialize the GPU texture atlas
    pub fn init_atlas(&mut self, device: &Device) {
        let (width, height) = self.glyph_cache.atlas_dimensions();
        
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glyph Atlas"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm, // Single channel for alpha
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.atlas_texture = Some(texture);
    }

    /// Measure text dimensions using actual font metrics
    pub fn measure_text(&mut self, text: &str, font_family: &str, font_size: f32) -> (f32, f32) {
        let font = self.font_manager.get_font(font_family);
        let font_id = self.glyph_cache.register_font(font);
        
        let mut width = 0.0_f32;
        let mut max_height = 0.0_f32;
        
        for ch in text.chars() {
            let key = GlyphKey {
                character: ch,
                size: font_size as u32,
                font_id,
            };
            
            if let Some(glyph) = self.glyph_cache.get_or_rasterize(key) {
                width += glyph.advance;
                max_height = max_height.max(glyph.height as f32);
            }
        }
        
        (width, max_height.max(font_size))
    }

    /// Upload atlas texture to GPU if dirty
    pub fn upload_atlas(&mut self, device: &Device, queue: &Queue) {
        if !self.glyph_cache.is_dirty() {
            return;
        }
        
        // Ensure texture exists
        if self.atlas_texture.is_none() {
            self.init_atlas(device);
        }
        
        if let Some(texture) = &self.atlas_texture {
            let (width, height) = self.glyph_cache.atlas_dimensions();
            let data = self.glyph_cache.atlas_data();
            
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(width),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
            
            self.glyph_cache.mark_clean();
        }
    }

    /// Prepare text for rendering (rasterize glyphs and upload to GPU)
    pub fn prepare_text(
        &mut self,
        device: &Device,
        queue: &Queue,
        font_family: &str,
        texts: &[(String, f32)], // (text, font_size)
    ) {
        let font = self.font_manager.get_font(font_family);
        let font_id = self.glyph_cache.register_font(font);
        
        // Rasterize all glyphs
        for (text, font_size) in texts {
            let size = *font_size as u32;
            for ch in text.chars() {
                let key = GlyphKey {
                    character: ch,
                    size,
                    font_id,
                };
                let _ = self.glyph_cache.get_or_rasterize(key);
            }
        }
        
        // Upload to GPU if needed
        self.upload_atlas(device, queue);
    }

    /// Get the atlas texture for binding
    pub fn atlas_texture(&self) -> Option<&Texture> {
        self.atlas_texture.as_ref()
    }
    
    /// Get the glyph cache
    pub fn glyph_cache(&self) -> &GlyphCache {
        &self.glyph_cache
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to create text renderer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_renderer_creation() {
        let renderer = TextRenderer::new();
        assert!(renderer.is_ok());
        
        let renderer = renderer.unwrap();
        assert_eq!(renderer.glyph_cache.atlas_dimensions(), (1024, 1024));
    }

    #[test]
    fn test_measure_text() {
        let mut renderer = TextRenderer::new().unwrap();
        let (width, height) = renderer.measure_text("Hello", "sans-serif", 16.0);
        assert!(width > 0.0);
        assert!(height > 0.0);
    }
    
    #[test]
    fn test_atlas_dimensions() {
        let renderer = TextRenderer::new().unwrap();
        let (width, height) = renderer.glyph_cache().atlas_dimensions();
        assert_eq!(width, 1024);
        assert_eq!(height, 1024);
    }
}
