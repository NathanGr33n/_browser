// WebSocket Protocol (RFC 6455) - Phase 7 Task 5

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// WebSocket connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

/// WebSocket close code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseCode {
    Normal = 1000,
    GoingAway = 1001,
    ProtocolError = 1002,
    UnsupportedData = 1003,
    NoStatusReceived = 1005,
    AbnormalClosure = 1006,
    InvalidFramePayload = 1007,
    PolicyViolation = 1008,
    MessageTooBig = 1009,
    MandatoryExtension = 1010,
    InternalError = 1011,
    ServiceRestart = 1012,
    TryAgainLater = 1013,
    BadGateway = 1014,
    TlsHandshake = 1015,
}

impl CloseCode {
    /// Convert to u16
    pub fn as_u16(self) -> u16 {
        self as u16
    }
    
    /// Create from u16
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            1000 => Some(CloseCode::Normal),
            1001 => Some(CloseCode::GoingAway),
            1002 => Some(CloseCode::ProtocolError),
            1003 => Some(CloseCode::UnsupportedData),
            1005 => Some(CloseCode::NoStatusReceived),
            1006 => Some(CloseCode::AbnormalClosure),
            1007 => Some(CloseCode::InvalidFramePayload),
            1008 => Some(CloseCode::PolicyViolation),
            1009 => Some(CloseCode::MessageTooBig),
            1010 => Some(CloseCode::MandatoryExtension),
            1011 => Some(CloseCode::InternalError),
            1012 => Some(CloseCode::ServiceRestart),
            1013 => Some(CloseCode::TryAgainLater),
            1014 => Some(CloseCode::BadGateway),
            1015 => Some(CloseCode::TlsHandshake),
            _ => None,
        }
    }
}

/// WebSocket message type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebSocketMessage {
    /// Text message (UTF-8)
    Text(String),
    /// Binary message
    Binary(Vec<u8>),
    /// Ping frame
    Ping(Vec<u8>),
    /// Pong frame
    Pong(Vec<u8>),
    /// Close frame
    Close(Option<CloseCode>, Option<String>),
}

/// WebSocket frame opcode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Continuation = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

impl Opcode {
    /// Create from u8
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x0 => Some(Opcode::Continuation),
            0x1 => Some(Opcode::Text),
            0x2 => Some(Opcode::Binary),
            0x8 => Some(Opcode::Close),
            0x9 => Some(Opcode::Ping),
            0xA => Some(Opcode::Pong),
            _ => None,
        }
    }
}

/// WebSocket frame
#[derive(Debug, Clone)]
struct Frame {
    /// Final fragment flag
    fin: bool,
    /// Opcode
    opcode: Opcode,
    /// Payload data
    payload: Vec<u8>,
}

impl Frame {
    /// Create a new frame
    fn new(opcode: Opcode, payload: Vec<u8>) -> Self {
        Self {
            fin: true,
            opcode,
            payload,
        }
    }
    
    /// Encode frame to bytes
    fn encode(&self, mask: bool) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // First byte: FIN + RSV + opcode
        let mut first_byte = self.opcode as u8;
        if self.fin {
            first_byte |= 0x80;
        }
        bytes.push(first_byte);
        
        // Second byte: MASK + payload length
        let payload_len = self.payload.len();
        let mut second_byte = 0u8;
        if mask {
            second_byte |= 0x80;
        }
        
        if payload_len < 126 {
            second_byte |= payload_len as u8;
            bytes.push(second_byte);
        } else if payload_len < 65536 {
            second_byte |= 126;
            bytes.push(second_byte);
            bytes.extend_from_slice(&(payload_len as u16).to_be_bytes());
        } else {
            second_byte |= 127;
            bytes.push(second_byte);
            bytes.extend_from_slice(&(payload_len as u64).to_be_bytes());
        }
        
        // Masking key (simplified - would use random in production)
        if mask {
            let mask_key = [0x12, 0x34, 0x56, 0x78];
            bytes.extend_from_slice(&mask_key);
            
            // Masked payload
            for (i, byte) in self.payload.iter().enumerate() {
                bytes.push(byte ^ mask_key[i % 4]);
            }
        } else {
            bytes.extend_from_slice(&self.payload);
        }
        
        bytes
    }
}

/// WebSocket connection
pub struct WebSocket {
    /// URL
    url: String,
    /// Connection state
    state: WebSocketState,
    /// Is secure (wss://)
    secure: bool,
    /// Incoming message queue
    incoming_messages: VecDeque<WebSocketMessage>,
    /// Outgoing frame queue
    outgoing_frames: VecDeque<Vec<u8>>,
    /// Last ping time
    last_ping: Option<Instant>,
    /// Ping interval
    ping_interval: Duration,
    /// Close code
    close_code: Option<CloseCode>,
    /// Close reason
    close_reason: Option<String>,
}

impl WebSocket {
    /// Create a new WebSocket connection
    pub fn new(url: String) -> Result<Self, WebSocketError> {
        let secure = url.starts_with("wss://");
        
        if !url.starts_with("ws://") && !secure {
            return Err(WebSocketError::InvalidUrl);
        }
        
        Ok(Self {
            url,
            state: WebSocketState::Connecting,
            secure,
            incoming_messages: VecDeque::new(),
            outgoing_frames: VecDeque::new(),
            last_ping: None,
            ping_interval: Duration::from_secs(30),
            close_code: None,
            close_reason: None,
        })
    }
    
    /// Get URL
    pub fn url(&self) -> &str {
        &self.url
    }
    
    /// Get state
    pub fn state(&self) -> WebSocketState {
        self.state
    }
    
    /// Is secure
    pub fn is_secure(&self) -> bool {
        self.secure
    }
    
    /// Open connection (simplified - would do actual handshake)
    pub fn open(&mut self) -> Result<(), WebSocketError> {
        if self.state != WebSocketState::Connecting {
            return Err(WebSocketError::InvalidState);
        }
        
        // In production, would perform WebSocket handshake here
        self.state = WebSocketState::Open;
        self.last_ping = Some(Instant::now());
        
        Ok(())
    }
    
    /// Send text message
    pub fn send_text(&mut self, text: String) -> Result<(), WebSocketError> {
        if self.state != WebSocketState::Open {
            return Err(WebSocketError::NotConnected);
        }
        
        let frame = Frame::new(Opcode::Text, text.into_bytes());
        self.outgoing_frames.push_back(frame.encode(true));
        
        Ok(())
    }
    
    /// Send binary message
    pub fn send_binary(&mut self, data: Vec<u8>) -> Result<(), WebSocketError> {
        if self.state != WebSocketState::Open {
            return Err(WebSocketError::NotConnected);
        }
        
        let frame = Frame::new(Opcode::Binary, data);
        self.outgoing_frames.push_back(frame.encode(true));
        
        Ok(())
    }
    
    /// Send ping
    pub fn send_ping(&mut self, data: Vec<u8>) -> Result<(), WebSocketError> {
        if self.state != WebSocketState::Open {
            return Err(WebSocketError::NotConnected);
        }
        
        let frame = Frame::new(Opcode::Ping, data);
        self.outgoing_frames.push_back(frame.encode(true));
        self.last_ping = Some(Instant::now());
        
        Ok(())
    }
    
    /// Send pong
    pub fn send_pong(&mut self, data: Vec<u8>) -> Result<(), WebSocketError> {
        if self.state != WebSocketState::Open && self.state != WebSocketState::Closing {
            return Err(WebSocketError::NotConnected);
        }
        
        let frame = Frame::new(Opcode::Pong, data);
        self.outgoing_frames.push_back(frame.encode(true));
        
        Ok(())
    }
    
    /// Close connection
    pub fn close(&mut self, code: CloseCode, reason: Option<String>) -> Result<(), WebSocketError> {
        if self.state == WebSocketState::Closed {
            return Err(WebSocketError::AlreadyClosed);
        }
        
        self.state = WebSocketState::Closing;
        self.close_code = Some(code);
        self.close_reason = reason.clone();
        
        // Encode close frame
        let mut payload = Vec::new();
        payload.extend_from_slice(&code.as_u16().to_be_bytes());
        if let Some(reason_str) = reason {
            payload.extend_from_slice(reason_str.as_bytes());
        }
        
        let frame = Frame::new(Opcode::Close, payload);
        self.outgoing_frames.push_back(frame.encode(true));
        
        Ok(())
    }
    
    /// Receive message
    pub fn receive(&mut self) -> Option<WebSocketMessage> {
        self.incoming_messages.pop_front()
    }
    
    /// Check if ping should be sent
    pub fn should_ping(&self) -> bool {
        if let Some(last_ping) = self.last_ping {
            last_ping.elapsed() >= self.ping_interval
        } else {
            false
        }
    }
    
    /// Get next outgoing frame
    pub fn next_outgoing_frame(&mut self) -> Option<Vec<u8>> {
        self.outgoing_frames.pop_front()
    }
    
    /// Handle incoming frame (simplified)
    pub fn handle_incoming_frame(&mut self, opcode: u8, payload: Vec<u8>) -> Result<(), WebSocketError> {
        let opcode = Opcode::from_u8(opcode).ok_or(WebSocketError::InvalidFrame)?;
        
        match opcode {
            Opcode::Text => {
                let text = String::from_utf8(payload)
                    .map_err(|_| WebSocketError::InvalidUtf8)?;
                self.incoming_messages.push_back(WebSocketMessage::Text(text));
            }
            Opcode::Binary => {
                self.incoming_messages.push_back(WebSocketMessage::Binary(payload));
            }
            Opcode::Ping => {
                // Auto-respond with pong
                self.send_pong(payload.clone())?;
                self.incoming_messages.push_back(WebSocketMessage::Ping(payload));
            }
            Opcode::Pong => {
                self.incoming_messages.push_back(WebSocketMessage::Pong(payload));
            }
            Opcode::Close => {
                let code = if payload.len() >= 2 {
                    let code_bytes = [payload[0], payload[1]];
                    CloseCode::from_u16(u16::from_be_bytes(code_bytes))
                } else {
                    None
                };
                
                let reason = if payload.len() > 2 {
                    String::from_utf8(payload[2..].to_vec()).ok()
                } else {
                    None
                };
                
                self.incoming_messages.push_back(WebSocketMessage::Close(code, reason));
                self.state = WebSocketState::Closed;
            }
            Opcode::Continuation => {
                // Would handle fragmented messages
                return Err(WebSocketError::UnsupportedFeature);
            }
        }
        
        Ok(())
    }
    
    /// Get close code
    pub fn close_code(&self) -> Option<CloseCode> {
        self.close_code
    }
    
    /// Get close reason
    pub fn close_reason(&self) -> Option<&str> {
        self.close_reason.as_deref()
    }
}

/// WebSocket error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebSocketError {
    /// Invalid URL
    InvalidUrl,
    /// Invalid state
    InvalidState,
    /// Not connected
    NotConnected,
    /// Already closed
    AlreadyClosed,
    /// Invalid frame
    InvalidFrame,
    /// Invalid UTF-8
    InvalidUtf8,
    /// Connection error
    ConnectionError,
    /// Unsupported feature
    UnsupportedFeature,
}

impl std::fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WebSocketError::InvalidUrl => write!(f, "Invalid WebSocket URL"),
            WebSocketError::InvalidState => write!(f, "Invalid WebSocket state"),
            WebSocketError::NotConnected => write!(f, "WebSocket not connected"),
            WebSocketError::AlreadyClosed => write!(f, "WebSocket already closed"),
            WebSocketError::InvalidFrame => write!(f, "Invalid WebSocket frame"),
            WebSocketError::InvalidUtf8 => write!(f, "Invalid UTF-8 in text message"),
            WebSocketError::ConnectionError => write!(f, "WebSocket connection error"),
            WebSocketError::UnsupportedFeature => write!(f, "Unsupported WebSocket feature"),
        }
    }
}

impl std::error::Error for WebSocketError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_websocket_creation() {
        let ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        assert_eq!(ws.state(), WebSocketState::Connecting);
        assert!(!ws.is_secure());
    }
    
    #[test]
    fn test_secure_websocket() {
        let ws = WebSocket::new("wss://example.com".to_string()).unwrap();
        assert!(ws.is_secure());
    }
    
    #[test]
    fn test_invalid_url() {
        let result = WebSocket::new("http://example.com".to_string());
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, WebSocketError::InvalidUrl);
        }
    }
    
    #[test]
    fn test_open_connection() {
        let mut ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        ws.open().unwrap();
        assert_eq!(ws.state(), WebSocketState::Open);
    }
    
    #[test]
    fn test_send_text() {
        let mut ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        ws.open().unwrap();
        
        ws.send_text("Hello".to_string()).unwrap();
        assert!(ws.next_outgoing_frame().is_some());
    }
    
    #[test]
    fn test_send_binary() {
        let mut ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        ws.open().unwrap();
        
        ws.send_binary(vec![1, 2, 3, 4]).unwrap();
        assert!(ws.next_outgoing_frame().is_some());
    }
    
    #[test]
    fn test_close() {
        let mut ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        ws.open().unwrap();
        
        ws.close(CloseCode::Normal, Some("Goodbye".to_string())).unwrap();
        assert_eq!(ws.state(), WebSocketState::Closing);
        assert_eq!(ws.close_code(), Some(CloseCode::Normal));
    }
    
    #[test]
    fn test_ping_pong() {
        let mut ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        ws.open().unwrap();
        
        ws.send_ping(vec![1, 2, 3]).unwrap();
        assert!(ws.next_outgoing_frame().is_some());
    }
    
    #[test]
    fn test_close_code_conversion() {
        assert_eq!(CloseCode::Normal.as_u16(), 1000);
        assert_eq!(CloseCode::from_u16(1000), Some(CloseCode::Normal));
        assert_eq!(CloseCode::from_u16(9999), None);
    }
    
    #[test]
    fn test_handle_incoming_text() {
        let mut ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        ws.open().unwrap();
        
        let payload = "Hello".as_bytes().to_vec();
        ws.handle_incoming_frame(Opcode::Text as u8, payload).unwrap();
        
        if let Some(WebSocketMessage::Text(text)) = ws.receive() {
            assert_eq!(text, "Hello");
        } else {
            panic!("Expected text message");
        }
    }
    
    #[test]
    fn test_handle_incoming_binary() {
        let mut ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        ws.open().unwrap();
        
        let payload = vec![1, 2, 3, 4];
        ws.handle_incoming_frame(Opcode::Binary as u8, payload.clone()).unwrap();
        
        if let Some(WebSocketMessage::Binary(data)) = ws.receive() {
            assert_eq!(data, payload);
        } else {
            panic!("Expected binary message");
        }
    }
    
    #[test]
    fn test_frame_encoding() {
        let frame = Frame::new(Opcode::Text, "Hello".as_bytes().to_vec());
        let encoded = frame.encode(false);
        
        // Check FIN and opcode
        assert_eq!(encoded[0] & 0x80, 0x80); // FIN bit set
        assert_eq!(encoded[0] & 0x0F, Opcode::Text as u8);
    }
    
    #[test]
    fn test_send_before_open() {
        let mut ws = WebSocket::new("ws://example.com".to_string()).unwrap();
        
        let result = ws.send_text("Hello".to_string());
        assert_eq!(result, Err(WebSocketError::NotConnected));
    }
}
