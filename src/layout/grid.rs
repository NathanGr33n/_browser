// CSS Grid Layout implementation

use crate::css::{Value, Unit};
use crate::layout::{Dimensions, Rect};
use crate::style::StyledNode;

/// Grid track size specification
#[derive(Debug, Clone)]
pub enum TrackSize {
    /// Fixed size in pixels
    Fixed(f32),
    /// Fraction of available space
    Fr(f32),
    /// Auto-sized based on content
    Auto,
}

/// Grid container properties
#[derive(Debug, Clone)]
pub struct GridContainer {
    /// Column track sizes
    pub columns: Vec<TrackSize>,
    /// Row track sizes
    pub rows: Vec<TrackSize>,
    /// Gap between columns
    pub column_gap: f32,
    /// Gap between rows
    pub row_gap: f32,
}

impl Default for GridContainer {
    fn default() -> Self {
        Self {
            columns: vec![TrackSize::Auto],
            rows: vec![TrackSize::Auto],
            column_gap: 0.0,
            row_gap: 0.0,
        }
    }
}

impl GridContainer {
    /// Parse grid properties from a styled node
    pub fn from_styled_node(node: &StyledNode) -> Option<Self> {
        // Only create grid container if display is grid
        let display = node.value("display")?;
        if let Value::Keyword(s) = display {
            if s != "grid" && s != "inline-grid" {
                return None;
            }
        }

        let columns = Self::parse_track_list(node, "grid-template-columns");
        let rows = Self::parse_track_list(node, "grid-template-rows");
        let column_gap = Self::parse_gap(node, "column-gap");
        let row_gap = Self::parse_gap(node, "row-gap");

        Some(Self {
            columns,
            rows,
            column_gap,
            row_gap,
        })
    }

    /// Parse track list (e.g., "200px 1fr 2fr")
    fn parse_track_list(node: &StyledNode, property: &str) -> Vec<TrackSize> {
        // Simple parser for track lists
        // TODO: Support repeat(), minmax(), auto-fit, auto-fill
        match node.value(property) {
            Some(Value::Keyword(s)) => {
                s.split_whitespace()
                    .filter_map(|part| Self::parse_track_size(part))
                    .collect()
            }
            _ => vec![TrackSize::Auto],
        }
    }

    /// Parse a single track size
    fn parse_track_size(s: &str) -> Option<TrackSize> {
        if s == "auto" {
            return Some(TrackSize::Auto);
        }

        if s.ends_with("px") {
            let num = s.trim_end_matches("px").parse::<f32>().ok()?;
            return Some(TrackSize::Fixed(num));
        }

        if s.ends_with("fr") {
            let num = s.trim_end_matches("fr").parse::<f32>().ok()?;
            return Some(TrackSize::Fr(num));
        }

        None
    }

    /// Parse gap property
    fn parse_gap(node: &StyledNode, property: &str) -> f32 {
        match node.value(property) {
            Some(Value::Length(val, Unit::Px)) => *val,
            _ => 0.0,
        }
    }

    /// Calculate grid layout
    pub fn layout(
        &self,
        container_dimensions: Dimensions,
        items: &[GridItem],
    ) -> Vec<Dimensions> {
        let available_width = container_dimensions.content.width;
        let available_height = container_dimensions.content.height;

        // Calculate track sizes
        let column_sizes = self.calculate_track_sizes(&self.columns, available_width, self.column_gap);
        let row_sizes = self.calculate_track_sizes(&self.rows, available_height, self.row_gap);

        // Position items
        let mut results = Vec::new();
        
        for (idx, item) in items.iter().enumerate() {
            let mut dims = Dimensions::default();

            // Determine grid position (simple auto-placement for now)
            // Protect against division by zero if no columns defined
            let num_cols = column_sizes.len().max(1);
            let col = item.column_start.unwrap_or(idx % num_cols).min(column_sizes.len().saturating_sub(1));
            let row = item.row_start.unwrap_or(idx / num_cols);

            let col_span = item.column_span.unwrap_or(1).min(column_sizes.len() - col);
            let row_span = item.row_span.unwrap_or(1).min(row_sizes.len() - row);

            // Calculate position and size
            dims.content.x = container_dimensions.content.x
                + column_sizes.iter().take(col).sum::<f32>()
                + self.column_gap * col as f32;

            dims.content.y = container_dimensions.content.y
                + row_sizes.iter().take(row).sum::<f32>()
                + self.row_gap * row as f32;

            dims.content.width = column_sizes.iter().skip(col).take(col_span).sum::<f32>()
                + self.column_gap * (col_span.saturating_sub(1)) as f32;

            dims.content.height = row_sizes.iter().skip(row).take(row_span).sum::<f32>()
                + self.row_gap * (row_span.saturating_sub(1)) as f32;

            results.push(dims);
        }

        results
    }

    /// Calculate actual track sizes from specifications
    fn calculate_track_sizes(
        &self,
        tracks: &[TrackSize],
        available_space: f32,
        gap: f32,
    ) -> Vec<f32> {
        if tracks.is_empty() {
            return vec![available_space];
        }

        let total_gaps = gap * (tracks.len().saturating_sub(1)) as f32;
        let space_for_tracks = available_space - total_gaps;

        // First pass: calculate fixed and auto sizes
        let mut sizes = vec![0.0; tracks.len()];
        let mut fixed_space = 0.0;
        let mut fr_sum = 0.0;

        for (i, track) in tracks.iter().enumerate() {
            match track {
                TrackSize::Fixed(size) => {
                    sizes[i] = *size;
                    fixed_space += size;
                }
                TrackSize::Auto => {
                    // Auto tracks get minimum space for now
                    sizes[i] = 100.0; // Default auto size
                    fixed_space += 100.0;
                }
                TrackSize::Fr(fr) => {
                    fr_sum += fr;
                }
            }
        }

        // Second pass: distribute remaining space to fr tracks
        let remaining_space = (space_for_tracks - fixed_space).max(0.0);
        
        if fr_sum > 0.0 {
            let fr_unit = remaining_space / fr_sum;
            for (i, track) in tracks.iter().enumerate() {
                if let TrackSize::Fr(fr) = track {
                    sizes[i] = fr * fr_unit;
                }
            }
        }

        sizes
    }
}

/// Grid item properties
#[derive(Debug, Clone, Default)]
pub struct GridItem {
    /// Column start position (0-indexed)
    pub column_start: Option<usize>,
    /// Row start position (0-indexed)
    pub row_start: Option<usize>,
    /// Number of columns to span
    pub column_span: Option<usize>,
    /// Number of rows to span
    pub row_span: Option<usize>,
}

impl GridItem {
    /// Parse grid item properties from styled node
    pub fn from_styled_node(node: &StyledNode) -> Self {
        Self {
            column_start: Self::parse_line(node, "grid-column-start"),
            row_start: Self::parse_line(node, "grid-row-start"),
            column_span: Self::parse_span(node, "grid-column-end", "grid-column-start"),
            row_span: Self::parse_span(node, "grid-row-end", "grid-row-start"),
        }
    }

    /// Parse grid line position
    fn parse_line(node: &StyledNode, property: &str) -> Option<usize> {
        match node.value(property) {
            Some(Value::Number(n)) => Some((*n as usize).saturating_sub(1)), // CSS is 1-indexed
            Some(Value::Length(n, Unit::Px)) => Some((*n as usize).saturating_sub(1)),
            _ => None,
        }
    }

    /// Parse span from end and start positions
    fn parse_span(node: &StyledNode, end_prop: &str, start_prop: &str) -> Option<usize> {
        let end = match node.value(end_prop) {
            Some(Value::Number(n)) => *n as usize,
            Some(Value::Length(n, Unit::Px)) => *n as usize,
            Some(Value::Keyword(s)) if s.starts_with("span ") => {
                let span_str = s.trim_start_matches("span ");
                return span_str.parse::<usize>().ok();
            }
            _ => return Some(1), // Default span is 1
        };

        let start = match node.value(start_prop) {
            Some(Value::Number(n)) => *n as usize,
            Some(Value::Length(n, Unit::Px)) => *n as usize,
            _ => 1, // CSS is 1-indexed, default to 1
        };

        if end > start {
            Some(end - start)
        } else {
            Some(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn container(width: f32, height: f32) -> Dimensions {
        let mut dims = Dimensions::default();
        dims.content.width = width;
        dims.content.height = height;
        dims
    }

    #[test]
    fn test_grid_container_default() {
        let grid = GridContainer::default();
        assert_eq!(grid.columns.len(), 1);
        assert_eq!(grid.rows.len(), 1);
        assert_eq!(grid.column_gap, 0.0);
        assert_eq!(grid.row_gap, 0.0);
    }

    #[test]
    fn test_parse_track_size() {
        assert!(matches!(
            GridContainer::parse_track_size("100px"),
            Some(TrackSize::Fixed(100.0))
        ));
        assert!(matches!(
            GridContainer::parse_track_size("2fr"),
            Some(TrackSize::Fr(2.0))
        ));
        assert!(matches!(
            GridContainer::parse_track_size("auto"),
            Some(TrackSize::Auto)
        ));
    }

    #[test]
    fn test_simple_grid_layout() {
        let grid = GridContainer {
            columns: vec![TrackSize::Fixed(100.0), TrackSize::Fixed(200.0)],
            rows: vec![TrackSize::Fixed(50.0), TrackSize::Fixed(50.0)],
            column_gap: 10.0,
            row_gap: 10.0,
        };

        let items = vec![
            GridItem::default(),
            GridItem::default(),
            GridItem::default(),
        ];

        let container_dims = container(400.0, 300.0);
        let result = grid.layout(container_dims, &items);

        assert_eq!(result.len(), 3);

        // First item: column 0, row 0
        assert_eq!(result[0].content.x, 0.0);
        assert_eq!(result[0].content.y, 0.0);
        assert_eq!(result[0].content.width, 100.0);
        assert_eq!(result[0].content.height, 50.0);

        // Second item: column 1, row 0
        assert_eq!(result[1].content.x, 110.0); // 100 + 10 gap
        assert_eq!(result[1].content.y, 0.0);
        assert_eq!(result[1].content.width, 200.0);
        assert_eq!(result[1].content.height, 50.0);

        // Third item: column 0, row 1
        assert_eq!(result[2].content.x, 0.0);
        assert_eq!(result[2].content.y, 60.0); // 50 + 10 gap
        assert_eq!(result[2].content.width, 100.0);
        assert_eq!(result[2].content.height, 50.0);
    }

    #[test]
    fn test_fr_tracks() {
        let grid = GridContainer {
            columns: vec![TrackSize::Fr(1.0), TrackSize::Fr(2.0)],
            rows: vec![TrackSize::Fixed(100.0)],
            column_gap: 0.0,
            row_gap: 0.0,
        };

        let items = vec![GridItem::default(), GridItem::default()];
        let container_dims = container(600.0, 300.0);
        let result = grid.layout(container_dims, &items);

        // 1fr gets 200px, 2fr gets 400px (total 600px)
        assert_eq!(result[0].content.width, 200.0);
        assert_eq!(result[1].content.width, 400.0);
    }

    #[test]
    fn test_grid_spanning() {
        let grid = GridContainer {
            columns: vec![
                TrackSize::Fixed(100.0),
                TrackSize::Fixed(100.0),
                TrackSize::Fixed(100.0),
            ],
            rows: vec![TrackSize::Fixed(50.0), TrackSize::Fixed(50.0)],
            column_gap: 10.0,
            row_gap: 10.0,
        };

        let items = vec![
            GridItem {
                column_start: Some(0),
                row_start: Some(0),
                column_span: Some(2), // Span 2 columns
                row_span: Some(1),
            },
        ];

        let container_dims = container(400.0, 200.0);
        let result = grid.layout(container_dims, &items);

        // Item spans 2 columns: 100 + 10 (gap) + 100 = 210
        assert_eq!(result[0].content.width, 210.0);
        assert_eq!(result[0].content.height, 50.0);
    }

    #[test]
    fn test_grid_item_default() {
        let item = GridItem::default();
        assert!(item.column_start.is_none());
        assert!(item.row_start.is_none());
        assert!(item.column_span.is_none());
        assert!(item.row_span.is_none());
    }
}
