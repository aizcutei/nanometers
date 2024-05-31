use crate::NanometersApp;
use crate::{frame::*, setting::*, utils::*};
use egui::*;

impl NanometersApp {
    pub fn main_canvas(&mut self, ctx: &egui::Context) {
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
                resize_ui(ui, app_rect);
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
            ui.painter()
                .rect_filled(meters_rect, 0.0, Color32::from_black_alpha(200));
            ui.painter().text(
                meters_rect.center(),
                Align2::CENTER_CENTER,
                "Add a Meter",
                FontId::proportional(20.0),
                Color32::WHITE,
            );
        }

        let mut update_waveform_data = WaveformSendData::new();
        let mut update_stereogram_data = VectorscopeSendData::new();
        let mut update_iir_data = Vec::new();
        let mut update_db_data = DBData::new();
        let mut update_osc_data = OscilloscopeSendData::new();

        let mut update_spectrogram_image = Vec::new();

        self.rx_data.as_mut().unwrap().try_iter().for_each(|data| {
            update_iir_data.extend_from_slice(&data.iir);
            update_db_data.l = data.db.l;
            update_db_data.r = data.db.r;
            update_stereogram_data.max = data.vectorscope.max;
            update_stereogram_data
                .lissa
                .extend_from_slice(&data.vectorscope.lissa);
            update_stereogram_data
                .linear
                .extend_from_slice(&data.vectorscope.linear);
            update_stereogram_data
                .log
                .extend_from_slice(&data.vectorscope.log);
            update_waveform_data.concat(&data.waveform);
            update_osc_data = data.oscilloscope;

            update_spectrogram_image = data.spectrogram_image;
        });

        ui.ctx().request_repaint();

        for (i, meter) in self.setting.sequence[1].clone().iter().enumerate() {
            let mut meter_rect = self.meters_rects[i];
            match meter {
                ModuleList::Waveform => {
                    self.waveform_meter(&update_waveform_data, meter_rect, ui);
                }
                ModuleList::Spectrogram => {
                    self.spectrogram_meter(update_spectrogram_image.clone(), meter_rect, ui);
                }
                ModuleList::Peak => {
                    self.peak_meter(&update_iir_data, &update_db_data, meter_rect, ui);
                }
                ModuleList::Oscilloscope => {
                    self.oscilloscope_meter(&update_osc_data, meter_rect, ui);
                }
                ModuleList::Spectrum => {
                    self.spectrum_meter(meter_rect, ui);
                }
                ModuleList::Vectorscope => {
                    self.vectorscope_meter(&update_stereogram_data, meter_rect, ui);
                }
            }
        }

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
                    ui.painter()
                        .rect_filled(rect, 0.0, Color32::from_black_alpha(200));
                } else if rect_response.clone().dragged() {
                    let pointer_pos = ui.ctx().pointer_interact_pos();
                    self.meters_rects[i].max.x = pointer_pos.unwrap().x;
                    self.meters_rects[i + 1].min.x = pointer_pos.unwrap().x;
                    ui.painter()
                        .rect_filled(rect, 0.0, Color32::from_black_alpha(200));
                }
            }
        }

        let meters_response = ui.interact(meters_rect, Id::new("meters_buttons"), Sense::click());
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

        ui.painter()
            .rect_filled(setting_rect, 0.0, self.setting.theme.bg);

        setting_area_ui.vertical_centered_justified(|ui| {
            ui.separator();
            Grid::new("Setting_ui").show(ui, |ui| {
                self.modules_sequence_block(ui);
                self.waveform_setting_block(ui);
                self.vectorscope_settiing_block(ui);
                ui.end_row();

                self.spectrogram_setting_block(ui);
                self.spectrum_setting_block(ui);
                self.oscilloscope_setting_block(ui);
                ui.end_row();

                self.device_setting_block(ui);
                self.theme_setting_block(ui);
                self.cpu_setting_block(ui);
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
