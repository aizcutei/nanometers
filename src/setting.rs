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
    pub theme: Theme,
    pub audio_device: AudioDeviceSetting,
    pub waveform: Waveform,
    pub spectrogram: Spectrogram,
    pub stereogram: Stereogram,
    pub oscilloscope: Oscilloscope,
    pub spectrum: Spectrum,
    pub sequence: Vec<Vec<ModuleList>>,
    pub themelist: Vec<ThemeType>,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            audio_device: AudioDeviceSetting::default(),
            waveform: Waveform::default(),
            spectrogram: Spectrogram::default(),
            stereogram: Stereogram::default(),
            oscilloscope: Oscilloscope::default(),
            spectrum: Spectrum::default(),
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
            themelist: vec![ThemeType::Light, ThemeType::Dark, ThemeType::Custom],
        }
    }
}
