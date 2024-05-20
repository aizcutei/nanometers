use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
use rayon::prelude::*;

impl NanometersApp {
    pub fn waveform_frame(&mut self, data: &WaveformSendData, rect: Rect, ui: &mut Ui) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        let upper_rect = Rect::from_two_pos(rect.min, pos2(rect.max.x, rect.center().y));
        let lower_rect = Rect::from_two_pos(pos2(rect.min.x, rect.center().y), rect.max);

        match self.setting.waveform.channel_1 {
            WaveformChannel::None => {}
            WaveformChannel::Left => {
                self.waveform_upper_channel_frame(&data.l, &data.l_freq, upper_rect, ui);
            }
            WaveformChannel::Right => {
                self.waveform_upper_channel_frame(&data.r, &data.r_freq, upper_rect, ui);
            }
            WaveformChannel::Mid => {
                self.waveform_upper_channel_frame(&data.m, &data.m_freq, upper_rect, ui);
            }
            WaveformChannel::Side => {
                self.waveform_upper_channel_frame(&data.s, &data.s_freq, upper_rect, ui);
            }
        }

        match self.setting.waveform.channel_2 {
            WaveformChannel::None => {}
            WaveformChannel::Left => {
                self.waveform_lower_channel_frame(&data.l, &data.l_freq, lower_rect, ui);
            }
            WaveformChannel::Right => {
                self.waveform_lower_channel_frame(&data.r, &data.r_freq, lower_rect, ui);
            }
            WaveformChannel::Mid => {
                self.waveform_lower_channel_frame(&data.m, &data.m_freq, lower_rect, ui);
            }
            WaveformChannel::Side => {
                self.waveform_lower_channel_frame(&data.s, &data.s_freq, lower_rect, ui);
            }
        }
    }

    fn waveform_upper_channel_frame(
        &mut self,
        data: &[MAXMIN],
        color: &[usize],
        rect: Rect,
        ui: &mut Ui,
    ) {
        if !data.is_empty() {
            data.iter().zip(color).for_each(|(v, c)| {
                if self.waveform.plot_point.uu.len() >= self.waveform.history_length {
                    self.waveform.plot_point.uu.pop_front();
                    self.waveform.plot_point.ud.pop_front();
                    self.waveform.plot_point.ucolor.pop_front();
                }
                self.waveform
                    .plot_point
                    .uu
                    .push_back(rect.center().y - rect.height() * v.max / 2.0);
                self.waveform
                    .plot_point
                    .ud
                    .push_back(rect.center().y - rect.height() * v.min / 2.0);
                self.waveform
                    .plot_point
                    .ucolor
                    .push_back(self.color_lut_129[*c]);
            });
        }
        let len = self.waveform.plot_point.uu.len();
        let shapes: Vec<epaint::Shape> = (0..rect.width() as usize)
            .into_iter()
            .map(|i| {
                epaint::Shape::vline(
                    rect.max.x - i as f32,
                    Rangef::new(
                        *self.waveform.plot_point.uu.get(len - i).unwrap_or(&0.0),
                        *self.waveform.plot_point.ud.get(len - i).unwrap_or(&0.0),
                    ),
                    match self.setting.waveform.mode {
                        WaveformMode::Static => Stroke::new(1.0, self.setting.theme.main),
                        WaveformMode::MultiBand => Stroke::new(
                            1.1,
                            self.waveform
                                .plot_point
                                .ucolor
                                .get(len - i)
                                .unwrap_or(&self.setting.theme.main)
                                .clone(),
                        ),
                    },
                )
            })
            .collect();
        ui.painter().extend(shapes);
    }

    fn waveform_lower_channel_frame(
        &mut self,
        data: &[MAXMIN],
        color: &[usize],
        rect: Rect,
        ui: &mut Ui,
    ) {
        if !data.is_empty() {
            data.iter().zip(color).for_each(|(v, c)| {
                if self.waveform.plot_point.du.len() >= self.waveform.history_length {
                    self.waveform.plot_point.du.pop_front();
                    self.waveform.plot_point.dd.pop_front();
                    self.waveform.plot_point.dcolor.pop_front();
                }
                self.waveform
                    .plot_point
                    .du
                    .push_back(rect.center().y - rect.height() * v.max / 2.0);
                self.waveform
                    .plot_point
                    .dd
                    .push_back(rect.center().y - rect.height() * v.min / 2.0);
                self.waveform
                    .plot_point
                    .dcolor
                    .push_back(self.color_lut_129[*c]);
            });
        }
        let len = self.waveform.plot_point.du.len();
        let shapes: Vec<epaint::Shape> = (0..rect.width() as usize)
            .into_iter()
            .map(|i| {
                epaint::Shape::vline(
                    rect.max.x - i as f32,
                    Rangef::new(
                        *self.waveform.plot_point.du.get(len - i).unwrap_or(&0.0),
                        *self.waveform.plot_point.dd.get(len - i).unwrap_or(&0.0),
                    ),
                    match self.setting.waveform.mode {
                        WaveformMode::Static => Stroke::new(1.0, self.setting.theme.main),
                        WaveformMode::MultiBand => Stroke::new(
                            1.1,
                            self.waveform
                                .plot_point
                                .dcolor
                                .get(len - i)
                                .unwrap_or(&self.setting.theme.main)
                                .clone(),
                        ),
                    },
                )
            })
            .collect();
        ui.painter().extend(shapes);
    }
}
