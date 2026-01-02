// CSS Positioning implementation

use crate::css::{Value, Unit};
use crate::layout::{Dimensions, Rect};
use crate::style::StyledNode;

/// CSS position property values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// Default flow layout (not positioned)
    Static,
    /// Positioned relative to normal position
    Relative,
    /// Positioned relative to nearest positioned ancestor
    Absolute,
    /// Positioned relative to viewport (doesn't scroll)
    Fixed,
    /// Hybrid: relative until scroll threshold, then fixed
    Sticky,
}

impl Default for Position {
    fn default() -> Self {
        Position::Static
    }
}

/// Offset properties for positioned elements (top, right, bottom, left)
#[derive(Debug, Clone, Copy, Default)]
pub struct Offsets {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

/// Information about a positioned element
#[derive(Debug, Clone)]
pub struct PositionedElement {
    pub position: Position,
    pub offsets: Offsets,
    pub z_index: i32,
}

impl Default for PositionedElement {
    fn default() -> Self {
        Self {
            position: Position::Static,
            offsets: Offsets::default(),
            z_index: 0,
        }
    }
}

impl PositionedElement {
    /// Parse positioning properties from a styled node
    pub fn from_styled_node(node: &StyledNode) -> Self {
        let position = Self::parse_position(node);
        let offsets = Self::parse_offsets(node);
        let z_index = Self::parse_z_index(node);

        Self {
            position,
            offsets,
            z_index,
        }
    }

    /// Parse the position property
    fn parse_position(node: &StyledNode) -> Position {
        match node.value("position") {
            Some(Value::Keyword(s)) => match s.as_str() {
                "static" => Position::Static,
                "relative" => Position::Relative,
                "absolute" => Position::Absolute,
                "fixed" => Position::Fixed,
                "sticky" => Position::Sticky,
                _ => Position::Static,
            },
            _ => Position::Static,
        }
    }

    /// Parse offset properties (top, right, bottom, left)
    fn parse_offsets(node: &StyledNode) -> Offsets {
        Offsets {
            top: Self::parse_offset(node, "top"),
            right: Self::parse_offset(node, "right"),
            bottom: Self::parse_offset(node, "bottom"),
            left: Self::parse_offset(node, "left"),
        }
    }

    /// Parse a single offset property
    fn parse_offset(node: &StyledNode, property: &str) -> Option<f32> {
        match node.value(property) {
            Some(Value::Length(val, Unit::Px)) => Some(*val),
            Some(Value::Keyword(s)) if s == "auto" => None,
            _ => None,
        }
    }

    /// Parse z-index property
    fn parse_z_index(node: &StyledNode) -> i32 {
        match node.value("z-index") {
            Some(Value::Number(n)) => *n as i32,
            Some(Value::Length(n, Unit::Px)) => *n as i32,
            Some(Value::Keyword(s)) if s == "auto" => 0,
            _ => 0,
        }
    }

    /// Check if this element is positioned (not static)
    pub fn is_positioned(&self) -> bool {
        self.position != Position::Static
    }

    /// Apply positioning to dimensions
    pub fn apply_positioning(
        &self,
        dimensions: &mut Dimensions,
        containing_block: &Dimensions,
        viewport: &Rect,
    ) {
        match self.position {
            Position::Static => {
                // No adjustment needed
            }
            Position::Relative => {
                self.apply_relative_positioning(dimensions);
            }
            Position::Absolute => {
                self.apply_absolute_positioning(dimensions, containing_block);
            }
            Position::Fixed => {
                self.apply_fixed_positioning(dimensions, viewport);
            }
            Position::Sticky => {
                // Sticky is complex - for now treat as relative
                // TODO: Implement proper sticky behavior with scroll tracking
                self.apply_relative_positioning(dimensions);
            }
        }
    }

    /// Apply relative positioning (offset from normal position)
    fn apply_relative_positioning(&self, dimensions: &mut Dimensions) {
        // Apply offsets to the positioned element
        if let Some(top) = self.offsets.top {
            dimensions.content.y += top;
        } else if let Some(bottom) = self.offsets.bottom {
            dimensions.content.y -= bottom;
        }

        if let Some(left) = self.offsets.left {
            dimensions.content.x += left;
        } else if let Some(right) = self.offsets.right {
            dimensions.content.x -= right;
        }
    }

    /// Apply absolute positioning (relative to containing block)
    fn apply_absolute_positioning(&self, dimensions: &mut Dimensions, containing_block: &Dimensions) {
        let cb = &containing_block.content;

        // Start with containing block position
        dimensions.content.x = cb.x;
        dimensions.content.y = cb.y;

        // Apply offsets
        if let Some(left) = self.offsets.left {
            dimensions.content.x = cb.x + left;
        } else if let Some(right) = self.offsets.right {
            dimensions.content.x = cb.x + cb.width - dimensions.content.width - right;
        }

        if let Some(top) = self.offsets.top {
            dimensions.content.y = cb.y + top;
        } else if let Some(bottom) = self.offsets.bottom {
            dimensions.content.y = cb.y + cb.height - dimensions.content.height - bottom;
        }
    }

    /// Apply fixed positioning (relative to viewport)
    fn apply_fixed_positioning(&self, dimensions: &mut Dimensions, viewport: &Rect) {
        // Start with viewport position
        dimensions.content.x = viewport.x;
        dimensions.content.y = viewport.y;

        // Apply offsets
        if let Some(left) = self.offsets.left {
            dimensions.content.x = viewport.x + left;
        } else if let Some(right) = self.offsets.right {
            dimensions.content.x = viewport.x + viewport.width - dimensions.content.width - right;
        }

        if let Some(top) = self.offsets.top {
            dimensions.content.y = viewport.y + top;
        } else if let Some(bottom) = self.offsets.bottom {
            dimensions.content.y = viewport.y + viewport.height - dimensions.content.height - bottom;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_default() {
        let pos = Position::default();
        assert_eq!(pos, Position::Static);
    }

    #[test]
    fn test_positioned_element_default() {
        let elem = PositionedElement::default();
        assert_eq!(elem.position, Position::Static);
        assert!(!elem.is_positioned());
        assert_eq!(elem.z_index, 0);
    }

    #[test]
    fn test_is_positioned() {
        let static_elem = PositionedElement {
            position: Position::Static,
            ..Default::default()
        };
        assert!(!static_elem.is_positioned());

        let relative_elem = PositionedElement {
            position: Position::Relative,
            ..Default::default()
        };
        assert!(relative_elem.is_positioned());

        let absolute_elem = PositionedElement {
            position: Position::Absolute,
            ..Default::default()
        };
        assert!(absolute_elem.is_positioned());
    }

    #[test]
    fn test_relative_positioning() {
        let mut dims = Dimensions::default();
        dims.content.x = 100.0;
        dims.content.y = 100.0;
        dims.content.width = 50.0;
        dims.content.height = 50.0;

        let positioned = PositionedElement {
            position: Position::Relative,
            offsets: Offsets {
                top: Some(20.0),
                left: Some(30.0),
                bottom: None,
                right: None,
            },
            z_index: 0,
        };

        let viewport = Rect::default();
        let containing_block = Dimensions::default();
        positioned.apply_positioning(&mut dims, &containing_block, &viewport);

        assert_eq!(dims.content.x, 130.0); // 100 + 30
        assert_eq!(dims.content.y, 120.0); // 100 + 20
    }

    #[test]
    fn test_absolute_positioning() {
        let mut dims = Dimensions::default();
        dims.content.width = 50.0;
        dims.content.height = 50.0;

        let mut containing_block = Dimensions::default();
        containing_block.content.x = 200.0;
        containing_block.content.y = 200.0;
        containing_block.content.width = 400.0;
        containing_block.content.height = 400.0;

        let positioned = PositionedElement {
            position: Position::Absolute,
            offsets: Offsets {
                top: Some(10.0),
                left: Some(20.0),
                bottom: None,
                right: None,
            },
            z_index: 0,
        };

        let viewport = Rect::default();
        positioned.apply_positioning(&mut dims, &containing_block, &viewport);

        assert_eq!(dims.content.x, 220.0); // 200 + 20
        assert_eq!(dims.content.y, 210.0); // 200 + 10
    }

    #[test]
    fn test_absolute_positioning_right_bottom() {
        let mut dims = Dimensions::default();
        dims.content.width = 50.0;
        dims.content.height = 50.0;

        let mut containing_block = Dimensions::default();
        containing_block.content.x = 0.0;
        containing_block.content.y = 0.0;
        containing_block.content.width = 400.0;
        containing_block.content.height = 400.0;

        let positioned = PositionedElement {
            position: Position::Absolute,
            offsets: Offsets {
                top: None,
                left: None,
                bottom: Some(10.0),
                right: Some(20.0),
            },
            z_index: 0,
        };

        let viewport = Rect::default();
        positioned.apply_positioning(&mut dims, &containing_block, &viewport);

        // Right: 400 - 50 (width) - 20 = 330
        // Bottom: 400 - 50 (height) - 10 = 340
        assert_eq!(dims.content.x, 330.0);
        assert_eq!(dims.content.y, 340.0);
    }

    #[test]
    fn test_fixed_positioning() {
        let mut dims = Dimensions::default();
        dims.content.width = 50.0;
        dims.content.height = 50.0;

        let viewport = Rect {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        };

        let positioned = PositionedElement {
            position: Position::Fixed,
            offsets: Offsets {
                top: Some(10.0),
                right: Some(10.0),
                bottom: None,
                left: None,
            },
            z_index: 0,
        };

        let containing_block = Dimensions::default();
        positioned.apply_positioning(&mut dims, &containing_block, &viewport);

        // Right: 800 - 50 (width) - 10 = 740
        assert_eq!(dims.content.x, 740.0);
        assert_eq!(dims.content.y, 10.0);
    }
}
