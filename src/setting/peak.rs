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
    pub(crate) past_3s: VecDeque<f32>,
    #[serde(skip)]
    pub(crate) data_buffer_l: VecDeque<f32>,
    #[serde(skip)]
    pub(crate) data_buffer_r: VecDeque<f32>,
}

impl Default for Peak {
    fn default() -> Self {
        Self {
            l: f32::NEG_INFINITY,
            r: f32::NEG_INFINITY,
            plot_l: 0.0,
            plot_r: 0.0,
            lufs: f32::NEG_INFINITY,
            past_3s: vec![f32::NEG_INFINITY; 27].into(), //3000ms, 400ms per block, overlap 75%
            data_buffer_l: VecDeque::new(),
            data_buffer_r: VecDeque::new(),
        }
    }
}
