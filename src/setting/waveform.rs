use crate::utils::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WaveformMode {
    #[default]
    Static,
    MultiBand,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WaveformChannel {
    #[default]
    None,
    Left,
    Right,
    Mid,
    Side,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WaveformHistory {
    #[default]
    Off,
    Fast,
    Slow,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct WaveformSetting {
    pub(crate) channel_1: WaveformChannel,
    pub(crate) channel_2: WaveformChannel,
    pub(crate) mode: WaveformMode,
    pub(crate) peak_history: WaveformHistory,
    pub(crate) speed: usize,
}

#[derive(Debug)]
pub struct WaveformPlotPoint {
    pub(crate) uu: VecDeque<f32>,
    pub(crate) ud: VecDeque<f32>,
    pub(crate) ucolor: VecDeque<egui::Color32>,
    pub(crate) du: VecDeque<f32>,
    pub(crate) dd: VecDeque<f32>,
    pub(crate) dcolor: VecDeque<egui::Color32>,
    pub(crate) r: VecDeque<f32>,
    pub(crate) g: VecDeque<f32>,
    pub(crate) b: VecDeque<f32>,
}

impl WaveformPlotPoint {
    pub fn new(size: usize) -> Self {
        Self {
            uu: VecDeque::with_capacity(size),
            ud: VecDeque::with_capacity(size),
            ucolor: VecDeque::with_capacity(size),
            du: VecDeque::with_capacity(size),
            dd: VecDeque::with_capacity(size),
            dcolor: VecDeque::with_capacity(size),
            r: VecDeque::with_capacity(size),
            g: VecDeque::with_capacity(size),
            b: VecDeque::with_capacity(size),
        }
    }
}

impl Default for WaveformPlotPoint {
    fn default() -> Self {
        Self::new(1920)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Waveform {
    pub(crate) history_length: usize,
    #[serde(skip)]
    pub(crate) plot_point: WaveformPlotPoint,
    pub(crate) update_speed: usize,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            history_length: 3840,
            plot_point: WaveformPlotPoint::new(3840),
            update_speed: 256,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WaveformCalcBuffer {
    pub index: usize,
    pub raw: RawData,
    pub l: MAXMIN,
    pub r: MAXMIN,
    pub m: MAXMIN,
    pub s: MAXMIN,
}

impl WaveformCalcBuffer {
    pub fn new() -> Self {
        Self {
            index: 0,
            raw: RawData::new(),
            l: MAXMIN::new(),
            r: MAXMIN::new(),
            m: MAXMIN::new(),
            s: MAXMIN::new(),
        }
    }
    pub fn update_l(&mut self, val: f32) {
        self.l.max = self.l.max.max(val);
        self.l.min = self.l.min.min(val);
        self.raw.l.push(val * HANN_280[self.index]);
    }
    pub fn update_r(&mut self, val: f32) {
        self.r.max = self.r.max.max(val);
        self.r.min = self.r.min.min(val);
        self.raw.r.push(val * HANN_280[self.index]);
    }
    pub fn update_m(&mut self, val: f32) {
        self.m.max = self.m.max.max(val);
        self.m.min = self.m.min.min(val);
        self.raw.m.push(val * HANN_280[self.index]);
    }
    pub fn update_s(&mut self, val: f32) {
        self.s.max = self.s.max.max(val);
        self.s.min = self.s.min.min(val);
        self.raw.s.push(val * HANN_280[self.index]);
    }
    pub fn reset(&mut self) {
        self.index = 0;
        self.raw = RawData::new();
        self.l = MAXMIN::new();
        self.r = MAXMIN::new();
        self.m = MAXMIN::new();
        self.s = MAXMIN::new();
    }
}

#[derive(Debug, Clone, Default)]
pub struct WaveformSendFrame {
    pub value: MAXMIN,
    pub color: [f32; 3],
}

#[derive(Debug, Clone, Default)]
pub struct WaveformSendData {
    pub l: Vec<WaveformSendFrame>,
    pub r: Vec<WaveformSendFrame>,
    pub m: Vec<WaveformSendFrame>,
    pub s: Vec<WaveformSendFrame>,
}

impl WaveformSendData {
    pub fn new() -> Self {
        Self {
            l: vec![],
            r: vec![],
            m: vec![],
            s: vec![],
        }
    }
    pub fn concat(&mut self, data: &WaveformSendData) {
        self.l.extend_from_slice(&data.l);
        self.r.extend_from_slice(&data.r);
        self.m.extend_from_slice(&data.m);
        self.s.extend_from_slice(&data.s);
    }
}
