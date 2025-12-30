use crate::dom::{Node, AttrMap};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText, ElementFlags};
use html5ever::{QualName, Attribute, ExpandedName};
use markup5ever::{LocalName, Namespace};
use std::borrow::Cow;
use std::collections::HashMap;

/// HTML parser that converts HTML strings into our DOM tree
pub struct HtmlParser;

impl HtmlParser {
    /// Parse an HTML string into a DOM tree
    pub fn parse(source: &str) -> Node {
        let sink = DomTreeSink::new();
        let parser = parse_document(sink, Default::default());
        let dom_sink = parser.one(source);
        
        dom_sink.finish()
    }
}

/// Custom tree sink that builds our DOM structure
struct DomTreeSink {
    nodes: Vec<DomNode>,
    root: usize,
}

#[derive(Clone)]
struct DomNode {
    node_type: DomNodeType,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Clone)]
enum DomNodeType {
    Document,
    Element { local_name: LocalName, namespace: Namespace, attrs: AttrMap },
    Text(String),
    Comment(String),
}

impl DomTreeSink {
    fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push(DomNode {
            node_type: DomNodeType::Document,
            parent: None,
            children: Vec::new(),
        });
        
        Self { nodes, root: 0 }
    }

    fn add_node(&mut self, node_type: DomNodeType, parent: usize) -> usize {
        let id = self.nodes.len();
        self.nodes.push(DomNode {
            node_type,
            parent: Some(parent),
            children: Vec::new(),
        });
        self.nodes[parent].children.push(id);
        id
    }

    fn to_dom_tree(&self, node_id: usize) -> Node {
        let node = &self.nodes[node_id];
        let children: Vec<Node> = node.children
            .iter()
            .map(|&child_id| self.to_dom_tree(child_id))
            .collect();

        match &node.node_type {
            DomNodeType::Document => {
                // Return the html element or first child as root
                if !children.is_empty() {
                    children[0].clone()
                } else {
                    Node::element("html".to_string(), HashMap::new(), vec![])
                }
            }
            DomNodeType::Element { local_name, namespace: _, attrs } => {
                Node::element(local_name.to_string(), attrs.clone(), children)
            }
            DomNodeType::Text(text) => Node::text(text.clone()),
            DomNodeType::Comment(text) => Node::comment(text.clone()),
        }
    }

    fn finish(self) -> Node {
        self.to_dom_tree(self.root)
    }
}

impl TreeSink for DomTreeSink {
    type Handle = usize;
    type Output = Self;

    fn finish(self) -> Self {
        self
    }

    fn parse_error(&mut self, _msg: Cow<'static, str>) {
        // Ignore parse errors for now
    }

    fn get_document(&mut self) -> usize {
        self.root
    }

    fn elem_name<'a>(&'a self, target: &'a usize) -> ExpandedName<'a> {
        match &self.nodes[*target].node_type {
            DomNodeType::Element { local_name, namespace, .. } => {
                ExpandedName {
                    ns: namespace,
                    local: local_name,
                }
            }
            _ => panic!("not an element"),
        }
    }

    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> usize {
        let attr_map: AttrMap = attrs
            .into_iter()
            .map(|attr| (attr.name.local.to_string(), attr.value.to_string()))
            .collect();

        let parent = self.root;
        self.add_node(
            DomNodeType::Element {
                local_name: name.local.clone(),
                namespace: name.ns.clone(),
                attrs: attr_map,
            },
            parent,
        )
    }

    fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> usize {
        let parent = self.root;
        self.add_node(DomNodeType::Comment(text.to_string()), parent)
    }

    fn create_pi(&mut self, _target: html5ever::tendril::StrTendril, _data: html5ever::tendril::StrTendril) -> usize {
        self.root
    }

    fn append(&mut self, parent: &usize, child: NodeOrText<usize>) {
        let child_id = match child {
            NodeOrText::AppendNode(node_id) => {
                // Update parent relationship
                if let Some(old_parent) = self.nodes[node_id].parent {
                    self.nodes[old_parent].children.retain(|&id| id != node_id);
                }
                self.nodes[node_id].parent = Some(*parent);
                node_id
            }
            NodeOrText::AppendText(text) => {
                self.add_node(DomNodeType::Text(text.to_string()), *parent)
            }
        };

        if !self.nodes[*parent].children.contains(&child_id) {
            self.nodes[*parent].children.push(child_id);
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &usize,
        _prev_element: &usize,
        child: NodeOrText<usize>,
    ) {
        self.append(element, child);
    }

    fn append_doctype_to_document(
        &mut self,
        _name: html5ever::tendril::StrTendril,
        _public_id: html5ever::tendril::StrTendril,
        _system_id: html5ever::tendril::StrTendril,
    ) {
        // Ignore doctype for now
    }

    fn get_template_contents(&mut self, target: &usize) -> usize {
        *target
    }

    fn same_node(&self, x: &usize, y: &usize) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, _mode: QuirksMode) {
        // Ignore quirks mode for now
    }

    fn append_before_sibling(&mut self, sibling: &usize, new_node: NodeOrText<usize>) {
        if let Some(parent) = self.nodes[*sibling].parent {
            self.append(&parent, new_node);
        }
    }

    fn add_attrs_if_missing(&mut self, target: &usize, attrs: Vec<Attribute>) {
        if let DomNodeType::Element { attrs: ref mut existing_attrs, .. } = 
            &mut self.nodes[*target].node_type 
        {
            for attr in attrs {
                let key = attr.name.local.to_string();
                existing_attrs.entry(key).or_insert_with(|| attr.value.to_string());
            }
        }
    }

    fn remove_from_parent(&mut self, target: &usize) {
        if let Some(parent) = self.nodes[*target].parent {
            self.nodes[parent].children.retain(|&id| id != *target);
            self.nodes[*target].parent = None;
        }
    }

    fn reparent_children(&mut self, node: &usize, new_parent: &usize) {
        let children = self.nodes[*node].children.clone();
        for child in children {
            self.remove_from_parent(&child);
            self.append(new_parent, NodeOrText::AppendNode(child));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let html = "<html><body><h1>Hello</h1></body></html>";
        let dom = HtmlParser::parse(html);
        
        assert!(dom.element_data().is_some());
        assert_eq!(dom.element_data().unwrap().tag_name, "html");
    }

    #[test]
    fn test_parse_text_content() {
        let html = "<p>Hello, World!</p>";
        let dom = HtmlParser::parse(html);
        
        // Should auto-wrap in html/body
        assert!(dom.children.len() > 0);
    }

    #[test]
    fn test_parse_with_attributes() {
        let html = r#"<div id="main" class="container">Content</div>"#;
        let dom = HtmlParser::parse(html);
        
        assert!(dom.element_data().is_some());
    }
}
