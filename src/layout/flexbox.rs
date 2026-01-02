// Flexbox layout implementation

use crate::css::{Value, Unit};
use crate::layout::{Dimensions, Rect};
use crate::style::StyledNode;

/// Flexbox direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Row
    }
}

/// Flexbox wrap
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        FlexWrap::NoWrap
    }
}

/// Justify content
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::FlexStart
    }
}

/// Align items
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Stretch
    }
}

/// Flexbox container properties
#[derive(Debug, Clone)]
pub struct FlexContainer {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
}

impl Default for FlexContainer {
    fn default() -> Self {
        Self {
            direction: FlexDirection::default(),
            wrap: FlexWrap::default(),
            justify_content: JustifyContent::default(),
            align_items: AlignItems::default(),
        }
    }
}

/// Flex item properties
#[derive(Debug, Clone)]
pub struct FlexItem {
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
    pub min_size: Option<f32>,
    pub max_size: Option<f32>,
}

/// Internal flex item state during layout calculation
#[derive(Debug, Clone)]
struct FlexItemState {
    /// Base size before flex calculations
    base_size: f32,
    /// Hypothetical main size
    hypothetical_size: f32,
    /// Final main size after flex
    main_size: f32,
    /// Cross size
    cross_size: f32,
    /// Flex factor (grow or shrink)
    flex_factor: f32,
    /// Is frozen (no longer flexible)
    frozen: bool,
    /// Outer main size (including margins)
    outer_main_size: f32,
    /// Outer cross size (including margins)
    outer_cross_size: f32,
}

/// Flex line containing items
#[derive(Debug)]
struct FlexLine {
    /// Indices of items in this line
    items: Vec<usize>,
    /// Main size of the line
    main_size: f32,
    /// Cross size of the line
    cross_size: f32,
}

impl Default for FlexItem {
    fn default() -> Self {
        Self {
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            min_size: None,
            max_size: None,
        }
    }
}

impl FlexContainer {
    /// Parse flexbox properties from styled node
    pub fn from_styled_node(node: &StyledNode) -> Option<Self> {
        let display = node.value("display");
        
        match display {
            Some(Value::Keyword(ref s)) if s == "flex" || s == "inline-flex" => {
                let direction = Self::parse_direction(node);
                let wrap = Self::parse_wrap(node);
                let justify_content = Self::parse_justify_content(node);
                let align_items = Self::parse_align_items(node);
                
                Some(Self {
                    direction,
                    wrap,
                    justify_content,
                    align_items,
                })
            }
            _ => None,
        }
    }
    
    fn parse_direction(node: &StyledNode) -> FlexDirection {
        match node.value("flex-direction") {
            Some(Value::Keyword(ref s)) => match s.as_str() {
                "row" => FlexDirection::Row,
                "row-reverse" => FlexDirection::RowReverse,
                "column" => FlexDirection::Column,
                "column-reverse" => FlexDirection::ColumnReverse,
                _ => FlexDirection::default(),
            },
            _ => FlexDirection::default(),
        }
    }
    
    fn parse_wrap(node: &StyledNode) -> FlexWrap {
        match node.value("flex-wrap") {
            Some(Value::Keyword(ref s)) => match s.as_str() {
                "nowrap" => FlexWrap::NoWrap,
                "wrap" => FlexWrap::Wrap,
                "wrap-reverse" => FlexWrap::WrapReverse,
                _ => FlexWrap::default(),
            },
            _ => FlexWrap::default(),
        }
    }
    
    fn parse_justify_content(node: &StyledNode) -> JustifyContent {
        match node.value("justify-content") {
            Some(Value::Keyword(ref s)) => match s.as_str() {
                "flex-start" => JustifyContent::FlexStart,
                "flex-end" => JustifyContent::FlexEnd,
                "center" => JustifyContent::Center,
                "space-between" => JustifyContent::SpaceBetween,
                "space-around" => JustifyContent::SpaceAround,
                "space-evenly" => JustifyContent::SpaceEvenly,
                _ => JustifyContent::default(),
            },
            _ => JustifyContent::default(),
        }
    }
    
    fn parse_align_items(node: &StyledNode) -> AlignItems {
        match node.value("align-items") {
            Some(Value::Keyword(ref s)) => match s.as_str() {
                "flex-start" => AlignItems::FlexStart,
                "flex-end" => AlignItems::FlexEnd,
                "center" => AlignItems::Center,
                "stretch" => AlignItems::Stretch,
                "baseline" => AlignItems::Baseline,
                _ => AlignItems::default(),
            },
            _ => AlignItems::default(),
        }
    }
    
    /// Calculate flexbox layout following CSS Flexbox specification
    pub fn layout(
        &self,
        container_dimensions: Dimensions,
        items: &[FlexItem],
    ) -> Vec<Dimensions> {
        if items.is_empty() {
            return Vec::new();
        }

        // Step 1: Determine main and cross axis dimensions
        let (main_axis_size, cross_axis_size) = self.get_axis_sizes(&container_dimensions);
        
        // Step 2: Initialize flex item states
        let mut item_states = self.initialize_item_states(items, main_axis_size);
        
        // Step 3: Collect items into flex lines
        let lines = self.collect_flex_lines(&item_states, main_axis_size);
        
        // Step 4: Resolve flexible lengths (grow/shrink)
        let mut line_states = Vec::new();
        for line in &lines {
            let line_item_states = self.resolve_flexible_lengths(
                &mut item_states,
                line,
                main_axis_size,
            );
            line_states.push(line_item_states);
        }
        
        // Step 5: Calculate cross sizes
        self.calculate_cross_sizes(&mut item_states, &lines, cross_axis_size);
        
        // Step 6: Main axis alignment (justify-content)
        let positioned_items = self.align_main_axis(
            &item_states,
            &lines,
            main_axis_size,
        );
        
        // Step 7: Cross axis alignment (align-items)
        let final_positions = self.align_cross_axis(
            positioned_items,
            &item_states,
            &lines,
            cross_axis_size,
        );
        
        // Convert to Dimensions
        final_positions
    }
    
    /// Get main and cross axis sizes from container dimensions
    fn get_axis_sizes(&self, container: &Dimensions) -> (f32, f32) {
        match self.direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                (container.content.width, container.content.height)
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (container.content.height, container.content.width)
            }
        }
    }
    
    /// Initialize flex item states with base sizes
    fn initialize_item_states(&self, items: &[FlexItem], main_axis_size: f32) -> Vec<FlexItemState> {
        items
            .iter()
            .map(|item| {
                // Determine base size from flex-basis or content size
                let base_size = item.flex_basis.unwrap_or(0.0);
                
                // Calculate hypothetical main size
                let hypothetical_size = self.clamp_size(
                    base_size,
                    item.min_size,
                    item.max_size,
                );
                
                FlexItemState {
                    base_size,
                    hypothetical_size,
                    main_size: hypothetical_size,
                    cross_size: 0.0,
                    flex_factor: if hypothetical_size < base_size {
                        item.flex_shrink
                    } else {
                        item.flex_grow
                    },
                    frozen: false,
                    outer_main_size: hypothetical_size,
                    outer_cross_size: 0.0,
                }
            })
            .collect()
    }
    
    /// Clamp size between min and max constraints
    fn clamp_size(&self, size: f32, min: Option<f32>, max: Option<f32>) -> f32 {
        let mut result = size;
        if let Some(min_size) = min {
            result = result.max(min_size);
        }
        if let Some(max_size) = max {
            result = result.min(max_size);
        }
        result
    }
    
    /// Collect items into flex lines based on wrapping
    fn collect_flex_lines(&self, items: &[FlexItemState], main_axis_size: f32) -> Vec<FlexLine> {
        if self.wrap == FlexWrap::NoWrap || items.is_empty() {
            // Single line with all items
            return vec![FlexLine {
                items: (0..items.len()).collect(),
                main_size: items.iter().map(|item| item.outer_main_size).sum(),
                cross_size: items
                    .iter()
                    .map(|item| item.outer_cross_size)
                    .fold(0.0, f32::max),
            }];
        }
        
        // Multi-line: break into lines
        let mut lines = Vec::new();
        let mut current_line = Vec::new();
        let mut current_line_size = 0.0;
        
        for (idx, item) in items.iter().enumerate() {
            if current_line_size + item.outer_main_size > main_axis_size && !current_line.is_empty() {
                // Start new line
                let line_cross_size: f32 = current_line
                    .iter()
                    .map(|i: &usize| items[*i].outer_cross_size)
                    .fold(0.0_f32, f32::max);
                
                lines.push(FlexLine {
                    items: current_line.clone(),
                    main_size: current_line_size,
                    cross_size: line_cross_size,
                });
                
                current_line.clear();
                current_line_size = 0.0;
            }
            
            current_line.push(idx);
            current_line_size += item.outer_main_size;
        }
        
        // Add final line
        if !current_line.is_empty() {
            let line_cross_size = current_line
                .iter()
                .map(|&i| items[i].outer_cross_size)
                .fold(0.0, f32::max);
            
            lines.push(FlexLine {
                items: current_line,
                main_size: current_line_size,
                cross_size: line_cross_size,
            });
        }
        
        lines
    }
    
    /// Resolve flexible lengths (grow/shrink algorithm)
    fn resolve_flexible_lengths(
        &self,
        items: &mut [FlexItemState],
        line: &FlexLine,
        main_axis_size: f32,
    ) -> Vec<usize> {
        // Calculate available space
        let used_space: f32 = line.items.iter().map(|&i| items[i].hypothetical_size).sum();
        let free_space = main_axis_size - used_space;
        
        if free_space.abs() < 0.001 {
            // No flex needed
            return line.items.clone();
        }
        
        if free_space > 0.0 {
            // Grow items
            let total_grow: f32 = line.items.iter().map(|&i| items[i].flex_factor).sum();
            if total_grow > 0.0 {
                for &item_idx in &line.items {
                    let item = &mut items[item_idx];
                    let grow_factor = item.flex_factor / total_grow;
                    item.main_size = item.hypothetical_size + (free_space * grow_factor);
                    item.outer_main_size = item.main_size;
                }
            }
        } else {
            // Shrink items
            let total_shrink: f32 = line.items.iter().map(|&i| items[i].flex_factor).sum();
            if total_shrink > 0.0 {
                for &item_idx in &line.items {
                    let item = &mut items[item_idx];
                    let shrink_factor = item.flex_factor / total_shrink;
                    item.main_size = item.hypothetical_size + (free_space * shrink_factor);
                    item.outer_main_size = item.main_size.max(0.0);
                }
            }
        }
        
        line.items.clone()
    }
    
    /// Calculate cross sizes for items
    fn calculate_cross_sizes(
        &self,
        items: &mut [FlexItemState],
        lines: &[FlexLine],
        cross_axis_size: f32,
    ) {
        for line in lines {
            for &item_idx in &line.items {
                let item = &mut items[item_idx];
                // Simplified: use a default cross size
                item.cross_size = line.cross_size;
                item.outer_cross_size = item.cross_size;
            }
        }
    }
    
    /// Align items on main axis (justify-content)
    fn align_main_axis(
        &self,
        items: &[FlexItemState],
        lines: &[FlexLine],
        main_axis_size: f32,
    ) -> Vec<Dimensions> {
        let mut results = vec![Dimensions::default(); items.len()];
        
        for line in lines {
            let used_space: f32 = line.items.iter().map(|&i| items[i].outer_main_size).sum();
            let free_space = main_axis_size - used_space;
            
            let (mut position, spacing) = match self.justify_content {
                JustifyContent::FlexStart => (0.0, 0.0),
                JustifyContent::FlexEnd => (free_space, 0.0),
                JustifyContent::Center => (free_space / 2.0, 0.0),
                JustifyContent::SpaceBetween => {
                    let count = line.items.len();
                    if count > 1 {
                        (0.0, free_space / (count - 1) as f32)
                    } else {
                        (0.0, 0.0)
                    }
                }
                JustifyContent::SpaceAround => {
                    let count = line.items.len() as f32;
                    let gap = free_space / count;
                    (gap / 2.0, gap)
                }
                JustifyContent::SpaceEvenly => {
                    let count = (line.items.len() + 1) as f32;
                    let gap = free_space / count;
                    (gap, gap)
                }
            };
            
            for &item_idx in &line.items {
                let item = &items[item_idx];
                
                // Set position based on direction
                match self.direction {
                    FlexDirection::Row => {
                        results[item_idx].content.x = position;
                        results[item_idx].content.width = item.main_size;
                    }
                    FlexDirection::RowReverse => {
                        results[item_idx].content.x = main_axis_size - position - item.main_size;
                        results[item_idx].content.width = item.main_size;
                    }
                    FlexDirection::Column => {
                        results[item_idx].content.y = position;
                        results[item_idx].content.height = item.main_size;
                    }
                    FlexDirection::ColumnReverse => {
                        results[item_idx].content.y = main_axis_size - position - item.main_size;
                        results[item_idx].content.height = item.main_size;
                    }
                }
                
                position += item.outer_main_size + spacing;
            }
        }
        
        results
    }
    
    /// Align items on cross axis (align-items)
    fn align_cross_axis(
        &self,
        mut results: Vec<Dimensions>,
        items: &[FlexItemState],
        lines: &[FlexLine],
        cross_axis_size: f32,
    ) -> Vec<Dimensions> {
        let mut current_cross = 0.0;
        
        for line in lines {
            for &item_idx in &line.items {
                let item = &items[item_idx];
                
                let cross_position = match self.align_items {
                    AlignItems::FlexStart => current_cross,
                    AlignItems::FlexEnd => current_cross + line.cross_size - item.cross_size,
                    AlignItems::Center => current_cross + (line.cross_size - item.cross_size) / 2.0,
                    AlignItems::Stretch | AlignItems::Baseline => current_cross,
                };
                
                match self.direction {
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        results[item_idx].content.y = cross_position;
                        results[item_idx].content.height = if self.align_items == AlignItems::Stretch {
                            line.cross_size
                        } else {
                            item.cross_size
                        };
                    }
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        results[item_idx].content.x = cross_position;
                        results[item_idx].content.width = if self.align_items == AlignItems::Stretch {
                            line.cross_size
                        } else {
                            item.cross_size
                        };
                    }
                }
            }
            
            current_cross += line.cross_size;
        }
        
        results
    }
}

impl FlexItem {
    /// Parse flex item properties from styled node
    pub fn from_styled_node(node: &StyledNode) -> Self {
        let flex_grow = Self::parse_flex_grow(node);
        let flex_shrink = Self::parse_flex_shrink(node);
        let flex_basis = Self::parse_flex_basis(node);
        let min_size = Self::parse_min_size(node);
        let max_size = Self::parse_max_size(node);
        
        Self {
            flex_grow,
            flex_shrink,
            flex_basis,
            min_size,
            max_size,
        }
    }
    
    fn parse_flex_grow(node: &StyledNode) -> f32 {
        match node.value("flex-grow") {
            Some(Value::Length(val, Unit::Px)) => *val,
            _ => 0.0,
        }
    }
    
    fn parse_flex_shrink(node: &StyledNode) -> f32 {
        match node.value("flex-shrink") {
            Some(Value::Length(val, Unit::Px)) => *val,
            _ => 1.0,
        }
    }
    
    fn parse_flex_basis(node: &StyledNode) -> Option<f32> {
        match node.value("flex-basis") {
            Some(Value::Length(val, Unit::Px)) => Some(*val),
            _ => None,
        }
    }
    
    fn parse_min_size(node: &StyledNode) -> Option<f32> {
        match node.value("min-width") {
            Some(Value::Length(val, Unit::Px)) => Some(*val),
            _ => None,
        }
    }
    
    fn parse_max_size(node: &StyledNode) -> Option<f32> {
        match node.value("max-width") {
            Some(Value::Length(val, Unit::Px)) => Some(*val),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_flex_container_default() {
        let container = FlexContainer::default();
        assert_eq!(container.direction, FlexDirection::Row);
        assert_eq!(container.wrap, FlexWrap::NoWrap);
        assert_eq!(container.justify_content, JustifyContent::FlexStart);
        assert_eq!(container.align_items, AlignItems::Stretch);
    }
    
    #[test]
    fn test_flex_item_default() {
        let item = FlexItem::default();
        assert_eq!(item.flex_grow, 0.0);
        assert_eq!(item.flex_shrink, 1.0);
        assert!(item.flex_basis.is_none());
    }
}
