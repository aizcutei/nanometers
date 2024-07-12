use crate::setting::*;
use egui::*;
use std::collections::VecDeque;
use std::{collections::binary_heap, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct RawData {
    pub l: Vec<f32>,
    pub r: Vec<f32>,
    pub m: Vec<f32>,
    pub s: Vec<f32>,
}
impl RawData {
    pub fn new() -> Self {
        Self {
            l: vec![],
            r: vec![],
            m: vec![],
            s: vec![],
        }
    }

    pub fn keep_last(&mut self, size: usize) {
        let len = self.l.len();
        if len > size {
            self.l.drain(0..len - size);
            self.r.drain(0..len - size);
            self.m.drain(0..len - size);
            self.s.drain(0..len - size);
        }
    }

    pub fn clear(&mut self) {
        self.l.clear();
        self.r.clear();
        self.m.clear();
        self.s.clear();
    }
}

#[derive(Debug, Clone, Default)]
pub struct MAXMIN {
    pub max: f32,
    pub min: f32,
}

impl Display for MAXMIN {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "max: {}, min: {}", self.max, self.min)
    }
}

impl MAXMIN {
    pub fn new() -> Self {
        Self {
            max: f32::NEG_INFINITY,
            min: f32::INFINITY,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MultibandChannelBuffer {
    pub l: IIRBuffer,
    pub r: IIRBuffer,
}

impl MultibandChannelBuffer {
    pub fn new() -> Self {
        Self {
            l: IIRBuffer::new(),
            r: IIRBuffer::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MultibandCalcBuffer {
    pub low_buf: MultibandChannelBuffer,
    pub mid_buf: MultibandChannelBuffer,
    pub high_buf: MultibandChannelBuffer,
}

impl MultibandCalcBuffer {
    pub fn new() -> Self {
        Self {
            low_buf: MultibandChannelBuffer::new(),
            mid_buf: MultibandChannelBuffer::new(),
            high_buf: MultibandChannelBuffer::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VectorscopeSendData {
    pub max: f32,
    pub r_max: f32,
    pub g_max: f32,
    pub b_max: f32,
    pub r: Vec<Pos2>, // Non multiband mode as default channel
    pub g: Vec<Pos2>,
    pub b: Vec<Pos2>,
    pub c: Vec<Color32>,
}

impl VectorscopeSendData {
    pub fn new() -> Self {
        Self {
            max: f32::NEG_INFINITY,
            r_max: f32::NEG_INFINITY,
            g_max: f32::NEG_INFINITY,
            b_max: f32::NEG_INFINITY,
            r: Vec::new(),
            g: Vec::new(),
            b: Vec::new(),
            c: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VectorscopeCalcBuffer {
    pub index: usize,
    pub max: f32,
    pub r_max: f32,
    pub g_max: f32,
    pub b_max: f32,
}

impl VectorscopeCalcBuffer {
    pub fn new() -> Self {
        Self {
            index: 0,
            max: f32::NEG_INFINITY,
            r_max: f32::NEG_INFINITY,
            g_max: f32::NEG_INFINITY,
            b_max: f32::NEG_INFINITY,
        }
    }

    pub fn update(
        &mut self,
        l: f32,
        r: f32,
        r_l: f32,
        r_r: f32,
        g_l: f32,
        g_r: f32,
        b_l: f32,
        b_r: f32,
    ) {
        self.index += 1;
        self.max = self.max.max(l.abs()).max(r.abs());
        self.r_max = self.r_max.max(r_l.abs()).max(r_r.abs());
        self.g_max = self.g_max.max(g_l.abs()).max(g_r.abs());
        self.b_max = self.b_max.max(b_l.abs()).max(b_r.abs());
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.max = f32::NEG_INFINITY;
        self.r_max = f32::NEG_INFINITY;
        self.g_max = f32::NEG_INFINITY;
        self.b_max = f32::NEG_INFINITY;
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpectrogramCalcFrame {
    pub index: usize,
    pub raw_hann: Vec<f32>,
    pub raw_hann_dt: Vec<f32>,
    pub raw_hann_t: Vec<f32>,
}

impl SpectrogramCalcFrame {
    pub fn new() -> Self {
        Self {
            index: 0,
            raw_hann: Vec::new(),
            raw_hann_dt: Vec::new(),
            raw_hann_t: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.raw_hann.clear();
        self.raw_hann_dt.clear();
        self.raw_hann_t.clear();
    }
}

#[derive(Debug, Clone)]
pub struct SpectrogramCalcBuffer {
    // pub ab: bool,
    // pub a: SpectrogramOneWindow,
    // pub b: SpectrogramOneWindow,
    pub frames: VecDeque<SpectrogramCalcFrame>,
}

impl SpectrogramCalcBuffer {
    pub fn new() -> Self {
        Self {
            // ab: false,
            // a: SpectrogramOneWindow::new(),
            // b: SpectrogramOneWindow::new(),
            frames: VecDeque::new(),
        }
    }
}

impl Default for SpectrogramCalcBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpectrumCalcBuffer {
    pub ab: bool,
    pub a: RawData,
    pub b: RawData,
}

impl SpectrumCalcBuffer {
    pub fn new() -> Self {
        Self {
            ab: false,
            a: RawData::new(),
            b: RawData::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AudioSourceBuffer {
    pub fft_2048_index: usize,
    pub raw: RawData,
    pub low_raw: RawData,
    pub mid_raw: RawData,
    pub high_raw: RawData,
    pub multiband: MultibandCalcBuffer,
    pub peak: PeakCalcBuffer,
    pub waveform: WaveformCalcBuffer,
    pub vector: VectorscopeCalcBuffer,
    pub spectrogram: SpectrogramCalcBuffer,
    pub spectrum: SpectrumCalcBuffer,
    pub osc: OscCalcBuffer,
    pub setting: Setting,
}

impl AudioSourceBuffer {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn new_with_setting(setting: Setting) -> Self {
        Self {
            setting,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpectrogramFrame {
    pub f: Vec<f32>,
    pub fc: Vec<f32>,
    pub tc: Vec<f32>,
    pub cc: Vec<f32>,
}

impl SpectrogramFrame {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpectrumSendData {
    pub frames: Vec<RawData>,
}

impl SpectrumSendData {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpectrogramSendFrame {
    pub f: Vec<f32>,
    pub t: Vec<f32>,
    pub i: Vec<u8>,
    pub classic_i: Vec<u8>,
}

impl SpectrogramSendFrame {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SendData {
    pub waveform: WaveformSendData,
    pub iir: Vec<f32>,
    pub db: DBData,
    pub vectorscope: VectorscopeSendData,
    pub spectrum: RawData,
    pub oscilloscope: OscilloscopeSendData,
    pub spectrogram: SpectrogramSendFrame,
}

impl SendData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for SendData {
    fn default() -> Self {
        Self {
            waveform: WaveformSendData::new(),
            iir: Vec::new(),
            db: DBData::new(),
            vectorscope: VectorscopeSendData::new(),
            spectrum: RawData::new(),
            oscilloscope: OscilloscopeSendData::new(),
            spectrogram: SpectrogramSendFrame::new(),
        }
    }
}
