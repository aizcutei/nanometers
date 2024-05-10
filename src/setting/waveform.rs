use std::collections::VecDeque;

use crate::utils::*;
use egui::Pos2;
use serde::{Deserialize, Serialize};

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
    pub(crate) len: usize,
    pub(crate) uu: RingBuffer,
    pub(crate) ud: RingBuffer,
    pub(crate) du: RingBuffer,
    pub(crate) dd: RingBuffer,
}

impl WaveformPlotPoint {
    pub fn new(size: usize) -> Self {
        Self {
            len: size,
            uu: RingBuffer::new_with_default(size, 0.0),
            ud: RingBuffer::new_with_default(size, 0.0),
            du: RingBuffer::new_with_default(size, 0.0),
            dd: RingBuffer::new_with_default(size, 0.0),
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
    #[serde(skip)]
    pub(crate) plot_point: WaveformPlotPoint,
    #[serde(skip)]
    pub(crate) data_buffer: RawData,
    pub(crate) update_speed: usize,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            plot_point: WaveformPlotPoint::new(3840),
            data_buffer: RawData::new(),
            update_speed: 400,
        }
    }
}
