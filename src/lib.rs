#![warn(clippy::all, rust_2018_idioms)]
#[allow(unused)]
mod app;
pub mod audio;
pub mod setting;
pub mod utils;

use crate::audio::SystemCapture;
pub use crate::utils::ringbuffer::*;
pub use app::NanometersApp;

pub trait AudioSource {
    fn get_name(&self) -> String;
    fn start(&mut self);
    fn stop(&mut self);
}
