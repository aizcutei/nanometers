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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub audio_device: AudioDeviceSetting,
    pub waveform: WaveformSetting,
    pub spectrogram: SpectrogramSetting,
    pub stereogram: StereogramSetting,
    pub oscilloscope: OscilloscopeSetting,
    pub spectrum: SpectrumSetting,
    pub sequence: Vec<Vec<ModuleList>>,
    pub theme: Theme,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            audio_device: AudioDeviceSetting::default(),
            waveform: WaveformSetting::default(),
            spectrogram: SpectrogramSetting::default(),
            stereogram: StereogramSetting::default(),
            oscilloscope: OscilloscopeSetting::default(),
            spectrum: SpectrumSetting::default(),
            sequence: vec![
                vec![
                    ModuleList::Waveform,
                    ModuleList::Spectrogram,
                    ModuleList::Stereogram,
                    ModuleList::Oscilloscope,
                    ModuleList::Spectrum,
                    ModuleList::Peak,
                ],
                vec![],
            ],
            theme: DARK_THEME,
        }
    }
}
