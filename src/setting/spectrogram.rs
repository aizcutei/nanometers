use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrogramOrientation {
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
pub struct SpectrogramSetting {
    pub(crate) orientation: SpectrogramOrientation,
    pub(crate) mode: SpectrogramMode,
    pub(crate) curve: SpectrogramCurve,
    pub(crate) brightness_boost: f64,
}
