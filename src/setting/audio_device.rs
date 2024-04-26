use serde::{Deserialize, Serialize};
use std::default;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioDevice {
    #[default]
    OutputCapture,
    PluginCapture,
    InputCapture,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct AudioDeviceSetting {
    pub(crate) device: AudioDevice,
}
