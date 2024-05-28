use egui::Shape;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VectorscopeMode {
    #[default]
    Logarithmic,
    Linear,
    Lissajous,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VectorscopeColor {
    #[default]
    Static,
    RGB,
    MultiBand,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VectorscopePolarity {
    #[default]
    Uni,
    Bi,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct VectorscopeSetting {
    pub(crate) mode: VectorscopeMode,
    pub(crate) color: VectorscopeColor,
    pub(crate) polarity: VectorscopePolarity,
    pub(crate) normalize: bool,
    pub(crate) guides: bool,
    pub(crate) point_size: f32,
}

impl Default for VectorscopeSetting {
    fn default() -> Self {
        Self {
            mode: VectorscopeMode::default(),
            color: VectorscopeColor::default(),
            polarity: VectorscopePolarity::default(),
            normalize: false,
            guides: false,
            point_size: 0.9,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Vectorscope {
    #[serde(skip)]
    pub(crate) plot: Vec<Shape>,
}

impl Vectorscope {
    pub fn new() -> Self {
        Self { plot: Vec::new() }
    }
}
