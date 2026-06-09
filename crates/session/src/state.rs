#[derive(Debug, Clone)]
pub enum SessionState {
    Discovered,
    Connecting,
    Established,
    Closed,
}
