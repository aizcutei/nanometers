use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;

impl NanometersApp {
    pub fn peak_frame(&mut self, iir_data: &Vec<f32>, db_data: &DBData, rect: Rect, ui: &mut Ui) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        // DB
        if !db_data.l.is_finite() {
            let l_rect = Rect::from_two_pos(
                pos2(rect.center().x - 40.0, self.peak.plot_l),
                pos2(rect.center().x - 34.0, rect.max.y),
            );
            let r_rect = Rect::from_two_pos(
                pos2(rect.center().x - 33.0, self.peak.plot_r),
                pos2(rect.center().x - 27.0, rect.max.y),
            );
            ui.painter()
                .rect_filled(l_rect, 0.0, self.setting.theme.main);
            ui.painter()
                .rect_filled(r_rect, 0.0, self.setting.theme.main);
        } else {
            if db_data.l > self.peak.l || self.peak.l.is_nan() {
                self.peak.l = db_data.l;
            } else {
                self.peak.l = self.peak.l * 0.99 + 0.01 * db_data.l;
            }

            if db_data.r > self.peak.r || self.peak.r.is_nan() {
                self.peak.r = db_data.r;
            } else {
                self.peak.r = self.peak.r * 0.99 + 0.01 * db_data.r;
            }

            let l_height = -rect.height() * (db_data.l / 60.0) - 5.0;
            let r_height = -rect.height() * (db_data.r / 60.0) - 5.0;
            self.peak.plot_l = l_height;
            self.peak.plot_r = r_height;

            let l_rect = Rect::from_two_pos(
                pos2(rect.center().x - 40.0, l_height),
                pos2(rect.center().x - 34.0, rect.max.y),
            );
            let r_rect = Rect::from_two_pos(
                pos2(rect.center().x - 33.0, r_height),
                pos2(rect.center().x - 27.0, rect.max.y),
            );
            ui.painter().hline(
                (rect.center().x - 40.0)..=(rect.center().x - 34.0),
                -rect.height() * (self.peak.l / 60.0) - 5.0,
                Stroke::new(1.0, self.setting.theme.main),
            );
            ui.painter().hline(
                (rect.center().x - 33.0)..=(rect.center().x - 27.0),
                -rect.height() * (self.peak.r / 60.0) - 5.0,
                Stroke::new(1.0, self.setting.theme.main),
            );
            ui.painter()
                .rect_filled(l_rect, 0.0, self.setting.theme.main);
            ui.painter()
                .rect_filled(r_rect, 0.0, self.setting.theme.main);
        }

        // LUFS
        if !iir_data.is_empty() {
            iir_data.iter().for_each(|&sample| {
                self.peak.data_buffer.push_back(sample);

                if self.peak.data_buffer.len() >= 4 {
                    let sigma: f32 = self.peak.data_buffer.iter().take(4).sum::<f32>() / 4.0;
                    let lkfs = sigma.log10() * 10.0 - 10.691;

                    self.peak
                        .past_1500ms
                        .push_back(if lkfs > -70.0 { sigma } else { 0.0 });

                    if self.peak.past_1500ms.len() > 12 {
                        self.peak.past_1500ms.pop_front();
                    }

                    self.peak.data_buffer.pop_front();
                }
            });
        }
        let (count, sum) = self
            .peak
            .past_1500ms
            .iter()
            .filter(|&&x| x != 0.0)
            .fold((0, 0.0), |(count, sum), &x| (count + 1, sum + x));
        if count != 0 {
            self.peak.lufs = (sum / count as f32).log10() * 10.0 - 0.691;
        } else {
            self.peak.lufs = f32::NEG_INFINITY;
        }
        if self.peak.lufs > -40.0 {
            let lufs_rect = Rect::from_two_pos(
                pos2(
                    rect.center().x - 22.0,
                    -rect.height() * (self.peak.lufs / 40.0),
                ),
                pos2(rect.center().x - 12.0, rect.max.y),
            );
            let text_rect = Rect::from_two_pos(
                pos2(
                    rect.center().x - 10.0,
                    10.0 - rect.height() * (self.peak.lufs / 40.0),
                ),
                pos2(
                    rect.center().x + 48.0,
                    -rect.height() * (self.peak.lufs / 40.0) - 10.0,
                ),
            );
            ui.painter()
                .rect_filled(lufs_rect, 0.0, self.setting.theme.main);
            ui.painter()
                .rect_filled(text_rect, 0.0, self.setting.theme.main);
            ui.painter().text(
                text_rect.center(),
                Align2::CENTER_CENTER,
                format!("{:.1}LU", self.peak.lufs),
                FontId::monospace(12.0),
                self.setting.theme.bg,
            );
        }
    }
}
