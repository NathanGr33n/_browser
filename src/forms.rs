// HTML Forms and Input Handling

use crate::dom::{Node, NodeType};
use std::collections::HashMap;

/// Form input types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Checkbox,
    Radio,
    Submit,
    Button,
    Hidden,
}

impl InputType {
    /// Parse input type from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "password" => InputType::Password,
            "email" => InputType::Email,
            "number" => InputType::Number,
            "checkbox" => InputType::Checkbox,
            "radio" => InputType::Radio,
            "submit" => InputType::Submit,
            "button" => InputType::Button,
            "hidden" => InputType::Hidden,
            _ => InputType::Text,
        }
    }
}

/// Input element state
#[derive(Debug, Clone)]
pub struct InputState {
    pub input_type: InputType,
    pub value: String,
    pub checked: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub placeholder: Option<String>,
    pub max_length: Option<usize>,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            input_type: InputType::Text,
            value: String::new(),
            checked: false,
            disabled: false,
            readonly: false,
            placeholder: None,
            max_length: None,
        }
    }
}

impl InputState {
    /// Create from DOM attributes
    pub fn from_attributes(attrs: &HashMap<String, String>) -> Self {
        let input_type = attrs
            .get("type")
            .map(|t| InputType::from_str(t))
            .unwrap_or(InputType::Text);

        let value = attrs.get("value").cloned().unwrap_or_default();
        let checked = attrs.contains_key("checked");
        let disabled = attrs.contains_key("disabled");
        let readonly = attrs.contains_key("readonly");
        let placeholder = attrs.get("placeholder").cloned();
        let max_length = attrs
            .get("maxlength")
            .and_then(|s| s.parse().ok());

        Self {
            input_type,
            value,
            checked,
            disabled,
            readonly,
            placeholder,
            max_length,
        }
    }

    /// Update value (respecting maxlength and readonly)
    pub fn set_value(&mut self, new_value: String) -> bool {
        if self.readonly || self.disabled {
            return false;
        }

        let value = if let Some(max_len) = self.max_length {
            new_value.chars().take(max_len).collect()
        } else {
            new_value
        };

        self.value = value;
        true
    }

    /// Toggle checked state (for checkbox/radio)
    pub fn toggle_checked(&mut self) -> bool {
        if self.disabled {
            return false;
        }

        match self.input_type {
            InputType::Checkbox => {
                self.checked = !self.checked;
                true
            }
            InputType::Radio => {
                self.checked = true;
                true
            }
            _ => false,
        }
    }
}

/// Textarea element state
#[derive(Debug, Clone)]
pub struct TextAreaState {
    pub value: String,
    pub disabled: bool,
    pub readonly: bool,
    pub placeholder: Option<String>,
    pub rows: usize,
    pub cols: usize,
    pub max_length: Option<usize>,
}

impl Default for TextAreaState {
    fn default() -> Self {
        Self {
            value: String::new(),
            disabled: false,
            readonly: false,
            placeholder: None,
            rows: 2,
            cols: 20,
            max_length: None,
        }
    }
}

impl TextAreaState {
    /// Create from DOM attributes
    pub fn from_attributes(attrs: &HashMap<String, String>) -> Self {
        let disabled = attrs.contains_key("disabled");
        let readonly = attrs.contains_key("readonly");
        let placeholder = attrs.get("placeholder").cloned();
        let rows = attrs
            .get("rows")
            .and_then(|s| s.parse().ok())
            .unwrap_or(2)
            .clamp(1, 1000); // Limit to reasonable range
        let cols = attrs
            .get("cols")
            .and_then(|s| s.parse().ok())
            .unwrap_or(20)
            .clamp(1, 1000); // Limit to reasonable range
        let max_length = attrs.get("maxlength").and_then(|s| s.parse().ok());

        Self {
            value: String::new(),
            disabled,
            readonly,
            placeholder,
            rows,
            cols,
            max_length,
        }
    }

    /// Update value
    pub fn set_value(&mut self, new_value: String) -> bool {
        if self.readonly || self.disabled {
            return false;
        }

        let value = if let Some(max_len) = self.max_length {
            new_value.chars().take(max_len).collect()
        } else {
            new_value
        };

        self.value = value;
        true
    }
}

/// Form element state
#[derive(Debug, Clone)]
pub struct FormState {
    pub action: Option<String>,
    pub method: String,
    pub inputs: HashMap<String, InputState>,
    pub textareas: HashMap<String, TextAreaState>,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            action: None,
            method: "GET".to_string(),
            inputs: HashMap::new(),
            textareas: HashMap::new(),
        }
    }
}

impl FormState {
    /// Create from form attributes
    pub fn from_attributes(attrs: &HashMap<String, String>) -> Self {
        let action = attrs.get("action").cloned();
        let method = attrs
            .get("method")
            .map(|m| m.to_uppercase())
            .unwrap_or_else(|| "GET".to_string());

        Self {
            action,
            method,
            inputs: HashMap::new(),
            textareas: HashMap::new(),
        }
    }

    /// Collect form data for submission
    pub fn collect_data(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();

        // Collect input values
        for (name, input) in &self.inputs {
            match input.input_type {
                InputType::Checkbox | InputType::Radio => {
                    if input.checked {
                        data.insert(name.clone(), input.value.clone());
                    }
                }
                InputType::Submit | InputType::Button => {
                    // Don't include buttons in form data
                }
                _ => {
                    data.insert(name.clone(), input.value.clone());
                }
            }
        }

        // Collect textarea values
        for (name, textarea) in &self.textareas {
            data.insert(name.clone(), textarea.value.clone());
        }

        data
    }

    /// Reset form to initial state
    pub fn reset(&mut self) {
        for input in self.inputs.values_mut() {
            input.value.clear();
            if matches!(input.input_type, InputType::Checkbox | InputType::Radio) {
                input.checked = false;
            }
        }

        for textarea in self.textareas.values_mut() {
            textarea.value.clear();
        }
    }
}

/// Focus manager for form elements
#[derive(Debug, Clone)]
pub struct FocusManager {
    focused_element: Option<String>, // Element ID
    focusable_elements: Vec<String>, // Ordered list of focusable element IDs
}

impl FocusManager {
    /// Create new focus manager
    pub fn new() -> Self {
        Self {
            focused_element: None,
            focusable_elements: Vec::new(),
        }
    }

    /// Register a focusable element
    pub fn register_focusable(&mut self, element_id: String) {
        if !self.focusable_elements.contains(&element_id) {
            self.focusable_elements.push(element_id);
        }
    }

    /// Unregister a focusable element
    pub fn unregister_focusable(&mut self, element_id: &str) {
        self.focusable_elements.retain(|id| id != element_id);
        if self.focused_element.as_deref() == Some(element_id) {
            self.focused_element = None;
        }
    }

    /// Set focus to an element
    pub fn focus(&mut self, element_id: String) -> bool {
        if self.focusable_elements.contains(&element_id) {
            self.focused_element = Some(element_id);
            true
        } else {
            false
        }
    }

    /// Clear focus
    pub fn blur(&mut self) {
        self.focused_element = None;
    }

    /// Get currently focused element
    pub fn focused_element(&self) -> Option<&String> {
        self.focused_element.as_ref()
    }

    /// Move focus to next element (Tab key)
    pub fn focus_next(&mut self) -> Option<&String> {
        if self.focusable_elements.is_empty() {
            return None;
        }

        let next_index = match &self.focused_element {
            Some(current) => {
                let current_index = self
                    .focusable_elements
                    .iter()
                    .position(|id| id == current)
                    .unwrap_or(0);
                (current_index + 1) % self.focusable_elements.len()
            }
            None => 0,
        };

        self.focused_element = Some(self.focusable_elements[next_index].clone());
        self.focused_element.as_ref()
    }

    /// Move focus to previous element (Shift+Tab)
    pub fn focus_previous(&mut self) -> Option<&String> {
        if self.focusable_elements.is_empty() {
            return None;
        }

        let prev_index = match &self.focused_element {
            Some(current) => {
                let current_index = self
                    .focusable_elements
                    .iter()
                    .position(|id| id == current)
                    .unwrap_or(0);
                if current_index == 0 {
                    self.focusable_elements.len() - 1
                } else {
                    current_index - 1
                }
            }
            None => self.focusable_elements.len() - 1,
        };

        self.focused_element = Some(self.focusable_elements[prev_index].clone());
        self.focused_element.as_ref()
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_type_parsing() {
        assert_eq!(InputType::from_str("text"), InputType::Text);
        assert_eq!(InputType::from_str("password"), InputType::Password);
        assert_eq!(InputType::from_str("checkbox"), InputType::Checkbox);
        assert_eq!(InputType::from_str("unknown"), InputType::Text);
    }

    #[test]
    fn test_input_state_set_value() {
        let mut input = InputState::default();
        
        assert!(input.set_value("test".to_string()));
        assert_eq!(input.value, "test");

        // Test max_length
        input.max_length = Some(5);
        assert!(input.set_value("toolong".to_string()));
        assert_eq!(input.value, "toolo");

        // Test readonly
        input.readonly = true;
        assert!(!input.set_value("newvalue".to_string()));
        assert_eq!(input.value, "toolo"); // Unchanged
    }

    #[test]
    fn test_input_toggle_checked() {
        let mut checkbox = InputState {
            input_type: InputType::Checkbox,
            ..Default::default()
        };

        assert!(!checkbox.checked);
        assert!(checkbox.toggle_checked());
        assert!(checkbox.checked);
        assert!(checkbox.toggle_checked());
        assert!(!checkbox.checked);

        // Test disabled
        checkbox.disabled = true;
        assert!(!checkbox.toggle_checked());
    }

    #[test]
    fn test_textarea_state() {
        let mut textarea = TextAreaState::default();
        
        assert!(textarea.set_value("Hello\nWorld".to_string()));
        assert_eq!(textarea.value, "Hello\nWorld");

        textarea.readonly = true;
        assert!(!textarea.set_value("New".to_string()));
    }

    #[test]
    fn test_form_collect_data() {
        let mut form = FormState::default();
        
        let mut input1 = InputState::default();
        input1.value = "John".to_string();
        form.inputs.insert("name".to_string(), input1);

        let mut input2 = InputState {
            input_type: InputType::Checkbox,
            checked: true,
            value: "on".to_string(),
            ..Default::default()
        };
        form.inputs.insert("subscribe".to_string(), input2);

        let data = form.collect_data();
        assert_eq!(data.get("name"), Some(&"John".to_string()));
        assert_eq!(data.get("subscribe"), Some(&"on".to_string()));
    }

    #[test]
    fn test_form_reset() {
        let mut form = FormState::default();
        
        let mut input = InputState::default();
        input.value = "test".to_string();
        form.inputs.insert("field".to_string(), input);

        form.reset();
        assert_eq!(form.inputs.get("field").unwrap().value, "");
    }

    #[test]
    fn test_focus_manager() {
        let mut focus = FocusManager::new();
        
        focus.register_focusable("input1".to_string());
        focus.register_focusable("input2".to_string());
        focus.register_focusable("input3".to_string());

        // Focus first element
        assert!(focus.focus("input1".to_string()));
        assert_eq!(focus.focused_element(), Some(&"input1".to_string()));

        // Tab to next
        focus.focus_next();
        assert_eq!(focus.focused_element(), Some(&"input2".to_string()));

        focus.focus_next();
        assert_eq!(focus.focused_element(), Some(&"input3".to_string()));

        // Wrap around
        focus.focus_next();
        assert_eq!(focus.focused_element(), Some(&"input1".to_string()));

        // Shift+Tab (previous)
        focus.focus_previous();
        assert_eq!(focus.focused_element(), Some(&"input3".to_string()));
    }

    #[test]
    fn test_focus_unregister() {
        let mut focus = FocusManager::new();
        
        focus.register_focusable("input1".to_string());
        focus.focus("input1".to_string());
        
        focus.unregister_focusable("input1");
        assert_eq!(focus.focused_element(), None);
    }
}
