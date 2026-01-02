// Test binary for border rendering
use browser_engine::window::{Window, WindowConfig};
use browser_engine::css::Color;
use browser_engine::layout::Rect;
use winit::event::WindowEvent;

fn main() {
    println!("=== Border Rendering Test ===\n");
    
    let window = Window::new(WindowConfig::default())
        .expect("Failed to create window");
    
    println!("✓ Window created");
    println!("\nRendering boxes with borders...");
    println!("Press ESC or close window to exit.\n");
    
    // Create test data
    let backgrounds = vec![
        // Light blue background for first box
        (
            Rect { x: 50.0, y: 50.0, width: 200.0, height: 150.0 },
            Color::new(200, 220, 255, 255),
        ),
        // Light green background for second box
        (
            Rect { x: 300.0, y: 50.0, width: 200.0, height: 150.0 },
            Color::new(200, 255, 200, 255),
        ),
        // Light pink background for third box
        (
            Rect { x: 550.0, y: 50.0, width: 200.0, height: 150.0 },
            Color::new(255, 200, 220, 255),
        ),
        // Large box at bottom
        (
            Rect { x: 50.0, y: 250.0, width: 700.0, height: 250.0 },
            Color::new(240, 240, 240, 255),
        ),
    ];
    
    let borders = vec![
        // Red border on first box (uniform 5px)
        (
            Rect { x: 50.0, y: 50.0, width: 200.0, height: 150.0 },
            Color::new(255, 0, 0, 255),
            (5.0, 5.0, 5.0, 5.0), // left, right, top, bottom
        ),
        // Green border on second box (thick top and bottom)
        (
            Rect { x: 300.0, y: 50.0, width: 200.0, height: 150.0 },
            Color::new(0, 200, 0, 255),
            (2.0, 2.0, 10.0, 10.0),
        ),
        // Blue border on third box (asymmetric)
        (
            Rect { x: 550.0, y: 50.0, width: 200.0, height: 150.0 },
            Color::new(0, 0, 255, 255),
            (10.0, 2.0, 2.0, 8.0),
        ),
        // Dark gray border on large box
        (
            Rect { x: 50.0, y: 250.0, width: 700.0, height: 250.0 },
            Color::new(80, 80, 80, 255),
            (8.0, 8.0, 8.0, 8.0),
        ),
    ];
    
    // Run event loop with rendering
    window.run_with_renderer(move |renderer, event| {
        match event {
            WindowEvent::RedrawRequested => {
                if let Err(e) = renderer.render_rects_and_borders(&backgrounds, &borders) {
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
