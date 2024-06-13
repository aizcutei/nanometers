use crate::utils::*;
use egui::Color32;
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
pub struct WaveformFrame {
    pub l: f32,
    pub r: f32,
    pub m: f32,
    pub s: f32,
}

impl WaveformFrame {
    pub fn new() -> Self {
        Self {
            l: 0.0,
            r: 0.0,
            m: 0.0,
            s: 0.0,
        }
    }

    pub fn new_max() -> Self {
        Self {
            l: f32::NEG_INFINITY,
            r: f32::NEG_INFINITY,
            m: f32::NEG_INFINITY,
            s: f32::NEG_INFINITY,
        }
    }

    pub fn new_min() -> Self {
        Self {
            l: f32::INFINITY,
            r: f32::INFINITY,
            m: f32::INFINITY,
            s: f32::INFINITY,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WaveformCalcBuffer {
    pub index: usize,
    pub fft_raw: RawData,
    pub l: MAXMIN,
    pub r: MAXMIN,
    pub m: MAXMIN,
    pub s: MAXMIN,
    pub low: WaveformFrame,
    pub mid: WaveformFrame,
    pub high: WaveformFrame,
}

impl WaveformCalcBuffer {
    pub fn new() -> Self {
        Self {
            index: 0,
            fft_raw: RawData::new(),
            l: MAXMIN::new(),
            r: MAXMIN::new(),
            m: MAXMIN::new(),
            s: MAXMIN::new(),
            low: WaveformFrame::new(),
            mid: WaveformFrame::new(),
            high: WaveformFrame::new(),
        }
    }
    pub fn update(&mut self, l: f32, r: f32, m: f32, s: f32) {
        self.l.max = self.l.max.max(l);
        self.l.min = self.l.min.min(l);
        self.fft_raw.l.push(l * HANN_280[self.index]);
        self.r.max = self.r.max.max(r);
        self.r.min = self.r.min.min(r);
        self.fft_raw.r.push(r * HANN_280[self.index]);
        self.m.max = self.m.max.max(m);
        self.m.min = self.m.min.min(m);
        self.fft_raw.m.push(m * HANN_280[self.index]);
        self.s.max = self.s.max.max(s);
        self.s.min = self.s.min.min(s);
        self.fft_raw.s.push(s * HANN_280[self.index]);
    }
    pub fn update_low(&mut self, l: f32, r: f32, m: f32, s: f32) {
        self.low.l += l.abs();
        self.low.r += r.abs();
        self.low.m += m.abs();
        self.low.s += s.abs();
    }
    pub fn update_mid(&mut self, l: f32, r: f32, m: f32, s: f32) {
        self.mid.l += l.abs();
        self.mid.r += r.abs();
        self.mid.m += m.abs();
        self.mid.s += s.abs();
    }
    pub fn update_high(&mut self, l: f32, r: f32, m: f32, s: f32) {
        self.high.l += l.abs();
        self.high.r += r.abs();
        self.high.m += m.abs();
        self.high.s += s.abs();
    }
    pub fn reset(&mut self) {
        self.index = 0;
        self.fft_raw = RawData::new();
        self.l = MAXMIN::new();
        self.r = MAXMIN::new();
        self.m = MAXMIN::new();
        self.s = MAXMIN::new();
        self.low = WaveformFrame::new();
        self.mid = WaveformFrame::new();
        self.high = WaveformFrame::new();
    }
}

#[derive(Debug, Clone, Default)]
pub struct WaveformSendFrame {
    pub value: MAXMIN,
    pub color: Color32,
}

#[derive(Debug, Clone, Default)]
pub struct WaveformSendData {
    pub l: Vec<WaveformSendFrame>,
    pub r: Vec<WaveformSendFrame>,
    pub m: Vec<WaveformSendFrame>,
    pub s: Vec<WaveformSendFrame>,
    // pub history: Vec<f32>,
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
