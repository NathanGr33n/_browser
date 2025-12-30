use crate::css::{Value, Unit};
use crate::style::{StyledNode, Display};

/// A box in the layout tree
#[derive(Debug, Clone)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

/// Type of box (block, inline, or anonymous)
#[derive(Debug, Clone)]
pub enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}

/// CSS box model dimensions
#[derive(Debug, Clone, Copy, Default)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

/// Rectangle dimensions
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Edge sizes (margin, border, padding)
#[derive(Debug, Clone, Copy, Default)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Dimensions {
    /// Total area occupied by the box including padding, border, and margin
    pub fn padding_box(&self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    pub fn border_box(&self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }

    pub fn margin_box(&self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

impl Rect {
    pub fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

impl<'a> LayoutBox<'a> {
    /// Create a new layout box
    fn new(box_type: BoxType<'a>) -> LayoutBox<'a> {
        LayoutBox {
            box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    /// Get the styled node for this box
    fn get_styled_node(&self) -> Option<&'a StyledNode<'a>> {
        match &self.box_type {
            BoxType::BlockNode(node) | BoxType::InlineNode(node) => Some(node),
            BoxType::AnonymousBlock => None,
        }
    }

    /// Lay out a box and its descendants
    pub fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => {
                // Simplified: treat inline as block for now
                self.layout_block(containing_block)
            }
        }
    }

    /// Lay out a block-level element
    fn layout_block(&mut self, containing_block: Dimensions) {
        // Calculate width
        self.calculate_block_width(containing_block);

        // Determine position
        self.calculate_block_position(containing_block);

        // Lay out children
        self.layout_block_children();

        // Calculate height based on children
        self.calculate_block_height();
    }

    /// Calculate width of a block box
    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = match self.get_styled_node() {
            Some(node) => node,
            None => return,
        };

        let auto = Value::Keyword("auto".to_string());
        let zero = Value::Length(0.0, Unit::Px);

        let mut width = style.value("width").unwrap_or(&auto).clone();

        let margin_left = style.lookup("margin-left", "margin", &zero);
        let margin_right = style.lookup("margin-right", "margin", &zero);
        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);
        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total = [
            &margin_left,
            &margin_right,
            &border_left,
            &border_right,
            &padding_left,
            &padding_right,
            &width,
        ]
        .iter()
        .map(|v| v.to_px())
        .sum::<f32>();

        // If width is not auto and total is larger than container, treat auto margins as 0
        if width != auto && total > containing_block.content.width {
            // For now, just clip
        }

        // Calculate width
        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => {
                // Over-constrained: ignore margin-right
                // margin_right = underflow
            }
            (false, false, true) => {
                // Set margin-right
            }
            (false, true, false) => {
                // Set margin-left
            }
            (true, _, _) => {
                // Width is auto
                if margin_left == auto {
                    // margin_left = 0
                }
                if margin_right == auto {
                    // margin_right = 0
                }

                if underflow >= 0.0 {
                    width = Value::Length(underflow, Unit::Px);
                } else {
                    // Width would be negative, set to 0
                    width = Value::Length(0.0, Unit::Px);
                }
            }
            (false, true, true) => {
                // Both margins auto: center
            }
        }

        let d = &mut self.dimensions;
        d.content.width = width.to_px();
        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();
        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();
        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    /// Calculate position of a block box
    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = match self.get_styled_node() {
            Some(node) => node,
            None => return,
        };

        let zero = Value::Length(0.0, Unit::Px);

        let d = &mut self.dimensions;

        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style
            .lookup("border-top-width", "border-width", &zero)
            .to_px();
        d.border.bottom = style
            .lookup("border-bottom-width", "border-width", &zero)
            .to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;

        d.content.y = containing_block.content.y
            + containing_block.content.height
            + d.margin.top
            + d.border.top
            + d.padding.top;
    }

    /// Lay out children of a block box
    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            // Track cumulative height
            d.content.height += child.dimensions.margin_box().height;
        }
    }

    /// Calculate height of a block box
    fn calculate_block_height(&mut self) {
        // If height is explicitly set, use that
        if let Some(style) = self.get_styled_node() {
            if let Some(Value::Length(h, Unit::Px)) = style.value("height") {
                self.dimensions.content.height = *h;
            }
        }
    }
}

/// Build the layout tree from a styled tree
pub fn layout_tree<'a>(
    node: &'a StyledNode<'a>,
    mut containing_block: Dimensions,
) -> LayoutBox<'a> {
    // Initialize containing block
    containing_block.content.height = 0.0;

    let mut root_box = build_layout_tree(node);
    root_box.layout(containing_block);
    root_box
}

/// Build layout tree from styled node
fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => BoxType::BlockNode(style_node),
        Display::Inline => BoxType::InlineNode(style_node),
        Display::None => panic!("Root node has display: none"),
    });

    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => {
                // For simplicity, treat inline as block for now
                root.children.push(build_layout_tree(child))
            }
            Display::None => {} // Skip nodes with display: none
        }
    }

    root
}

/// Extension trait to convert CSS values to pixels
trait ToPx {
    fn to_px(&self) -> f32;
}

impl ToPx for Value {
    fn to_px(&self) -> f32 {
        match self {
            Value::Length(length, Unit::Px) => *length,
            Value::Number(n) => *n,
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::CssParser;
    use crate::dom::Node;
    use crate::style::style_tree;
    use std::collections::HashMap;

    #[test]
    fn test_layout_basic() {
        let html = Node::element("div".to_string(), HashMap::new(), vec![]);
        let css = CssParser::parse("div { width: 100px; height: 50px; }");
        let styled = style_tree(&html, &css);

        let mut viewport = Dimensions::default();
        viewport.content.width = 800.0;
        viewport.content.height = 600.0;

        let layout = layout_tree(&styled, viewport);

        assert_eq!(layout.dimensions.content.width, 100.0);
        assert_eq!(layout.dimensions.content.height, 50.0);
    }
}
