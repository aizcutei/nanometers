use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;

impl NanometersApp {
    pub fn oscilloscope_meter(
        &mut self,
        data: &OscilloscopeSendData,
        rect: eframe::epaint::Rect,
        ui: &mut Ui,
    ) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        if data.data.is_empty() {
            ui.painter().add(Shape::line(
                self.oscilloscope.plot.clone(),
                Stroke::new(2.0, self.setting.theme.main),
            ));
        } else {
            if self.setting.oscilloscope.follow_pitch {
            } else {
                let points: Vec<Pos2> = (0..2400)
                    .map(|i| {
                        let x = rect.min.x + rect.width() * i as f32 / 2400.0;
                        let y = (1.0
                            - data
                                .data
                                .get(data.data.len().saturating_sub(2400) + i)
                                .unwrap_or(&0.0))
                            * rect.center().y;
                        pos2(x, y)
                    })
                    .collect();
                self.oscilloscope.plot = points.clone();
                ui.painter().add(Shape::line(
                    points,
                    Stroke::new(2.0, self.setting.theme.main),
                ));
            }
        }
    }
}
