use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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
pub struct PeakCalcBuffer {
    pub index: usize,
    pub iir_l: IIRBuffer,
    pub iir_r: IIRBuffer,
    pub sum_l: f32,
    pub sum_r: f32,
    pub sum: f32,
}

impl PeakCalcBuffer {
    pub fn new() -> Self {
        Self {
            index: 0,
            iir_l: IIRBuffer::new(),
            iir_r: IIRBuffer::new(),
            sum_l: 0.0,
            sum_r: 0.0,
            sum: 0.0,
        }
    }

    pub fn reset_sum(&mut self) {
        self.index = 0;
        self.sum_l = 0.0;
        self.sum_r = 0.0;
        self.sum = 0.0;
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

#[derive(Debug, Clone, Default)]
pub struct IIRBuffer {
    pub x_1: f32,
    pub x_2: f32,
    pub y_1: f32,
    pub y_2: f32,
    pub z_1: f32,
    pub z_2: f32,
}

impl IIRBuffer {
    pub fn new() -> Self {
        Self {
            x_1: 0.0,
            x_2: 0.0,
            y_1: 0.0,
            y_2: 0.0,
            z_1: 0.0,
            z_2: 0.0,
        }
    }
}
