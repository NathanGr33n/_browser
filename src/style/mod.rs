use crate::css::{Stylesheet, Selector, SimpleSelector, Value, specificity, Specificity};
use crate::dom::{Node, NodeType, ElementData};
use std::collections::HashMap;

/// A node with computed styles
#[derive(Debug, Clone)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

pub type PropertyMap = HashMap<String, Value>;

/// Represents CSS display property
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Display {
    Inline,
    Block,
    Flex,
    None,
}

impl<'a> StyledNode<'a> {
    /// Get the value of a specific CSS property
    pub fn value(&self, name: &str) -> Option<&Value> {
        self.specified_values.get(name)
    }

    /// Get the lookup value or a default
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .or_else(|| self.value(fallback_name))
            .unwrap_or(default)
            .clone()
    }

    /// Get the display property
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match s.as_str() {
                "block" => Display::Block,
                "flex" | "inline-flex" => Display::Flex,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }
}

/// Apply a stylesheet to a DOM tree to create a styled tree
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match &root.node_type {
            NodeType::Element(elem) => specified_values(elem, stylesheet),
            _ => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}

/// Get the specified values for an element
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Sort by specificity (lowest to highest)
    rules.sort_by_key(|&(spec, _)| spec);

    // Apply rules in order (later rules override earlier ones)
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}

/// Find all CSS rules that match an element
fn matching_rules<'a>(
    elem: &ElementData,
    stylesheet: &'a Stylesheet,
) -> Vec<(Specificity, &'a crate::css::Rule)> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| {
            rule.selectors
                .iter()
                .find(|selector| matches(elem, selector))
                .map(|selector| (specificity(selector), rule))
        })
        .collect()
}

/// Check if a selector matches an element
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match selector {
        Selector::Simple(simple) => matches_simple_selector(elem, simple),
    }
}

/// Check if a simple selector matches an element
fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check tag name
    if let Some(ref tag) = selector.tag_name {
        if elem.tag_name != *tag {
            return false;
        }
    }

    // Check id
    if let Some(ref id) = selector.id {
        if elem.id() != Some(id) {
            return false;
        }
    }

    // Check classes
    let elem_classes = elem.classes();
    for class in &selector.classes {
        if !elem_classes.contains(&class.as_str()) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::{CssParser, Color, Unit};
    use crate::dom::Node;
    use std::collections::HashMap;

    #[test]
    fn test_matches_tag_selector() {
        let mut attrs = HashMap::new();
        let elem = ElementData {
            tag_name: "div".to_string(),
            attributes: attrs,
        };

        let selector = SimpleSelector {
            tag_name: Some("div".to_string()),
            id: None,
            classes: Vec::new(),
        };

        assert!(matches_simple_selector(&elem, &selector));
    }

    #[test]
    fn test_matches_id_selector() {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "main".to_string());
        
        let elem = ElementData {
            tag_name: "div".to_string(),
            attributes: attrs,
        };

        let selector = SimpleSelector {
            tag_name: None,
            id: Some("main".to_string()),
            classes: Vec::new(),
        };

        assert!(matches_simple_selector(&elem, &selector));
    }

    #[test]
    fn test_matches_class_selector() {
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "container active".to_string());
        
        let elem = ElementData {
            tag_name: "div".to_string(),
            attributes: attrs,
        };

        let selector = SimpleSelector {
            tag_name: None,
            id: None,
            classes: vec!["container".to_string()],
        };

        assert!(matches_simple_selector(&elem, &selector));
    }

    #[test]
    fn test_style_tree() {
        let css = "div { color: red; font-size: 16px; }";
        let stylesheet = CssParser::parse(css);

        let mut attrs = HashMap::new();
        let node = Node::element("div".to_string(), attrs, vec![]);

        let styled = style_tree(&node, &stylesheet);
        
        assert!(styled.value("color").is_some());
        assert!(styled.value("font-size").is_some());
    }
}
