#![allow(dead_code)]
use crate::audio::{plugin_client, system_capture, PluginClient, SystemCapture};
use crate::frame;
use crate::frame::setting_frame;
use crate::frame::*;
use crate::setting::waveform;
use crate::setting::*;
use crate::utils::rect_alloc;
use crate::AudioSource;
use crate::RingBuffer;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use eframe::egui::{self, ViewportCommand};
use eframe::wgpu::rwh::HasWindowHandle;
use egui::*;
use std::sync::{Arc, Mutex};
use std::thread;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NanometersApp {
    #[serde(skip)]
    pub(crate) audio_source: Option<Box<dyn AudioSource>>,

    #[serde(skip)]
    raw_l: RingBuffer,

    #[serde(skip)]
    raw_r: RingBuffer,

    #[serde(skip)]
    pub(crate) tx_lr: Option<Sender<Vec<Vec<f32>>>>,

    #[serde(skip)]
    pub(crate) rx_lr: Option<Receiver<Vec<Vec<f32>>>>,

    #[serde(skip)]
    db: f64,

    #[serde(skip)]
    pub(crate) spectrum_switch: SpectrumSwitch,

    pub(crate) setting: Setting,

    setting_switch: bool,
    allways_on_top: bool,
    meter_size: eframe::epaint::Rect,
    meters_rects: Vec<eframe::epaint::Rect>,
}

impl Default for NanometersApp {
    fn default() -> Self {
        let raw_l = RingBuffer::new(240000);
        let raw_r = RingBuffer::new(240000);

        let (tx_lr, rx_lr) = unbounded();
        let tx_lr_save = Some(tx_lr.clone());
        let rx_lr_save = Some(rx_lr.clone());
        let callback = Box::new(move |data: Vec<Vec<f32>>| {
            tx_lr.send(data).unwrap();
        });

        let mut system_capture = SystemCapture::new(callback);
        system_capture.start();
        let audio_source = Some(Box::new(system_capture) as Box<dyn AudioSource>);

        Self {
            audio_source,
            raw_l,
            raw_r,
            tx_lr: tx_lr_save,
            rx_lr: rx_lr_save,
            db: 0.0,
            spectrum_switch: SpectrumSwitch::Main,
            setting: Default::default(),
            setting_switch: false,
            allways_on_top: false,
            meter_size: Rect::from_two_pos([0.0, 0.0].into(), [600.0, 200.0].into()),
            meters_rects: vec![],
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

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.main_frame(ctx);
        ctx.request_repaint();
    }
}

impl NanometersApp {
    fn main_frame(&mut self, ctx: &egui::Context) {
        let full_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(full_frame)
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();

                if self.setting_switch {
                    let meters_rect = {
                        let mut rect = app_rect;
                        rect.max.y = rect.max.y - 400.0;
                        rect
                    };
                    let setting_rect = {
                        let mut rect = app_rect;
                        rect.min.y = rect.max.y - 400.0;
                        rect
                    };

                    self.meters_ui(ui, meters_rect);
                    self.setting_ui(ui, setting_rect);
                } else {
                    self.meters_ui(ui, app_rect);
                }
                #[cfg(target_os = "windows")]
                frame::resize_ui(ui, app_rect);
            });
    }

    fn meters_ui(&mut self, ui: &mut Ui, meters_rect: Rect) {
        // If window resize
        if meters_rect != self.meter_size {
            self.meter_size = meters_rect;
            self.meters_rects = rect_alloc(
                self.setting.sequence[1].clone(),
                self.meters_rects.clone(),
                meters_rect,
            );
        }
        // If ModuleList changed
        if self.setting.sequence[1].len() != self.meters_rects.len() {
            self.meters_rects = rect_alloc(
                self.setting.sequence[1].clone(),
                self.meters_rects.clone(),
                meters_rect,
            );
        }

        if self.setting.sequence[1].is_empty() {
            let painter = ui.painter();
            painter.rect_filled(meters_rect, 0.0, Color32::from_black_alpha(200));
            painter.text(
                meters_rect.center(),
                Align2::CENTER_CENTER,
                "Add a Meter",
                FontId::proportional(20.0),
                Color32::WHITE,
            );
        }

        self.rx_lr.as_mut().unwrap().try_iter().for_each(|data| {
            self.raw_l.push(data[0].clone());
            self.raw_r.push(data[1].clone());
        });
        // println!("raw_l: {:?}", self.raw_l.index());
        // println!("raw_r: {:?}", self.raw_r.index());

        for (i, meter) in self.setting.sequence[1].clone().iter().enumerate() {
            let mut meter_rect = self.meters_rects[i];
            match meter {
                ModuleList::Waveform => {
                    self.waveform_frame(meter_rect, ui);
                }
                ModuleList::Spectrogram => {
                    self.spectrogram_frame(meter_rect, ui);
                }
                ModuleList::Peak => {
                    self.peak_frame(meter_rect, ui);
                }
                ModuleList::Oscilloscope => {
                    self.oscilloscope_frame(meter_rect, ui);
                }
                ModuleList::Spectrum => {
                    self.spectrum_frame(meter_rect, ui);
                }
                ModuleList::Stereogram => {
                    self.stereogram_frame(meter_rect, ui);
                }
            }
        }

        let painter2 = ui.painter();
        for (i, rect) in self.meters_rects.clone().iter().enumerate() {
            if i != self.meters_rects.len() - 1 {
                let mut rect = rect.clone();
                rect.min.x = rect.max.x - 5.0;
                rect.max.x += 5.0;
                let rect_response = ui.interact(
                    rect,
                    Id::new(format!("resize {}", i)),
                    Sense::click_and_drag(),
                );
                rect_response
                    .clone()
                    .on_hover_cursor(CursorIcon::ResizeHorizontal);
                if rect_response.clone().contains_pointer() {
                    ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
                    painter2.rect_filled(rect, 0.0, Color32::from_black_alpha(200));
                } else if rect_response.clone().dragged() {
                    let pointer_pos = ui.ctx().pointer_interact_pos();
                    self.meters_rects[i].max.x = pointer_pos.unwrap().x;
                    self.meters_rects[i + 1].min.x = pointer_pos.unwrap().x;
                    painter2.rect_filled(rect, 0.0, Color32::from_black_alpha(200));
                }
            }
        }

        let meters_response = ui.interact(
            meters_rect,
            Id::new("meters_buttons"),
            // Sense::union(Sense::click(), Sense::click_and_drag()),
            Sense::click(),
        );
        if meters_response.is_pointer_button_down_on() {
            if ui.ctx().input(|key| key.key_pressed(Key::Space)) {
                ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
            }
            ui.ctx().send_viewport_cmd(ViewportCommand::MaxInnerSize(
                ui.ctx().input(|i| i.viewport().monitor_size.unwrap()),
            ))
        } else if meters_response.contains_pointer() {
            ui.label("");
            ui.horizontal(|ui| {
                ui.label("  ");
                if ui.button("SETTING").clicked() && !self.setting_switch {
                    let new_size = [meters_rect.max.x, meters_rect.max.y + 400.0];
                    self.setting_switch = true;
                    ui.ctx()
                        .send_viewport_cmd(ViewportCommand::InnerSize(new_size.into()));
                    ui.ctx()
                        .send_viewport_cmd(ViewportCommand::MinInnerSize([800.0, 500.0].into()));
                }

                if ui.button("PIN TOP").clicked() {
                    self.allways_on_top = !self.allways_on_top;
                    ui.ctx().send_viewport_cmd(ViewportCommand::WindowLevel({
                        if self.allways_on_top {
                            egui::WindowLevel::AlwaysOnTop
                        } else {
                            egui::WindowLevel::Normal
                        }
                    }));
                }

                if ui.button("QUIT").clicked() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                }

                ui.label("Hold SPACE then drag to move").highlight();
            });
        }
    }

    fn setting_ui(&mut self, ui: &mut egui::Ui, setting_rect: eframe::epaint::Rect) {
        let setting_layout = Layout::centered_and_justified(Direction::TopDown);
        let mut setting_area_ui = ui.child_ui(setting_rect, setting_layout);

        setting_area_ui.vertical_centered_justified(|ui| {
            ui.separator();
            Grid::new("Setting_ui").show(ui, |ui| {
                self.device_sequence_block(ui);
                self.waveform_setting_block(ui);
                self.stereogram_settiing_block(ui);
                ui.end_row();

                self.spectrogram_setting_block(ui);
                self.spectrum_setting_block(ui);
                self.oscilloscope_setting_block(ui);
                ui.end_row();

                self.device_setting_block(ui);
                ui.end_row();
            });

            if ui.button("CLOSE SETTING").clicked() && self.setting_switch {
                self.setting_switch = false;
                let new_size = [setting_rect.max.x, setting_rect.max.y - 400.0];
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::InnerSize(new_size.into()));
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::MinInnerSize([800.0, 100.0].into()));
            }
        });
    }
}
