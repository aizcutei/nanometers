use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum WaveformMode {
    #[default]
    Static,
    MultiBand,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Waveform {
    pub(crate) combine: bool,
    pub(crate) mode: WaveformMode,
    pub(crate) peak_history: bool,
    pub(crate) speed: usize,
}
