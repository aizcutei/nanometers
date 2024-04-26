#[allow(unused)]
use crate::audio::{plugin_client, PluginClient, SystemCapture};
use crate::setting;
use crate::setting::Channel;
use crate::AudioSource;
use crate::RingBuffer;
use eframe::egui::{self, ViewportCommand};
use eframe::wgpu::rwh::HasWindowHandle;
use egui::Sense;
use egui::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NanometersApp {
    #[serde(skip)]
    audio_source: Option<Box<dyn AudioSource>>,

    #[serde(skip)]
    raw_l: RingBuffer,

    #[serde(skip)]
    raw_r: RingBuffer,

    #[serde(skip)]
    tx: Option<Sender<Vec<Vec<f32>>>>,

    #[serde(skip)]
    update_handle: Option<thread::JoinHandle<()>>,

    #[serde(skip)]
    db: f64,

    setting_switch: bool,
    allways_on_top: bool,

    setting: setting::Setting,
}

impl Default for NanometersApp {
    fn default() -> Self {
        let raw_l = RingBuffer::new(240000);
        let raw_r = RingBuffer::new(240000);

        // let (tx, rx) = std::sync::mpsc::channel();
        // let txs = tx.clone();
        // let callback = Box::new(move |data: Vec<Vec<f32>>| {
        //     txs.send(data).unwrap();
        // });

        // let mut plugin_client = SystemCapture::new(callback);
        // plugin_client.start();
        // let audio_source = Some(Box::new(plugin_client) as Box<dyn AudioSource>);

        // let mut update_handle = Some(thread::spawn(move || loop {
        //     let temp = rx.recv().unwrap();
        //     println!("{:?}", temp[0].len());
        // }));

        Self {
            audio_source: None,
            raw_l,
            raw_r,
            tx: None,
            update_handle: None,
            db: 0.0,
            setting_switch: false,
            allways_on_top: false,
            setting: Default::default(),
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
            });
    }

    fn meters_ui(&mut self, ui: &mut egui::Ui, meters_rect: eframe::epaint::Rect) {
        let painter = ui.painter();
        painter.rect_filled(meters_rect, 0.0, Color32::from_black_alpha(200));

        let meters_response = ui.interact(meters_rect, Id::new("meters_click"), Sense::click());

        if meters_response.is_pointer_button_down_on() {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
            ui.ctx().send_viewport_cmd(ViewportCommand::MaxInnerSize(
                ui.ctx().input(|i| i.viewport().monitor_size.unwrap()),
            ))
        } else if meters_response.contains_pointer() {
            ui.label("");
            ui.horizontal(|ui| {
                ui.label("   ");
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
            });
        }
    }

    fn setting_ui(&mut self, ui: &mut egui::Ui, setting_rect: eframe::epaint::Rect) {
        let setting_layout = Layout::centered_and_justified(Direction::TopDown);
        let mut setting_area_ui = ui.child_ui(setting_rect, setting_layout);

        setting_area_ui.vertical_centered_justified(|ui| {
            ui.separator();
            // First row
            ui.horizontal_top(|ui| {
                // Waveform
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Waveform");
                        ui.horizontal(|ui| {
                            ui.label("Channel 1");
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_1,
                                Channel::None,
                                "None",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_1,
                                Channel::Left,
                                "Left",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_1,
                                Channel::Right,
                                "Right",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_1,
                                Channel::Mid,
                                "Mid",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_1,
                                Channel::Side,
                                "Side",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Channel 2");
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_2,
                                Channel::None,
                                "None",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_2,
                                Channel::Left,
                                "Left",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_2,
                                Channel::Right,
                                "Right",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_2,
                                Channel::Mid,
                                "Mid",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.channel_2,
                                Channel::Side,
                                "Side",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Mode");
                            ui.selectable_value(
                                &mut self.setting.waveform.mode,
                                setting::WaveformMode::Static,
                                "Static",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.mode,
                                setting::WaveformMode::MultiBand,
                                "MultiBand",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Peak History");
                            ui.selectable_value(
                                &mut self.setting.waveform.peak_history,
                                setting::WaveformHistory::Off,
                                "Off",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.peak_history,
                                setting::WaveformHistory::Fast,
                                "Fast",
                            );
                            ui.selectable_value(
                                &mut self.setting.waveform.peak_history,
                                setting::WaveformHistory::Slow,
                                "Slow",
                            );
                        });
                    });
                });
                // Spectrogram
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Spectrogram");
                        ui.horizontal(|ui| {
                            ui.label("Orientation");
                            ui.selectable_value(
                                &mut self.setting.spectrogram.orientation,
                                setting::Orientation::H,
                                "Horizontal",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrogram.orientation,
                                setting::Orientation::V,
                                "Vertical",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Mode");
                            ui.selectable_value(
                                &mut self.setting.spectrogram.mode,
                                setting::SpectrogramMode::Sharp,
                                "Sharp",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrogram.mode,
                                setting::SpectrogramMode::Classic,
                                "Classic",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Curve");
                            ui.selectable_value(
                                &mut self.setting.spectrogram.curve,
                                setting::SpectrogramCurve::Linear,
                                "Linear",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrogram.curve,
                                setting::SpectrogramCurve::Logarithmic,
                                "Logarithmic",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Brightness Boost");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.setting.spectrogram.brightness_boost,
                                    0.0..=1.0,
                                )
                                .text(""),
                            );
                        });
                    });
                });
                // Oscilloscope
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Oscilloscope");
                        ui.horizontal(|ui| {
                            ui.label("Follow Pitch");
                            ui.selectable_value(
                                &mut self.setting.oscilloscope.follow_pitch,
                                true,
                                "On",
                            );
                            ui.selectable_value(
                                &mut self.setting.oscilloscope.follow_pitch,
                                false,
                                "Off",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Cycle");
                            ui.selectable_value(
                                &mut self.setting.oscilloscope.cycle,
                                setting::OscilloscopeCycle::Multi,
                                "Multi",
                            );
                            ui.selectable_value(
                                &mut self.setting.oscilloscope.cycle,
                                setting::OscilloscopeCycle::Single,
                                "Single",
                            );
                        });
                    });
                });
                // Audio Device
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Audio Device");
                        ui.selectable_value(
                            &mut self.setting.audio_device.device,
                            setting::AudioDevice::OutputCapture,
                            "System Output",
                        );
                        ui.selectable_value(
                            &mut self.setting.audio_device.device,
                            setting::AudioDevice::PluginCapture,
                            "Plugin Capture",
                        );
                        ui.selectable_value(
                            &mut self.setting.audio_device.device,
                            setting::AudioDevice::InputCapture,
                            "System Input",
                        );
                    });
                })
            });
            // Second row
            ui.horizontal_wrapped(|ui| {
                // Modules
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Enable Module");
                        ui.checkbox(&mut self.setting.module.spectrogram, "Spectrogram");
                        ui.checkbox(&mut self.setting.module.waveform, "Waveform");
                        ui.checkbox(&mut self.setting.module.peak, "Peak/LUFS");
                        ui.checkbox(&mut self.setting.module.stereogram, "Stereogram");
                        ui.checkbox(&mut self.setting.module.oscilloscope, "Oscilloscope");
                        ui.checkbox(&mut self.setting.module.spectrum, "Spectrum");
                    });
                });
                // Stereogram
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Sterogram");
                        ui.horizontal(|ui| {
                            ui.label("Mode");
                            ui.selectable_value(
                                &mut self.setting.stereogram.mode,
                                setting::StereogramMode::Logarithmic,
                                "Logarithmic",
                            );
                            ui.selectable_value(
                                &mut self.setting.stereogram.mode,
                                setting::StereogramMode::Linear,
                                "Linear",
                            );
                            ui.selectable_value(
                                &mut self.setting.stereogram.mode,
                                setting::StereogramMode::Lissajous,
                                "Lissajous",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Color");
                            ui.selectable_value(
                                &mut self.setting.stereogram.color,
                                setting::StereogramColor::Static,
                                "Static",
                            );
                            ui.selectable_value(
                                &mut self.setting.stereogram.color,
                                setting::StereogramColor::RGB,
                                "RGB",
                            );
                            ui.selectable_value(
                                &mut self.setting.stereogram.color,
                                setting::StereogramColor::MultiBand,
                                "MultiBand",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Polarity");
                            ui.selectable_value(
                                &mut self.setting.stereogram.polarity,
                                setting::StereogramPolarity::Uni,
                                "Uniploar",
                            );
                            ui.selectable_value(
                                &mut self.setting.stereogram.polarity,
                                setting::StereogramPolarity::Bi,
                                "Biploar",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Normalize");
                            ui.selectable_value(
                                &mut self.setting.stereogram.normalize,
                                false,
                                "Off",
                            );
                            ui.selectable_value(&mut self.setting.stereogram.normalize, true, "On");
                        });
                        ui.horizontal(|ui| {
                            ui.label("Guides");
                            ui.selectable_value(&mut self.setting.stereogram.guides, false, "Off");
                            ui.selectable_value(&mut self.setting.stereogram.guides, true, "On");
                        });
                    });
                });
                // Spectrum
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Spectrum");
                        ui.horizontal(|ui| {
                            ui.label("Mode");
                            ui.selectable_value(
                                &mut self.setting.spectrum.mode,
                                setting::SpectrumMode::FFT,
                                "FFT",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.mode,
                                setting::SpectrumMode::ColorBar,
                                "ColorBar",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.mode,
                                setting::SpectrumMode::Both,
                                "Both",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Smoothing");
                            ui.add(
                                egui::Slider::new(&mut self.setting.spectrum.smoothing, 0.0..=1.25)
                                    .text(""),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Slope");
                            ui.add(
                                egui::Slider::new(&mut self.setting.spectrum.slope, -9.0..=9.0)
                                    .text("dB"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Channel");
                            ui.selectable_value(
                                &mut self.setting.spectrum.channel,
                                setting::SpectrumChannel::LR,
                                "L/R",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.channel,
                                setting::SpectrumChannel::MS,
                                "Mid/Side",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Low");
                            ui.add(
                                egui::Slider::new(&mut self.setting.spectrum.low, -150.0..=-20.0)
                                    .text("dB"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("High");
                            ui.add(
                                egui::Slider::new(&mut self.setting.spectrum.high, -50.0..=20.0)
                                    .text("dB"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Freq Readout");
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_readout,
                                setting::SpectrumFreqReadout::Off,
                                "Off",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_readout,
                                setting::SpectrumFreqReadout::Dyn,
                                "Dyn",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_readout,
                                setting::SpectrumFreqReadout::Static,
                                "Static",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Freq Line");
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_line,
                                setting::SpectrumFreqLine::Off,
                                "Off",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_line,
                                setting::SpectrumFreqLine::On,
                                "On",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_line,
                                setting::SpectrumFreqLine::Bright,
                                "Bright",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Ref Line");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.setting.spectrum.ref_line,
                                    0.0..=22000.0,
                                )
                                .text("Hz"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Threshold");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.setting.spectrum.threshold,
                                    -150.0..=0.0,
                                )
                                .text("dB"),
                            );
                        });
                    });
                });
                // Close button
                if ui.button("CLOSE SETTING").clicked() && self.setting_switch {
                    self.setting_switch = false;
                    let new_size = [setting_rect.max.x, setting_rect.max.y - 400.0];
                    ui.ctx()
                        .send_viewport_cmd(ViewportCommand::InnerSize(new_size.into()));
                    ui.ctx()
                        .send_viewport_cmd(ViewportCommand::MinInnerSize([800.0, 100.0].into()));
                }
            });
        });
    }
}
