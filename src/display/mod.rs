use crate::css::{Color, Value};
use crate::layout::{LayoutBox, Rect};
use url::Url;

/// A display list is a list of graphics operations to perform
pub type DisplayList = Vec<DisplayCommand>;

/// A single graphics operation
#[derive(Debug, Clone)]
pub enum DisplayCommand {
    /// Fill a solid rectangle
    SolidRect {
        color: Color,
        rect: Rect,
    },
    /// Draw a border around a rectangle
    Border {
        color: Color,
        rect: Rect,
        /// Border widths: left, right, top, bottom
        widths: (f32, f32, f32, f32),
    },
    /// Draw text at a position (placeholder for now)
    Text {
        text: String,
        rect: Rect,
        color: Color,
    },
    /// Draw an image
    Image {
        url: Url,
        rect: Rect,
    },
}

/// Build a display list from a layout tree
pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

/// Render a layout box and its descendants into the display list
fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    // Render the box's background first
    render_background(list, layout_box);
    
    // Then render borders on top
    render_borders(list, layout_box);
    
    // Render images if this is an img element
    render_image(list, layout_box);
    
    // Render text content if present
    render_text(list, layout_box);
    
    // Recursively render children
    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

/// Render the background of a layout box
fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    // Get the background color from the styled node
    if let Some(color) = get_color(layout_box, "background-color") {
        list.push(DisplayCommand::SolidRect {
            color,
            rect: layout_box.dimensions.border_box(),
        });
    }
}

/// Render the borders of a layout box
fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let d = &layout_box.dimensions;
    let border = &d.border;
    
    // Only render if at least one border has width
    if border.left > 0.0 || border.right > 0.0 || border.top > 0.0 || border.bottom > 0.0 {
        if let Some(color) = get_color(layout_box, "border-color") {
            list.push(DisplayCommand::Border {
                color,
                rect: layout_box.dimensions.border_box(),
                widths: (border.left, border.right, border.top, border.bottom),
            });
        }
    }
}

/// Render image element
fn render_image(list: &mut DisplayList, layout_box: &LayoutBox) {
    if let Some(style_node) = layout_box.get_styled_node() {
        if let Some(elem) = style_node.node.element_data() {
            // Check if this is an img element
            if elem.tag_name.to_lowercase() == "img" {
                if let Some(src) = elem.attributes.get("src") {
                    // Parse URL from src attribute
                    if let Ok(url) = Url::parse(src) {
                        list.push(DisplayCommand::Image {
                            url,
                            rect: layout_box.dimensions.content,
                        });
                    }
                }
            }
        }
    }
}

/// Render text content of a layout box (placeholder)
fn render_text(list: &mut DisplayList, layout_box: &LayoutBox) {
    // Get the styled node
    if let Some(style_node) = layout_box.get_styled_node() {
        // Check if this is a text node
        if let Some(text) = style_node.node.text_content() {
            if !text.trim().is_empty() {
                let color = get_color(layout_box, "color")
                    .unwrap_or(Color::new(0, 0, 0, 255)); // Default to black
                
                list.push(DisplayCommand::Text {
                    text: text.to_string(),
                    rect: layout_box.dimensions.border_box(),
                    color,
                });
            }
        }
    }
}

/// Helper to extract a color value from a layout box
fn get_color(layout_box: &LayoutBox, property: &str) -> Option<Color> {
    layout_box
        .get_styled_node()
        .and_then(|style_node| style_node.value(property))
        .and_then(|value| match value {
            Value::Color(color) => Some(*color),
            _ => None,
        })
}

/// Optimize display list by removing occluded items
/// 
/// This is a simple optimization that removes items completely covered by opaque items
pub fn optimize_display_list(list: DisplayList) -> DisplayList {
    // For now, just return the original list
    // TODO: Implement occlusion culling and other optimizations
    list
}

/// Cull display items outside the viewport
pub fn cull_display_list(list: DisplayList, viewport: Rect) -> DisplayList {
    list.into_iter()
        .filter(|item| {
            let rect = match item {
                DisplayCommand::SolidRect { rect, .. } => rect,
                DisplayCommand::Border { rect, .. } => rect,
                DisplayCommand::Text { rect, .. } => rect,
                DisplayCommand::Image { rect, .. } => rect,
            };
            
            // Check if rectangles intersect
            rectangles_intersect(rect, &viewport)
        })
        .collect()
}

/// Check if two rectangles intersect
fn rectangles_intersect(a: &Rect, b: &Rect) -> bool {
    a.x < b.x + b.width
        && a.x + a.width > b.x
        && a.y < b.y + b.height
        && a.y + a.height > b.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::{CssParser, Unit};
    use crate::dom::Node;
    use crate::style::style_tree;
    use crate::layout::{layout_tree, Dimensions};
    use std::collections::HashMap;

    #[test]
    fn test_build_display_list_simple() {
        // Create a simple DOM with styled div
        let css = "div { background-color: #ff0000; width: 100px; height: 50px; }";
        let stylesheet = CssParser::parse(css);
        
        let node = Node::element("div".to_string(), HashMap::new(), vec![]);
        let styled = style_tree(&node, &stylesheet);
        
        let mut viewport = Dimensions::default();
        viewport.content.width = 800.0;
        viewport.content.height = 600.0;
        
        let layout = layout_tree(&styled, viewport);
        let display_list = build_display_list(&layout);
        
        // Should have at least the background rectangle
        assert!(!display_list.is_empty());
    }

    #[test]
    fn test_rectangles_intersect() {
        let a = Rect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
        let b = Rect { x: 50.0, y: 50.0, width: 100.0, height: 100.0 };
        let c = Rect { x: 200.0, y: 200.0, width: 100.0, height: 100.0 };
        
        assert!(rectangles_intersect(&a, &b));
        assert!(rectangles_intersect(&b, &a));
        assert!(!rectangles_intersect(&a, &c));
        assert!(!rectangles_intersect(&c, &a));
    }

    #[test]
    fn test_cull_display_list() {
        let viewport = Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 };
        
        let list = vec![
            DisplayCommand::SolidRect {
                color: Color::new(255, 0, 0, 255),
                rect: Rect { x: 10.0, y: 10.0, width: 50.0, height: 50.0 },
            },
            DisplayCommand::SolidRect {
                color: Color::new(0, 255, 0, 255),
                rect: Rect { x: 1000.0, y: 1000.0, width: 50.0, height: 50.0 },
            },
        ];
        
        let culled = cull_display_list(list, viewport);
        
        // Only the first item should remain (second is outside viewport)
        assert_eq!(culled.len(), 1);
    }
    
    #[test]
    fn test_image_display_command() {
        // Create DOM with img element
        let mut attrs = HashMap::new();
        attrs.insert("src".to_string(), "http://example.com/test.png".to_string());
        
        let node = Node::element("img".to_string(), attrs, vec![]);
        let css = "img { width: 100px; height: 100px; }";
        let stylesheet = CssParser::parse(css);
        let styled = style_tree(&node, &stylesheet);
        
        let mut viewport = Dimensions::default();
        viewport.content.width = 800.0;
        viewport.content.height = 600.0;
        
        let layout = layout_tree(&styled, viewport);
        let display_list = build_display_list(&layout);
        
        // Should have an image command
        let has_image = display_list.iter().any(|cmd| matches!(cmd, DisplayCommand::Image { .. }));
        assert!(has_image);
    }
}
