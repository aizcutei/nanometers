use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ModuleList {
    Waveform,
    Spectrogram,
    Stereogram,
    Oscilloscope,
    Spectrum,
    Peak,
}

impl ToString for ModuleList {
    fn to_string(&self) -> String {
        match self {
            ModuleList::Waveform => "Waveform".to_string(),
            ModuleList::Spectrogram => "Spectrogram".to_string(),
            ModuleList::Stereogram => "Stereogram".to_string(),
            ModuleList::Oscilloscope => "Oscilloscope".to_string(),
            ModuleList::Spectrum => "Spectrum".to_string(),
            ModuleList::Peak => "Peak".to_string(),
        }
    }
}
