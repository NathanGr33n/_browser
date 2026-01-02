// JavaScript runtime using Boa engine

use boa_engine::{Context, Source, JsValue as BoaJsValue, property::PropertyKey};
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
    
    /// Convert from Boa JsValue
    fn from_boa(value: &BoaJsValue, context: &mut Context) -> Self {
        if value.is_undefined() {
            JsValue::Undefined
        } else if value.is_null() {
            JsValue::Null
        } else if value.is_boolean() {
            JsValue::Boolean(value.as_boolean().unwrap())
        } else if value.is_number() {
            JsValue::Number(value.as_number().unwrap())
        } else if value.is_string() {
            JsValue::String(value.to_string(context).unwrap().to_std_string_escaped())
        } else if value.is_object() {
            // Simplified: convert to string representation
            JsValue::String(value.to_string(context).unwrap().to_std_string_escaped())
        } else {
            JsValue::Undefined
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

/// JavaScript runtime using Boa engine
pub struct JsRuntime {
    /// Boa context
    context: Context<'static>,
    /// Console log buffer
    console_logs: Vec<String>,
}

impl JsRuntime {
    /// Create a new JavaScript runtime
    pub fn new() -> Self {
        let context = Context::default();
        
        Self {
            context,
            console_logs: Vec::new(),
        }
    }
    
    /// Execute JavaScript code
    pub fn execute(&mut self, code: &str) -> Result<JsValue, JsError> {
        let source = Source::from_bytes(code);
        
        match self.context.eval(source) {
            Ok(value) => Ok(JsValue::from_boa(&value, &mut self.context)),
            Err(e) => {
                let error_string = e.to_string();
                
                // Classify error type based on message
                if error_string.contains("SyntaxError") {
                    Err(JsError::SyntaxError(error_string))
                } else if error_string.contains("ReferenceError") {
                    Err(JsError::ReferenceError(error_string))
                } else if error_string.contains("TypeError") {
                    Err(JsError::TypeError(error_string))
                } else {
                    Err(JsError::RuntimeError(error_string))
                }
            }
        }
    }
    
    /// Set a global variable
    pub fn set_global(&mut self, name: String, value: JsValue) {
        let boa_value = match value {
            JsValue::Undefined => BoaJsValue::undefined(),
            JsValue::Null => BoaJsValue::null(),
            JsValue::Boolean(b) => BoaJsValue::from(b),
            JsValue::Number(n) => BoaJsValue::from(n),
            JsValue::String(s) => BoaJsValue::from(s),
            _ => BoaJsValue::undefined(), // Simplified for now
        };
        
        // Set property on global object
        let global_obj = self.context.global_object().clone();
        let key = PropertyKey::from(name.as_str());
        global_obj.set(key, boa_value, false, &mut self.context).unwrap();
    }
    
    /// Get a global variable
    pub fn get_global(&mut self, name: &str) -> Option<JsValue> {
        let global_obj = self.context.global_object();
        let key = PropertyKey::from(name);
        let property = global_obj.get(key, &mut self.context).ok()?;
        Some(JsValue::from_boa(&property, &mut self.context))
    }
    
    /// Get console logs (stub for now - would need console capture)
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
        let _runtime = JsRuntime::new();
        // Context creation successful if we reach here
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
        let x = runtime.get_global("x").unwrap();
        assert_eq!(x.to_number(), 42.0);
    }
    
    #[test]
    fn test_arithmetic() {
        let mut runtime = JsRuntime::new();
        let result = runtime.execute("2 + 3").unwrap();
        assert_eq!(result.to_number(), 5.0);
    }
    
    #[test]
    fn test_string_operations() {
        let mut runtime = JsRuntime::new();
        let result = runtime.execute("\"hello\" + \" \" + \"world\"").unwrap();
        assert_eq!(result.to_string(), "hello world");
    }
    
    #[test]
    fn test_syntax_error() {
        let mut runtime = JsRuntime::new();
        let result = runtime.execute("var x = ");
        assert!(result.is_err());
        match result {
            Err(JsError::SyntaxError(_)) => (),
            _ => panic!("Expected SyntaxError"),
        }
    }
}
