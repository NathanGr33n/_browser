// Event handling for JavaScript

use std::collections::HashMap;

/// JavaScript event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    Click,
    MouseDown,
    MouseUp,
    MouseMove,
    KeyDown,
    KeyUp,
    KeyPress,
    Load,
    DOMContentLoaded,
    Resize,
    Scroll,
}

impl EventType {
    /// Parse event type from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "click" => Some(EventType::Click),
            "mousedown" => Some(EventType::MouseDown),
            "mouseup" => Some(EventType::MouseUp),
            "mousemove" => Some(EventType::MouseMove),
            "keydown" => Some(EventType::KeyDown),
            "keyup" => Some(EventType::KeyUp),
            "keypress" => Some(EventType::KeyPress),
            "load" => Some(EventType::Load),
            "domcontentloaded" => Some(EventType::DOMContentLoaded),
            "resize" => Some(EventType::Resize),
            "scroll" => Some(EventType::Scroll),
            _ => None,
        }
    }
    
    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Click => "click",
            EventType::MouseDown => "mousedown",
            EventType::MouseUp => "mouseup",
            EventType::MouseMove => "mousemove",
            EventType::KeyDown => "keydown",
            EventType::KeyUp => "keyup",
            EventType::KeyPress => "keypress",
            EventType::Load => "load",
            EventType::DOMContentLoaded => "DOMContentLoaded",
            EventType::Resize => "resize",
            EventType::Scroll => "scroll",
        }
    }
}

/// Event handler registry
pub struct EventHandler {
    /// Event listeners by event type
    listeners: HashMap<EventType, Vec<String>>,
}

impl EventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }
    
    /// Add an event listener
    pub fn add_listener(
        &mut self,
        event_type: EventType,
        callback: String,
    ) -> Result<(), crate::js::JsError> {
        self.listeners
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(callback);
        Ok(())
    }
    
    /// Remove an event listener
    pub fn remove_listener(&mut self, event_type: EventType, callback: &str) -> bool {
        if let Some(listeners) = self.listeners.get_mut(&event_type) {
            if let Some(pos) = listeners.iter().position(|x| x == callback) {
                listeners.remove(pos);
                return true;
            }
        }
        false
    }
    
    /// Get all handlers for an event type
    pub fn get_handlers(&self, event_type: &EventType) -> Vec<String> {
        self.listeners
            .get(event_type)
            .map(|v| v.clone())
            .unwrap_or_default()
    }
    
    /// Check if there are any listeners for an event type
    pub fn has_listeners(&self, event_type: &EventType) -> bool {
        self.listeners
            .get(event_type)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }
    
    /// Clear all listeners
    pub fn clear_all(&mut self) {
        self.listeners.clear();
    }
    
    /// Clear listeners for a specific event type
    pub fn clear_event(&mut self, event_type: EventType) {
        self.listeners.remove(&event_type);
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_type_from_str() {
        assert_eq!(EventType::from_str("click"), Some(EventType::Click));
        assert_eq!(EventType::from_str("keydown"), Some(EventType::KeyDown));
        assert_eq!(EventType::from_str("invalid"), None);
    }
    
    #[test]
    fn test_event_handler_creation() {
        let handler = EventHandler::new();
        assert!(!handler.has_listeners(&EventType::Click));
    }
    
    #[test]
    fn test_add_listener() {
        let mut handler = EventHandler::new();
        handler
            .add_listener(EventType::Click, "console.log('clicked')".to_string())
            .unwrap();
        assert!(handler.has_listeners(&EventType::Click));
    }
    
    #[test]
    fn test_get_handlers() {
        let mut handler = EventHandler::new();
        handler
            .add_listener(EventType::Click, "handler1()".to_string())
            .unwrap();
        handler
            .add_listener(EventType::Click, "handler2()".to_string())
            .unwrap();
        
        let handlers = handler.get_handlers(&EventType::Click);
        assert_eq!(handlers.len(), 2);
    }
    
    #[test]
    fn test_remove_listener() {
        let mut handler = EventHandler::new();
        handler
            .add_listener(EventType::Click, "handler()".to_string())
            .unwrap();
        
        assert!(handler.remove_listener(EventType::Click, "handler()"));
        assert!(!handler.has_listeners(&EventType::Click));
    }
    
    #[test]
    fn test_clear_all() {
        let mut handler = EventHandler::new();
        handler
            .add_listener(EventType::Click, "handler()".to_string())
            .unwrap();
        handler
            .add_listener(EventType::KeyDown, "handler()".to_string())
            .unwrap();
        
        handler.clear_all();
        assert!(!handler.has_listeners(&EventType::Click));
        assert!(!handler.has_listeners(&EventType::KeyDown));
    }
}
