use crate::state::SessionState;
use common::types::Device;

pub struct Session {
    pub peer: Device,
    pub state: SessionState,
}
