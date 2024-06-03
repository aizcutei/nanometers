use crate::setting::*;
use egui::*;
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

    // pub fn concat(&mut self, data: RawData) {
    //     self.l.extend(data.l);
    //     self.r.extend(data.r);
    //     self.m.extend(data.m);
    //     self.s.extend(data.s);
    // }

    // pub fn concat_front(&mut self, data: RawData) {
    //     self.l.splice(0..0, data.l);
    //     self.r.splice(0..0, data.r);
    //     self.m.splice(0..0, data.m);
    //     self.s.splice(0..0, data.s);
    // }

    // pub fn split_index(&self, index: usize) -> RawData {
    //     let l = self.l[index..].to_vec();
    //     let r = self.r[index..].to_vec();
    //     let m = self.m[index..].to_vec();
    //     let s = self.s[index..].to_vec();
    //     RawData { l, r, m, s }
    // }

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
pub struct MultibandCalcBuffer {
    pub low_buf: IIRBuffer,
    pub mid_buf: IIRBuffer,
    pub high_buf: IIRBuffer,
}

impl MultibandCalcBuffer {
    pub fn new() -> Self {
        Self {
            low_buf: IIRBuffer::new(),
            mid_buf: IIRBuffer::new(),
            high_buf: IIRBuffer::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VectorscopeSendData {
    pub max: f32,
    pub lissa: Vec<Pos2>,
    pub log: Vec<Pos2>,
    pub linear: Vec<Pos2>,
}

impl VectorscopeSendData {
    pub fn new() -> Self {
        Self {
            max: f32::NEG_INFINITY,
            lissa: Vec::new(),
            log: Vec::new(),
            linear: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VectorscopeCalcBuffer {
    pub index: usize,
    pub max: f32,
}

impl VectorscopeCalcBuffer {
    pub fn new() -> Self {
        Self {
            index: 0,
            max: f32::NEG_INFINITY,
        }
    }

    pub fn update(&mut self, l: f32, r: f32) {
        self.index += 1;
        self.max = self.max.max(l.abs()).max(r.abs());
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpectrogramOneWindow {
    pub index: usize,
    pub raw_hann: Vec<f32>,
    pub raw_hann_dt: Vec<f32>,
    pub raw_hann_t: Vec<f32>,
}

impl SpectrogramOneWindow {
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

#[derive(Debug, Clone, Default)]
pub struct SpectrogramCalcBuffer {
    pub ab: bool,
    pub a: SpectrogramOneWindow,
    pub b: SpectrogramOneWindow,
    pub image: Vec<Color32>,
}

impl SpectrogramCalcBuffer {
    pub fn new() -> Self {
        Self {
            ab: false,
            a: SpectrogramOneWindow::new(),
            b: SpectrogramOneWindow::new(),
            image: vec![Color32::TRANSPARENT; 4096 * 4096],
        }
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
    pub stereo: VectorscopeCalcBuffer,
    pub spectrogram: SpectrogramCalcBuffer,
    pub spectrum: SpectrumCalcBuffer,
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
pub struct OscilloscopeSendData {
    pub len: usize,
    pub data: Vec<f32>,
}

impl OscilloscopeSendData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SendData {
    pub waveform: WaveformSendData,
    pub iir: Vec<f32>,
    pub db: DBData,
    pub vectorscope: VectorscopeSendData,
    pub spectrum: RawData,
    pub oscilloscope: OscilloscopeSendData,
    pub spectrogram_image: Vec<Color32>,
}

impl SendData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
