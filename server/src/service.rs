pub mod message;

use common::proto::{
    MessageRequest, MessageResponse, message_request,
};
use std::sync::Arc;

pub trait MessageService {
    fn execute(self) -> Option<MessageResponse>;
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Service {
    inner: Arc<ServiceInner>,
}

#[derive(Debug, Clone)]
pub struct ServiceInner;

impl ServiceInner {
    pub fn new() -> Self {
        Self
    }
}

impl Service {
    pub fn execute(&self, msg: MessageRequest) -> Option<MessageResponse> {
        dispatch(msg)
    }
}

impl From<ServiceInner> for Service {
    fn from(inner: ServiceInner) -> Self {
        Self { 
            inner: Arc::new(inner)
        }
    }
}

fn dispatch(msg: MessageRequest) -> Option<MessageResponse> {
    println!("dispatch {:?}", msg);
    match msg.inner {
        Some(message_request::Inner::Connect(param)) => {
            println!("dispatch connect");
            param.execute()
        }
        Some(message_request::Inner::Text(param)) => {
            println!("dispatch text");
            param.execute()
        }
        Some(_) => None,
        None =>  None,
    }
}