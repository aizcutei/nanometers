use egui::Pos2;

use crate::setting::*;
use std::fmt::Display;

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

    pub fn concat(&mut self, data: RawData) {
        self.l.extend(data.l);
        self.r.extend(data.r);
        self.m.extend(data.m);
        self.s.extend(data.s);
    }

    pub fn concat_front(&mut self, data: RawData) {
        self.l.splice(0..0, data.l);
        self.r.splice(0..0, data.r);
        self.m.splice(0..0, data.m);
        self.s.splice(0..0, data.s);
    }

    pub fn split_index(&self, index: usize) -> RawData {
        let l = self.l[index..].to_vec();
        let r = self.r[index..].to_vec();
        let m = self.m[index..].to_vec();
        let s = self.s[index..].to_vec();
        RawData { l, r, m, s }
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
pub struct Multiband {
    pub low: f32,
    pub mid: f32,
    pub high: f32,
}

impl Multiband {
    pub fn new() -> Self {
        Self {
            low: 0.0,
            mid: 0.0,
            high: 0.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StereoSendData {
    pub max: f32,
    pub lissa: Vec<Pos2>,
    pub log: Vec<Pos2>,
    pub linear: Vec<Pos2>,
}

impl StereoSendData {
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
pub struct StereoCalcBuffer {
    pub index: usize,
    pub max: f32,
}

impl StereoCalcBuffer {
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
pub struct AudioSourceBuffer {
    pub peak: PeakCalcBuffer,
    pub waveform: WaveformCalcBuffer,
    pub stereo: StereoCalcBuffer,
}

impl AudioSourceBuffer {
    pub fn new() -> Self {
        Self {
            peak: PeakCalcBuffer::new(),
            waveform: WaveformCalcBuffer::new(),
            stereo: StereoCalcBuffer::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SendData {
    pub waveform_data: WaveformSendData,
    pub iir_data: Vec<f32>,
    pub db_data: DBData,
    pub stereo_data: StereoSendData,
}

impl SendData {
    pub fn new() -> Self {
        Self {
            waveform_data: WaveformSendData::new(),
            iir_data: Vec::new(),
            db_data: DBData::new(),
            stereo_data: StereoSendData::new(),
        }
    }
}
