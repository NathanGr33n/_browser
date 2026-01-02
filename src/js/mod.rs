// JavaScript engine integration module

mod runtime;
mod dom_bindings;
mod event_handler;

pub use runtime::{JsRuntime, JsValue, JsError};
pub use dom_bindings::DomBindings;
pub use event_handler::{EventType, EventHandler};

use crate::dom::Node;
use std::sync::{Arc, Mutex};

/// JavaScript execution context for a page
pub struct JsContext {
    /// Runtime instance
    runtime: JsRuntime,
    /// DOM bindings
    dom_bindings: DomBindings,
    /// Event handlers
    event_handler: EventHandler,
    /// Execution enabled
    enabled: bool,
}

impl JsContext {
    /// Create a new JavaScript context
    pub fn new() -> Self {
        Self {
            runtime: JsRuntime::new(),
            dom_bindings: DomBindings::new(),
            event_handler: EventHandler::new(),
            enabled: true,
        }
    }
    
    /// Execute JavaScript code
    pub fn execute(&mut self, code: &str) -> Result<JsValue, JsError> {
        if !self.enabled {
            return Err(JsError::ExecutionDisabled);
        }
        
        self.runtime.execute(code)
    }
    
    /// Bind a DOM tree to the JavaScript context
    pub fn bind_dom(&mut self, dom: Arc<Mutex<Node>>) {
        self.dom_bindings.bind_dom_tree(dom);
    }
    
    /// Register an event listener
    pub fn add_event_listener(
        &mut self,
        event_type: EventType,
        callback: String,
    ) -> Result<(), JsError> {
        self.event_handler.add_listener(event_type, callback)
    }
    
    /// Dispatch an event
    pub fn dispatch_event(&mut self, event_type: EventType, _target: String) -> Result<(), JsError> {
        let handlers = self.event_handler.get_handlers(&event_type);
        
        for handler in handlers {
            self.runtime.execute(&handler)?;
        }
        
        Ok(())
    }
    
    /// Enable or disable JavaScript execution
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Check if JavaScript execution is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get a reference to the DOM bindings
    pub fn dom_bindings(&self) -> &DomBindings {
        &self.dom_bindings
    }
    
    /// Get a mutable reference to the DOM bindings
    pub fn dom_bindings_mut(&mut self) -> &mut DomBindings {
        &mut self.dom_bindings
    }
}

impl Default for JsContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_js_context_creation() {
        let ctx = JsContext::new();
        assert!(ctx.is_enabled());
    }
    
    #[test]
    fn test_enable_disable() {
        let mut ctx = JsContext::new();
        ctx.set_enabled(false);
        assert!(!ctx.is_enabled());
        
        let result = ctx.execute("1 + 1");
        assert!(result.is_err());
    }
}
