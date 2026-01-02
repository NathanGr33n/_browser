use url::Url;

use super::{NetError, ResourceLoader};
use crate::dom::Node;
use crate::html::HtmlParser;
use crate::css::{Stylesheet, CssParser};

/// Page loader that fetches and processes web pages
pub struct PageLoader {
    resource_loader: ResourceLoader,
}

impl PageLoader {
    /// Create a new page loader with default cache
    pub fn new() -> Self {
        Self {
            resource_loader: ResourceLoader::with_default_cache(),
        }
    }

    /// Create a page loader with specific cache size (in bytes)
    pub fn with_cache_size(size: usize) -> Self {
        Self {
            resource_loader: ResourceLoader::new(size),
        }
    }

    /// Load a complete page: fetch HTML, parse DOM, fetch CSS, extract images
    pub fn load_page(&self, url: &Url) -> Result<LoadedPage, NetError> {
        // Fetch HTML
        let html_text = self.resource_loader.load_text(url)?;

        // Parse HTML to DOM
        let dom = HtmlParser::parse(&html_text);

        // Extract and fetch CSS resources
        let stylesheets = self.extract_and_load_css(&dom, url)?;
        
        // Extract image URLs
        let image_urls = self.extract_image_urls(&dom, url);

        Ok(LoadedPage {
            url: url.clone(),
            dom,
            stylesheets,
            image_urls,
        })
    }

    /// Extract CSS from <style> tags and <link> tags, then fetch external stylesheets
    fn extract_and_load_css(&self, dom: &Node, base_url: &Url) -> Result<Vec<Stylesheet>, NetError> {
        let mut stylesheets = Vec::new();

        // Recursively find style and link elements
        self.collect_css_from_node(dom, base_url, &mut stylesheets)?;

        Ok(stylesheets)
    }

    /// Recursively collect CSS from DOM nodes
    fn collect_css_from_node(
        &self,
        node: &Node,
        base_url: &Url,
        stylesheets: &mut Vec<Stylesheet>,
    ) -> Result<(), NetError> {
        if let Some(elem) = node.element_data() {
            // Handle <style> tags with inline CSS
            if elem.tag_name.to_lowercase() == "style" {
                // Get text content from child text nodes
                let css_text = self.extract_text_content(node);
                if !css_text.is_empty() {
                    let stylesheet = CssParser::parse(&css_text);
                    stylesheets.push(stylesheet);
                }
            }

            // Handle <link rel="stylesheet"> tags
            if elem.tag_name.to_lowercase() == "link" {
                if let Some(rel) = elem.attributes.get("rel") {
                    if rel.to_lowercase() == "stylesheet" {
                        if let Some(href) = elem.attributes.get("href") {
                            // Resolve relative URL
                            if let Ok(css_url) = base_url.join(href) {
                                // Fetch CSS
                                match self.resource_loader.load_text(&css_url) {
                                    Ok(css_text) => {
                                        let stylesheet = CssParser::parse(&css_text);
                                        stylesheets.push(stylesheet);
                                    }
                                    Err(_) => {
                                        // Silently ignore CSS loading errors
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Recursively process children
        for child in &node.children {
            self.collect_css_from_node(child, base_url, stylesheets)?;
        }

        Ok(())
    }

    /// Extract text content from a node and its children
    fn extract_text_content(&self, node: &Node) -> String {
        if let Some(text) = node.text_content() {
            return text.to_string();
        }
        
        let mut result = String::new();
        for child in &node.children {
            result.push_str(&self.extract_text_content(child));
        }
        result
    }

    /// Get access to the resource loader for manual resource loading
    pub fn resource_loader(&self) -> &ResourceLoader {
        &self.resource_loader
    }

    /// Extract image URLs from <img> tags
    fn extract_image_urls(&self, dom: &Node, base_url: &Url) -> Vec<Url> {
        let mut urls = Vec::new();
        self.collect_image_urls(dom, base_url, &mut urls);
        urls
    }
    
    /// Recursively collect image URLs from DOM nodes
    fn collect_image_urls(&self, node: &Node, base_url: &Url, urls: &mut Vec<Url>) {
        if let Some(elem) = node.element_data() {
            // Handle <img> tags
            if elem.tag_name.to_lowercase() == "img" {
                if let Some(src) = elem.attributes.get("src") {
                    // Resolve relative URL
                    if let Ok(img_url) = base_url.join(src) {
                        urls.push(img_url);
                    }
                }
            }
        }
        
        // Recursively process children
        for child in &node.children {
            self.collect_image_urls(child, base_url, urls);
        }
    }
    
    /// Clear the cache
    pub fn clear_cache(&self) {
        self.resource_loader.clear_cache();
    }
}

impl Default for PageLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// A loaded page with DOM, stylesheets, and image URLs
pub struct LoadedPage {
    pub url: Url,
    pub dom: Node,
    pub stylesheets: Vec<Stylesheet>,
    pub image_urls: Vec<Url>,
}

impl LoadedPage {
    /// Get a merged stylesheet from all loaded stylesheets
    pub fn merged_stylesheet(&self) -> Stylesheet {
        if !self.stylesheets.is_empty() {
            let mut all_rules = Vec::new();
            for stylesheet in &self.stylesheets {
                all_rules.extend(stylesheet.rules.clone());
            }
            Stylesheet::new(all_rules)
        } else {
            Stylesheet::new(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_loader_creation() {
        let loader = PageLoader::new();
        assert!(loader.resource_loader.cache_count() == 0);
    }

    #[test]
    fn test_page_loader_with_cache_size() {
        let loader = PageLoader::with_cache_size(1024);
        assert!(loader.resource_loader.cache_count() == 0);
    }

    #[test]
    fn test_extract_text_content() {
        let loader = PageLoader::new();
        
        // Create a simple DOM tree
        let text_node = Node::text("Hello World".to_string());
        let content = loader.extract_text_content(&text_node);
        assert_eq!(content, "Hello World");
    }

    #[test]
    fn test_loaded_page_merged_stylesheet() {
        // Create a minimal page
        let html = "<html><body><p>Test</p></body></html>";
        let dom = HtmlParser::parse(html);
        
        let page = LoadedPage {
            url: Url::parse("http://example.com").unwrap(),
            dom,
            stylesheets: Vec::new(),
            image_urls: Vec::new(),
        };

        // Should return empty stylesheet
        let stylesheet = page.merged_stylesheet();
        assert_eq!(stylesheet.rules.len(), 0);
    }

    #[test]
    fn test_loaded_page_merged_stylesheet_multiple() {
        let html = "<html><body><p>Test</p></body></html>";
        let dom = HtmlParser::parse(html);
        
        let css1 = CssParser::parse("p { color: red; }");
        let css2 = CssParser::parse("p { font-size: 16px; }");
        
        let page = LoadedPage {
            url: Url::parse("http://example.com").unwrap(),
            dom,
            stylesheets: vec![css1, css2],
            image_urls: Vec::new(),
        };

        // Should merge both stylesheets
        let stylesheet = page.merged_stylesheet();
        assert_eq!(stylesheet.rules.len(), 2);
    }
    
    #[test]
    fn test_extract_image_urls() {
        use std::collections::HashMap;
        
        let loader = PageLoader::new();
        let base_url = Url::parse("http://example.com/page").unwrap();
        
        // Create DOM with img tags
        let mut attrs1 = HashMap::new();
        attrs1.insert("src".to_string(), "image1.png".to_string());
        let img1 = Node::element("img".to_string(), attrs1, vec![]);
        
        let mut attrs2 = HashMap::new();
        attrs2.insert("src".to_string(), "/images/image2.jpg".to_string());
        let img2 = Node::element("img".to_string(), attrs2, vec![]);
        
        let body = Node::element("body".to_string(), HashMap::new(), vec![img1, img2]);
        
        let urls = loader.extract_image_urls(&body, &base_url);
        
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0].as_str(), "http://example.com/image1.png");
        assert_eq!(urls[1].as_str(), "http://example.com/images/image2.jpg");
    }
}
