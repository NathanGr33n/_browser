// Test binary for rectangle rendering
use browser_engine::window::{Window, WindowConfig};
use browser_engine::css::Color;
use browser_engine::layout::Rect;
use winit::event::WindowEvent;

fn main() {
    println!("=== Rectangle Rendering Test ===\n");
    
    let window = Window::new(WindowConfig::default())
        .expect("Failed to create window");
    
    println!("✓ Window created");
    println!("\nRendering colored rectangles...");
    println!("Press ESC or close window to exit.\n");
    
    // Create test rectangles
    let rects = vec![
        // Red square
        (
            Rect { x: 50.0, y: 50.0, width: 100.0, height: 100.0 },
            Color::new(255, 0, 0, 255)
        ),
        // Green square
        (
            Rect { x: 200.0, y: 50.0, width: 100.0, height: 100.0 },
            Color::new(0, 255, 0, 255)
        ),
        // Blue square
        (
            Rect { x: 350.0, y: 50.0, width: 100.0, height: 100.0 },
            Color::new(0, 0, 255, 255)
        ),
        // Semi-transparent yellow rectangle
        (
            Rect { x: 100.0, y: 200.0, width: 300.0, height: 150.0 },
            Color::new(255, 255, 0, 128)
        ),
    ];
    
    // Run event loop with rendering
    window.run_with_renderer(move |renderer, event| {
        match event {
            WindowEvent::RedrawRequested => {
                if let Err(e) = renderer.render_rects(&rects) {
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
