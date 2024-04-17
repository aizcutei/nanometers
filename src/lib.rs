#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod audio;

use crate::audio::SystemCapture;
pub use app::NanometersApp;

pub trait AudioSource {
    fn get_name(&self) -> String;
    fn start(&mut self);
    fn stop(&mut self);
}
