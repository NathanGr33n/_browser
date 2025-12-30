mod dom;
mod html;
mod css;
mod style;
mod layout;

use html::HtmlParser;
use css::CssParser;
use style::style_tree;
use layout::{layout_tree, Dimensions};

fn main() {
    println!("=== Browser Engine Phase 1 Demo ===");
    
    // Sample HTML
    let html_source = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Test Page</title>
            </head>
            <body>
                <div id="main" class="container">
                    <h1>Hello, Browser!</h1>
                    <p>This is a test paragraph.</p>
                </div>
            </body>
        </html>
    "#;
    
    // Sample CSS
    let css_source = r#"
        body {
            display: block;
            margin: 8px;
        }
        
        #main {
            width: 600px;
            padding: 20px;
            margin: 10px;
        }
        
        h1 {
            font-size: 24px;
            color: #333;
            margin: 10px;
        }
        
        p {
            font-size: 16px;
            color: #666;
            line-height: 1.5;
        }
    "#;
    
    println!("\n1. Parsing HTML...");
    let dom = HtmlParser::parse(html_source);
    println!("   ✓ DOM tree created with {} children", dom.children.len());
    
    println!("\n2. Parsing CSS...");
    let stylesheet = CssParser::parse(css_source);
    println!("   ✓ Stylesheet parsed with {} rules", stylesheet.rules.len());
    
    println!("\n3. Computing styles...");
    let styled_tree = style_tree(&dom, &stylesheet);
    println!("   ✓ Style tree computed");
    
    println!("\n4. Calculating layout...");
    let mut viewport = Dimensions::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;
    
    let layout_tree = layout_tree(&styled_tree, viewport);
    println!("   ✓ Layout calculated");
    println!("   Root box: {}x{} at ({}, {})",
        layout_tree.dimensions.content.width,
        layout_tree.dimensions.content.height,
        layout_tree.dimensions.content.x,
        layout_tree.dimensions.content.y
    );
    
    println!("\n=== Phase 1 Complete ===");
    println!("HTML Parser: ✓");
    println!("CSS Parser: ✓");
    println!("Style Computation: ✓");
    println!("Layout Engine: ✓");
    println!("\nNext steps: Phase 2 - Rendering Pipeline");
}
