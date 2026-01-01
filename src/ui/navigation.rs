// Navigation buttons (back, forward, refresh)

use crate::layout::Rect;

/// Navigation state (history)
#[derive(Clone)]
pub struct NavigationState {
    /// URL history
    history: Vec<String>,
    /// Current position in history
    current_index: isize,
}

impl NavigationState {
    /// Create new navigation state
    pub fn new() -> Self {
        Self {
            history: vec!["about:blank".to_string()],
            current_index: 0,
        }
    }
    
    /// Navigate to a new URL
    pub fn navigate(&mut self, url: String) {
        // Remove any forward history
        self.history.truncate((self.current_index + 1) as usize);
        
        // Add new URL
        self.history.push(url);
        self.current_index += 1;
    }
    
    /// Go back in history
    pub fn go_back(&mut self) -> Option<String> {
        if self.can_go_back() {
            self.current_index -= 1;
            Some(self.current_url().to_string())
        } else {
            None
        }
    }
    
    /// Go forward in history
    pub fn go_forward(&mut self) -> Option<String> {
        if self.can_go_forward() {
            self.current_index += 1;
            Some(self.current_url().to_string())
        } else {
            None
        }
    }
    
    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }
    
    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        self.current_index < (self.history.len() as isize) - 1
    }
    
    /// Get current URL
    pub fn current_url(&self) -> &str {
        &self.history[self.current_index as usize]
    }
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Navigation button type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavButton {
    Back,
    Forward,
    Refresh,
}

/// Navigation buttons UI component
pub struct NavigationButtons {
    /// Back button bounds
    back_bounds: Rect,
    /// Forward button bounds
    forward_bounds: Rect,
    /// Refresh button bounds
    refresh_bounds: Rect,
    /// Currently hovered button
    hovered: Option<NavButton>,
}

impl NavigationButtons {
    /// Create new navigation buttons
    pub fn new() -> Self {
        Self {
            back_bounds: Rect {
                x: 10.0,
                y: 15.0,
                width: 30.0,
                height: 30.0,
            },
            forward_bounds: Rect {
                x: 50.0,
                y: 15.0,
                width: 30.0,
                height: 30.0,
            },
            refresh_bounds: Rect {
                x: 90.0,
                y: 15.0,
                width: 30.0,
                height: 30.0,
            },
            hovered: None,
        }
    }
    
    /// Get bounds for a button
    pub fn button_bounds(&self, button: NavButton) -> &Rect {
        match button {
            NavButton::Back => &self.back_bounds,
            NavButton::Forward => &self.forward_bounds,
            NavButton::Refresh => &self.refresh_bounds,
        }
    }
    
    /// Check which button contains a point
    pub fn hit_test(&mut self, x: f32, y: f32) -> Option<NavButton> {
        if self.back_bounds.contains(x, y) {
            self.hovered = Some(NavButton::Back);
            Some(NavButton::Back)
        } else if self.forward_bounds.contains(x, y) {
            self.hovered = Some(NavButton::Forward);
            Some(NavButton::Forward)
        } else if self.refresh_bounds.contains(x, y) {
            self.hovered = Some(NavButton::Refresh);
            Some(NavButton::Refresh)
        } else {
            self.hovered = None;
            None
        }
    }
    
    /// Get currently hovered button
    pub fn hovered(&self) -> Option<NavButton> {
        self.hovered
    }
    
    /// Clear hover state
    pub fn clear_hover(&mut self) {
        self.hovered = None;
    }
}

impl Default for NavigationButtons {
    fn default() -> Self {
        Self::new()
    }
}

// Helper trait for Rect to check containment
trait RectContains {
    fn contains(&self, x: f32, y: f32) -> bool;
}

impl RectContains for Rect {
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_navigation_state() {
        let mut nav = NavigationState::new();
        assert_eq!(nav.current_url(), "about:blank");
        assert!(!nav.can_go_back());
        assert!(!nav.can_go_forward());
    }
    
    #[test]
    fn test_navigate_forward() {
        let mut nav = NavigationState::new();
        nav.navigate("https://example.com".to_string());
        assert_eq!(nav.current_url(), "https://example.com");
        assert!(nav.can_go_back());
    }
    
    #[test]
    fn test_go_back() {
        let mut nav = NavigationState::new();
        nav.navigate("https://example.com".to_string());
        nav.navigate("https://example.org".to_string());
        
        let url = nav.go_back();
        assert_eq!(url, Some("https://example.com".to_string()));
        assert_eq!(nav.current_url(), "https://example.com");
    }
    
    #[test]
    fn test_go_forward() {
        let mut nav = NavigationState::new();
        nav.navigate("https://example.com".to_string());
        nav.navigate("https://example.org".to_string());
        nav.go_back();
        
        let url = nav.go_forward();
        assert_eq!(url, Some("https://example.org".to_string()));
        assert_eq!(nav.current_url(), "https://example.org");
    }
    
    #[test]
    fn test_navigation_buttons() {
        let mut buttons = NavigationButtons::new();
        
        // Test back button hit
        let hit = buttons.hit_test(20.0, 25.0);
        assert_eq!(hit, Some(NavButton::Back));
        assert_eq!(buttons.hovered(), Some(NavButton::Back));
        
        // Test forward button hit
        let hit = buttons.hit_test(60.0, 25.0);
        assert_eq!(hit, Some(NavButton::Forward));
        
        // Test miss
        let hit = buttons.hit_test(500.0, 25.0);
        assert_eq!(hit, None);
    }
}
