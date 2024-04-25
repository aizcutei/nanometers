use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Module {
    pub waveform: bool,
    pub peak: bool,
    pub spectrogram: bool,
    pub stereogram: bool,
    pub oscilloscope: bool,
    pub spectrum: bool,
}
