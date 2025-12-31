use fontdue::Font;
use std::collections::HashMap;
use std::sync::Arc;

/// Key for identifying a cached glyph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    /// Character being rendered
    pub character: char,
    /// Font size in pixels
    pub size: u32,
    /// Font index (for multiple fonts)
    pub font_id: usize,
}

/// Information about a rasterized glyph in the atlas
#[derive(Debug, Clone, Copy)]
pub struct GlyphInfo {
    /// Position in atlas texture (pixels)
    pub atlas_x: u32,
    pub atlas_y: u32,
    /// Size of the glyph bitmap (pixels)
    pub width: u32,
    pub height: u32,
    /// Offset from the pen position to the top-left of the glyph
    pub bearing_x: f32,
    pub bearing_y: f32,
    /// Horizontal advance to the next glyph position
    pub advance: f32,
}

/// Simple texture atlas allocator for glyphs
#[derive(Debug)]
pub struct TextureAtlas {
    /// Atlas dimensions
    width: u32,
    height: u32,
    /// Current packing position
    current_x: u32,
    current_y: u32,
    /// Current row height
    row_height: u32,
    /// Actual texture data (grayscale, R8)
    data: Vec<u8>,
}

impl TextureAtlas {
    /// Create a new texture atlas
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            current_x: 0,
            current_y: 0,
            row_height: 0,
            data: vec![0; size],
        }
    }

    /// Try to allocate space for a glyph in the atlas
    /// Returns (x, y) position if successful
    pub fn allocate(&mut self, width: u32, height: u32) -> Option<(u32, u32)> {
        // Check if glyph fits in current row
        if self.current_x + width > self.width {
            // Move to next row
            self.current_x = 0;
            self.current_y += self.row_height;
            self.row_height = 0;
        }

        // Check if we have vertical space
        if self.current_y + height > self.height {
            return None; // Atlas is full
        }

        let x = self.current_x;
        let y = self.current_y;

        // Update position
        self.current_x += width;
        self.row_height = self.row_height.max(height);

        Some((x, y))
    }

    /// Upload glyph bitmap data to the atlas
    pub fn upload_glyph(&mut self, x: u32, y: u32, width: u32, height: u32, data: &[u8]) {
        for row in 0..height {
            let src_start = (row * width) as usize;
            let src_end = src_start + width as usize;
            let dst_start = ((y + row) * self.width + x) as usize;
            let dst_end = dst_start + width as usize;

            if src_end <= data.len() && dst_end <= self.data.len() {
                self.data[dst_start..dst_end].copy_from_slice(&data[src_start..src_end]);
            }
        }
    }

    /// Get the atlas texture data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get atlas dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

/// Glyph cache for rasterizing and caching glyphs
pub struct GlyphCache {
    /// Cached glyphs
    glyphs: HashMap<GlyphKey, GlyphInfo>,
    /// Texture atlas
    atlas: TextureAtlas,
    /// Font registry (font_id -> Font)
    fonts: Vec<Arc<Font>>,
    /// Dirty flag for atlas updates
    dirty: bool,
}

impl GlyphCache {
    /// Create a new glyph cache with specified atlas size
    pub fn new(atlas_width: u32, atlas_height: u32) -> Self {
        Self {
            glyphs: HashMap::new(),
            atlas: TextureAtlas::new(atlas_width, atlas_height),
            fonts: Vec::new(),
            dirty: false,
        }
    }

    /// Register a font and return its ID
    pub fn register_font(&mut self, font: Arc<Font>) -> usize {
        let id = self.fonts.len();
        self.fonts.push(font);
        id
    }

    /// Get or rasterize a glyph
    pub fn get_or_rasterize(&mut self, key: GlyphKey) -> Option<GlyphInfo> {
        // Check if already cached
        if let Some(info) = self.glyphs.get(&key) {
            return Some(*info);
        }

        // Get the font
        let font = self.fonts.get(key.font_id)?;

        // Rasterize the glyph
        let (metrics, bitmap) = font.rasterize(key.character, key.size as f32);

        // Handle empty glyphs (e.g., space)
        if bitmap.is_empty() {
            let info = GlyphInfo {
                atlas_x: 0,
                atlas_y: 0,
                width: 0,
                height: 0,
                bearing_x: metrics.xmin as f32,
                bearing_y: metrics.ymin as f32,
                advance: metrics.advance_width,
            };
            self.glyphs.insert(key, info);
            return Some(info);
        }

        // Allocate space in atlas
        let (atlas_x, atlas_y) = self.atlas.allocate(metrics.width as u32, metrics.height as u32)?;

        // Upload glyph to atlas
        self.atlas.upload_glyph(
            atlas_x,
            atlas_y,
            metrics.width as u32,
            metrics.height as u32,
            &bitmap,
        );

        // Create glyph info
        let info = GlyphInfo {
            atlas_x,
            atlas_y,
            width: metrics.width as u32,
            height: metrics.height as u32,
            bearing_x: metrics.xmin as f32,
            bearing_y: metrics.ymin as f32,
            advance: metrics.advance_width,
        };

        self.glyphs.insert(key, info);
        self.dirty = true;

        Some(info)
    }

    /// Get the atlas texture data
    pub fn atlas_data(&self) -> &[u8] {
        self.atlas.data()
    }

    /// Get atlas dimensions
    pub fn atlas_dimensions(&self) -> (u32, u32) {
        self.atlas.dimensions()
    }

    /// Check if atlas has been updated
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark atlas as clean (after GPU upload)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Get number of cached glyphs
    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_atlas_creation() {
        let atlas = TextureAtlas::new(512, 512);
        assert_eq!(atlas.dimensions(), (512, 512));
        assert_eq!(atlas.data().len(), 512 * 512);
    }

    #[test]
    fn test_texture_atlas_allocation() {
        let mut atlas = TextureAtlas::new(100, 100);

        // Allocate first glyph
        let pos1 = atlas.allocate(20, 30);
        assert_eq!(pos1, Some((0, 0)));

        // Allocate second glyph in same row
        let pos2 = atlas.allocate(25, 30);
        assert_eq!(pos2, Some((20, 0)));

        // Allocate third glyph - should wrap to next row
        let pos3 = atlas.allocate(60, 25);
        assert_eq!(pos3, Some((0, 30)));
    }

    #[test]
    fn test_texture_atlas_full() {
        let mut atlas = TextureAtlas::new(50, 50);

        // Fill the atlas
        let mut allocations = Vec::new();
        loop {
            match atlas.allocate(10, 10) {
                Some(pos) => allocations.push(pos),
                None => break,
            }
        }

        // Should have filled most of the atlas
        assert!(allocations.len() >= 20); // At least 5x5 grid
    }

    #[test]
    fn test_glyph_cache_creation() {
        let cache = GlyphCache::new(512, 512);
        assert_eq!(cache.glyph_count(), 0);
        assert_eq!(cache.atlas_dimensions(), (512, 512));
        assert!(!cache.is_dirty());
    }

    // Test for glyph rasterization would require actual font file
    // This is tested in integration tests with real fonts
}
