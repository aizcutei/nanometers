use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumSwitch {
    #[default]
    Main,
    Audio,
    Ref,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumMode {
    #[default]
    FFT,
    ColorBar,
    Both,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumChannel {
    #[default]
    LR,
    MS,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumFreqReadout {
    #[default]
    Off,
    Dyn,
    Static,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumFreqLine {
    #[default]
    Off,
    On,
    Bright,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpectrumSetting {
    #[serde(skip)]
    pub(crate) spectrum_switch: SpectrumSwitch,
    pub(crate) mode: SpectrumMode,
    pub(crate) smoothing: f32,
    pub(crate) slope: f32,
    pub(crate) channel: SpectrumChannel,
    pub(crate) low: f32,
    pub(crate) high: f32,
    pub(crate) freq_readout: SpectrumFreqReadout,
    pub(crate) freq_line: SpectrumFreqLine,
    pub(crate) ref_line: f32,
    pub(crate) threshold: f32,
    pub(crate) threshold_follow_slope: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Spectrum {
    #[serde(skip)]
    pub last_rect: Option<Rect>,
    #[serde(skip)]
    pub lines: Vec<Shape>,
    #[serde(skip)]
    pub line_brightness: bool,
    #[serde(skip)]
    pub pos: Vec<f32>,
    #[serde(skip)]
    pub ch0: Vec<f32>,
    #[serde(skip)]
    pub ch1: Vec<f32>,
}

impl Spectrum {
    pub fn new() -> Self {
        Self {
            ch0: vec![0.0; 2049],
            ch1: vec![0.0; 2049],
            ..Default::default()
        }
    }
}
