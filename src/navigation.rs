// Browser navigation and history management

use url::Url;
use std::collections::VecDeque;

/// Browser navigation history
#[derive(Debug, Clone)]
pub struct NavigationHistory {
    /// Stack of visited URLs
    entries: VecDeque<HistoryEntry>,
    /// Current position in history (index into entries)
    current_index: usize,
    /// Maximum history size
    max_size: usize,
}

/// A single history entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// URL of the page
    pub url: Url,
    /// Page title (if available)
    pub title: Option<String>,
    /// Timestamp when visited
    pub timestamp: std::time::SystemTime,
}

impl NavigationHistory {
    /// Create a new navigation history with default capacity
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create a new navigation history with specified capacity
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            current_index: 0,
            max_size,
        }
    }

    /// Navigate to a new URL (user clicked link or entered URL)
    pub fn navigate_to(&mut self, url: Url) {
        // Remove any forward history when navigating to new page
        if self.current_index < self.entries.len().saturating_sub(1) {
            self.entries.truncate(self.current_index + 1);
        }

        // Add new entry
        let entry = HistoryEntry {
            url,
            title: None,
            timestamp: std::time::SystemTime::now(),
        };

        self.entries.push_back(entry);

        // Maintain max size
        if self.entries.len() > self.max_size {
            self.entries.pop_front();
            // After pop_front, current_index stays at max_size - 1
            self.current_index = self.max_size.saturating_sub(1);
        } else {
            self.current_index = self.entries.len().saturating_sub(1);
        }
    }

    /// Go back in history
    pub fn go_back(&mut self) -> Option<&HistoryEntry> {
        if self.can_go_back() {
            self.current_index = self.current_index.saturating_sub(1);
            self.current_entry()
        } else {
            None
        }
    }

    /// Go forward in history
    pub fn go_forward(&mut self) -> Option<&HistoryEntry> {
        if self.can_go_forward() {
            self.current_index += 1;
            self.current_entry()
        } else {
            None
        }
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    /// Check if we can go forward
    pub fn can_go_forward(&self) -> bool {
        !self.entries.is_empty() && self.current_index < self.entries.len() - 1
    }

    /// Get the current history entry
    pub fn current_entry(&self) -> Option<&HistoryEntry> {
        self.entries.get(self.current_index)
    }

    /// Get the current URL
    pub fn current_url(&self) -> Option<&Url> {
        self.current_entry().map(|e| &e.url)
    }

    /// Update the title of the current page
    pub fn update_current_title(&mut self, title: String) {
        if let Some(entry) = self.entries.get_mut(self.current_index) {
            entry.title = Some(title);
        }
    }

    /// Get all history entries
    pub fn entries(&self) -> &VecDeque<HistoryEntry> {
        &self.entries
    }

    /// Get current position
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_index = 0;
    }

    /// Get recent history (last N entries)
    pub fn recent(&self, count: usize) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .rev()
            .take(count)
            .collect()
    }
}

impl Default for NavigationHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple bookmark manager
#[derive(Debug, Clone)]
pub struct BookmarkManager {
    bookmarks: Vec<Bookmark>,
}

/// A single bookmark
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub url: Url,
    pub title: String,
    pub created_at: std::time::SystemTime,
}

impl BookmarkManager {
    /// Create a new bookmark manager
    pub fn new() -> Self {
        Self {
            bookmarks: Vec::new(),
        }
    }

    /// Add a bookmark
    pub fn add(&mut self, url: Url, title: String) -> bool {
        // Check if already bookmarked
        if self.bookmarks.iter().any(|b| b.url == url) {
            return false;
        }

        self.bookmarks.push(Bookmark {
            url,
            title,
            created_at: std::time::SystemTime::now(),
        });
        true
    }

    /// Remove a bookmark by URL
    pub fn remove(&mut self, url: &Url) -> bool {
        let len_before = self.bookmarks.len();
        self.bookmarks.retain(|b| &b.url != url);
        self.bookmarks.len() < len_before
    }

    /// Check if URL is bookmarked
    pub fn is_bookmarked(&self, url: &Url) -> bool {
        self.bookmarks.iter().any(|b| &b.url == url)
    }

    /// Get all bookmarks
    pub fn all(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    /// Clear all bookmarks
    pub fn clear(&mut self) {
        self.bookmarks.clear();
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_url(path: &str) -> Url {
        Url::parse(&format!("http://example.com{}", path)).unwrap()
    }

    #[test]
    fn test_navigation_history_creation() {
        let history = NavigationHistory::new();
        assert_eq!(history.current_index(), 0);
        assert!(history.current_url().is_none());
    }

    #[test]
    fn test_navigate_to() {
        let mut history = NavigationHistory::new();
        
        history.navigate_to(test_url("/page1"));
        assert_eq!(history.current_url().unwrap().path(), "/page1");
        assert_eq!(history.current_index(), 0);

        history.navigate_to(test_url("/page2"));
        assert_eq!(history.current_url().unwrap().path(), "/page2");
        assert_eq!(history.current_index(), 1);
    }

    #[test]
    fn test_go_back_forward() {
        let mut history = NavigationHistory::new();
        
        history.navigate_to(test_url("/page1"));
        history.navigate_to(test_url("/page2"));
        history.navigate_to(test_url("/page3"));

        // Go back
        assert!(history.can_go_back());
        history.go_back();
        assert_eq!(history.current_url().unwrap().path(), "/page2");

        history.go_back();
        assert_eq!(history.current_url().unwrap().path(), "/page1");

        // Can't go back further
        assert!(!history.can_go_back());

        // Go forward
        assert!(history.can_go_forward());
        history.go_forward();
        assert_eq!(history.current_url().unwrap().path(), "/page2");

        history.go_forward();
        assert_eq!(history.current_url().unwrap().path(), "/page3");

        // Can't go forward further
        assert!(!history.can_go_forward());
    }

    #[test]
    fn test_navigate_clears_forward_history() {
        let mut history = NavigationHistory::new();
        
        history.navigate_to(test_url("/page1"));
        history.navigate_to(test_url("/page2"));
        history.navigate_to(test_url("/page3"));
        
        // Go back twice
        history.go_back();
        history.go_back();
        assert_eq!(history.current_url().unwrap().path(), "/page1");

        // Navigate to new page should clear forward history
        history.navigate_to(test_url("/page4"));
        assert_eq!(history.current_url().unwrap().path(), "/page4");
        assert!(!history.can_go_forward());
    }

    #[test]
    fn test_update_title() {
        let mut history = NavigationHistory::new();
        history.navigate_to(test_url("/page1"));
        
        history.update_current_title("Test Page".to_string());
        assert_eq!(history.current_entry().unwrap().title.as_deref(), Some("Test Page"));
    }

    #[test]
    fn test_recent_history() {
        let mut history = NavigationHistory::new();
        history.navigate_to(test_url("/page1"));
        history.navigate_to(test_url("/page2"));
        history.navigate_to(test_url("/page3"));

        let recent = history.recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].url.path(), "/page3");
        assert_eq!(recent[1].url.path(), "/page2");
    }

    #[test]
    fn test_bookmark_manager() {
        let mut bookmarks = BookmarkManager::new();
        
        let url = test_url("/page1");
        assert!(bookmarks.add(url.clone(), "Page 1".to_string()));
        assert!(bookmarks.is_bookmarked(&url));

        // Can't add duplicate
        assert!(!bookmarks.add(url.clone(), "Page 1".to_string()));

        assert_eq!(bookmarks.all().len(), 1);

        // Remove bookmark
        assert!(bookmarks.remove(&url));
        assert!(!bookmarks.is_bookmarked(&url));
        assert_eq!(bookmarks.all().len(), 0);
    }

    #[test]
    fn test_max_history_size() {
        let mut history = NavigationHistory::with_capacity(3);
        
        history.navigate_to(test_url("/page1"));
        history.navigate_to(test_url("/page2"));
        history.navigate_to(test_url("/page3"));
        history.navigate_to(test_url("/page4")); // Should evict page1

        assert_eq!(history.entries().len(), 3);
        // Oldest should be page2 now
        assert_eq!(history.entries()[0].url.path(), "/page2");
    }
}
