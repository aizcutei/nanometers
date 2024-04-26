use serde::{Deserialize, Serialize};

pub(crate) mod audio_device;
pub(crate) mod module;
pub(crate) mod oscilloscope;
pub(crate) mod spectrogram;
pub(crate) mod spectrum;
pub(crate) mod stereogram;
pub(crate) mod theme;
pub(crate) mod waveform;

pub use audio_device::*;
pub use module::*;
pub use oscilloscope::*;
pub use spectrogram::*;
pub use spectrum::*;
pub use stereogram::*;
pub use theme::*;
pub use waveform::*;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Setting {
    pub module: Module,
    pub theme: Theme,
    pub audio_device: AudioDeviceSetting,
    pub waveform: Waveform,
    pub spectrogram: Spectrogram,
    pub stereogram: Stereogram,
    pub oscilloscope: Oscilloscope,
    pub spectrum: Spectrum,
}
