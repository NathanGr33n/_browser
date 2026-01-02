// Full browser engine demo - HTML+CSS to pixels
use browser_engine::{
    html::HtmlParser,
    css::CssParser,
    style::style_tree,
    layout::{layout_tree, Dimensions},
    display::{build_display_list, DisplayCommand},
    window::{Window, WindowConfig},
    css::Color,
    layout::Rect,
};
use winit::event::WindowEvent;

fn main() {
    println!("=== Browser Engine: Full Pipeline Demo ===\n");
    
    // Sample HTML document
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Browser Engine Demo</title>
</head>
<body>
    <div id="header">
        <h1>Browser Engine Demo</h1>
        <p>Rust-based rendering pipeline</p>
    </div>
    
    <div class="content">
        <div class="box red">
            Red Box
        </div>
        <div class="box green">
            Green Box
        </div>
        <div class="box blue">
            Blue Box
        </div>
    </div>
    
    <div id="footer">
        Phase 2 Complete!
    </div>
</body>
</html>
    "#;
    
    // CSS styles
    let css = r#"
body {
    display: block;
    margin: 20px;
    background-color: #f5f5f5;
}

#header {
    background-color: #2c3e50;
    padding: 20px;
    margin-bottom: 20px;
}

h1 {
    color: #ecf0f1;
    font-size: 32px;
    margin: 0 0 10px 0;
}

p {
    color: #bdc3c7;
    font-size: 16px;
    margin: 0;
}

.content {
    display: block;
    margin-bottom: 20px;
}

.box {
    width: 150px;
    height: 100px;
    margin: 10px;
    padding: 15px;
    display: block;
    border-width: 3px;
    border-color: #34495e;
}

.red {
    background-color: #e74c3c;
}

.green {
    background-color: #2ecc71;
}

.blue {
    background-color: #3498db;
}

#footer {
    background-color: #34495e;
    color: #ecf0f1;
    padding: 15px;
    text-align: center;
}
    "#;
    
    println!("1. Parsing HTML...");
    let dom = HtmlParser::parse(html);
    println!("   ✓ DOM tree created");
    
    println!("\n2. Parsing CSS...");
    let stylesheet = CssParser::parse(css);
    println!("   ✓ Stylesheet parsed ({} rules)", stylesheet.rules.len());
    
    println!("\n3. Computing styles...");
    let styled = style_tree(&dom, &stylesheet);
    println!("   ✓ Style tree computed");
    
    println!("\n4. Calculating layout...");
    let mut viewport = Dimensions::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;
    let layout_root = layout_tree(&styled, viewport);
    println!("   ✓ Layout complete");
    
    println!("\n5. Building display list...");
    let display_list = build_display_list(&layout_root);
    println!("   ✓ {} display commands generated", display_list.len());
    
    // Convert display list to renderable data
    let (backgrounds, borders) = extract_render_data(&display_list);
    println!("   - {} backgrounds", backgrounds.len());
    println!("   - {} borders", borders.len());
    
    println!("\n6. Opening render window...");
    let window = Window::new(WindowConfig {
        title: "Browser Engine - Full Demo".to_string(),
        width: 800,
        height: 600,
        resizable: true,
    }).expect("Failed to create window");
    
    println!("   ✓ Window created\n");
    println!("Rendering HTML+CSS...");
    println!("Press ESC to exit.\n");
    
    // Run with renderer
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
                println!("\nDemo finished!");
                return false;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) {
                    println!("\nESC pressed. Exiting...");
                    return false;
                }
            }
            _ => {}
        }
        true
    }).expect("Event loop error");
}

/// Extract renderable data from display list
fn extract_render_data(display_list: &[DisplayCommand]) -> (Vec<(Rect, Color)>, Vec<(Rect, Color, (f32, f32, f32, f32))>) {
    let mut backgrounds = Vec::new();
    let mut borders = Vec::new();
    
    for cmd in display_list {
        match cmd {
            DisplayCommand::SolidRect { color, rect } => {
                backgrounds.push((*rect, *color));
            }
            DisplayCommand::Border { color, rect, widths } => {
                borders.push((*rect, *color, *widths));
            }
            DisplayCommand::Text { .. } => {
                // Text rendering not yet implemented
            }
            DisplayCommand::Image { .. } => {
                // Image rendering not yet implemented
            }
        }
    }
    
    (backgrounds, borders)
}
