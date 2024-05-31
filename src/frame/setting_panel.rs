use crate::audio::*;
use crate::setting::{self, set_theme};
use crate::utils::*;
use crate::NanometersApp;
use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    col: usize,
    row: usize,
}

impl NanometersApp {
    pub fn waveform_setting_block(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Waveform");
                ui.horizontal(|ui| {
                    ui.label("Channel 1");
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::None,
                        "None",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::Left,
                        "Left",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::Right,
                        "Right",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::Mid,
                        "Mid",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::Side,
                        "Side",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Channel 2");
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::None,
                        "None",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::Left,
                        "Left",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::Right,
                        "Right",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::Mid,
                        "Mid",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::Side,
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
    }

    pub fn spectrogram_setting_block(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Spectrogram");
                ui.horizontal(|ui| {
                    ui.label("Orientation");
                    ui.selectable_value(
                        &mut self.setting.spectrogram.orientation,
                        setting::SpectrogramOrientation::H,
                        "Horizontal",
                    );
                    ui.selectable_value(
                        &mut self.setting.spectrogram.orientation,
                        setting::SpectrogramOrientation::V,
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
                            0.01..=1.0,
                        )
                        .text(""),
                    );
                });
            });
        });
    }

    pub fn oscilloscope_setting_block(&mut self, ui: &mut Ui) {
        // Oscilloscope
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Oscilloscope");
                ui.horizontal(|ui| {
                    ui.label("Follow Pitch");
                    ui.selectable_value(&mut self.setting.oscilloscope.follow_pitch, true, "On");
                    ui.selectable_value(&mut self.setting.oscilloscope.follow_pitch, false, "Off");
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
    }

    pub fn device_setting_block(&mut self, ui: &mut Ui) {
        // Audio Device
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Audio Device");
                if ui
                    .selectable_value(
                        &mut self.setting.audio_device.device,
                        setting::AudioDevice::OutputCapture,
                        "System Output",
                    )
                    .changed()
                {
                    self.audio_source.as_mut().unwrap().stop();
                    let callback = get_callback(
                        self.tx_data.clone().unwrap(),
                        self.rx_setting.clone().unwrap(),
                        self.audio_source_buffer.clone(),
                    );
                    let mut system_capture = SystemCapture::new(callback);
                    system_capture.start();
                    self.audio_source = Some(Box::new(system_capture) as Box<dyn AudioSource>);
                }
                if ui
                    .selectable_value(
                        &mut self.setting.audio_device.device,
                        setting::AudioDevice::PluginCapture,
                        "Plugin Capture",
                    )
                    .changed()
                {
                    self.audio_source.as_mut().unwrap().stop();

                    let callback = get_callback(
                        self.tx_data.clone().unwrap(),
                        self.rx_setting.clone().unwrap(),
                        self.audio_source_buffer.clone(),
                    );
                    let mut plugin_client = PluginClient::new(callback);
                    plugin_client.start();
                    self.audio_source = Some(Box::new(plugin_client) as Box<dyn AudioSource>);
                }
                if ui
                    .selectable_value(
                        &mut self.setting.audio_device.device,
                        setting::AudioDevice::InputCapture,
                        "System Input",
                    )
                    .changed()
                {
                    self.audio_source.as_mut().unwrap().stop();
                    self.audio_source = None;
                }
            });
        });
    }

    pub fn modules_sequence_block(&mut self, ui: &mut Ui) {
        // Sequence
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Modules Off/On");
                let mut from = None;
                let mut to = None;
                ui.columns(self.setting.sequence.len(), |uis| {
                    for (col_idx, column) in self.setting.sequence.clone().into_iter().enumerate() {
                        let ui = &mut uis[col_idx];
                        let frame = Frame::default();
                        let (_, dropped_payload) = ui.dnd_drop_zone::<Location, ()>(frame, |ui| {
                            ui.set_min_size(vec2(128.0, 100.0));
                            ui.painter().rect_filled(
                                ui.max_rect(),
                                0.0,
                                self.setting.theme.bgaccent,
                            );
                            for (row_idx, item) in column.iter().enumerate() {
                                let item_id = Id::new(("dnd", col_idx, row_idx));
                                let item_location = Location {
                                    col: col_idx,
                                    row: row_idx,
                                };
                                let response = ui
                                    .dnd_drag_source(item_id, item_location, |ui| {
                                        ui.label(item.to_string());
                                    })
                                    .response;

                                if let (Some(pointer), Some(hovered_payload)) = (
                                    ui.input(|i| i.pointer.interact_pos()),
                                    response.dnd_hover_payload::<Location>(),
                                ) {
                                    let rect = response.rect;
                                    let stroke = Stroke::new(1.0, Color32::WHITE);
                                    let insert_row_idx = if *hovered_payload == item_location {
                                        ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                                        row_idx
                                    } else if pointer.y < rect.center().y {
                                        ui.painter().hline(rect.x_range(), rect.top(), stroke);
                                        row_idx
                                    } else {
                                        ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                                        row_idx + 1
                                    };
                                    if let Some(dragged_payload) = response.dnd_release_payload() {
                                        // The user dropped onto this item.
                                        from = Some(dragged_payload);
                                        to = Some(Location {
                                            col: col_idx,
                                            row: insert_row_idx,
                                        });
                                    }
                                }
                            }
                        });
                        if let Some(dragged_payload) = dropped_payload {
                            // The user dropped onto the column, but not on any one item.
                            from = Some(dragged_payload);
                            to = Some(Location {
                                col: col_idx,
                                row: usize::MAX, // Inset last
                            });
                        }
                    }
                });
                if let (Some(from), Some(mut to)) = (from, to) {
                    if from.col == to.col {
                        // Dragging within the same column.
                        // Adjust row index if we are re-ordering:
                        to.row -= (from.row < to.row) as usize;
                    }

                    let item = self.setting.sequence[from.col].remove(from.row);

                    let column = &mut self.setting.sequence[to.col];
                    to.row = to.row.min(column.len());
                    column.insert(to.row, item);
                    self.tx_setting
                        .as_ref()
                        .unwrap()
                        .send(self.setting.clone())
                        .unwrap();
                }
            });
        });
    }

    pub fn vectorscope_settiing_block(&mut self, ui: &mut Ui) {
        // Vectorscope
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Vectorscope");
                ui.horizontal(|ui| {
                    ui.label("Mode");
                    ui.selectable_value(
                        &mut self.setting.vectorscope.mode,
                        setting::VectorscopeMode::Logarithmic,
                        "Logarithmic",
                    );
                    ui.selectable_value(
                        &mut self.setting.vectorscope.mode,
                        setting::VectorscopeMode::Linear,
                        "Linear",
                    );
                    ui.selectable_value(
                        &mut self.setting.vectorscope.mode,
                        setting::VectorscopeMode::Lissajous,
                        "Lissajous",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Color");
                    ui.selectable_value(
                        &mut self.setting.vectorscope.color,
                        setting::VectorscopeColor::Static,
                        "Static",
                    );
                    ui.selectable_value(
                        &mut self.setting.vectorscope.color,
                        setting::VectorscopeColor::RGB,
                        "RGB",
                    );
                    ui.selectable_value(
                        &mut self.setting.vectorscope.color,
                        setting::VectorscopeColor::MultiBand,
                        "MultiBand",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Polarity");
                    ui.selectable_value(
                        &mut self.setting.vectorscope.polarity,
                        setting::VectorscopePolarity::Uni,
                        "Uniploar",
                    );
                    ui.selectable_value(
                        &mut self.setting.vectorscope.polarity,
                        setting::VectorscopePolarity::Bi,
                        "Biploar",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Normalize");
                    ui.selectable_value(&mut self.setting.vectorscope.normalize, false, "Off");
                    ui.selectable_value(&mut self.setting.vectorscope.normalize, true, "On");
                });
                ui.horizontal(|ui| {
                    ui.label("Guides");
                    ui.selectable_value(&mut self.setting.vectorscope.guides, false, "Off");
                    ui.selectable_value(&mut self.setting.vectorscope.guides, true, "On");
                });
            });
        });
    }

    pub fn spectrum_setting_block(&mut self, ui: &mut Ui) {
        // Spectrum
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Spectrum");
                    ui.selectable_value(
                        &mut self.setting.spectrum.spectrum_switch,
                        setting::SpectrumSwitch::Main,
                        "MAIN",
                    );
                    ui.selectable_value(
                        &mut self.setting.spectrum.spectrum_switch,
                        setting::SpectrumSwitch::Audio,
                        "AUDIO",
                    );
                    ui.selectable_value(
                        &mut self.setting.spectrum.spectrum_switch,
                        setting::SpectrumSwitch::Ref,
                        "REF",
                    );
                });

                match self.setting.spectrum.spectrum_switch {
                    setting::SpectrumSwitch::Main => {
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
                    }
                    setting::SpectrumSwitch::Audio => {
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
                    }
                    setting::SpectrumSwitch::Ref => {
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
                    }
                }
            });
        });
    }

    pub fn theme_setting_block(&mut self, ui: &mut Ui) {
        // Theme
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Theme");
                ui.horizontal(|ui| {
                    if ui
                        .selectable_value(&mut self.setting.theme, setting::DARK_THEME, "Dark")
                        .changed()
                    {
                        self.setting.theme = setting::DARK_THEME;
                        ui.ctx().set_visuals(set_theme(self));
                    };
                    if ui
                        .selectable_value(&mut self.setting.theme, setting::LIGHT_THEME, "Light")
                        .changed()
                    {
                        self.setting.theme = setting::LIGHT_THEME;
                        ui.ctx().set_visuals(set_theme(self));
                    };
                    if ui
                        .selectable_value(&mut self.setting.theme, setting::PINK_THEME, "Pink")
                        .changed()
                    {
                        self.setting.theme = setting::PINK_THEME;
                        ui.ctx().set_visuals(set_theme(self));
                    }
                });
            });
        });
    }

    pub fn cpu_setting_block(&mut self, ui: &mut Ui) {
        // CPU
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("CPU");
                ui.horizontal(|ui| {
                    ui.label("FPS");
                    ui.label(format!("{:.1}", self.frame_history.fps()));
                });
                ui.horizontal(|ui| {
                    ui.label("Mean Frame Time");
                    ui.label(format!(
                        "{:.1} ms",
                        self.frame_history.mean_frame_time() * 1000.0
                    ));
                });
            });
        });
    }
}
