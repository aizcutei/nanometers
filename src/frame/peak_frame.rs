use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rustfft::num_traits::Pow;

impl NanometersApp {
    pub fn peak_frame(&mut self, data: &RawData, rect: Rect, ui: &mut Ui) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        if data.l.is_empty() || data.r.is_empty() {
            let l_rect = Rect::from_two_pos(
                pos2(rect.center().x - 12.0, self.peak.plot_l),
                pos2(rect.center().x - 7.0, rect.max.y),
            );
            let r_rect = Rect::from_two_pos(
                pos2(rect.center().x - 6.0, self.peak.plot_r),
                pos2(rect.center().x - 1.0, rect.max.y),
            );
            ui.painter()
                .rect_filled(l_rect, 0.0, self.setting.theme.main);
            ui.painter()
                .rect_filled(r_rect, 0.0, self.setting.theme.main);
        } else {
            let average_l = (data
                .l
                .par_iter()
                .fold(|| 0f32, |a, b| a + b.pow(2.0))
                .sum::<f32>()
                / data.l.len() as f32)
                .sqrt();
            let average_r = (data
                .r
                .par_iter()
                .fold(|| 0f32, |a, b| a + b.pow(2.0))
                .sum::<f32>()
                / data.r.len() as f32)
                .sqrt();

            if average_l > self.peak.l || self.peak.l.is_nan() {
                self.peak.l = average_l;
            } else {
                self.peak.l = self.peak.l * self.setting.peak.decay
                    + (1.0 - self.setting.peak.decay) * average_l;
            }

            if average_r > self.peak.r || self.peak.r.is_nan() {
                self.peak.r = average_r;
            } else {
                self.peak.r = self.peak.r * self.setting.peak.decay
                    + (1.0 - self.setting.peak.decay) * average_r;
            }
            let l_height = -rect.height() * (gain_to_db(self.peak.l) / 60.0);
            let r_height = -rect.height() * (gain_to_db(self.peak.r) / 60.0);
            self.peak.plot_l = l_height;
            self.peak.plot_r = r_height;

            let l_rect = Rect::from_two_pos(
                pos2(rect.center().x - 12.0, l_height),
                pos2(rect.center().x - 7.0, rect.max.y),
            );
            let r_rect = Rect::from_two_pos(
                pos2(rect.center().x - 6.0, r_height),
                pos2(rect.center().x - 1.0, rect.max.y),
            );
            ui.painter()
                .rect_filled(l_rect, 0.0, self.setting.theme.main);
            ui.painter()
                .rect_filled(r_rect, 0.0, self.setting.theme.main);
        }
    }
}
