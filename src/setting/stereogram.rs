use egui::Shape;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StereogramSetting {
    pub(crate) mode: StereogramMode,
    pub(crate) color: StereogramColor,
    pub(crate) polarity: StereogramPolarity,
    pub(crate) normalize: bool,
    pub(crate) guides: bool,
    pub(crate) point_size: f32,
}

impl Default for StereogramSetting {
    fn default() -> Self {
        Self {
            mode: StereogramMode::default(),
            color: StereogramColor::default(),
            polarity: StereogramPolarity::default(),
            normalize: false,
            guides: false,
            point_size: 0.9,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Stereo {
    #[serde(skip)]
    pub(crate) plot: Vec<Shape>,
}

impl Stereo {
    pub fn new() -> Self {
        Self { plot: Vec::new() }
    }
}
