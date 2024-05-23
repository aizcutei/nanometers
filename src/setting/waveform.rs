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
        self.raw.l.push(val);
        self.l.max = self.l.max.max(val);
        self.l.min = self.l.min.min(val);
    }
    pub fn update_r(&mut self, val: f32) {
        self.raw.r.push(val);
        self.r.max = self.r.max.max(val);
        self.r.min = self.r.min.min(val);
    }
    pub fn update_m(&mut self, val: f32) {
        self.raw.m.push(val);
        self.m.max = self.m.max.max(val);
        self.m.min = self.m.min.min(val);
    }
    pub fn update_s(&mut self, val: f32) {
        self.raw.s.push(val);
        self.s.max = self.s.max.max(val);
        self.s.min = self.s.min.min(val);
    }
    pub fn reset(&mut self) {
        self.index = 0;
        self.raw.clear();
        self.l = MAXMIN::new();
        self.r = MAXMIN::new();
        self.m = MAXMIN::new();
        self.s = MAXMIN::new();
    }
}

#[derive(Debug, Clone, Default)]
pub struct WaveformSendData {
    pub l: Vec<MAXMIN>,
    pub r: Vec<MAXMIN>,
    pub m: Vec<MAXMIN>,
    pub s: Vec<MAXMIN>,
    pub l_freq: Vec<usize>,
    pub r_freq: Vec<usize>,
    pub m_freq: Vec<usize>,
    pub s_freq: Vec<usize>,
    pub l_color: Vec<egui::Color32>,
    pub r_color: Vec<egui::Color32>,
    pub m_color: Vec<egui::Color32>,
    pub s_color: Vec<egui::Color32>,
}

impl WaveformSendData {
    pub fn new() -> Self {
        Self {
            l: vec![],
            r: vec![],
            m: vec![],
            s: vec![],
            l_freq: vec![],
            r_freq: vec![],
            m_freq: vec![],
            s_freq: vec![],
            l_color: vec![],
            r_color: vec![],
            m_color: vec![],
            s_color: vec![],
        }
    }
    pub fn concat(&mut self, data: &WaveformSendData) {
        self.l.extend_from_slice(&data.l);
        self.r.extend_from_slice(&data.r);
        self.m.extend_from_slice(&data.m);
        self.s.extend_from_slice(&data.s);
        self.l_freq.extend_from_slice(&data.l_freq);
        self.r_freq.extend_from_slice(&data.r_freq);
        self.m_freq.extend_from_slice(&data.m_freq);
        self.s_freq.extend_from_slice(&data.s_freq);
        self.l_color.extend_from_slice(&data.l_color);
        self.r_color.extend_from_slice(&data.r_color);
        self.m_color.extend_from_slice(&data.m_color);
        self.s_color.extend_from_slice(&data.s_color);
    }
}
