use serde::{Deserialize, Serialize};
use std::default;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StereogramMode {
    #[default]
    Logarithmic,
    Linear,
    Lissajous,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StereogramColor {
    #[default]
    Static,
    RGB,
    MultiBand,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StereogramPolarity {
    #[default]
    Uni,
    Bi,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Stereogram {
    pub(crate) mode: StereogramMode,
    pub(crate) color: StereogramColor,
    pub(crate) polarity: StereogramPolarity,
    pub(crate) normalize: bool,
    pub(crate) guides: bool,
}
