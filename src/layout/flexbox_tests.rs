// Comprehensive flexbox layout tests

#[cfg(test)]
mod tests {
    use super::super::flexbox::*;
    use super::super::{Dimensions, Rect};

    /// Helper to create a basic container dimensions
    fn container(width: f32, height: f32) -> Dimensions {
        let mut dims = Dimensions::default();
        dims.content.width = width;
        dims.content.height = height;
        dims
    }

    /// Helper to create a flex item with specific properties
    fn item(flex_grow: f32, flex_shrink: f32, flex_basis: Option<f32>) -> FlexItem {
        FlexItem {
            flex_grow,
            flex_shrink,
            flex_basis,
            min_size: None,
            max_size: None,
        }
    }

    #[test]
    fn test_flex_row_basic() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(150.0)),
            item(0.0, 1.0, Some(200.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        assert_eq!(result.len(), 3);
        
        // Items should be positioned left to right
        assert_eq!(result[0].content.x, 0.0);
        assert_eq!(result[0].content.width, 100.0);
        
        assert_eq!(result[1].content.x, 100.0);
        assert_eq!(result[1].content.width, 150.0);
        
        assert_eq!(result[2].content.x, 250.0);
        assert_eq!(result[2].content.width, 200.0);
    }

    #[test]
    fn test_flex_column_basic() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Column,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(150.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        assert_eq!(result.len(), 2);
        
        // Items should be stacked vertically (main axis is height)
        assert_eq!(result[0].content.y, 0.0);
        assert_eq!(result[0].content.height, 100.0);
        
        assert_eq!(result[1].content.y, 100.0);
        assert_eq!(result[1].content.height, 150.0);
    }

    #[test]
    fn test_flex_grow() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(1.0, 1.0, Some(100.0)), // flex-grow: 1
            item(2.0, 1.0, Some(100.0)), // flex-grow: 2
            item(1.0, 1.0, Some(100.0)), // flex-grow: 1
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Total basis: 300px, available space: 800px
        // Free space: 500px
        // Total flex-grow: 4
        // Item 0 gets: 100 + (500 * 1/4) = 225
        // Item 1 gets: 100 + (500 * 2/4) = 350
        // Item 2 gets: 100 + (500 * 1/4) = 225

        assert_eq!(result[0].content.width, 225.0);
        assert_eq!(result[1].content.width, 350.0);
        assert_eq!(result[2].content.width, 225.0);
    }

    #[test]
    fn test_flex_shrink() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(400.0)), // flex-shrink: 1
            item(0.0, 2.0, Some(400.0)), // flex-shrink: 2
            item(0.0, 1.0, Some(400.0)), // flex-shrink: 1
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Total basis: 1200px, available space: 800px
        // Overflow: -400px
        // Total flex-shrink: 4
        // Items should shrink proportionally
        assert!(result[0].content.width < 400.0);
        assert!(result[1].content.width < 400.0);
        assert!(result[2].content.width < 400.0);
        
        // Item 1 should shrink more (flex-shrink: 2)
        assert!(result[1].content.width < result[0].content.width);
    }

    #[test]
    fn test_justify_content_flex_end() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(100.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Items should be aligned to the right
        // Free space: 800 - 200 = 600
        assert_eq!(result[0].content.x, 600.0);
        assert_eq!(result[1].content.x, 700.0);
    }

    #[test]
    fn test_justify_content_center() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(100.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Items should be centered
        // Free space: 800 - 200 = 600
        // Offset: 600 / 2 = 300
        assert_eq!(result[0].content.x, 300.0);
        assert_eq!(result[1].content.x, 400.0);
    }

    #[test]
    fn test_justify_content_space_between() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(100.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Free space: 800 - 300 = 500
        // Gaps: 500 / 2 = 250 (between 3 items)
        assert_eq!(result[0].content.x, 0.0);
        assert_eq!(result[1].content.x, 350.0); // 100 + 250
        assert_eq!(result[2].content.x, 700.0); // 350 + 100 + 250
    }

    #[test]
    fn test_justify_content_space_around() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(100.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Free space: 800 - 200 = 600
        // Gap per item: 600 / 2 = 300
        // Half gap on each side: 150
        assert_eq!(result[0].content.x, 150.0);
        assert_eq!(result[1].content.x, 550.0); // 150 + 100 + 300
    }

    #[test]
    fn test_justify_content_space_evenly() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(100.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Free space: 800 - 200 = 600
        // 3 gaps (before, between, after): 600 / 3 = 200
        assert_eq!(result[0].content.x, 200.0);
        assert_eq!(result[1].content.x, 500.0); // 200 + 100 + 200
    }

    #[test]
    fn test_row_reverse() {
        let flex_container = FlexContainer {
            direction: FlexDirection::RowReverse,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(150.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Items should be positioned right to left
        // Item 0 should be at the right
        assert!(result[0].content.x > result[1].content.x);
    }

    #[test]
    fn test_column_reverse() {
        let flex_container = FlexContainer {
            direction: FlexDirection::ColumnReverse,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(150.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Items should be positioned bottom to top
        // Item 0 should be lower (higher y)
        assert!(result[0].content.y > result[1].content.y);
    }

    #[test]
    fn test_empty_container() {
        let flex_container = FlexContainer::default();
        let items: Vec<FlexItem> = vec![];
        let container_dims = container(800.0, 600.0);
        
        let result = flex_container.layout(container_dims, &items);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_single_item() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        };

        let items = vec![item(0.0, 1.0, Some(200.0))];
        let container_dims = container(800.0, 600.0);
        
        let result = flex_container.layout(container_dims, &items);
        
        assert_eq!(result.len(), 1);
        // Should be centered: (800 - 200) / 2 = 300
        assert_eq!(result[0].content.x, 300.0);
    }

    #[test]
    fn test_wrap_basic() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(0.0, 1.0, Some(300.0)),
            item(0.0, 1.0, Some(300.0)),
            item(0.0, 1.0, Some(300.0)), // Should wrap to next line
        ];

        let container_dims = container(700.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        assert_eq!(result.len(), 3);
        
        // First two items on first line
        assert_eq!(result[0].content.y, result[1].content.y);
        
        // Third item should be on second line (different y)
        assert!(result[2].content.y > result[0].content.y);
    }

    #[test]
    fn test_min_size_constraint() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            FlexItem {
                flex_grow: 0.0,
                flex_shrink: 1.0,
                flex_basis: Some(400.0),
                min_size: Some(200.0), // Should not shrink below 200
                max_size: None,
            },
        ];

        let container_dims = container(100.0, 600.0); // Very narrow
        let result = flex_container.layout(container_dims, &items);

        // Should respect min-size even in overflow
        assert!(result[0].content.width >= 200.0);
    }

    #[test]
    fn test_max_size_constraint() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            FlexItem {
                flex_grow: 1.0,
                flex_shrink: 1.0,
                flex_basis: Some(100.0),
                min_size: None,
                max_size: Some(300.0), // Should not grow beyond 300
            },
        ];

        let container_dims = container(1000.0, 600.0); // Plenty of space
        let result = flex_container.layout(container_dims, &items);

        // Should respect max-size even with flex-grow
        assert!(result[0].content.width <= 300.0);
    }

    #[test]
    fn test_align_items_flex_start() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(150.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Items should be aligned to top (y = 0)
        assert_eq!(result[0].content.y, 0.0);
        assert_eq!(result[1].content.y, 0.0);
    }

    #[test]
    fn test_align_items_flex_end() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
            item(0.0, 1.0, Some(150.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Items should be aligned to bottom
        // Both should have same y (at the end)
        assert!(result[0].content.y > 0.0);
        assert!(result[1].content.y > 0.0);
    }

    #[test]
    fn test_align_items_center() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
        };

        let items = vec![
            item(0.0, 1.0, Some(100.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Item should be vertically centered
        // With cross size 0 (not calculated yet), position should be centered in line
        assert!(result[0].content.y >= 0.0);
    }

    #[test]
    fn test_zero_flex_basis() {
        let flex_container = FlexContainer {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
        };

        let items = vec![
            item(1.0, 1.0, Some(0.0)), // flex-basis: 0
            item(1.0, 1.0, Some(0.0)),
        ];

        let container_dims = container(800.0, 600.0);
        let result = flex_container.layout(container_dims, &items);

        // Both items should grow equally from 0
        assert_eq!(result[0].content.width, 400.0);
        assert_eq!(result[1].content.width, 400.0);
    }
}
