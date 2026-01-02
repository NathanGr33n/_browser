pub mod scroll;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};
use std::sync::Arc;
use crate::renderer::Renderer;

pub use scroll::ScrollState;

/// Application window with integrated renderer
pub struct Window {
    window: Arc<WinitWindow>,
    event_loop: Option<EventLoop<()>>,
}

/// Window configuration options
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Browser Engine".to_string(),
            width: 1024,
            height: 768,
            resizable: true,
        }
    }
}

impl Window {
    /// Create a new window with the given configuration
    pub fn new(config: WindowConfig) -> Result<Self, WindowError> {
        let event_loop = EventLoop::new().map_err(|e| WindowError::Creation(e.to_string()))?;
        
        let window = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .with_resizable(config.resizable)
            .build(&event_loop)
            .map_err(|e| WindowError::Creation(e.to_string()))?;

        Ok(Self {
            window: Arc::new(window),
            event_loop: Some(event_loop),
        })
    }

    /// Get a reference to the underlying window
    pub fn inner(&self) -> &Arc<WinitWindow> {
        &self.window
    }

    /// Get the window's current size
    pub fn size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    /// Request a redraw of the window
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    /// Run the event loop with renderer and callback
    /// 
    /// The callback receives the renderer and window events
    pub fn run_with_renderer<F>(mut self, mut callback: F) -> Result<(), WindowError>
    where
        F: FnMut(&mut Renderer, WindowEvent) -> bool + 'static,
    {
        let event_loop = self.event_loop.take()
            .ok_or(WindowError::EventLoop("Event loop already consumed".to_string()))?;

        // Cache window ID and clone Arc for closure
        let window_id = self.window.id();
        let window_clone = self.window.clone();

        // Initialize renderer
        let mut renderer = pollster::block_on(Renderer::new(&self.window))
            .map_err(|e| WindowError::Renderer(e.to_string()))?;

        event_loop
            .run(move |event, target| {
                // Set control flow to wait for events (more efficient than polling)
                target.set_control_flow(ControlFlow::Wait);

                match event {
                    Event::WindowEvent { event, window_id: event_window_id } 
                        if event_window_id == window_id => 
                    {
                        // Pass event to callback; if it returns false, exit
                        if !callback(&mut renderer, event.clone()) {
                            target.exit();
                            return;
                        }

                        // Handle window close explicitly
                        if matches!(event, WindowEvent::CloseRequested) {
                            target.exit();
                        }
                    }
                    Event::AboutToWait => {
                        // Request redraw after processing all events
                        window_clone.request_redraw();
                    }
                    _ => {}
                }
            })
            .map_err(|e| WindowError::EventLoop(e.to_string()))
    }
}

/// Window-related errors
#[derive(Debug)]
pub enum WindowError {
    /// Error during window creation
    Creation(String),
    /// Error in event loop
    EventLoop(String),
    /// Error initializing renderer
    Renderer(String),
}

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowError::Creation(msg) => write!(f, "Window creation error: {}", msg),
            WindowError::EventLoop(msg) => write!(f, "Event loop error: {}", msg),
            WindowError::Renderer(msg) => write!(f, "Renderer error: {}", msg),
        }
    }
}

impl std::error::Error for WindowError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.title, "Browser Engine");
        assert_eq!(config.width, 1024);
        assert_eq!(config.height, 768);
        assert!(config.resizable);
    }

    #[test]
    fn test_window_config_custom() {
        let config = WindowConfig {
            title: "Test Window".to_string(),
            width: 800,
            height: 600,
            resizable: false,
        };
        assert_eq!(config.title, "Test Window");
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert!(!config.resizable);
    }
}
