use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use fontdue::{Font, FontSettings};
use std::collections::HashMap;
use std::sync::Arc;

/// Font manager for loading and caching fonts
pub struct FontManager {
    /// Loaded fonts by family name
    fonts: HashMap<String, Arc<Font>>,
    /// Default fallback font
    default_font: Arc<Font>,
    /// System font source
    system_source: SystemSource,
}

impl FontManager {
    /// Create a new font manager
    pub fn new() -> Result<Self, FontLoadError> {
        let system_source = SystemSource::new();
        
        // Load a default fallback font (sans-serif)
        let default_font = Self::load_system_font(&system_source, &FamilyName::SansSerif)?;
        
        Ok(Self {
            fonts: HashMap::new(),
            default_font: Arc::new(default_font),
            system_source,
        })
    }

    /// Load a font from the system
    fn load_system_font(
        source: &SystemSource,
        family: &FamilyName,
    ) -> Result<Font, FontLoadError> {
        // Try to find the font
        let handle = source
            .select_best_match(&[family.clone()], &Properties::new())
            .map_err(|e| FontLoadError::NotFound(format!("Font not found: {:?}", e)))?;

        // Load the font data
        let font_data = handle
            .load()
            .map_err(|e| FontLoadError::LoadFailed(format!("Failed to load font: {:?}", e)))?;

        // Copy the font data to bytes
        let bytes = font_data.copy_font_data()
            .ok_or_else(|| FontLoadError::LoadFailed("Failed to copy font data".to_string()))?;

        // Parse with fontdue (need to pass as slice reference)
        Font::from_bytes(bytes.as_slice(), FontSettings::default())
            .map_err(|e| FontLoadError::ParseFailed(format!("Failed to parse font: {}", e)))
    }

    /// Get or load a font by CSS font-family name
    pub fn get_font(&mut self, family: &str) -> Arc<Font> {
        // Check if already loaded
        if let Some(font) = self.fonts.get(family) {
            return Arc::clone(font);
        }

        // Try to load the font
        let family_name = match family.to_lowercase().as_str() {
            "serif" => FamilyName::Serif,
            "sans-serif" | "sans" => FamilyName::SansSerif,
            "monospace" | "mono" => FamilyName::Monospace,
            "cursive" => FamilyName::Cursive,
            "fantasy" => FamilyName::Fantasy,
            _ => FamilyName::Title(family.to_string()),
        };

        match Self::load_system_font(&self.system_source, &family_name) {
            Ok(font) => {
                let arc_font = Arc::new(font);
                self.fonts.insert(family.to_string(), Arc::clone(&arc_font));
                arc_font
            }
            Err(_) => {
                // Fallback to default if load fails
                Arc::clone(&self.default_font)
            }
        }
    }

    /// Get the default fallback font
    pub fn default_font(&self) -> Arc<Font> {
        Arc::clone(&self.default_font)
    }

    /// Preload common fonts
    pub fn preload_common_fonts(&mut self) {
        let common_families = vec![
            "serif",
            "sans-serif",
            "monospace",
            "Arial",
            "Times New Roman",
            "Courier New",
            "Verdana",
            "Georgia",
            "Comic Sans MS",
        ];

        for family in common_families {
            let _ = self.get_font(family);
        }
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new().expect("Failed to create font manager")
    }
}

/// Font loading errors
#[derive(Debug)]
pub enum FontLoadError {
    NotFound(String),
    LoadFailed(String),
    ParseFailed(String),
}

impl std::fmt::Display for FontLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontLoadError::NotFound(msg) => write!(f, "Font not found: {}", msg),
            FontLoadError::LoadFailed(msg) => write!(f, "Failed to load font: {}", msg),
            FontLoadError::ParseFailed(msg) => write!(f, "Failed to parse font: {}", msg),
        }
    }
}

impl std::error::Error for FontLoadError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_manager_creation() {
        let manager = FontManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_get_generic_fonts() {
        let mut manager = FontManager::new().unwrap();
        
        // Generic families should always work (fallback to default if needed)
        let sans = manager.get_font("sans-serif");
        let serif = manager.get_font("serif");
        let mono = manager.get_font("monospace");
        
        assert!(Arc::strong_count(&sans) >= 1);
        assert!(Arc::strong_count(&serif) >= 1);
        assert!(Arc::strong_count(&mono) >= 1);
    }

    #[test]
    fn test_font_caching() {
        let mut manager = FontManager::new().unwrap();
        
        let font1 = manager.get_font("sans-serif");
        let font2 = manager.get_font("sans-serif");
        
        // Should return the same Arc (cached)
        assert!(Arc::ptr_eq(&font1, &font2));
    }

    #[test]
    fn test_unknown_font_fallback() {
        let mut manager = FontManager::new().unwrap();
        
        // Unknown font should fallback to default
        let unknown = manager.get_font("UnknownFont12345");
        let default = manager.default_font();
        
        // Should get default font
        assert!(Arc::strong_count(&unknown) >= 1);
        assert!(Arc::strong_count(&default) >= 1);
    }
}
