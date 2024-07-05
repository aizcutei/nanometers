#![allow(unused)]
use crate::audio::*;
use crate::frame::*;
use crate::setting::*;
use crate::utils::*;

// use crossbeam_channel::unbounded;
// use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{self, ViewportCommand};
use eframe::wgpu::core::storage;
use eframe::wgpu::rwh::HasWindowHandle;
use egui::*;
use rayon::prelude::*;
use std::sync::atomic::AtomicU32;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, vec};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NanometersApp {
    /// Thread to capture audio data and process it.
    #[serde(skip)]
    pub(crate) audio_source: Option<Box<dyn AudioSource>>,
    /// Audio source data buffer.
    #[serde(skip)]
    pub(crate) audio_source_buffer: Arc<Mutex<AudioSourceBuffer>>,
    #[serde(skip)]
    pub(crate) audio_source_setting: Arc<Mutex<Setting>>,

    /// Calculate the frame time and FPS.
    #[serde(skip)]
    pub(crate) frame_history: FrameHistory,

    /// Data channel between audio source and GUI.
    #[serde(skip)]
    pub(crate) tx_data: Option<Sender<SendData>>,
    #[serde(skip)]
    pub(crate) rx_data: Option<Receiver<SendData>>,

    pub(crate) setting: Setting,
    pub(crate) sample_rate: AtomicU32,

    pub(crate) setting_switch: bool,
    pub(crate) allways_on_top: bool,
    pub(crate) meter_size: eframe::epaint::Rect,
    pub(crate) meters_rects: Vec<eframe::epaint::Rect>,

    pub(crate) waveform: Waveform,
    pub(crate) peak: Peak,
    pub(crate) vectorscope: Vectorscope,
    pub(crate) spectrogram: Spectrogram,
    pub(crate) oscilloscope: Oscilloscope,
    pub(crate) spectrum: Spectrum,
}

impl Default for NanometersApp {
    fn default() -> Self {
        let (tx_data, rx_data) = channel();
        let audio_source_buffer = Arc::new(Mutex::new(AudioSourceBuffer::new()));
        let setting = Setting::default();
        let audio_source_setting = Arc::new(Mutex::new(setting.clone()));
        let mut system_capture = SystemCapture::new(get_callback(
            tx_data.clone(),
            audio_source_setting.clone(),
            audio_source_buffer.clone(),
        ));
        system_capture.start();
        let audio_source = Some(Box::new(system_capture) as Box<dyn AudioSource>);

        Self {
            audio_source,
            audio_source_buffer,
            audio_source_setting,
            frame_history: Default::default(),
            tx_data: Some(tx_data),
            rx_data: Some(rx_data),
            setting,
            sample_rate: AtomicU32::new(48000),
            setting_switch: false,
            allways_on_top: false,
            meter_size: Rect::from_two_pos([0.0, 0.0].into(), [600.0, 200.0].into()),
            meters_rects: vec![],
            waveform: Waveform::default(),
            peak: Peak::default(),
            vectorscope: Vectorscope::default(),
            spectrogram: Spectrogram::default(),
            oscilloscope: Oscilloscope::default(),
            spectrum: Spectrum::default(),
        }
    }
}

impl NanometersApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();

        if let Some(storage) = cc.storage {
            let mut app: NanometersApp =
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            cc.egui_ctx.set_visuals(set_theme(&mut app));
            app.audio_source_setting = Arc::new(Mutex::new(app.setting.clone()));
            match app.setting.audio_device.device {
                AudioDevice::OutputCapture => {
                    let mut system_capture = SystemCapture::new(get_callback(
                        app.tx_data.clone().unwrap(),
                        app.audio_source_setting.clone(),
                        app.audio_source_buffer.clone(),
                    ));
                    system_capture.start();
                    app.audio_source = Some(Box::new(system_capture) as Box<dyn AudioSource>);
                }
                AudioDevice::PluginCapture => {
                    let mut plugin_client = PluginClient::new(get_callback(
                        app.tx_data.clone().unwrap(),
                        app.audio_source_setting.clone(),
                        app.audio_source_buffer.clone(),
                    ));
                    plugin_client.start();
                    app.audio_source = Some(Box::new(plugin_client) as Box<dyn AudioSource>);
                }
                _ => {}
            }

            return app;
        }
        Default::default()
    }
}

impl eframe::App for NanometersApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        self.main_canvas(ctx);
        if ctx.input(|i| i.viewport().close_requested()) {
            self.audio_source.as_mut().unwrap().stop();
        }
    }
}
