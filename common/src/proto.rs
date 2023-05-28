pub mod abi;

pub use abi::*;

impl MessageRequest {
    pub fn new_connect(identifier: impl Into<String>, keep_alive: u32) -> Self {
        Self { 
            inner: Some(message_request::Inner::Connect(Connect {
                identifier: identifier.into(),
                keep_alive,
            }))
        }
    }

    pub fn new_ping() -> Self {
        Self { 
            inner: Some(message_request::Inner::Ping(Ping {}))
        }
    }

    pub fn new_text(message_id: u64, sender_id: u32, receiver_id: u32, message_type: u32, content: &str) -> Self {
        Self { 
            inner: Some(message_request::Inner::Text(Text { 
                message_id, 
                sender_id, 
                receiver_id, 
                message_type, 
                content: String::from(content),
            }))
        }
    }

    pub fn new_textack(message_id: u64, store_id: u64, status: u32) -> Self {
        Self { 
            inner: Some(message_request::Inner::Textack(Textack { 
                message_id, 
                store_id, 
                status,
            }))
        }
    }

    pub fn new_disconnect() -> Self {
        Self { 
            inner: Some(message_request::Inner::Disconnect(Disconnect {}))
        }
    }
}

impl MessageResponse {
    pub fn new_connack(code: i32) -> Self {
        Self { 
            inner: Some(message_response::Inner::Connack(Connack {
                code,
            }))
        }
    }

    pub fn new_pong() -> Self {
        Self { 
            inner: Some(message_response::Inner::Pong(Pong {}))
        }
    }

    pub fn new_text(message_id: u64, sender_id: u32, receiver_id: u32, message_type: u32, content: &str) -> Self {
        Self { 
            inner: Some(message_response::Inner::Text(Text { 
                message_id, 
                sender_id, 
                receiver_id, 
                message_type, 
                content: String::from(content),
            }))
        }
    }

    pub fn new_textack(message_id: u64, store_id: u64, status: u32) -> Self {
        Self { 
            inner: Some(message_response::Inner::Textack(Textack { 
                message_id, 
                store_id, 
                status,
            }))
        }
    }

    pub fn new_disconnect() -> Self {
        Self { 
            inner: Some(message_response::Inner::Disconnect(Disconnect {}))
        }
    }
}