// Developer Tools - Console, DOM Inspector, Network Tab

use crate::dom::Node;
use std::time::SystemTime;
use url::Url;

/// Developer tools state
pub struct DevTools {
    /// Console log messages
    pub console: Console,
    /// DOM inspector state
    pub dom_inspector: DomInspector,
    /// Network activity log
    pub network: NetworkTab,
    /// Is devtools panel open
    pub is_open: bool,
    /// Current active tab
    pub active_tab: DevToolsTab,
}

/// Active developer tools tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevToolsTab {
    Console,
    DomInspector,
    Network,
}

impl DevTools {
    /// Create new developer tools
    pub fn new() -> Self {
        Self {
            console: Console::new(),
            dom_inspector: DomInspector::new(),
            network: NetworkTab::new(),
            is_open: false,
            active_tab: DevToolsTab::Console,
        }
    }
    
    /// Toggle devtools panel
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    
    /// Set active tab
    pub fn set_active_tab(&mut self, tab: DevToolsTab) {
        self.active_tab = tab;
    }
    
    /// Clear all devtools data
    pub fn clear_all(&mut self) {
        self.console.clear();
        self.network.clear();
    }
}

impl Default for DevTools {
    fn default() -> Self {
        Self::new()
    }
}

/// Console for logging JavaScript output and errors
pub struct Console {
    /// Console messages
    messages: Vec<ConsoleMessage>,
    /// Maximum messages to keep
    max_messages: usize,
}

/// Console message types
#[derive(Debug, Clone, PartialEq)]
pub enum ConsoleMessageType {
    Log,
    Info,
    Warn,
    Error,
    Debug,
}

/// A console message
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    /// Message type
    pub msg_type: ConsoleMessageType,
    /// Message content
    pub content: String,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Source file/location (if available)
    pub source: Option<String>,
}

impl Console {
    /// Create a new console
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            max_messages: 1000,
        }
    }
    
    /// Log a message
    pub fn log(&mut self, content: String) {
        self.add_message(ConsoleMessageType::Log, content, None);
    }
    
    /// Log an info message
    pub fn info(&mut self, content: String) {
        self.add_message(ConsoleMessageType::Info, content, None);
    }
    
    /// Log a warning
    pub fn warn(&mut self, content: String) {
        self.add_message(ConsoleMessageType::Warn, content, None);
    }
    
    /// Log an error
    pub fn error(&mut self, content: String) {
        self.add_message(ConsoleMessageType::Error, content, None);
    }
    
    /// Log a debug message
    pub fn debug(&mut self, content: String) {
        self.add_message(ConsoleMessageType::Debug, content, None);
    }
    
    /// Add a message with source
    pub fn add_message(&mut self, msg_type: ConsoleMessageType, content: String, source: Option<String>) {
        let message = ConsoleMessage {
            msg_type,
            content,
            timestamp: SystemTime::now(),
            source,
        };
        
        self.messages.push(message);
        
        // Maintain max size
        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }
    }
    
    /// Get all messages
    pub fn messages(&self) -> &[ConsoleMessage] {
        &self.messages
    }
    
    /// Get messages by type
    pub fn messages_by_type(&self, msg_type: ConsoleMessageType) -> Vec<&ConsoleMessage> {
        self.messages
            .iter()
            .filter(|m| m.msg_type == msg_type)
            .collect()
    }
    
    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
    }
    
    /// Get message count
    pub fn count(&self) -> usize {
        self.messages.len()
    }
    
    /// Get error count
    pub fn error_count(&self) -> usize {
        self.messages.iter().filter(|m| m.msg_type == ConsoleMessageType::Error).count()
    }
    
    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.messages.iter().filter(|m| m.msg_type == ConsoleMessageType::Warn).count()
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

/// DOM inspector for viewing and inspecting the DOM tree
pub struct DomInspector {
    /// Currently selected node (path from root)
    selected_node_path: Vec<usize>,
    /// Expanded nodes (paths)
    expanded_nodes: Vec<Vec<usize>>,
    /// Show text nodes
    show_text_nodes: bool,
    /// Show comments
    show_comments: bool,
}

impl DomInspector {
    /// Create a new DOM inspector
    pub fn new() -> Self {
        Self {
            selected_node_path: vec![],
            expanded_nodes: vec![vec![]],  // Root is expanded by default
            show_text_nodes: true,
            show_comments: false,
        }
    }
    
    /// Select a node by path
    pub fn select_node(&mut self, path: Vec<usize>) {
        self.selected_node_path = path;
    }
    
    /// Get selected node path
    pub fn selected_path(&self) -> &[usize] {
        &self.selected_node_path
    }
    
    /// Toggle node expansion
    pub fn toggle_node(&mut self, path: Vec<usize>) {
        if let Some(idx) = self.expanded_nodes.iter().position(|p| p == &path) {
            self.expanded_nodes.remove(idx);
        } else {
            self.expanded_nodes.push(path);
        }
    }
    
    /// Check if node is expanded
    pub fn is_expanded(&self, path: &[usize]) -> bool {
        self.expanded_nodes.iter().any(|p| p == path)
    }
    
    /// Get expanded nodes
    pub fn expanded_nodes(&self) -> &[Vec<usize>] {
        &self.expanded_nodes
    }
    
    /// Collapse all nodes
    pub fn collapse_all(&mut self) {
        self.expanded_nodes.clear();
        self.expanded_nodes.push(vec![]);  // Keep root expanded
    }
    
    /// Expand all nodes (up to a depth limit)
    pub fn expand_all(&mut self, dom: &Node, max_depth: usize) {
        self.expanded_nodes.clear();
        self.expand_recursive(dom, vec![], 0, max_depth);
    }
    
    fn expand_recursive(&mut self, node: &Node, path: Vec<usize>, depth: usize, max_depth: usize) {
        if depth >= max_depth {
            return;
        }
        
        self.expanded_nodes.push(path.clone());
        
        for (i, _child) in node.children.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.push(i);
            self.expand_recursive(&node.children[i], child_path, depth + 1, max_depth);
        }
    }
    
    /// Toggle showing text nodes
    pub fn toggle_text_nodes(&mut self) {
        self.show_text_nodes = !self.show_text_nodes;
    }
    
    /// Toggle showing comments
    pub fn toggle_comments(&mut self) {
        self.show_comments = !self.show_comments;
    }
    
    /// Check if text nodes are shown
    pub fn shows_text_nodes(&self) -> bool {
        self.show_text_nodes
    }
    
    /// Check if comments are shown
    pub fn shows_comments(&self) -> bool {
        self.show_comments
    }
    
    /// Get node at path
    pub fn get_node_at_path<'a>(&self, root: &'a Node, path: &[usize]) -> Option<&'a Node> {
        let mut current = root;
        for &idx in path {
            current = current.children.get(idx)?;
        }
        Some(current)
    }
}

impl Default for DomInspector {
    fn default() -> Self {
        Self::new()
    }
}

/// Network activity logger
pub struct NetworkTab {
    /// Network requests
    requests: Vec<NetworkRequest>,
    /// Maximum requests to keep
    max_requests: usize,
}

/// Network request record
#[derive(Debug, Clone)]
pub struct NetworkRequest {
    /// Request URL
    pub url: Url,
    /// HTTP method
    pub method: String,
    /// Status code (if completed)
    pub status: Option<u16>,
    /// Response size in bytes
    pub size: Option<usize>,
    /// Request duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Request start time
    pub started_at: SystemTime,
    /// Request end time
    pub completed_at: Option<SystemTime>,
    /// Content type
    pub content_type: Option<String>,
    /// Request type (Document, Stylesheet, Script, Image, etc.)
    pub request_type: NetworkRequestType,
}

/// Type of network request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkRequestType {
    Document,
    Stylesheet,
    Script,
    Image,
    Font,
    Media,
    XHR,
    Fetch,
    WebSocket,
    Other,
}

impl NetworkTab {
    /// Create a new network tab
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
            max_requests: 500,
        }
    }
    
    /// Log a new request
    pub fn log_request(&mut self, url: Url, method: String, request_type: NetworkRequestType) -> usize {
        let request = NetworkRequest {
            url,
            method,
            status: None,
            size: None,
            duration_ms: None,
            started_at: SystemTime::now(),
            completed_at: None,
            content_type: None,
            request_type,
        };
        
        self.requests.push(request);
        
        // Maintain max size
        if self.requests.len() > self.max_requests {
            self.requests.remove(0);
            return self.requests.len() - 1;
        }
        
        self.requests.len() - 1
    }
    
    /// Complete a request with response data
    pub fn complete_request(&mut self, idx: usize, status: u16, size: usize, content_type: Option<String>) {
        if let Some(request) = self.requests.get_mut(idx) {
            request.status = Some(status);
            request.size = Some(size);
            request.content_type = content_type;
            request.completed_at = Some(SystemTime::now());
            
            // Calculate duration
            if let Ok(duration) = request.completed_at.unwrap().duration_since(request.started_at) {
                request.duration_ms = Some(duration.as_millis() as u64);
            }
        }
    }
    
    /// Get all requests
    pub fn requests(&self) -> &[NetworkRequest] {
        &self.requests
    }
    
    /// Get requests by type
    pub fn requests_by_type(&self, request_type: NetworkRequestType) -> Vec<&NetworkRequest> {
        self.requests
            .iter()
            .filter(|r| r.request_type == request_type)
            .collect()
    }
    
    /// Get total requests count
    pub fn count(&self) -> usize {
        self.requests.len()
    }
    
    /// Get total transferred bytes
    pub fn total_size(&self) -> usize {
        self.requests.iter().filter_map(|r| r.size).sum()
    }
    
    /// Get failed requests count (status >= 400 or no status)
    pub fn failed_count(&self) -> usize {
        self.requests
            .iter()
            .filter(|r| match r.status {
                Some(status) => status >= 400,
                None => r.completed_at.is_some(), // Started but no status = failed
            })
            .count()
    }
    
    /// Clear all requests
    pub fn clear(&mut self) {
        self.requests.clear();
    }
}

impl Default for NetworkTab {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_devtools_creation() {
        let devtools = DevTools::new();
        assert!(!devtools.is_open);
        assert_eq!(devtools.active_tab, DevToolsTab::Console);
    }
    
    #[test]
    fn test_devtools_toggle() {
        let mut devtools = DevTools::new();
        devtools.toggle();
        assert!(devtools.is_open);
        devtools.toggle();
        assert!(!devtools.is_open);
    }
    
    #[test]
    fn test_console_logging() {
        let mut console = Console::new();
        console.log("Test message".to_string());
        console.error("Error message".to_string());
        
        assert_eq!(console.count(), 2);
        assert_eq!(console.error_count(), 1);
        assert_eq!(console.warning_count(), 0);
    }
    
    #[test]
    fn test_console_clear() {
        let mut console = Console::new();
        console.log("Message 1".to_string());
        console.log("Message 2".to_string());
        assert_eq!(console.count(), 2);
        
        console.clear();
        assert_eq!(console.count(), 0);
    }
    
    #[test]
    fn test_console_filter_by_type() {
        let mut console = Console::new();
        console.log("Log 1".to_string());
        console.error("Error 1".to_string());
        console.warn("Warn 1".to_string());
        console.error("Error 2".to_string());
        
        let errors = console.messages_by_type(ConsoleMessageType::Error);
        assert_eq!(errors.len(), 2);
        
        let warnings = console.messages_by_type(ConsoleMessageType::Warn);
        assert_eq!(warnings.len(), 1);
    }
    
    #[test]
    fn test_dom_inspector_selection() {
        let mut inspector = DomInspector::new();
        inspector.select_node(vec![0, 1, 2]);
        assert_eq!(inspector.selected_path(), &[0, 1, 2]);
    }
    
    #[test]
    fn test_dom_inspector_expansion() {
        let mut inspector = DomInspector::new();
        let path = vec![0, 1];
        
        assert!(!inspector.is_expanded(&path));
        inspector.toggle_node(path.clone());
        assert!(inspector.is_expanded(&path));
        inspector.toggle_node(path.clone());
        assert!(!inspector.is_expanded(&path));
    }
    
    #[test]
    fn test_dom_inspector_collapse_all() {
        let mut inspector = DomInspector::new();
        inspector.toggle_node(vec![0]);
        inspector.toggle_node(vec![1]);
        inspector.collapse_all();
        
        // Only root should be expanded
        assert_eq!(inspector.expanded_nodes().len(), 1);
        assert_eq!(&inspector.expanded_nodes()[0], &Vec::<usize>::new());
    }
    
    #[test]
    fn test_network_tab_logging() {
        let mut network = NetworkTab::new();
        let url = Url::parse("https://example.com").unwrap();
        
        let idx = network.log_request(url.clone(), "GET".to_string(), NetworkRequestType::Document);
        assert_eq!(network.count(), 1);
        
        network.complete_request(idx, 200, 1024, Some("text/html".to_string()));
        assert_eq!(network.requests()[0].status, Some(200));
        assert_eq!(network.requests()[0].size, Some(1024));
    }
    
    #[test]
    fn test_network_tab_total_size() {
        let mut network = NetworkTab::new();
        
        let url1 = Url::parse("https://example.com/page1").unwrap();
        let url2 = Url::parse("https://example.com/page2").unwrap();
        
        let idx1 = network.log_request(url1, "GET".to_string(), NetworkRequestType::Document);
        let idx2 = network.log_request(url2, "GET".to_string(), NetworkRequestType::Stylesheet);
        
        network.complete_request(idx1, 200, 1000, None);
        network.complete_request(idx2, 200, 500, None);
        
        assert_eq!(network.total_size(), 1500);
    }
    
    #[test]
    fn test_network_tab_failed_count() {
        let mut network = NetworkTab::new();
        
        let url1 = Url::parse("https://example.com/ok").unwrap();
        let url2 = Url::parse("https://example.com/notfound").unwrap();
        
        let idx1 = network.log_request(url1, "GET".to_string(), NetworkRequestType::Document);
        let idx2 = network.log_request(url2, "GET".to_string(), NetworkRequestType::Document);
        
        network.complete_request(idx1, 200, 1000, None);
        network.complete_request(idx2, 404, 100, None);
        
        assert_eq!(network.failed_count(), 1);
    }
    
    #[test]
    fn test_network_tab_filter_by_type() {
        let mut network = NetworkTab::new();
        
        let url1 = Url::parse("https://example.com/page").unwrap();
        let url2 = Url::parse("https://example.com/style.css").unwrap();
        let url3 = Url::parse("https://example.com/script.js").unwrap();
        
        network.log_request(url1, "GET".to_string(), NetworkRequestType::Document);
        network.log_request(url2, "GET".to_string(), NetworkRequestType::Stylesheet);
        network.log_request(url3, "GET".to_string(), NetworkRequestType::Script);
        
        let stylesheets = network.requests_by_type(NetworkRequestType::Stylesheet);
        assert_eq!(stylesheets.len(), 1);
    }
    
    #[test]
    fn test_network_tab_clear() {
        let mut network = NetworkTab::new();
        let url = Url::parse("https://example.com").unwrap();
        
        network.log_request(url, "GET".to_string(), NetworkRequestType::Document);
        assert_eq!(network.count(), 1);
        
        network.clear();
        assert_eq!(network.count(), 0);
    }
}
