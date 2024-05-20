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
pub struct AudioSourceBuffer {
    pub peak: PeakCalcBuffer,
    pub waveform: WaveformCalcBuffer,
}

impl AudioSourceBuffer {
    pub fn new() -> Self {
        Self {
            peak: PeakCalcBuffer::new(),
            waveform: WaveformCalcBuffer::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SendData {
    pub waveform_data: WaveformSendData,
    pub iir_data: Vec<f32>,
    pub db_data: DBData,
}

impl SendData {
    pub fn new() -> Self {
        Self {
            waveform_data: WaveformSendData::new(),
            iir_data: Vec::new(),
            db_data: DBData::new(),
        }
    }
}
