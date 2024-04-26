use serde::{Deserialize, Serialize};
use std::default;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    H,
    V,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrogramMode {
    #[default]
    Sharp,
    Classic,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrogramCurve {
    #[default]
    Linear,
    Logarithmic,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Spectrogram {
    pub(crate) orientation: Orientation,
    pub(crate) mode: SpectrogramMode,
    pub(crate) curve: SpectrogramCurve,
    pub(crate) brightness_boost: f64,
}
