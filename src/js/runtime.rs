// Minimal JavaScript runtime
// Note: This is a stub for a full JS engine integration (V8, SpiderMonkey, etc.)

use std::collections::HashMap;

/// JavaScript value types
#[derive(Debug, Clone, PartialEq)]
pub enum JsValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(HashMap<String, JsValue>),
    Array(Vec<JsValue>),
    Function(String), // Function code as string (simplified)
}

impl JsValue {
    /// Convert to boolean (JavaScript truthy/falsy)
    pub fn to_bool(&self) -> bool {
        match self {
            JsValue::Undefined | JsValue::Null => false,
            JsValue::Boolean(b) => *b,
            JsValue::Number(n) => *n != 0.0 && !n.is_nan(),
            JsValue::String(s) => !s.is_empty(),
            JsValue::Object(_) | JsValue::Array(_) | JsValue::Function(_) => true,
        }
    }
    
    /// Convert to number
    pub fn to_number(&self) -> f64 {
        match self {
            JsValue::Number(n) => *n,
            JsValue::Boolean(b) => if *b { 1.0 } else { 0.0 },
            JsValue::String(s) => s.parse::<f64>().unwrap_or(f64::NAN),
            _ => f64::NAN,
        }
    }
    
    /// Convert to string
    pub fn to_string(&self) -> String {
        match self {
            JsValue::Undefined => "undefined".to_string(),
            JsValue::Null => "null".to_string(),
            JsValue::Boolean(b) => b.to_string(),
            JsValue::Number(n) => n.to_string(),
            JsValue::String(s) => s.clone(),
            JsValue::Object(_) => "[object Object]".to_string(),
            JsValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                items.join(",")
            }
            JsValue::Function(_) => "[Function]".to_string(),
        }
    }
}

/// JavaScript runtime errors
#[derive(Debug, Clone, PartialEq)]
pub enum JsError {
    SyntaxError(String),
    ReferenceError(String),
    TypeError(String),
    RuntimeError(String),
    ExecutionDisabled,
}

impl std::fmt::Display for JsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsError::SyntaxError(msg) => write!(f, "SyntaxError: {}", msg),
            JsError::ReferenceError(msg) => write!(f, "ReferenceError: {}", msg),
            JsError::TypeError(msg) => write!(f, "TypeError: {}", msg),
            JsError::RuntimeError(msg) => write!(f, "RuntimeError: {}", msg),
            JsError::ExecutionDisabled => write!(f, "JavaScript execution is disabled"),
        }
    }
}

impl std::error::Error for JsError {}

/// Minimal JavaScript runtime
/// NOTE: This is a simplified stub. A real implementation would integrate
/// V8, SpiderMonkey, or another full-featured JS engine.
pub struct JsRuntime {
    /// Global variables
    globals: HashMap<String, JsValue>,
    /// Console log buffer
    console_logs: Vec<String>,
}

impl JsRuntime {
    /// Create a new JavaScript runtime
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        
        // Add basic global objects
        globals.insert("undefined".to_string(), JsValue::Undefined);
        globals.insert("null".to_string(), JsValue::Null);
        globals.insert("true".to_string(), JsValue::Boolean(true));
        globals.insert("false".to_string(), JsValue::Boolean(false));
        
        Self {
            globals,
            console_logs: Vec::new(),
        }
    }
    
    /// Execute JavaScript code
    /// NOTE: This is a very basic stub that only handles simple expressions
    /// A real implementation would use a proper JS engine
    pub fn execute(&mut self, code: &str) -> Result<JsValue, JsError> {
        let code = code.trim();
        
        // Handle console.log (simplified)
        if code.starts_with("console.log(") && code.ends_with(')') {
            let content = &code[12..code.len() - 1];
            self.console_logs.push(content.to_string());
            return Ok(JsValue::Undefined);
        }
        
        // Handle simple variable assignments
        if code.contains('=') && !code.contains("==") {
            let parts: Vec<&str> = code.splitn(2, '=').collect();
            if parts.len() == 2 {
                let var_name = parts[0].trim().replace("var ", "").replace("let ", "").replace("const ", "");
                let value = self.parse_value(parts[1].trim())?;
                self.globals.insert(var_name, value.clone());
                return Ok(value);
            }
        }
        
        // Handle variable lookup
        if let Some(value) = self.globals.get(code) {
            return Ok(value.clone());
        }
        
        // Try to parse as literal
        self.parse_value(code)
    }
    
    /// Parse a JavaScript value (simplified)
    fn parse_value(&self, input: &str) -> Result<JsValue, JsError> {
        let input = input.trim();
        
        // Boolean literals
        if input == "true" {
            return Ok(JsValue::Boolean(true));
        }
        if input == "false" {
            return Ok(JsValue::Boolean(false));
        }
        
        // Null and undefined
        if input == "null" {
            return Ok(JsValue::Null);
        }
        if input == "undefined" {
            return Ok(JsValue::Undefined);
        }
        
        // String literals
        if (input.starts_with('"') && input.ends_with('"'))
            || (input.starts_with('\'') && input.ends_with('\''))
        {
            return Ok(JsValue::String(input[1..input.len() - 1].to_string()));
        }
        
        // Number literals
        if let Ok(num) = input.parse::<f64>() {
            return Ok(JsValue::Number(num));
        }
        
        // Variable reference
        if let Some(value) = self.globals.get(input) {
            return Ok(value.clone());
        }
        
        Err(JsError::SyntaxError(format!("Unexpected token: {}", input)))
    }
    
    /// Set a global variable
    pub fn set_global(&mut self, name: String, value: JsValue) {
        self.globals.insert(name, value);
    }
    
    /// Get a global variable
    pub fn get_global(&self, name: &str) -> Option<&JsValue> {
        self.globals.get(name)
    }
    
    /// Get console logs
    pub fn console_logs(&self) -> &[String] {
        &self.console_logs
    }
    
    /// Clear console logs
    pub fn clear_console(&mut self) {
        self.console_logs.clear();
    }
}

impl Default for JsRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_js_value_to_bool() {
        assert!(!JsValue::Undefined.to_bool());
        assert!(!JsValue::Null.to_bool());
        assert!(JsValue::Boolean(true).to_bool());
        assert!(!JsValue::Boolean(false).to_bool());
        assert!(JsValue::Number(1.0).to_bool());
        assert!(!JsValue::Number(0.0).to_bool());
        assert!(JsValue::String("hello".to_string()).to_bool());
        assert!(!JsValue::String("".to_string()).to_bool());
    }
    
    #[test]
    fn test_js_value_to_string() {
        assert_eq!(JsValue::Undefined.to_string(), "undefined");
        assert_eq!(JsValue::Null.to_string(), "null");
        assert_eq!(JsValue::Boolean(true).to_string(), "true");
        assert_eq!(JsValue::Number(42.0).to_string(), "42");
        assert_eq!(JsValue::String("hello".to_string()).to_string(), "hello");
    }
    
    #[test]
    fn test_runtime_creation() {
        let runtime = JsRuntime::new();
        assert!(runtime.get_global("undefined").is_some());
        assert!(runtime.get_global("null").is_some());
    }
    
    #[test]
    fn test_simple_execution() {
        let mut runtime = JsRuntime::new();
        
        // Test literals
        assert_eq!(runtime.execute("42").unwrap(), JsValue::Number(42.0));
        assert_eq!(runtime.execute("true").unwrap(), JsValue::Boolean(true));
        assert_eq!(
            runtime.execute("\"hello\"").unwrap(),
            JsValue::String("hello".to_string())
        );
    }
    
    #[test]
    fn test_variable_assignment() {
        let mut runtime = JsRuntime::new();
        runtime.execute("var x = 42").unwrap();
        assert_eq!(runtime.get_global("x"), Some(&JsValue::Number(42.0)));
    }
    
    #[test]
    fn test_console_log() {
        let mut runtime = JsRuntime::new();
        runtime.execute("console.log(\"test\")").unwrap();
        assert_eq!(runtime.console_logs().len(), 1);
    }
}
