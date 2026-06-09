use crate::message::MessageType;

#[derive(Debug, Clone)]
pub struct Envelope {
    pub version: u8,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}
