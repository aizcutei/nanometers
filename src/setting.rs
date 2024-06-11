use serde::{Deserialize, Serialize};

pub(crate) mod audio_device;
pub(crate) mod audio_source;
pub(crate) mod meter;
pub(crate) mod oscilloscope;
pub(crate) mod peak;
pub(crate) mod spectrogram;
pub(crate) mod spectrum;
pub(crate) mod theme;
pub(crate) mod vectorscope;
pub(crate) mod waveform;

pub use audio_device::*;
pub use audio_source::*;
pub use meter::*;
pub use oscilloscope::*;
pub use peak::*;
pub use spectrogram::*;
pub use spectrum::*;
pub use theme::*;
pub use vectorscope::*;
pub use waveform::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub audio_device: AudioDeviceSetting,
    pub waveform: WaveformSetting,
    pub spectrogram: SpectrogramSetting,
    pub vectorscope: VectorscopeSetting,
    pub oscilloscope: OscilloscopeSetting,
    pub spectrum: SpectrumSetting,
    pub meters: Vec<Vec<MeterList>>,
    pub theme: Theme,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            audio_device: AudioDeviceSetting::default(),
            waveform: WaveformSetting::default(),
            spectrogram: SpectrogramSetting::default(),
            vectorscope: VectorscopeSetting::default(),
            oscilloscope: OscilloscopeSetting::default(),
            spectrum: SpectrumSetting::default(),
            meters: vec![
                vec![
                    MeterList::Waveform,
                    MeterList::Spectrogram,
                    MeterList::Vectorscope,
                    MeterList::Oscilloscope,
                    MeterList::Spectrum,
                    MeterList::Peak,
                    // MeterList::GPUTest,
                ],
                vec![],
            ],
            theme: DARK_THEME,
        }
    }
}
