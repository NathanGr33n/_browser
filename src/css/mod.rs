use cssparser::{Parser, ParserInput, Token};
use std::fmt;

/// A CSS stylesheet containing multiple rules
#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

/// A CSS rule with selectors and declarations
#[derive(Debug, Clone)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

/// A CSS selector (simplified)
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
}

/// A simple selector (tag, class, or id)
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

/// A CSS declaration (property: value)
#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

/// CSS property values
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
    Number(f32),
    Percentage(f32),
}

/// CSS length units
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Percent,
}

/// RGBA color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0, 255)
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255, 255)
    }
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Stylesheet { rules }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Keyword(s) => write!(f, "{}", s),
            Value::Length(n, Unit::Px) => write!(f, "{}px", n),
            Value::Length(n, Unit::Em) => write!(f, "{}em", n),
            Value::Length(n, Unit::Rem) => write!(f, "{}rem", n),
            Value::Length(n, Unit::Percent) => write!(f, "{}%", n),
            Value::Color(c) => write!(f, "rgba({}, {}, {}, {})", c.r, c.g, c.b, c.a),
            Value::Number(n) => write!(f, "{}", n),
            Value::Percentage(n) => write!(f, "{}%", n),
        }
    }
}

/// Calculate specificity for selector matching priority
pub fn specificity(selector: &Selector) -> Specificity {
    let Selector::Simple(ref simple) = selector;
    let id = if simple.id.is_some() { 1 } else { 0 };
    let class = simple.classes.len();
    let tag = if simple.tag_name.is_some() { 1 } else { 0 };
    
    Specificity(id, class, tag)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity(usize, usize, usize);

/// CSS Parser
pub struct CssParser;

impl CssParser {
    /// Parse a CSS stylesheet
    pub fn parse(source: &str) -> Stylesheet {
        let mut input = ParserInput::new(source);
        let mut parser = Parser::new(&mut input);
        let mut rules = Vec::new();

        while parser.is_exhausted() == false {
            // Skip whitespace and comments
            let _ = parser.skip_whitespace();
            
            if parser.is_exhausted() {
                break;
            }

            if let Ok(rule) = Self::parse_rule(&mut parser) {
                rules.push(rule);
            } else {
                // Skip to next rule on error
                let _ = Self::skip_to_next_rule(&mut parser);
            }
        }

        Stylesheet::new(rules)
    }

    fn parse_rule(parser: &mut Parser) -> Result<Rule, ()> {
        let selectors = Self::parse_selectors(parser)?;
        
        parser.skip_whitespace();
        parser.expect_curly_bracket_block().map_err(|_| ())?;
        
        let declarations = parser.parse_nested_block(|parser| {
            Ok::<Vec<Declaration>, cssparser::ParseError<()>>(Self::parse_declarations(parser))
        }).unwrap_or_default();

        Ok(Rule { selectors, declarations })
    }

    fn parse_selectors(parser: &mut Parser) -> Result<Vec<Selector>, ()> {
        let mut selectors = Vec::new();
        
        loop {
            parser.skip_whitespace();
            if let Ok(selector) = Self::parse_simple_selector(parser) {
                selectors.push(Selector::Simple(selector));
            }

            parser.skip_whitespace();
            
            // Check for comma (multiple selectors)
            if parser.try_parse(|p| p.expect_comma()).is_err() {
                break;
            }
        }

        if selectors.is_empty() {
            Err(())
        } else {
            Ok(selectors)
        }
    }

    fn parse_simple_selector(parser: &mut Parser) -> Result<SimpleSelector, ()> {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        };

        parser.skip_whitespace();

        while let Ok(token) = parser.next_including_whitespace() {
            match token {
                Token::Ident(name) => {
                    selector.tag_name = Some(name.to_string());
                }
                Token::IDHash(id) => {
                    selector.id = Some(id.to_string());
                }
                Token::Delim('.') => {
                    if let Ok(Token::Ident(class)) = parser.next() {
                        selector.classes.push(class.to_string());
                    }
                }
                Token::Delim('*') => {
                    // Universal selector
                }
                Token::WhiteSpace(_) | Token::CurlyBracketBlock => {
                    break;
                }
                _ => break,
            }
        }

        Ok(selector)
    }

    fn parse_declarations(parser: &mut Parser) -> Vec<Declaration> {
        let mut declarations = Vec::new();

        while !parser.is_exhausted() {
            parser.skip_whitespace();
            
            if let Ok(declaration) = Self::parse_declaration(parser) {
                declarations.push(declaration);
            }

            // Skip semicolon
            let _ = parser.try_parse(|p| p.expect_semicolon());
        }

        declarations
    }

    fn parse_declaration(parser: &mut Parser) -> Result<Declaration, ()> {
        parser.skip_whitespace();
        
        let name = parser.expect_ident().map_err(|_| ())?.to_string();
        
        parser.skip_whitespace();
        parser.expect_colon().map_err(|_| ())?;
        parser.skip_whitespace();

        let value = Self::parse_value(parser)?;

        Ok(Declaration { name, value })
    }

    fn parse_value(parser: &mut Parser) -> Result<Value, ()> {
        parser.skip_whitespace();
        
        let token = parser.next().map_err(|_| ())?;
        
        match token {
            Token::Ident(keyword) => {
                Ok(Value::Keyword(keyword.to_string()))
            }
            Token::Number { value, .. } => {
                Ok(Value::Number(*value))
            }
            Token::Percentage { unit_value, .. } => {
                Ok(Value::Percentage(*unit_value * 100.0))
            }
            Token::Dimension { value, unit, .. } => {
                let unit = match unit.as_ref() {
                    "px" => Unit::Px,
                    "em" => Unit::Em,
                    "rem" => Unit::Rem,
                    "%" => Unit::Percent,
                    _ => return Err(()),
                };
                Ok(Value::Length(*value, unit))
            }
            Token::Hash(hex) | Token::IDHash(hex) => {
                Self::parse_hex_color(hex.as_ref())
            }
            _ => Err(()),
        }
    }

    fn parse_hex_color(hex: &str) -> Result<Value, ()> {
        let hex = hex.trim_start_matches('#');
        
        let (r, g, b) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| ())?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| ())?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| ())?;
                (r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ())?;
                (r, g, b)
            }
            _ => return Err(()),
        };

        Ok(Value::Color(Color::new(r, g, b, 255)))
    }

    fn skip_to_next_rule(parser: &mut Parser) -> Result<(), ()> {
        // Skip until we find a closing brace
        while !parser.is_exhausted() {
            if let Ok(Token::CurlyBracketBlock) = parser.next() {
                let _ = parser.parse_nested_block(|_| Ok::<(), cssparser::ParseError<()>>(()));
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_rule() {
        let css = "h1 { color: red; font-size: 16px; }";
        let stylesheet = CssParser::parse(css);
        
        assert_eq!(stylesheet.rules.len(), 1);
        assert_eq!(stylesheet.rules[0].declarations.len(), 2);
    }

    #[test]
    fn test_parse_selector() {
        let css = ".container { margin: 0; }";
        let stylesheet = CssParser::parse(css);
        
        assert_eq!(stylesheet.rules.len(), 1);
    }

    #[test]
    fn test_specificity() {
        let selector = Selector::Simple(SimpleSelector {
            tag_name: Some("div".to_string()),
            id: Some("main".to_string()),
            classes: vec!["container".to_string()],
        });
        
        let spec = specificity(&selector);
        assert_eq!(spec, Specificity(1, 1, 1));
    }
}
