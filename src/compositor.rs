// Layer-Based Compositor - Phase 7
// Implements tile-based rendering, damage tracking, and partial invalidation

use crate::layout::Rect;
use crate::css::Color;
use std::collections::HashSet;

/// Tile size for rendering (256x256 pixels is a common choice)
const TILE_SIZE: u32 = 256;

/// A single compositing layer
#[derive(Debug, Clone)]
pub struct Layer {
    /// Unique identifier for this layer
    pub id: LayerId,
    /// Layer bounds in document coordinates
    pub bounds: Rect,
    /// Z-index for stacking
    pub z_index: i32,
    /// Opacity (0.0 to 1.0)
    pub opacity: f32,
    /// Transform matrix (for future CSS transforms)
    pub transform: Transform,
    /// Whether this layer is visible
    pub visible: bool,
    /// Damaged regions that need repainting
    damaged_tiles: HashSet<TileCoord>,
    /// Cached tile data
    tiles: Vec<Tile>,
    /// Parent layer ID (for hierarchical compositing)
    pub parent_id: Option<LayerId>,
    /// Child layer IDs
    pub children: Vec<LayerId>,
}

/// Unique layer identifier
pub type LayerId = u64;

/// 2D transform matrix (simplified for now)
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    /// Translation X
    pub translate_x: f32,
    /// Translation Y
    pub translate_y: f32,
    /// Scale X
    pub scale_x: f32,
    /// Scale Y
    pub scale_y: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

impl Transform {
    /// Create identity transform
    pub fn identity() -> Self {
        Self::default()
    }
    
    /// Apply transform to a point
    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        let transformed_x = x * self.scale_x + self.translate_x;
        let transformed_y = y * self.scale_y + self.translate_y;
        (transformed_x, transformed_y)
    }
}

/// Tile coordinate (x, y in tile space)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoord {
    pub x: u32,
    pub y: u32,
}

impl TileCoord {
    /// Convert pixel coordinates to tile coordinates
    pub fn from_pixel(x: f32, y: f32) -> Self {
        Self {
            x: (x / TILE_SIZE as f32).floor() as u32,
            y: (y / TILE_SIZE as f32).floor() as u32,
        }
    }
    
    /// Get pixel bounds for this tile
    pub fn to_rect(&self) -> Rect {
        Rect {
            x: (self.x * TILE_SIZE) as f32,
            y: (self.y * TILE_SIZE) as f32,
            width: TILE_SIZE as f32,
            height: TILE_SIZE as f32,
        }
    }
}

/// A single rendered tile
#[derive(Debug, Clone)]
pub struct Tile {
    /// Tile coordinate
    pub coord: TileCoord,
    /// Whether this tile has been rendered
    pub rendered: bool,
    /// Tile version (incremented on damage)
    pub version: u32,
}

impl Layer {
    /// Create a new layer
    pub fn new(id: LayerId, bounds: Rect) -> Self {
        let tiles = Self::create_tiles_for_bounds(&bounds);
        
        Self {
            id,
            bounds,
            z_index: 0,
            opacity: 1.0,
            transform: Transform::identity(),
            visible: true,
            damaged_tiles: tiles.iter().map(|t| t.coord).collect(),
            tiles,
            parent_id: None,
            children: Vec::new(),
        }
    }
    
    /// Create tiles covering the given bounds
    fn create_tiles_for_bounds(bounds: &Rect) -> Vec<Tile> {
        let mut tiles = Vec::new();
        
        let start_x = (bounds.x / TILE_SIZE as f32).floor() as u32;
        let start_y = (bounds.y / TILE_SIZE as f32).floor() as u32;
        let end_x = ((bounds.x + bounds.width) / TILE_SIZE as f32).ceil() as u32;
        let end_y = ((bounds.y + bounds.height) / TILE_SIZE as f32).ceil() as u32;
        
        for y in start_y..end_y {
            for x in start_x..end_x {
                tiles.push(Tile {
                    coord: TileCoord { x, y },
                    rendered: false,
                    version: 0,
                });
            }
        }
        
        tiles
    }
    
    /// Mark a region as damaged (needs repainting)
    pub fn damage(&mut self, rect: &Rect) {
        let start_coord = TileCoord::from_pixel(rect.x, rect.y);
        let end_coord = TileCoord::from_pixel(
            rect.x + rect.width,
            rect.y + rect.height,
        );
        
        for y in start_coord.y..=end_coord.y {
            for x in start_coord.x..=end_coord.x {
                self.damaged_tiles.insert(TileCoord { x, y });
            }
        }
    }
    
    /// Get all damaged tiles
    pub fn damaged_tiles(&self) -> Vec<TileCoord> {
        self.damaged_tiles.iter().copied().collect()
    }
    
    /// Mark tiles as rendered
    pub fn mark_tiles_rendered(&mut self, coords: &[TileCoord]) {
        for coord in coords {
            self.damaged_tiles.remove(coord);
            if let Some(tile) = self.tiles.iter_mut().find(|t| &t.coord == coord) {
                tile.rendered = true;
                tile.version += 1;
            }
        }
    }
    
    /// Clear all damage
    pub fn clear_damage(&mut self) {
        self.damaged_tiles.clear();
    }
    
    /// Check if layer intersects with viewport
    pub fn intersects_viewport(&self, viewport: &Rect) -> bool {
        self.bounds.x < viewport.x + viewport.width
            && self.bounds.x + self.bounds.width > viewport.x
            && self.bounds.y < viewport.y + viewport.height
            && self.bounds.y + self.bounds.height > viewport.y
    }
    
    /// Get tiles visible in viewport
    pub fn visible_tiles(&self, viewport: &Rect) -> Vec<TileCoord> {
        let start_coord = TileCoord::from_pixel(viewport.x.max(0.0), viewport.y.max(0.0));
        let end_coord = TileCoord::from_pixel(
            (viewport.x + viewport.width).min(self.bounds.x + self.bounds.width),
            (viewport.y + viewport.height).min(self.bounds.y + self.bounds.height),
        );
        
        let mut visible = Vec::new();
        for y in start_coord.y..=end_coord.y {
            for x in start_coord.x..=end_coord.x {
                visible.push(TileCoord { x, y });
            }
        }
        visible
    }
}

/// Compositor manages all layers and handles rendering
pub struct Compositor {
    /// All layers, indexed by ID
    layers: Vec<Layer>,
    /// Next layer ID to assign
    next_layer_id: LayerId,
    /// Viewport bounds
    viewport: Rect,
    /// Root layer ID
    root_layer_id: Option<LayerId>,
    /// Damaged regions in screen space
    screen_damage: Vec<Rect>,
}

impl Compositor {
    /// Create a new compositor
    pub fn new(viewport: Rect) -> Self {
        Self {
            layers: Vec::new(),
            next_layer_id: 1,
            viewport,
            root_layer_id: None,
            screen_damage: Vec::new(),
        }
    }
    
    /// Create a new layer
    pub fn create_layer(&mut self, bounds: Rect) -> LayerId {
        let id = self.next_layer_id;
        self.next_layer_id += 1;
        
        let layer = Layer::new(id, bounds);
        self.layers.push(layer);
        
        // If no root layer, make this the root
        if self.root_layer_id.is_none() {
            self.root_layer_id = Some(id);
        }
        
        id
    }
    
    /// Get a layer by ID
    pub fn get_layer(&self, id: LayerId) -> Option<&Layer> {
        self.layers.iter().find(|l| l.id == id)
    }
    
    /// Get a mutable layer by ID
    pub fn get_layer_mut(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }
    
    /// Remove a layer
    pub fn remove_layer(&mut self, id: LayerId) -> bool {
        if let Some(pos) = self.layers.iter().position(|l| l.id == id) {
            // Remove from parent's children
            if let Some(parent_id) = self.layers[pos].parent_id {
                if let Some(parent) = self.get_layer_mut(parent_id) {
                    parent.children.retain(|&child_id| child_id != id);
                }
            }
            
            self.layers.remove(pos);
            
            // If this was the root layer, clear root
            if self.root_layer_id == Some(id) {
                self.root_layer_id = None;
            }
            
            true
        } else {
            false
        }
    }
    
    /// Add a child layer to a parent
    pub fn add_child(&mut self, parent_id: LayerId, child_id: LayerId) -> bool {
        // Check both layers exist
        if !self.layers.iter().any(|l| l.id == parent_id) 
            || !self.layers.iter().any(|l| l.id == child_id) {
            return false;
        }
        
        // Update parent
        if let Some(parent) = self.get_layer_mut(parent_id) {
            if !parent.children.contains(&child_id) {
                parent.children.push(child_id);
            }
        }
        
        // Update child
        if let Some(child) = self.get_layer_mut(child_id) {
            child.parent_id = Some(parent_id);
        }
        
        true
    }
    
    /// Damage a region (mark for repainting)
    pub fn damage_region(&mut self, rect: Rect) {
        self.screen_damage.push(rect);
        
        // Damage all layers that intersect this region
        for layer in &mut self.layers {
            if layer.intersects_viewport(&rect) {
                layer.damage(&rect);
            }
        }
    }
    
    /// Get all damaged tiles across all layers
    pub fn get_damaged_tiles(&self) -> Vec<(LayerId, Vec<TileCoord>)> {
        self.layers
            .iter()
            .filter(|l| l.visible && !l.damaged_tiles.is_empty())
            .map(|l| (l.id, l.damaged_tiles()))
            .collect()
    }
    
    /// Mark tiles as rendered for a specific layer
    pub fn mark_tiles_rendered(&mut self, layer_id: LayerId, tiles: &[TileCoord]) {
        if let Some(layer) = self.get_layer_mut(layer_id) {
            layer.mark_tiles_rendered(tiles);
        }
    }
    
    /// Get layers in paint order (back to front)
    pub fn layers_in_paint_order(&self) -> Vec<&Layer> {
        let mut layers: Vec<&Layer> = self.layers
            .iter()
            .filter(|l| l.visible)
            .collect();
        
        // Sort by z-index
        layers.sort_by_key(|l| l.z_index);
        
        layers
    }
    
    /// Update viewport
    pub fn set_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }
    
    /// Get viewport
    pub fn viewport(&self) -> &Rect {
        &self.viewport
    }
    
    /// Get tiles that need rendering in current viewport
    pub fn get_tiles_to_render(&self) -> Vec<(LayerId, TileCoord, Rect)> {
        let mut tiles_to_render = Vec::new();
        
        for layer in self.layers_in_paint_order() {
            if !layer.intersects_viewport(&self.viewport) {
                continue;
            }
            
            let visible_tiles = layer.visible_tiles(&self.viewport);
            let damaged_tiles: HashSet<TileCoord> = layer.damaged_tiles.iter().copied().collect();
            
            for tile_coord in visible_tiles {
                // Only render if damaged or not yet rendered
                if damaged_tiles.contains(&tile_coord) 
                    || !layer.tiles.iter().any(|t| t.coord == tile_coord && t.rendered) {
                    tiles_to_render.push((layer.id, tile_coord, tile_coord.to_rect()));
                }
            }
        }
        
        tiles_to_render
    }
    
    /// Clear all damage
    pub fn clear_damage(&mut self) {
        self.screen_damage.clear();
        for layer in &mut self.layers {
            layer.clear_damage();
        }
    }
    
    /// Get layer count
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
    
    /// Check if compositor has pending work
    pub fn has_pending_work(&self) -> bool {
        !self.screen_damage.is_empty() || 
        self.layers.iter().any(|l| !l.damaged_tiles.is_empty())
    }
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new(Rect {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tile_coord_conversion() {
        let coord = TileCoord::from_pixel(300.0, 400.0);
        assert_eq!(coord.x, 1); // 300 / 256 = 1
        assert_eq!(coord.y, 1); // 400 / 256 = 1
        
        let rect = coord.to_rect();
        assert_eq!(rect.x, 256.0);
        assert_eq!(rect.y, 256.0);
        assert_eq!(rect.width, 256.0);
        assert_eq!(rect.height, 256.0);
    }
    
    #[test]
    fn test_layer_creation() {
        let bounds = Rect { x: 0.0, y: 0.0, width: 512.0, height: 512.0 };
        let layer = Layer::new(1, bounds);
        
        assert_eq!(layer.id, 1);
        assert_eq!(layer.z_index, 0);
        assert_eq!(layer.opacity, 1.0);
        assert!(layer.visible);
        
        // Should have 4 tiles (2x2) for 512x512 area
        assert_eq!(layer.tiles.len(), 4);
    }
    
    #[test]
    fn test_layer_damage() {
        let bounds = Rect { x: 0.0, y: 0.0, width: 512.0, height: 512.0 };
        let mut layer = Layer::new(1, bounds);
        layer.clear_damage(); // Start clean
        
        // Damage a small region
        let damage_rect = Rect { x: 100.0, y: 100.0, width: 50.0, height: 50.0 };
        layer.damage(&damage_rect);
        
        let damaged = layer.damaged_tiles();
        assert!(!damaged.is_empty());
        assert!(damaged.contains(&TileCoord { x: 0, y: 0 }));
    }
    
    #[test]
    fn test_compositor_layer_creation() {
        let mut compositor = Compositor::default();
        
        let bounds = Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 };
        let layer_id = compositor.create_layer(bounds);
        
        assert_eq!(layer_id, 1);
        assert_eq!(compositor.layer_count(), 1);
        assert!(compositor.get_layer(layer_id).is_some());
    }
    
    #[test]
    fn test_compositor_layer_hierarchy() {
        let mut compositor = Compositor::default();
        
        let parent_id = compositor.create_layer(Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 });
        let child_id = compositor.create_layer(Rect { x: 100.0, y: 100.0, width: 200.0, height: 200.0 });
        
        assert!(compositor.add_child(parent_id, child_id));
        
        let parent = compositor.get_layer(parent_id).unwrap();
        assert!(parent.children.contains(&child_id));
        
        let child = compositor.get_layer(child_id).unwrap();
        assert_eq!(child.parent_id, Some(parent_id));
    }
    
    #[test]
    fn test_compositor_damage_tracking() {
        let mut compositor = Compositor::default();
        let layer_id = compositor.create_layer(Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 });
        
        compositor.damage_region(Rect { x: 100.0, y: 100.0, width: 50.0, height: 50.0 });
        
        assert!(compositor.has_pending_work());
        
        let damaged = compositor.get_damaged_tiles();
        assert!(!damaged.is_empty());
        assert_eq!(damaged[0].0, layer_id);
    }
    
    #[test]
    fn test_compositor_layer_removal() {
        let mut compositor = Compositor::default();
        
        let layer_id = compositor.create_layer(Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 });
        assert_eq!(compositor.layer_count(), 1);
        
        assert!(compositor.remove_layer(layer_id));
        assert_eq!(compositor.layer_count(), 0);
        assert!(compositor.get_layer(layer_id).is_none());
    }
    
    #[test]
    fn test_compositor_paint_order() {
        let mut compositor = Compositor::default();
        
        let layer1_id = compositor.create_layer(Rect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 });
        let layer2_id = compositor.create_layer(Rect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 });
        let layer3_id = compositor.create_layer(Rect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 });
        
        // Set z-indices
        compositor.get_layer_mut(layer1_id).unwrap().z_index = 10;
        compositor.get_layer_mut(layer2_id).unwrap().z_index = 5;
        compositor.get_layer_mut(layer3_id).unwrap().z_index = 15;
        
        let paint_order = compositor.layers_in_paint_order();
        
        // Should be sorted by z-index: layer2 (5), layer1 (10), layer3 (15)
        assert_eq!(paint_order[0].id, layer2_id);
        assert_eq!(paint_order[1].id, layer1_id);
        assert_eq!(paint_order[2].id, layer3_id);
    }
    
    #[test]
    fn test_transform_apply() {
        let mut transform = Transform::identity();
        transform.translate_x = 10.0;
        transform.translate_y = 20.0;
        transform.scale_x = 2.0;
        transform.scale_y = 2.0;
        
        let (x, y) = transform.apply(5.0, 10.0);
        assert_eq!(x, 20.0); // 5 * 2 + 10
        assert_eq!(y, 40.0); // 10 * 2 + 20
    }
    
    #[test]
    fn test_viewport_intersection() {
        let bounds = Rect { x: 100.0, y: 100.0, width: 200.0, height: 200.0 };
        let layer = Layer::new(1, bounds);
        
        let viewport1 = Rect { x: 0.0, y: 0.0, width: 150.0, height: 150.0 };
        assert!(layer.intersects_viewport(&viewport1));
        
        let viewport2 = Rect { x: 400.0, y: 400.0, width: 100.0, height: 100.0 };
        assert!(!layer.intersects_viewport(&viewport2));
    }
}
