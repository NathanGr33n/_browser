// Performance APIs - Phase 8 Advanced JavaScript

use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Performance interface - high-resolution timing
pub struct Performance {
    /// Time origin (when navigation started)
    time_origin: Instant,
    /// Navigation timing
    navigation_timing: NavigationTiming,
    /// Resource timing entries
    resource_entries: Vec<PerformanceResourceTiming>,
    /// User timing marks
    marks: HashMap<String, DOMHighResTimeStamp>,
    /// User timing measures
    measures: HashMap<String, PerformanceMeasure>,
    /// Memory info (if available)
    memory: Option<MemoryInfo>,
}

/// High-resolution timestamp (milliseconds since time origin)
pub type DOMHighResTimeStamp = f64;

/// Navigation timing information
#[derive(Debug, Clone)]
pub struct NavigationTiming {
    /// Navigation start time
    pub navigation_start: DOMHighResTimeStamp,
    /// Unload event start
    pub unload_event_start: DOMHighResTimeStamp,
    /// Unload event end
    pub unload_event_end: DOMHighResTimeStamp,
    /// Redirect start
    pub redirect_start: DOMHighResTimeStamp,
    /// Redirect end
    pub redirect_end: DOMHighResTimeStamp,
    /// Fetch start
    pub fetch_start: DOMHighResTimeStamp,
    /// Domain lookup start
    pub domain_lookup_start: DOMHighResTimeStamp,
    /// Domain lookup end
    pub domain_lookup_end: DOMHighResTimeStamp,
    /// Connect start
    pub connect_start: DOMHighResTimeStamp,
    /// Connect end
    pub connect_end: DOMHighResTimeStamp,
    /// Secure connection start
    pub secure_connection_start: DOMHighResTimeStamp,
    /// Request start
    pub request_start: DOMHighResTimeStamp,
    /// Response start
    pub response_start: DOMHighResTimeStamp,
    /// Response end
    pub response_end: DOMHighResTimeStamp,
    /// DOM loading
    pub dom_loading: DOMHighResTimeStamp,
    /// DOM interactive
    pub dom_interactive: DOMHighResTimeStamp,
    /// DOM content loaded event start
    pub dom_content_loaded_event_start: DOMHighResTimeStamp,
    /// DOM content loaded event end
    pub dom_content_loaded_event_end: DOMHighResTimeStamp,
    /// DOM complete
    pub dom_complete: DOMHighResTimeStamp,
    /// Load event start
    pub load_event_start: DOMHighResTimeStamp,
    /// Load event end
    pub load_event_end: DOMHighResTimeStamp,
}

impl Default for NavigationTiming {
    fn default() -> Self {
        Self {
            navigation_start: 0.0,
            unload_event_start: 0.0,
            unload_event_end: 0.0,
            redirect_start: 0.0,
            redirect_end: 0.0,
            fetch_start: 0.0,
            domain_lookup_start: 0.0,
            domain_lookup_end: 0.0,
            connect_start: 0.0,
            connect_end: 0.0,
            secure_connection_start: 0.0,
            request_start: 0.0,
            response_start: 0.0,
            response_end: 0.0,
            dom_loading: 0.0,
            dom_interactive: 0.0,
            dom_content_loaded_event_start: 0.0,
            dom_content_loaded_event_end: 0.0,
            dom_complete: 0.0,
            load_event_start: 0.0,
            load_event_end: 0.0,
        }
    }
}

/// Performance resource timing entry
#[derive(Debug, Clone)]
pub struct PerformanceResourceTiming {
    /// Entry name (URL)
    pub name: String,
    /// Entry type
    pub entry_type: String,
    /// Start time
    pub start_time: DOMHighResTimeStamp,
    /// Duration
    pub duration: DOMHighResTimeStamp,
    /// Initiator type (e.g., "link", "script", "img")
    pub initiator_type: String,
    /// Next hop protocol (e.g., "http/1.1", "h2")
    pub next_hop_protocol: String,
    /// Worker start
    pub worker_start: DOMHighResTimeStamp,
    /// Redirect start
    pub redirect_start: DOMHighResTimeStamp,
    /// Redirect end
    pub redirect_end: DOMHighResTimeStamp,
    /// Fetch start
    pub fetch_start: DOMHighResTimeStamp,
    /// Domain lookup start
    pub domain_lookup_start: DOMHighResTimeStamp,
    /// Domain lookup end
    pub domain_lookup_end: DOMHighResTimeStamp,
    /// Connect start
    pub connect_start: DOMHighResTimeStamp,
    /// Connect end
    pub connect_end: DOMHighResTimeStamp,
    /// Secure connection start
    pub secure_connection_start: DOMHighResTimeStamp,
    /// Request start
    pub request_start: DOMHighResTimeStamp,
    /// Response start
    pub response_start: DOMHighResTimeStamp,
    /// Response end
    pub response_end: DOMHighResTimeStamp,
    /// Transfer size (bytes)
    pub transfer_size: u64,
    /// Encoded body size (bytes)
    pub encoded_body_size: u64,
    /// Decoded body size (bytes)
    pub decoded_body_size: u64,
}

/// Performance measure
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceMeasure {
    /// Measure name
    pub name: String,
    /// Entry type
    pub entry_type: String,
    /// Start time
    pub start_time: DOMHighResTimeStamp,
    /// Duration
    pub duration: DOMHighResTimeStamp,
}

/// Memory information
#[derive(Debug, Clone, Copy)]
pub struct MemoryInfo {
    /// Used JS heap size (bytes)
    pub used_js_heap_size: u64,
    /// Total JS heap size (bytes)
    pub total_js_heap_size: u64,
    /// JS heap size limit (bytes)
    pub js_heap_size_limit: u64,
}

impl Performance {
    /// Create a new Performance instance
    pub fn new() -> Self {
        Self {
            time_origin: Instant::now(),
            navigation_timing: NavigationTiming::default(),
            resource_entries: Vec::new(),
            marks: HashMap::new(),
            measures: HashMap::new(),
            memory: Some(MemoryInfo {
                used_js_heap_size: 0,
                total_js_heap_size: 0,
                js_heap_size_limit: 2 * 1024 * 1024 * 1024, // 2GB default
            }),
        }
    }
    
    /// Get current high-resolution time
    pub fn now(&self) -> DOMHighResTimeStamp {
        let elapsed = self.time_origin.elapsed();
        elapsed.as_secs_f64() * 1000.0
    }
    
    /// Get time origin as Unix timestamp
    pub fn time_origin(&self) -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs_f64() * 1000.0
            - self.now()
    }
    
    /// Create a performance mark
    pub fn mark(&mut self, name: String) -> Result<(), PerformanceError> {
        let timestamp = self.now();
        self.marks.insert(name, timestamp);
        Ok(())
    }
    
    /// Clear marks
    pub fn clear_marks(&mut self, name: Option<&str>) {
        if let Some(mark_name) = name {
            self.marks.remove(mark_name);
        } else {
            self.marks.clear();
        }
    }
    
    /// Create a performance measure
    pub fn measure(
        &mut self,
        name: String,
        start_mark: Option<&str>,
        end_mark: Option<&str>,
    ) -> Result<PerformanceMeasure, PerformanceError> {
        let start_time = if let Some(mark) = start_mark {
            *self.marks.get(mark).ok_or(PerformanceError::MarkNotFound)?
        } else {
            0.0
        };
        
        let end_time = if let Some(mark) = end_mark {
            *self.marks.get(mark).ok_or(PerformanceError::MarkNotFound)?
        } else {
            self.now()
        };
        
        let measure = PerformanceMeasure {
            name: name.clone(),
            entry_type: "measure".to_string(),
            start_time,
            duration: end_time - start_time,
        };
        
        self.measures.insert(name, measure.clone());
        Ok(measure)
    }
    
    /// Clear measures
    pub fn clear_measures(&mut self, name: Option<&str>) {
        if let Some(measure_name) = name {
            self.measures.remove(measure_name);
        } else {
            self.measures.clear();
        }
    }
    
    /// Get entries by type
    pub fn get_entries_by_type(&self, entry_type: &str) -> Vec<PerformanceEntry> {
        let mut entries = Vec::new();
        
        match entry_type {
            "mark" => {
                for (name, timestamp) in &self.marks {
                    entries.push(PerformanceEntry::Mark {
                        name: name.clone(),
                        start_time: *timestamp,
                    });
                }
            }
            "measure" => {
                for measure in self.measures.values() {
                    entries.push(PerformanceEntry::Measure(measure.clone()));
                }
            }
            "resource" => {
                for resource in &self.resource_entries {
                    entries.push(PerformanceEntry::Resource(resource.clone()));
                }
            }
            "navigation" => {
                entries.push(PerformanceEntry::Navigation(self.navigation_timing.clone()));
            }
            _ => {}
        }
        
        entries
    }
    
    /// Get entries by name
    pub fn get_entries_by_name(&self, name: &str) -> Vec<PerformanceEntry> {
        let mut entries = Vec::new();
        
        if let Some(timestamp) = self.marks.get(name) {
            entries.push(PerformanceEntry::Mark {
                name: name.to_string(),
                start_time: *timestamp,
            });
        }
        
        if let Some(measure) = self.measures.get(name) {
            entries.push(PerformanceEntry::Measure(measure.clone()));
        }
        
        for resource in &self.resource_entries {
            if resource.name == name {
                entries.push(PerformanceEntry::Resource(resource.clone()));
            }
        }
        
        entries
    }
    
    /// Get all entries
    pub fn get_entries(&self) -> Vec<PerformanceEntry> {
        let mut entries = Vec::new();
        
        for (name, timestamp) in &self.marks {
            entries.push(PerformanceEntry::Mark {
                name: name.clone(),
                start_time: *timestamp,
            });
        }
        
        for measure in self.measures.values() {
            entries.push(PerformanceEntry::Measure(measure.clone()));
        }
        
        for resource in &self.resource_entries {
            entries.push(PerformanceEntry::Resource(resource.clone()));
        }
        
        entries.push(PerformanceEntry::Navigation(self.navigation_timing.clone()));
        
        entries
    }
    
    /// Add resource timing entry
    pub fn add_resource_entry(&mut self, entry: PerformanceResourceTiming) {
        // Limit to 150 resource entries (browser default)
        if self.resource_entries.len() >= 150 {
            self.resource_entries.remove(0);
        }
        self.resource_entries.push(entry);
    }
    
    /// Clear resource timings
    pub fn clear_resource_timings(&mut self) {
        self.resource_entries.clear();
    }
    
    /// Set navigation timing value
    pub fn set_navigation_timing(&mut self, timing: NavigationTiming) {
        self.navigation_timing = timing;
    }
    
    /// Get navigation timing
    pub fn navigation_timing(&self) -> &NavigationTiming {
        &self.navigation_timing
    }
    
    /// Get memory info
    pub fn memory(&self) -> Option<&MemoryInfo> {
        self.memory.as_ref()
    }
    
    /// Update memory info
    pub fn update_memory(&mut self, used: u64, total: u64) {
        if let Some(memory) = &mut self.memory {
            memory.used_js_heap_size = used;
            memory.total_js_heap_size = total;
        }
    }
    
    /// Convert to JSON-like structure for debugging
    pub fn to_json(&self) -> String {
        format!(
            "{{\"timeOrigin\":{},\"now\":{},\"marks\":{},\"measures\":{}}}",
            self.time_origin(),
            self.now(),
            self.marks.len(),
            self.measures.len()
        )
    }
}

impl Default for Performance {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance entry types
#[derive(Debug, Clone)]
pub enum PerformanceEntry {
    Mark { name: String, start_time: DOMHighResTimeStamp },
    Measure(PerformanceMeasure),
    Resource(PerformanceResourceTiming),
    Navigation(NavigationTiming),
}

/// Performance errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerformanceError {
    MarkNotFound,
    InvalidMeasure,
    InvalidEntry,
}

impl std::fmt::Display for PerformanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PerformanceError::MarkNotFound => write!(f, "Performance mark not found"),
            PerformanceError::InvalidMeasure => write!(f, "Invalid performance measure"),
            PerformanceError::InvalidEntry => write!(f, "Invalid performance entry"),
        }
    }
}

impl std::error::Error for PerformanceError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_performance_now() {
        let perf = Performance::new();
        let t1 = perf.now();
        
        thread::sleep(Duration::from_millis(10));
        
        let t2 = perf.now();
        assert!(t2 > t1);
        assert!(t2 - t1 >= 10.0);
    }
    
    #[test]
    fn test_performance_mark() {
        let mut perf = Performance::new();
        
        perf.mark("mark1".to_string()).unwrap();
        assert!(perf.marks.contains_key("mark1"));
        
        perf.clear_marks(Some("mark1"));
        assert!(!perf.marks.contains_key("mark1"));
    }
    
    #[test]
    fn test_performance_measure() {
        let mut perf = Performance::new();
        
        perf.mark("start".to_string()).unwrap();
        thread::sleep(Duration::from_millis(10));
        perf.mark("end".to_string()).unwrap();
        
        let measure = perf.measure("test".to_string(), Some("start"), Some("end")).unwrap();
        assert!(measure.duration >= 10.0);
        assert_eq!(measure.name, "test");
    }
    
    #[test]
    fn test_measure_without_marks() {
        let mut perf = Performance::new();
        
        thread::sleep(Duration::from_millis(10));
        let measure = perf.measure("test".to_string(), None, None).unwrap();
        
        assert!(measure.duration >= 10.0);
    }
    
    #[test]
    fn test_mark_not_found() {
        let mut perf = Performance::new();
        
        let result = perf.measure("test".to_string(), Some("nonexistent"), None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PerformanceError::MarkNotFound);
    }
    
    #[test]
    fn test_get_entries_by_type() {
        let mut perf = Performance::new();
        
        perf.mark("mark1".to_string()).unwrap();
        perf.mark("mark2".to_string()).unwrap();
        
        let marks = perf.get_entries_by_type("mark");
        assert_eq!(marks.len(), 2);
    }
    
    #[test]
    fn test_get_entries_by_name() {
        let mut perf = Performance::new();
        
        perf.mark("test".to_string()).unwrap();
        
        let entries = perf.get_entries_by_name("test");
        assert_eq!(entries.len(), 1);
    }
    
    #[test]
    fn test_resource_timing() {
        let mut perf = Performance::new();
        
        let resource = PerformanceResourceTiming {
            name: "https://example.com/script.js".to_string(),
            entry_type: "resource".to_string(),
            start_time: 100.0,
            duration: 50.0,
            initiator_type: "script".to_string(),
            next_hop_protocol: "h2".to_string(),
            worker_start: 0.0,
            redirect_start: 0.0,
            redirect_end: 0.0,
            fetch_start: 100.0,
            domain_lookup_start: 105.0,
            domain_lookup_end: 110.0,
            connect_start: 110.0,
            connect_end: 120.0,
            secure_connection_start: 115.0,
            request_start: 120.0,
            response_start: 140.0,
            response_end: 150.0,
            transfer_size: 1024,
            encoded_body_size: 1024,
            decoded_body_size: 2048,
        };
        
        perf.add_resource_entry(resource);
        
        let resources = perf.get_entries_by_type("resource");
        assert_eq!(resources.len(), 1);
    }
    
    #[test]
    fn test_resource_limit() {
        let mut perf = Performance::new();
        
        for i in 0..200 {
            let resource = PerformanceResourceTiming {
                name: format!("resource{}", i),
                entry_type: "resource".to_string(),
                start_time: 0.0,
                duration: 0.0,
                initiator_type: "script".to_string(),
                next_hop_protocol: "h2".to_string(),
                worker_start: 0.0,
                redirect_start: 0.0,
                redirect_end: 0.0,
                fetch_start: 0.0,
                domain_lookup_start: 0.0,
                domain_lookup_end: 0.0,
                connect_start: 0.0,
                connect_end: 0.0,
                secure_connection_start: 0.0,
                request_start: 0.0,
                response_start: 0.0,
                response_end: 0.0,
                transfer_size: 0,
                encoded_body_size: 0,
                decoded_body_size: 0,
            };
            perf.add_resource_entry(resource);
        }
        
        // Should be limited to 150
        assert_eq!(perf.resource_entries.len(), 150);
    }
    
    #[test]
    fn test_memory_info() {
        let mut perf = Performance::new();
        
        assert!(perf.memory().is_some());
        
        perf.update_memory(1024, 2048);
        
        let memory = perf.memory().unwrap();
        assert_eq!(memory.used_js_heap_size, 1024);
        assert_eq!(memory.total_js_heap_size, 2048);
    }
    
    #[test]
    fn test_clear_all_marks() {
        let mut perf = Performance::new();
        
        perf.mark("mark1".to_string()).unwrap();
        perf.mark("mark2".to_string()).unwrap();
        
        perf.clear_marks(None);
        assert_eq!(perf.marks.len(), 0);
    }
    
    #[test]
    fn test_time_origin() {
        let perf = Performance::new();
        let origin = perf.time_origin();
        
        // Should be a reasonable Unix timestamp in milliseconds
        assert!(origin > 1_600_000_000_000.0); // After Sep 2020
    }
}
