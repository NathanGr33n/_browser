use browser_engine::net::PageLoader;
use browser_engine::style;
use browser_engine::layout;
use browser_engine::display::build_display_list;
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Browser Engine - Network Demo ===\n");

    // Create page loader
    let loader = PageLoader::new();

    // Example 1: Load example.com (simple HTML)
    println!("Example 1: Fetching example.com...");
    let url = Url::parse("http://example.com")?;
    
    match loader.load_page(&url) {
        Ok(page) => {
            println!("✓ Successfully fetched: {}", page.url);
            println!("  Stylesheets loaded: {}", page.stylesheets.len());
            
            // Merge stylesheets
            let stylesheet = page.merged_stylesheet();
            println!("  Total CSS rules: {}", stylesheet.rules.len());
            
            // Compute styles
            let styled = style::style_tree(&page.dom, &stylesheet);
            println!("✓ Computed styles");
            
            // Compute layout for 800x600 viewport
            let mut containing_block = layout::Dimensions::default();
            containing_block.content.width = 800.0;
            containing_block.content.height = 600.0;
            let layout_tree = layout::layout_tree(&styled, containing_block);
            println!("✓ Computed layout");
            
            // Generate display list
            let display_list = build_display_list(&layout_tree);
            println!("✓ Generated display list with {} items", display_list.len());
            
            println!();
        }
        Err(e) => {
            println!("✗ Failed to fetch: {}", e);
            println!();
        }
    }

    // Example 2: Try loading another URL (httpbin.org for testing)
    println!("Example 2: Fetching httpbin.org/html...");
    let url2 = Url::parse("http://httpbin.org/html")?;
    
    match loader.load_page(&url2) {
        Ok(page) => {
            println!("✓ Successfully fetched: {}", page.url);
            println!("  Stylesheets loaded: {}", page.stylesheets.len());
            
            let stylesheet = page.merged_stylesheet();
            println!("  Total CSS rules: {}", stylesheet.rules.len());
            
            let styled = style::style_tree(&page.dom, &stylesheet);
            let mut containing_block = layout::Dimensions::default();
            containing_block.content.width = 800.0;
            containing_block.content.height = 600.0;
            let layout_tree = layout::layout_tree(&styled, containing_block);
            let display_list = build_display_list(&layout_tree);
            
            println!("✓ Generated display list with {} items", display_list.len());
            println!();
        }
        Err(e) => {
            println!("✗ Failed to fetch: {}", e);
            println!();
        }
    }

    // Show cache statistics
    println!("Cache Statistics:");
    println!("  Resources cached: {}", loader.resource_loader().cache_count());
    println!("  Cache size: {} bytes", loader.resource_loader().cache_size());
    println!();

    println!("=== Demo Complete ===");
    println!("The browser engine successfully:");
    println!("  • Fetched HTML from the network");
    println!("  • Parsed HTML to DOM");
    println!("  • Extracted and fetched CSS");
    println!("  • Computed styles with cascade");
    println!("  • Built layout with box model");
    println!("  • Generated display list for rendering");
    println!("\nNext steps: Add window with address bar and navigation controls!");

    Ok(())
}
