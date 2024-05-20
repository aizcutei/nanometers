use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PeakOrientation {
    #[default]
    V,
    H,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct PeakSetting {
    pub(crate) decay: f32,
    pub(crate) orientation: PeakOrientation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peak {
    pub(crate) l: f32,
    pub(crate) r: f32,
    pub(crate) plot_l: f32,
    pub(crate) plot_r: f32,
    pub(crate) lufs: f32,
    #[serde(skip)]
    pub(crate) past_1500ms: VecDeque<f32>,
    #[serde(skip)]
    pub(crate) data_buffer: VecDeque<f32>,
}

impl Default for Peak {
    fn default() -> Self {
        Self {
            l: f32::NEG_INFINITY,
            r: f32::NEG_INFINITY,
            plot_l: 0.0,
            plot_r: 0.0,
            lufs: f32::NEG_INFINITY,
            past_1500ms: VecDeque::new(), //1500ms, 400ms per block, overlap 75%
            data_buffer: VecDeque::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DBData {
    pub l: f32,
    pub r: f32,
}

impl DBData {
    pub fn new() -> Self {
        Self {
            l: f32::NEG_INFINITY,
            r: f32::NEG_INFINITY,
        }
    }
}
