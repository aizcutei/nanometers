use crate::utils::*;
use egui::*;
use serde::{Deserialize, Serialize};

// #[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub enum SpectrogramContrast {
//     #[default]
//     L,
//     M,
//     H,
// }

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpectrogramSetting {
    pub(crate) orientation: SpectrogramOrientation,
    pub(crate) mode: SpectrogramMode,
    pub(crate) curve: SpectrogramCurve,
    // pub(crate) contrast: SpectrogramContrast,
    pub(crate) brightness_boost: f32,
    pub(crate) resolution: usize,
}

impl Default for SpectrogramSetting {
    fn default() -> Self {
        Self {
            orientation: SpectrogramOrientation::H,
            mode: SpectrogramMode::Sharp,
            curve: SpectrogramCurve::Linear,
            // contrast: SpectrogramContrast::L,
            brightness_boost: 0.05,
            resolution: 2048,
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpectrogramPoints {
    pub pos: Pos2,
    pub color: u8,
}

impl SpectrogramPoints {
    pub fn new(pos: Pos2, color: u8) -> Self {
        Self { pos, color }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpectrogramRects {
    pub pos: Pos2,
    pub color: u8,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Spectrogram {
    #[serde(skip)]
    pub(crate) texture: Option<egui::TextureHandle>,

    #[cfg(target_os = "macos")]
    #[serde(skip)]
    pub(crate) last_img: Option<piet_common::CoreGraphicsImage>,

    #[cfg(target_os = "windows")]
    #[serde(skip)]
    pub(crate) last_img: Option<piet_common::D2DImage>,

    #[cfg(target_os = "linux")]
    #[serde(skip)]
    pub(crate) last_img: Option<piet_common::CairoImage>,
}

impl Spectrogram {
    pub fn new() -> Self {
        Self {
            texture: None,
            last_img: None,
        }
    }
}

pub fn updata_spectrogram_window(window: &mut SpectrogramCalcFrame, index: usize, value: f32) {
    window.raw_hann.push(value * HANN_2048[index]);
    window.raw_hann_t.push(value * HANN_T_2048[index]);
    window.raw_hann_dt.push(value * HANN_DT_2048[index]);
    window.index += 1;
}
