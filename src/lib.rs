#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub(crate) mod audio;

pub use app::NanometersApp;
pub use audio::plugin_client::PluginClient;
pub use audio::system_capture::SystemCapture;

// enum AudioSource {
//     SystemOutput,
//     SystemInput,
//     PluginServer,
// }

pub trait AudioSource {
    fn get_name(&self) -> String;
    fn start(&mut self);
    fn stop(&mut self);
}
