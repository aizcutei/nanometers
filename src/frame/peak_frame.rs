use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;

impl NanometersApp {
    pub fn peak_frame(&mut self, iir_data: &IIRData, db_data: &DBData, rect: Rect, ui: &mut Ui) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        if iir_data.l.is_empty() || iir_data.r.is_empty() {
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
            if db_data.l > self.peak.l || self.peak.l.is_nan() {
                self.peak.l = db_data.l;
            } else {
                self.peak.l = self.peak.l * self.setting.peak.decay
                    + (1.0 - self.setting.peak.decay) * db_data.l;
            }

            if db_data.r > self.peak.r || self.peak.r.is_nan() {
                self.peak.r = db_data.r;
            } else {
                self.peak.r = self.peak.r * self.setting.peak.decay
                    + (1.0 - self.setting.peak.decay) * db_data.r;
            }

            let l_height = -rect.height() * (db_data.l / 60.0);
            let r_height = -rect.height() * (db_data.r / 60.0);
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

        // LUFS
        if !iir_data.l.is_empty() && !iir_data.r.is_empty() {
            let len = iir_data.l.len();
            for i in 0..len {
                self.peak.data_buffer_l.push_back(iir_data.l[i]);
                self.peak.data_buffer_r.push_back(iir_data.r[i]);
                if self.peak.data_buffer_l.len() >= 4 {
                    let sigma = (self.peak.data_buffer_l.iter().sum::<f32>()
                        + self.peak.data_buffer_r.iter().sum::<f32>())
                        / 19200.0;
                    if sigma.log10() * 10.0 - 0.691 > -70.0 {
                        self.peak.past_3s.push_back(sigma);
                    } else {
                        self.peak.past_3s.push_back(0.0);
                    }
                    self.peak.past_3s.pop_front();
                    self.peak.data_buffer_l.pop_front();
                    self.peak.data_buffer_r.pop_front();
                }
            }
        }
        self.peak.lufs = self
            .peak
            .past_3s
            .clone()
            .into_iter()
            .filter(|x| *x != 0.0)
            .sum::<f32>()
            .log10()
            * 10.0
            - 10.691;
        // println!("{}", self.peak.lufs);
        // ui.painter().text(
        //     rect.center(),
        //     Align2::CENTER_CENTER,
        //     format!("{}", self.peak.lufs),
        //     FontId::proportional(20.0),
        //     Color32::WHITE,
        // );
    }
}
