/// Scrolling state management
#[derive(Debug, Clone, Copy)]
pub struct ScrollState {
    /// Current scroll offset (x, y) in pixels
    pub offset_x: f32,
    pub offset_y: f32,
    /// Content size (width, height) in pixels
    pub content_width: f32,
    pub content_height: f32,
    /// Viewport size (width, height) in pixels
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl Default for ScrollState {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            content_width: 0.0,
            content_height: 0.0,
            viewport_width: 800.0,
            viewport_height: 600.0,
        }
    }
}

impl ScrollState {
    /// Create new scroll state
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            viewport_width,
            viewport_height,
            ..Default::default()
        }
    }

    /// Update viewport size (e.g., on window resize)
    pub fn set_viewport_size(&mut self, width: f32, height: f32) {
        self.viewport_width = width;
        self.viewport_height = height;
        self.clamp_scroll();
    }

    /// Update content size (e.g., after layout)
    pub fn set_content_size(&mut self, width: f32, height: f32) {
        self.content_width = width;
        self.content_height = height;
        self.clamp_scroll();
    }

    /// Scroll by delta amount
    pub fn scroll_by(&mut self, delta_x: f32, delta_y: f32) {
        self.offset_x += delta_x;
        self.offset_y += delta_y;
        self.clamp_scroll();
    }

    /// Scroll to absolute position
    pub fn scroll_to(&mut self, x: f32, y: f32) {
        self.offset_x = x;
        self.offset_y = y;
        self.clamp_scroll();
    }

    /// Clamp scroll to valid range
    fn clamp_scroll(&mut self) {
        // Calculate maximum scroll (content that extends beyond viewport)
        let max_scroll_x = (self.content_width - self.viewport_width).max(0.0);
        let max_scroll_y = (self.content_height - self.viewport_height).max(0.0);

        // Clamp to valid range
        self.offset_x = self.offset_x.clamp(0.0, max_scroll_x);
        self.offset_y = self.offset_y.clamp(0.0, max_scroll_y);
    }

    /// Check if content is scrollable horizontally
    pub fn can_scroll_x(&self) -> bool {
        self.content_width > self.viewport_width
    }

    /// Check if content is scrollable vertically
    pub fn can_scroll_y(&self) -> bool {
        self.content_height > self.viewport_height
    }

    /// Get scroll percentage (0.0 to 1.0)
    pub fn scroll_percentage_y(&self) -> f32 {
        if !self.can_scroll_y() {
            return 0.0;
        }
        let max_scroll = self.content_height - self.viewport_height;
        self.offset_y / max_scroll
    }

    /// Apply scroll offset to a position (for rendering)
    pub fn apply_offset(&self, x: f32, y: f32) -> (f32, f32) {
        (x - self.offset_x, y - self.offset_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_state_default() {
        let state = ScrollState::default();
        assert_eq!(state.offset_x, 0.0);
        assert_eq!(state.offset_y, 0.0);
    }

    #[test]
    fn test_scroll_clamping() {
        let mut state = ScrollState::new(800.0, 600.0);
        state.set_content_size(1000.0, 1200.0);
        
        // Can scroll vertically
        assert!(state.can_scroll_y());
        
        // Scroll beyond max
        state.scroll_to(0.0, 1000.0);
        assert_eq!(state.offset_y, 600.0); // Clamped to max (1200 - 600)
        
        // Scroll negative
        state.scroll_to(0.0, -100.0);
        assert_eq!(state.offset_y, 0.0); // Clamped to min
    }

    #[test]
    fn test_scroll_by() {
        let mut state = ScrollState::new(800.0, 600.0);
        state.set_content_size(800.0, 1200.0);
        
        state.scroll_by(0.0, 100.0);
        assert_eq!(state.offset_y, 100.0);
        
        state.scroll_by(0.0, -50.0);
        assert_eq!(state.offset_y, 50.0);
    }

    #[test]
    fn test_apply_offset() {
        let mut state = ScrollState::default();
        state.offset_y = 100.0;
        
        let (x, y) = state.apply_offset(50.0, 200.0);
        assert_eq!(x, 50.0);
        assert_eq!(y, 100.0); // 200 - 100 offset
    }
}
