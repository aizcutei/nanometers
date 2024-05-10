use serde::{Deserialize, Serialize};
use std::default;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PeakOrientation {
    #[default]
    V,
    H,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct PeakSetting {
    pub(crate) smooth: f32,
    pub(crate) orientation: PeakOrientation,
}
