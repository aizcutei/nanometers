use serde::{Deserialize, Serialize};

pub(crate) mod module;
pub(crate) mod theme;
pub(crate) mod waveform;

pub use module::*;
pub use theme::*;
pub use waveform::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Setting {
    pub module: Module,
    pub waveform: Waveform,
}
