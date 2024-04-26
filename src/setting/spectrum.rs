use serde::{Deserialize, Serialize};
use std::default;

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
pub struct Spectrum {
    pub(crate) mode: SpectrumMode,
    pub(crate) smoothing: f64,
    pub(crate) slope: f64,
    pub(crate) channel: SpectrumChannel,
    pub(crate) low: f64,
    pub(crate) high: f64,
    pub(crate) freq_readout: SpectrumFreqReadout,
    pub(crate) freq_line: SpectrumFreqLine,
    pub(crate) ref_line: f64,
    pub(crate) threshold: f64,
    pub(crate) threshold_follow_slope: bool,
}
