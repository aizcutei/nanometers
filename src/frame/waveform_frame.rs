use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
use rayon::prelude::*;

impl NanometersApp {
    pub fn waveform_frame(&mut self, mut data: RawData, rect: eframe::epaint::Rect, ui: &mut Ui) {
        let updata_speed = 280;
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);

        let last_data = self.waveform.data_buffer.clone();
        data.concat_front(last_data);
        let rest = data.l.len() % updata_speed;
        let len = data.l.len() / updata_speed;
        self.waveform.data_buffer = data.split_index(data.l.len() - rest);
        let upper_rect = Rect::from_two_pos(rect.min, pos2(rect.max.x, rect.center().y));
        let lower_rect = Rect::from_two_pos(pos2(rect.min.x, rect.center().y), rect.max);

        match self.setting.waveform.channel_1 {
            WaveformChannel::None => {}
            WaveformChannel::Left => {
                self.waveform_upper_channel_frame(&data.l, len, upper_rect, ui);
            }
            WaveformChannel::Right => {
                self.waveform_upper_channel_frame(&data.r, len, upper_rect, ui);
            }
            WaveformChannel::Mid => {
                self.waveform_upper_channel_frame(&data.m, len, upper_rect, ui);
            }
            WaveformChannel::Side => {
                self.waveform_upper_channel_frame(&data.s, len, upper_rect, ui);
            }
        }

        match self.setting.waveform.channel_2 {
            WaveformChannel::None => {}
            WaveformChannel::Left => {
                self.waveform_lower_channel_frame(&data.l, len, lower_rect, ui);
            }
            WaveformChannel::Right => {
                self.waveform_lower_channel_frame(&data.r, len, lower_rect, ui);
            }
            WaveformChannel::Mid => {
                self.waveform_lower_channel_frame(&data.m, len, lower_rect, ui);
            }
            WaveformChannel::Side => {
                self.waveform_lower_channel_frame(&data.s, len, lower_rect, ui);
            }
        }
    }

    fn waveform_upper_channel_frame(
        &mut self,
        data: &[f32],
        len: usize,
        rect: eframe::epaint::Rect,
        ui: &mut Ui,
    ) {
        for i in 0..len {
            let max = data
                .par_iter()
                .skip(i * self.waveform.update_speed)
                .take(self.waveform.update_speed)
                .max_by(|x, y| x.total_cmp(*y))
                .unwrap();
            let min = data
                .par_iter()
                .skip(i * self.waveform.update_speed)
                .take(self.waveform.update_speed)
                .min_by(|x, y| x.total_cmp(*y))
                .unwrap();
            self.waveform
                .plot_point
                .uu
                .push(rect.center().y - rect.height() * max / 2.0);
            self.waveform
                .plot_point
                .ud
                .push(rect.center().y - rect.height() * min / 2.0);
        }
        let shapes: Vec<epaint::Shape> = (0..rect.width() as usize)
            .into_iter()
            .map(|i| {
                epaint::Shape::vline(
                    rect.max.x - i as f32,
                    Rangef::new(
                        self.waveform.plot_point.uu.get(1920 - i),
                        self.waveform.plot_point.ud.get(1920 - i),
                    ),
                    match self.setting.waveform.mode {
                        WaveformMode::Static => Stroke::new(1.0, self.setting.theme.main),
                        WaveformMode::MultiBand => Stroke::new(1.0, self.setting.theme.main),
                    },
                )
            })
            .collect();
        ui.painter().extend(shapes);
    }

    fn waveform_lower_channel_frame(
        &mut self,
        data: &[f32],
        len: usize,
        rect: eframe::epaint::Rect,
        ui: &mut Ui,
    ) {
        for i in 0..len {
            let max = data
                .par_iter()
                .skip(i * self.waveform.update_speed)
                .take(self.waveform.update_speed)
                .max_by(|x, y| x.total_cmp(*y))
                .unwrap();
            let min = data
                .par_iter()
                .skip(i * self.waveform.update_speed)
                .take(self.waveform.update_speed)
                .min_by(|x, y| x.total_cmp(*y))
                .unwrap();
            self.waveform
                .plot_point
                .du
                .push(rect.center().y - rect.height() * max / 2.0);
            self.waveform
                .plot_point
                .dd
                .push(rect.center().y - rect.height() * min / 2.0);
        }
        let shapes: Vec<epaint::Shape> = (0..rect.width() as usize)
            .into_iter()
            .map(|i| {
                epaint::Shape::vline(
                    rect.max.x - i as f32,
                    Rangef::new(
                        self.waveform.plot_point.du.get(1920 - i),
                        self.waveform.plot_point.dd.get(1920 - i),
                    ),
                    match self.setting.waveform.mode {
                        WaveformMode::Static => Stroke::new(1.0, self.setting.theme.main),
                        WaveformMode::MultiBand => Stroke::new(1.0, self.setting.theme.main),
                    },
                )
            })
            .collect();
        ui.painter().extend(shapes);
    }
}
