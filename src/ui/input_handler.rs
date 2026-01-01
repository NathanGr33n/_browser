// Input handling for browser UI

use winit::event::{ElementState, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Input handler for keyboard and mouse events
pub struct InputHandler {
    /// Mouse position
    mouse_x: f32,
    mouse_y: f32,
    /// Left mouse button state
    left_button_down: bool,
    /// Modifier keys state
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
}

impl InputHandler {
    /// Create new input handler
    pub fn new() -> Self {
        Self {
            mouse_x: 0.0,
            mouse_y: 0.0,
            left_button_down: false,
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
        }
    }
    
    /// Update mouse position
    pub fn update_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }
    
    /// Get mouse position
    pub fn mouse_position(&self) -> (f32, f32) {
        (self.mouse_x, self.mouse_y)
    }
    
    /// Handle mouse button input
    pub fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left {
            self.left_button_down = state == ElementState::Pressed;
        }
    }
    
    /// Check if left mouse button is down
    pub fn is_left_button_down(&self) -> bool {
        self.left_button_down
    }
    
    /// Handle keyboard input for modifier keys
    pub fn handle_keyboard(&mut self, key: PhysicalKey, state: ElementState) {
        let pressed = state == ElementState::Pressed;
        
        if let PhysicalKey::Code(code) = key {
            match code {
                KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                    self.shift_pressed = pressed;
                }
                KeyCode::ControlLeft | KeyCode::ControlRight => {
                    self.ctrl_pressed = pressed;
                }
                KeyCode::AltLeft | KeyCode::AltRight => {
                    self.alt_pressed = pressed;
                }
                _ => {}
            }
        }
    }
    
    /// Check if Shift is pressed
    pub fn is_shift_pressed(&self) -> bool {
        self.shift_pressed
    }
    
    /// Check if Ctrl is pressed
    pub fn is_ctrl_pressed(&self) -> bool {
        self.ctrl_pressed
    }
    
    /// Check if Alt is pressed
    pub fn is_alt_pressed(&self) -> bool {
        self.alt_pressed
    }
    
    /// Reset all input state
    pub fn reset(&mut self) {
        self.left_button_down = false;
        self.shift_pressed = false;
        self.ctrl_pressed = false;
        self.alt_pressed = false;
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_input_handler_creation() {
        let handler = InputHandler::new();
        assert_eq!(handler.mouse_position(), (0.0, 0.0));
        assert!(!handler.is_left_button_down());
    }
    
    #[test]
    fn test_mouse_position() {
        let mut handler = InputHandler::new();
        handler.update_mouse_position(100.0, 200.0);
        assert_eq!(handler.mouse_position(), (100.0, 200.0));
    }
    
    #[test]
    fn test_mouse_button() {
        let mut handler = InputHandler::new();
        handler.handle_mouse_button(MouseButton::Left, ElementState::Pressed);
        assert!(handler.is_left_button_down());
        
        handler.handle_mouse_button(MouseButton::Left, ElementState::Released);
        assert!(!handler.is_left_button_down());
    }
    
    #[test]
    fn test_modifier_keys() {
        let mut handler = InputHandler::new();
        
        handler.handle_keyboard(PhysicalKey::Code(KeyCode::ShiftLeft), ElementState::Pressed);
        assert!(handler.is_shift_pressed());
        
        handler.handle_keyboard(PhysicalKey::Code(KeyCode::ControlLeft), ElementState::Pressed);
        assert!(handler.is_ctrl_pressed());
        
        handler.handle_keyboard(PhysicalKey::Code(KeyCode::AltLeft), ElementState::Pressed);
        assert!(handler.is_alt_pressed());
    }
    
    #[test]
    fn test_reset() {
        let mut handler = InputHandler::new();
        handler.handle_mouse_button(MouseButton::Left, ElementState::Pressed);
        handler.handle_keyboard(PhysicalKey::Code(KeyCode::ShiftLeft), ElementState::Pressed);
        
        handler.reset();
        assert!(!handler.is_left_button_down());
        assert!(!handler.is_shift_pressed());
    }
}
