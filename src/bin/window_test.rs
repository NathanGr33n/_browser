// Test binary for window and renderer
use browser_engine::window::{Window, WindowConfig};
use winit::event::WindowEvent;

fn main() {
    println!("=== Window & Renderer Test ===\n");
    
    // Create window
    let window = Window::new(WindowConfig::default())
        .expect("Failed to create window");
    
    println!("\u{2713} Window created: {}x{}", window.size().width, window.size().height);
    println!("\nInitializing renderer and opening window...");
    println!("Press ESC or close window to exit.\n");
    
    // Run event loop with integrated renderer
    window.run_with_renderer(|renderer, event| {
        match event {
            WindowEvent::RedrawRequested => {
                // Clear screen to blue-gray
                if let Err(e) = renderer.clear([0.2, 0.3, 0.4, 1.0]) {
                    eprintln!("Render error: {}", e);
                }
            }
            WindowEvent::Resized(size) => {
                println!("Window resized: {}x{}", size.width, size.height);
                renderer.resize(size.width, size.height);
            }
            WindowEvent::CloseRequested => {
                println!("Window closed.");
                return false;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) {
                    println!("ESC pressed. Exiting...");
                    return false;
                }
            }
            _ => {}
        }
        true
    }).expect("Event loop error");
}
