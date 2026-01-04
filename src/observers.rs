// DOM Observers - Phase 8 Advanced JavaScript

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Observer ID type
pub type ObserverId = u64;

/// Mutation Observer - watches DOM changes
pub struct MutationObserver {
    /// Observer ID
    id: ObserverId,
    /// Callback function (simplified)
    callback: Arc<Mutex<Box<dyn Fn(&[MutationRecord]) + Send>>>,
    /// Observed nodes
    observed_nodes: HashSet<u64>, // node IDs
    /// Configuration
    config: MutationObserverInit,
}

/// Mutation observer configuration
#[derive(Debug, Clone)]
pub struct MutationObserverInit {
    /// Watch child list changes
    pub child_list: bool,
    /// Watch attribute changes
    pub attributes: bool,
    /// Watch character data changes
    pub character_data: bool,
    /// Watch subtree
    pub subtree: bool,
    /// Record old attribute values
    pub attribute_old_value: bool,
    /// Record old character data
    pub character_data_old_value: bool,
    /// Attribute filter (None = all attributes)
    pub attribute_filter: Option<Vec<String>>,
}

impl Default for MutationObserverInit {
    fn default() -> Self {
        Self {
            child_list: false,
            attributes: false,
            character_data: false,
            subtree: false,
            attribute_old_value: false,
            character_data_old_value: false,
            attribute_filter: None,
        }
    }
}

/// Mutation record describing a DOM change
#[derive(Debug, Clone)]
pub struct MutationRecord {
    /// Type of mutation
    pub mutation_type: MutationType,
    /// Target node ID
    pub target: u64,
    /// Added nodes (for childList)
    pub added_nodes: Vec<u64>,
    /// Removed nodes (for childList)
    pub removed_nodes: Vec<u64>,
    /// Previous sibling
    pub previous_sibling: Option<u64>,
    /// Next sibling
    pub next_sibling: Option<u64>,
    /// Attribute name (for attributes)
    pub attribute_name: Option<String>,
    /// Old value
    pub old_value: Option<String>,
}

/// Mutation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationType {
    ChildList,
    Attributes,
    CharacterData,
}

impl MutationObserver {
    /// Create a new mutation observer
    pub fn new<F>(id: ObserverId, callback: F) -> Self
    where
        F: Fn(&[MutationRecord]) + Send + 'static,
    {
        Self {
            id,
            callback: Arc::new(Mutex::new(Box::new(callback))),
            observed_nodes: HashSet::new(),
            config: MutationObserverInit::default(),
        }
    }
    
    /// Get observer ID
    pub fn id(&self) -> ObserverId {
        self.id
    }
    
    /// Observe a node
    pub fn observe(&mut self, node_id: u64, config: MutationObserverInit) {
        self.observed_nodes.insert(node_id);
        self.config = config;
    }
    
    /// Disconnect observer
    pub fn disconnect(&mut self) {
        self.observed_nodes.clear();
    }
    
    /// Check if observing a node
    pub fn is_observing(&self, node_id: u64) -> bool {
        self.observed_nodes.contains(&node_id)
    }
    
    /// Notify observer of mutations
    pub fn notify(&self, records: &[MutationRecord]) {
        if let Ok(callback) = self.callback.lock() {
            callback(records);
        }
    }
    
    /// Get configuration
    pub fn config(&self) -> &MutationObserverInit {
        &self.config
    }
}

/// Intersection Observer - watches element visibility
pub struct IntersectionObserver {
    /// Observer ID
    id: ObserverId,
    /// Callback
    callback: Arc<Mutex<Box<dyn Fn(&[IntersectionObserverEntry]) + Send>>>,
    /// Observed elements
    observed_elements: HashSet<u64>,
    /// Root element (None = viewport)
    root: Option<u64>,
    /// Root margin (in pixels)
    root_margin: (f32, f32, f32, f32), // top, right, bottom, left
    /// Thresholds
    thresholds: Vec<f32>,
}

/// Intersection observer entry
#[derive(Debug, Clone)]
pub struct IntersectionObserverEntry {
    /// Target element ID
    pub target: u64,
    /// Bounding client rect
    pub bounding_client_rect: Rect,
    /// Root bounds
    pub root_bounds: Option<Rect>,
    /// Intersection rect
    pub intersection_rect: Rect,
    /// Intersection ratio (0.0 to 1.0)
    pub intersection_ratio: f32,
    /// Is intersecting
    pub is_intersecting: bool,
    /// Timestamp
    pub time: f64,
}

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Create a new rect
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    /// Calculate intersection with another rect
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);
        
        if x2 > x1 && y2 > y1 {
            Some(Rect::new(x1, y1, x2 - x1, y2 - y1))
        } else {
            None
        }
    }
    
    /// Calculate area
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl IntersectionObserver {
    /// Create a new intersection observer
    pub fn new<F>(id: ObserverId, callback: F) -> Self
    where
        F: Fn(&[IntersectionObserverEntry]) + Send + 'static,
    {
        Self {
            id,
            callback: Arc::new(Mutex::new(Box::new(callback))),
            observed_elements: HashSet::new(),
            root: None,
            root_margin: (0.0, 0.0, 0.0, 0.0),
            thresholds: vec![0.0],
        }
    }
    
    /// Set root element
    pub fn set_root(&mut self, root: Option<u64>) {
        self.root = root;
    }
    
    /// Set root margin
    pub fn set_root_margin(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
        self.root_margin = (top, right, bottom, left);
    }
    
    /// Set thresholds
    pub fn set_thresholds(&mut self, thresholds: Vec<f32>) {
        self.thresholds = thresholds;
    }
    
    /// Observe an element
    pub fn observe(&mut self, element_id: u64) {
        self.observed_elements.insert(element_id);
    }
    
    /// Unobserve an element
    pub fn unobserve(&mut self, element_id: u64) {
        self.observed_elements.remove(&element_id);
    }
    
    /// Disconnect observer
    pub fn disconnect(&mut self) {
        self.observed_elements.clear();
    }
    
    /// Check if observing an element
    pub fn is_observing(&self, element_id: u64) -> bool {
        self.observed_elements.contains(&element_id)
    }
    
    /// Notify observer of intersection changes
    pub fn notify(&self, entries: &[IntersectionObserverEntry]) {
        if let Ok(callback) = self.callback.lock() {
            callback(entries);
        }
    }
    
    /// Calculate intersection for an element
    pub fn calculate_intersection(&self, element_rect: Rect, root_rect: Rect) -> IntersectionObserverEntry {
        let intersection = element_rect.intersection(&root_rect);
        
        let (intersection_rect, intersection_ratio, is_intersecting) = if let Some(rect) = intersection {
            let ratio = rect.area() / element_rect.area();
            (rect, ratio, ratio > 0.0)
        } else {
            (Rect::new(0.0, 0.0, 0.0, 0.0), 0.0, false)
        };
        
        IntersectionObserverEntry {
            target: 0, // Would be set by caller
            bounding_client_rect: element_rect,
            root_bounds: Some(root_rect),
            intersection_rect,
            intersection_ratio,
            is_intersecting,
            time: 0.0, // Would use performance.now()
        }
    }
}

/// Resize Observer - watches element size changes
pub struct ResizeObserver {
    /// Observer ID
    id: ObserverId,
    /// Callback
    callback: Arc<Mutex<Box<dyn Fn(&[ResizeObserverEntry]) + Send>>>,
    /// Observed elements with last known sizes
    observed_elements: HashMap<u64, ResizeObserverSize>,
}

/// Resize observer entry
#[derive(Debug, Clone)]
pub struct ResizeObserverEntry {
    /// Target element ID
    pub target: u64,
    /// Content rect
    pub content_rect: Rect,
    /// Border box size
    pub border_box_size: ResizeObserverSize,
    /// Content box size
    pub content_box_size: ResizeObserverSize,
}

/// Resize observer size
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResizeObserverSize {
    pub inline_size: f32,
    pub block_size: f32,
}

impl ResizeObserver {
    /// Create a new resize observer
    pub fn new<F>(id: ObserverId, callback: F) -> Self
    where
        F: Fn(&[ResizeObserverEntry]) + Send + 'static,
    {
        Self {
            id,
            callback: Arc::new(Mutex::new(Box::new(callback))),
            observed_elements: HashMap::new(),
        }
    }
    
    /// Observe an element
    pub fn observe(&mut self, element_id: u64, initial_size: ResizeObserverSize) {
        self.observed_elements.insert(element_id, initial_size);
    }
    
    /// Unobserve an element
    pub fn unobserve(&mut self, element_id: u64) {
        self.observed_elements.remove(&element_id);
    }
    
    /// Disconnect observer
    pub fn disconnect(&mut self) {
        self.observed_elements.clear();
    }
    
    /// Check if observing an element
    pub fn is_observing(&self, element_id: u64) -> bool {
        self.observed_elements.contains_key(&element_id)
    }
    
    /// Check for size changes and notify if needed
    pub fn check_resize(&mut self, element_id: u64, new_size: ResizeObserverSize) -> bool {
        if let Some(old_size) = self.observed_elements.get(&element_id) {
            if *old_size != new_size {
                self.observed_elements.insert(element_id, new_size);
                
                let entry = ResizeObserverEntry {
                    target: element_id,
                    content_rect: Rect::new(0.0, 0.0, new_size.inline_size, new_size.block_size),
                    border_box_size: new_size,
                    content_box_size: new_size,
                };
                
                if let Ok(callback) = self.callback.lock() {
                    callback(&[entry]);
                }
                
                return true;
            }
        }
        false
    }
}

/// Observer manager
pub struct ObserverManager {
    /// Next observer ID
    next_id: ObserverId,
    /// Mutation observers
    mutation_observers: HashMap<ObserverId, MutationObserver>,
    /// Intersection observers
    intersection_observers: HashMap<ObserverId, IntersectionObserver>,
    /// Resize observers
    resize_observers: HashMap<ObserverId, ResizeObserver>,
    /// Pending mutation records
    pending_mutations: Vec<(ObserverId, MutationRecord)>,
}

impl ObserverManager {
    /// Create a new observer manager
    pub fn new() -> Self {
        Self {
            next_id: 1,
            mutation_observers: HashMap::new(),
            intersection_observers: HashMap::new(),
            resize_observers: HashMap::new(),
            pending_mutations: Vec::new(),
        }
    }
    
    /// Create a mutation observer
    pub fn create_mutation_observer<F>(&mut self, callback: F) -> ObserverId
    where
        F: Fn(&[MutationRecord]) + Send + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        
        let observer = MutationObserver::new(id, callback);
        self.mutation_observers.insert(id, observer);
        id
    }
    
    /// Create an intersection observer
    pub fn create_intersection_observer<F>(&mut self, callback: F) -> ObserverId
    where
        F: Fn(&[IntersectionObserverEntry]) + Send + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        
        let observer = IntersectionObserver::new(id, callback);
        self.intersection_observers.insert(id, observer);
        id
    }
    
    /// Create a resize observer
    pub fn create_resize_observer<F>(&mut self, callback: F) -> ObserverId
    where
        F: Fn(&[ResizeObserverEntry]) + Send + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        
        let observer = ResizeObserver::new(id, callback);
        self.resize_observers.insert(id, observer);
        id
    }
    
    /// Get mutation observer
    pub fn get_mutation_observer(&mut self, id: ObserverId) -> Option<&mut MutationObserver> {
        self.mutation_observers.get_mut(&id)
    }
    
    /// Get intersection observer
    pub fn get_intersection_observer(&mut self, id: ObserverId) -> Option<&mut IntersectionObserver> {
        self.intersection_observers.get_mut(&id)
    }
    
    /// Get resize observer
    pub fn get_resize_observer(&mut self, id: ObserverId) -> Option<&mut ResizeObserver> {
        self.resize_observers.get_mut(&id)
    }
    
    /// Record a mutation
    pub fn record_mutation(&mut self, node_id: u64, record: MutationRecord) {
        // Find all observers watching this node
        for (observer_id, observer) in &self.mutation_observers {
            if observer.is_observing(node_id) {
                self.pending_mutations.push((*observer_id, record.clone()));
            }
        }
    }
    
    /// Flush pending mutations
    pub fn flush_mutations(&mut self) {
        let mut by_observer: HashMap<ObserverId, Vec<MutationRecord>> = HashMap::new();
        
        for (observer_id, record) in self.pending_mutations.drain(..) {
            by_observer.entry(observer_id).or_insert_with(Vec::new).push(record);
        }
        
        for (observer_id, records) in by_observer {
            if let Some(observer) = self.mutation_observers.get(&observer_id) {
                observer.notify(&records);
            }
        }
    }
    
    /// Disconnect all observers
    pub fn disconnect_all(&mut self) {
        for observer in self.mutation_observers.values_mut() {
            observer.disconnect();
        }
        for observer in self.intersection_observers.values_mut() {
            observer.disconnect();
        }
        for observer in self.resize_observers.values_mut() {
            observer.disconnect();
        }
        self.pending_mutations.clear();
    }
}

impl Default for ObserverManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mutation_observer() {
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        
        let mut observer = MutationObserver::new(1, move |_records| {
            *called_clone.lock().unwrap() = true;
        });
        
        let config = MutationObserverInit {
            child_list: true,
            ..Default::default()
        };
        
        observer.observe(100, config);
        assert!(observer.is_observing(100));
        
        let record = MutationRecord {
            mutation_type: MutationType::ChildList,
            target: 100,
            added_nodes: vec![101],
            removed_nodes: vec![],
            previous_sibling: None,
            next_sibling: None,
            attribute_name: None,
            old_value: None,
        };
        
        observer.notify(&[record]);
        assert!(*called.lock().unwrap());
    }
    
    #[test]
    fn test_intersection_observer() {
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        
        let mut observer = IntersectionObserver::new(1, move |_entries| {
            *called_clone.lock().unwrap() = true;
        });
        
        observer.observe(100);
        assert!(observer.is_observing(100));
        
        let element_rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let root_rect = Rect::new(0.0, 0.0, 200.0, 200.0);
        
        let entry = observer.calculate_intersection(element_rect, root_rect);
        assert_eq!(entry.intersection_ratio, 1.0);
        assert!(entry.is_intersecting);
        
        observer.notify(&[entry]);
        assert!(*called.lock().unwrap());
    }
    
    #[test]
    fn test_rect_intersection() {
        let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        
        let intersection = r1.intersection(&r2).unwrap();
        assert_eq!(intersection.x, 50.0);
        assert_eq!(intersection.y, 50.0);
        assert_eq!(intersection.width, 50.0);
        assert_eq!(intersection.height, 50.0);
    }
    
    #[test]
    fn test_resize_observer() {
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        
        let mut observer = ResizeObserver::new(1, move |_entries| {
            *called_clone.lock().unwrap() = true;
        });
        
        let initial_size = ResizeObserverSize {
            inline_size: 100.0,
            block_size: 100.0,
        };
        
        observer.observe(100, initial_size);
        assert!(observer.is_observing(100));
        
        let new_size = ResizeObserverSize {
            inline_size: 200.0,
            block_size: 200.0,
        };
        
        let changed = observer.check_resize(100, new_size);
        assert!(changed);
        assert!(*called.lock().unwrap());
    }
    
    #[test]
    fn test_observer_manager() {
        let mut manager = ObserverManager::new();
        
        let id = manager.create_mutation_observer(|_| {});
        assert!(manager.get_mutation_observer(id).is_some());
        
        let config = MutationObserverInit::default();
        manager.get_mutation_observer(id).unwrap().observe(100, config);
        
        let record = MutationRecord {
            mutation_type: MutationType::ChildList,
            target: 100,
            added_nodes: vec![101],
            removed_nodes: vec![],
            previous_sibling: None,
            next_sibling: None,
            attribute_name: None,
            old_value: None,
        };
        
        manager.record_mutation(100, record);
        manager.flush_mutations();
    }
    
    #[test]
    fn test_observer_disconnect() {
        let mut observer = MutationObserver::new(1, |_| {});
        observer.observe(100, MutationObserverInit::default());
        
        assert!(observer.is_observing(100));
        observer.disconnect();
        assert!(!observer.is_observing(100));
    }
}
