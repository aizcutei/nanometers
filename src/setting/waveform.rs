use serde::{Deserialize, Serialize};
use std::default;

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
pub struct Waveform {
    pub(crate) channel_1: WaveformChannel,
    pub(crate) channel_2: WaveformChannel,
    pub(crate) mode: WaveformMode,
    pub(crate) peak_history: WaveformHistory,
    pub(crate) speed: usize,
}
