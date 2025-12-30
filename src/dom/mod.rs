use std::collections::HashMap;

/// Represents a node in the DOM tree
#[derive(Debug, Clone)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
    Comment(String),
}

/// Element data containing tag name and attributes
#[derive(Debug, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

/// A node in the DOM tree
#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

impl Node {
    /// Create a new text node
    pub fn text(data: String) -> Node {
        Node {
            node_type: NodeType::Text(data),
            children: Vec::new(),
        }
    }

    /// Create a new element node
    pub fn element(tag_name: String, attributes: AttrMap, children: Vec<Node>) -> Node {
        Node {
            node_type: NodeType::Element(ElementData { tag_name, attributes }),
            children,
        }
    }

    /// Create a new comment node
    pub fn comment(data: String) -> Node {
        Node {
            node_type: NodeType::Comment(data),
            children: Vec::new(),
        }
    }

    /// Get the element data if this is an element node
    pub fn element_data(&self) -> Option<&ElementData> {
        match &self.node_type {
            NodeType::Element(data) => Some(data),
            _ => None,
        }
    }

    /// Get the text content if this is a text node
    pub fn text_content(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Text(text) => Some(text),
            _ => None,
        }
    }
}

impl ElementData {
    /// Get an attribute value by name
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Get the id attribute
    pub fn id(&self) -> Option<&str> {
        self.get_attribute("id")
    }

    /// Get all classes from the class attribute
    pub fn classes(&self) -> Vec<&str> {
        self.get_attribute("class")
            .map(|s| s.split_whitespace().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_node() {
        let node = Node::text("Hello, World!".to_string());
        assert!(matches!(node.node_type, NodeType::Text(_)));
        assert_eq!(node.text_content(), Some("Hello, World!"));
    }

    #[test]
    fn test_element_node() {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "main".to_string());
        attrs.insert("class".to_string(), "container active".to_string());

        let node = Node::element("div".to_string(), attrs, vec![]);
        
        let elem_data = node.element_data().unwrap();
        assert_eq!(elem_data.tag_name, "div");
        assert_eq!(elem_data.id(), Some("main"));
        assert_eq!(elem_data.classes(), vec!["container", "active"]);
    }
}
