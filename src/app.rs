#![allow(unused)]
use crate::audio::*;
use crate::frame::*;
use crate::setting::*;
use crate::utils::*;
use crate::AudioSource;
use crate::RingBuffer;

use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{self, ViewportCommand};
use eframe::wgpu::rwh::HasWindowHandle;
use egui::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::{thread, vec};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NanometersApp {
    #[serde(skip)]
    pub(crate) audio_source: Option<Box<dyn AudioSource>>,

    #[serde(skip)]
    pub(crate) frame_history: FrameHistory,

    #[serde(skip)]
    pub(crate) tx_lrms: Option<Sender<RawData>>,

    #[serde(skip)]
    pub(crate) rx_lrms: Option<Receiver<RawData>>,

    pub(crate) setting: Setting,

    pub(crate) setting_switch: bool,
    pub(crate) allways_on_top: bool,
    pub(crate) meter_size: eframe::epaint::Rect,
    pub(crate) meters_rects: Vec<eframe::epaint::Rect>,

    pub(crate) waveform: Waveform,
    pub(crate) peak: Peak,
}

impl Default for NanometersApp {
    fn default() -> Self {
        let (tx_lrms, rx_lrms) = unbounded();
        let tx_lrms_save = Some(tx_lrms.clone());
        let rx_lrms_save = Some(rx_lrms.clone());
        let callback = Box::new(move |data: Vec<Vec<f32>>| {
            #[cfg(feature = "puffin")]
            puffin::profile_scope!("callback");
            let mut send_data = RawData::new();
            data[0].iter().zip(&data[1]).for_each(|(l, r)| {
                send_data.push_l(*l);
                send_data.push_r(*r);
                send_data.push_m((l + r) / 2.0);
                send_data.push_s((l - r) / 2.0);
            });
            tx_lrms.send(send_data).unwrap();
        });

        let mut system_capture = SystemCapture::new(callback);
        system_capture.start();
        let audio_source = Some(Box::new(system_capture) as Box<dyn AudioSource>);

        Self {
            audio_source,
            frame_history: Default::default(),
            tx_lrms: tx_lrms_save,
            rx_lrms: rx_lrms_save,
            setting: Default::default(),
            setting_switch: false,
            allways_on_top: false,
            meter_size: Rect::from_two_pos([0.0, 0.0].into(), [600.0, 200.0].into()),
            meters_rects: vec![],
            waveform: Default::default(),
            peak: Default::default(),
        }
    }
}

impl NanometersApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
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
        self.main_frame(ctx);
        if ctx.input(|i| i.viewport().close_requested()) {
            self.audio_source.as_mut().unwrap().stop();
        }
    }
}
