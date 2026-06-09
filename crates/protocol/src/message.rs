#[derive(Debug, Clone)]
pub enum MessageType {
    Hello,
    HelloAck,
    Ping,
    Pong,
    Data,
    Ack,
    Error,
}
