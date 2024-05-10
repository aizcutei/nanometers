use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OscilloscopeCycle {
    #[default]
    Multi,
    Single,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OscilloscopeSetting {
    pub(crate) follow_pitch: bool,
    pub(crate) cycle: OscilloscopeCycle,
}
