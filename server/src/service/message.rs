use common::proto::*;
use super::MessageService;

impl MessageService for Connect {
    fn execute(self) -> Option<MessageResponse> {
        println!("execute:{}-{}", self.identifier, self.keep_alive);

        Some(MessageResponse::new_connack(10088))
    }
}

impl MessageService for Text {
    fn execute(self) -> Option<MessageResponse> {
        println!("execute:{}-{}-{}", self.sender_id, self.receiver_id, self.content);

        Some(MessageResponse::new_textack(self.message_id, 10000, 8000))
    }
}