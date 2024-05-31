use std::collections::VecDeque;

use egui::{Color32, Pos2};
use serde::{Deserialize, Serialize};

use crate::utils::{SpectrogramFrame, SpectrogramOneWindow, HANN_2048, HANN_DT_2048, HANN_T_2048};

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
}

impl Spectrogram {
    pub fn new() -> Self {
        Self { texture: None }
    }
}

pub fn updata_spectrogram_window(window: &mut SpectrogramOneWindow, index: usize, value: f32) {
    window.raw_hann.push(value * HANN_2048[index]);
    window.raw_hann_t.push(value * HANN_T_2048[index]);
    window.raw_hann_dt.push(value * HANN_DT_2048[index]);
    window.index += 1;
}
