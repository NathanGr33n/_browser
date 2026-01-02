// Address bar for URL input

use crate::layout::Rect;

/// Address bar state and rendering
pub struct AddressBar {
    /// Current URL text
    url: String,
    /// Is the address bar focused
    focused: bool,
    /// Loading state
    loading: bool,
    /// Progress (0.0 to 1.0)
    progress: f32,
    /// Visual bounds
    bounds: Rect,
}

impl AddressBar {
    /// Create a new address bar
    pub fn new() -> Self {
        Self {
            url: String::from("about:blank"),
            focused: false,
            loading: false,
            progress: 0.0,
            bounds: Rect {
                x: 120.0,
                y: 10.0,
                width: 560.0,
                height: 40.0,
            },
        }
    }
    
    /// Get the current URL
    pub fn url(&self) -> &str {
        &self.url
    }
    
    /// Set the URL (e.g., after navigation)
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }
    
    /// Update the URL (e.g., during typing)
    pub fn update_url(&mut self, url: String) {
        self.url = url;
    }
    
    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
    
    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }
    
    /// Set loading state
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
        if !loading {
            self.progress = 0.0;
        }
    }
    
    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.loading
    }
    
    /// Update loading progress
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }
    
    /// Get loading progress
    pub fn progress(&self) -> f32 {
        self.progress
    }
    
    /// Get the visual bounds
    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }
    
    /// Set the width of the address bar
    pub fn set_width(&mut self, width: f32) {
        self.bounds.width = width;
    }
    
    /// Handle character input
    pub fn insert_char(&mut self, ch: char) {
        if self.focused {
            self.url.push(ch);
        }
    }
    
    /// Handle backspace
    pub fn backspace(&mut self) {
        if self.focused {
            self.url.pop();
        }
    }
    
    /// Clear the URL
    pub fn clear(&mut self) {
        if self.focused {
            self.url.clear();
        }
    }
    
    /// Check if the address bar contains a point
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.bounds.x
            && x <= self.bounds.x + self.bounds.width
            && y >= self.bounds.y
            && y <= self.bounds.y + self.bounds.height
    }
}

impl Default for AddressBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_address_bar_creation() {
        let bar = AddressBar::new();
        assert_eq!(bar.url(), "about:blank");
        assert!(!bar.is_focused());
        assert!(!bar.is_loading());
    }
    
    #[test]
    fn test_url_operations() {
        let mut bar = AddressBar::new();
        bar.set_url("https://example.com".to_string());
        assert_eq!(bar.url(), "https://example.com");
    }
    
    #[test]
    fn test_focus_operations() {
        let mut bar = AddressBar::new();
        bar.set_focused(true);
        assert!(bar.is_focused());
        bar.set_focused(false);
        assert!(!bar.is_focused());
    }
    
    #[test]
    fn test_loading_progress() {
        let mut bar = AddressBar::new();
        bar.set_loading(true);
        assert!(bar.is_loading());
        
        bar.set_progress(0.5);
        assert_eq!(bar.progress(), 0.5);
        
        bar.set_loading(false);
        assert_eq!(bar.progress(), 0.0);
    }
    
    #[test]
    fn test_input_handling() {
        let mut bar = AddressBar::new();
        bar.set_focused(true);
        bar.clear();
        
        bar.insert_char('h');
        bar.insert_char('i');
        assert_eq!(bar.url(), "hi");
        
        bar.backspace();
        assert_eq!(bar.url(), "h");
    }
    
    #[test]
    fn test_contains_point() {
        let bar = AddressBar::new();
        // Within bounds
        assert!(bar.contains_point(300.0, 30.0));
        // Outside bounds
        assert!(!bar.contains_point(50.0, 30.0));
    }
}
