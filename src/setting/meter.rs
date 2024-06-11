use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MeterList {
    Waveform,
    Spectrogram,
    Vectorscope,
    Oscilloscope,
    Spectrum,
    Peak,
    GPUTest,
}

impl ToString for MeterList {
    fn to_string(&self) -> String {
        match self {
            MeterList::Waveform => "Waveform".to_string(),
            MeterList::Spectrogram => "Spectrogram".to_string(),
            MeterList::Vectorscope => "Vectorscope".to_string(),
            MeterList::Oscilloscope => "Oscilloscope".to_string(),
            MeterList::Spectrum => "Spectrum".to_string(),
            MeterList::Peak => "Peak".to_string(),
            MeterList::GPUTest => "GPUTest".to_string(),
        }
    }
}
