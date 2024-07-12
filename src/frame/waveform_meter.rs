use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
// use rayon::prelude::*;
use rustfft::num_traits::WrappingSub;

impl NanometersApp {
    pub fn waveform_meter(&mut self, data: &WaveformSendData, rect: Rect, ui: &mut Ui) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        let upper_rect = Rect::from_two_pos(rect.min, pos2(rect.max.x, rect.center().y));
        let lower_rect = Rect::from_two_pos(pos2(rect.min.x, rect.center().y), rect.max);

        match self.setting.waveform.channel_1 {
            WaveformChannel::None => {}
            WaveformChannel::Left => {
                self.waveform_upper_channel(&data.l, upper_rect, ui);
            }
            WaveformChannel::Right => {
                self.waveform_upper_channel(&data.r, upper_rect, ui);
            }
            WaveformChannel::Mid => {
                self.waveform_upper_channel(&data.m, upper_rect, ui);
            }
            WaveformChannel::Side => {
                self.waveform_upper_channel(&data.s, upper_rect, ui);
            }
        }

        match self.setting.waveform.channel_2 {
            WaveformChannel::None => {}
            WaveformChannel::Left => {
                self.waveform_lower_channel(&data.l, lower_rect, ui);
            }
            WaveformChannel::Right => {
                self.waveform_lower_channel(&data.r, lower_rect, ui);
            }
            WaveformChannel::Mid => {
                self.waveform_lower_channel(&data.m, lower_rect, ui);
            }
            WaveformChannel::Side => {
                self.waveform_lower_channel(&data.s, lower_rect, ui);
            }
        }

        match self.setting.waveform.peak_history {
            WaveformHistory::Off => {}
            WaveformHistory::Fast => {
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(
                        self.setting.theme.bg.r(),
                        self.setting.theme.bg.g(),
                        self.setting.theme.bg.b(),
                        100,
                    ),
                );
                self.waveform_history_frame(&data.m, rect, ui)
            }
            WaveformHistory::Slow => {}
        }
    }

    fn waveform_upper_channel(&mut self, data: &[WaveformSendFrame], rect: Rect, ui: &mut Ui) {
        if !data.is_empty() {
            data.iter().for_each(|(v)| {
                if self.waveform.plot_point.uu.len() >= self.waveform.history_length {
                    self.waveform.plot_point.uu.pop_front();
                    self.waveform.plot_point.ud.pop_front();
                    self.waveform.plot_point.ucolor.pop_front();
                }
                self.waveform
                    .plot_point
                    .uu
                    .push_back(rect.center().y - rect.height() * v.value.max / 2.0);
                self.waveform
                    .plot_point
                    .ud
                    .push_back(rect.center().y - rect.height() * v.value.min / 2.0);
                self.waveform.plot_point.ucolor.push_back(v.color);
            });
        }
        let len = self.waveform.plot_point.uu.len();
        let shapes: Vec<epaint::Shape> = (0..rect.width() as usize)
            .into_iter()
            .map(|i| {
                epaint::Shape::vline(
                    rect.max.x - i as f32,
                    Rangef::new(
                        *self
                            .waveform
                            .plot_point
                            .uu
                            .get(len.wrapping_sub(i))
                            .unwrap_or(&0.0),
                        *self
                            .waveform
                            .plot_point
                            .ud
                            .get(len.wrapping_sub(i))
                            .unwrap_or(&0.0),
                    ),
                    match self.setting.waveform.mode {
                        WaveformMode::Static => Stroke::new(1.5, self.setting.theme.main),
                        WaveformMode::MultiBand => Stroke::new(
                            1.1,
                            self.waveform
                                .plot_point
                                .ucolor
                                .get(len.wrapping_sub(i))
                                .unwrap_or(&self.setting.theme.main)
                                .clone(),
                        ),
                    },
                )
            })
            .collect();
        ui.painter().extend(shapes);
    }

    fn waveform_lower_channel(&mut self, data: &[WaveformSendFrame], rect: Rect, ui: &mut Ui) {
        if !data.is_empty() {
            data.iter().for_each(|(v)| {
                if self.waveform.plot_point.du.len() >= self.waveform.history_length {
                    self.waveform.plot_point.du.pop_front();
                    self.waveform.plot_point.dd.pop_front();
                    self.waveform.plot_point.dcolor.pop_front();
                }
                self.waveform
                    .plot_point
                    .du
                    .push_back(rect.center().y - rect.height() * v.value.max / 2.0);
                self.waveform
                    .plot_point
                    .dd
                    .push_back(rect.center().y - rect.height() * v.value.min / 2.0);
                self.waveform.plot_point.dcolor.push_back(v.color);
            });
        }
        let len = self.waveform.plot_point.du.len();
        let shapes: Vec<epaint::Shape> = (0..rect.width() as usize)
            .into_iter()
            .map(|i| {
                epaint::Shape::vline(
                    rect.max.x - i as f32,
                    Rangef::new(
                        *self
                            .waveform
                            .plot_point
                            .du
                            .get(len.wrapping_sub(i))
                            .unwrap_or(&0.0),
                        *self
                            .waveform
                            .plot_point
                            .dd
                            .get(len.wrapping_sub(i))
                            .unwrap_or(&0.0),
                    ),
                    match self.setting.waveform.mode {
                        WaveformMode::Static => Stroke::new(1.5, self.setting.theme.main),
                        WaveformMode::MultiBand => Stroke::new(
                            1.1,
                            self.waveform
                                .plot_point
                                .dcolor
                                .get(len.wrapping_sub(i))
                                .unwrap_or(&self.setting.theme.main)
                                .clone(),
                        ),
                    },
                )
            })
            .collect();
        ui.painter().extend(shapes);
    }

    fn waveform_history_frame(&mut self, data: &[WaveformSendFrame], rect: Rect, ui: &mut Ui) {
        if !data.is_empty() {
            data.iter().for_each(|(v)| {
                if self.waveform.plot_point.r.len() >= self.waveform.history_length {
                    self.waveform.plot_point.r.pop_front();
                    self.waveform.plot_point.g.pop_front();
                    self.waveform.plot_point.b.pop_front();
                }

                self.waveform
                    .plot_point
                    .r
                    .push_back(rect.height() * (1.0 - v.color.r() as f32 / 255.0));
                self.waveform
                    .plot_point
                    .g
                    .push_back(rect.height() * (1.0 - v.color.g() as f32 / 255.0));
                self.waveform
                    .plot_point
                    .b
                    .push_back(rect.height() * (1.0 - v.color.b() as f32 / 255.0));
            });
        }
        let len = self.waveform.plot_point.r.len();
        let mut r_points = Vec::new();
        let mut g_points = Vec::new();
        let mut b_points = Vec::new();
        for i in 0..rect.width() as usize {
            r_points.push(pos2(
                rect.max.x - i as f32,
                *self
                    .waveform
                    .plot_point
                    .r
                    .get(len.wrapping_sub(i))
                    .unwrap_or(&0.0),
            ));
            g_points.push(pos2(
                rect.max.x - i as f32,
                *self
                    .waveform
                    .plot_point
                    .g
                    .get(len.wrapping_sub(i))
                    .unwrap_or(&0.0),
            ));
            b_points.push(pos2(
                rect.max.x - i as f32,
                *self
                    .waveform
                    .plot_point
                    .b
                    .get(len.wrapping_sub(i))
                    .unwrap_or(&0.0),
            ));
        }

        let r_line = Shape::line(r_points, Stroke::new(1.0, Color32::RED));
        let g_line = Shape::line(g_points, Stroke::new(1.0, Color32::GREEN));
        let b_line = Shape::line(b_points, Stroke::new(1.0, Color32::BLUE));
        ui.painter().extend(vec![r_line, g_line, b_line]);
    }
}
