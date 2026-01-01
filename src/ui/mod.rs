// Browser UI Module - Address bar and navigation controls

mod address_bar;
mod navigation;
mod input_handler;

pub use address_bar::AddressBar;
pub use navigation::{NavigationButtons, NavigationState};
pub use input_handler::InputHandler;

use crate::layout::Rect;

/// Browser chrome UI containing address bar and navigation
pub struct BrowserUI {
    pub address_bar: AddressBar,
    pub navigation: NavigationButtons,
    pub input_handler: InputHandler,
    pub bounds: Rect,
    pub chrome_height: f32,
}

impl BrowserUI {
    /// Create a new browser UI
    pub fn new(width: f32) -> Self {
        let chrome_height = 60.0;
        
        Self {
            address_bar: AddressBar::new(),
            navigation: NavigationButtons::new(),
            input_handler: InputHandler::new(),
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width,
                height: chrome_height,
            },
            chrome_height,
        }
    }
    
    /// Get the content viewport (below the chrome)
    pub fn content_viewport(&self) -> Rect {
        Rect {
            x: 0.0,
            y: self.chrome_height,
            width: self.bounds.width,
            height: self.bounds.height - self.chrome_height,
        }
    }
    
    /// Update UI layout when window resizes
    pub fn resize(&mut self, width: f32, height: f32) {
        self.bounds.width = width;
        self.bounds.height = height;
        
        // Update address bar width
        self.address_bar.set_width(width - 200.0); // Leave room for nav buttons
    }
    
    /// Check if a point is within the chrome area
    pub fn contains_point(&self, _x: f32, y: f32) -> bool {
        y < self.chrome_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ui_creation() {
        let ui = BrowserUI::new(800.0);
        assert_eq!(ui.chrome_height, 60.0);
        assert_eq!(ui.bounds.width, 800.0);
    }
    
    #[test]
    fn test_content_viewport() {
        let ui = BrowserUI::new(800.0);
        let viewport = ui.content_viewport();
        assert_eq!(viewport.y, 60.0);
        assert_eq!(viewport.width, 800.0);
    }
    
    #[test]
    fn test_contains_point() {
        let ui = BrowserUI::new(800.0);
        assert!(ui.contains_point(100.0, 30.0)); // In chrome
        assert!(!ui.contains_point(100.0, 100.0)); // Below chrome
    }
}
