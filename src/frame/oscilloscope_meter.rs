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
            ui.painter().extend(self.oscilloscope.plot.clone());
        } else {
            let len = data.len;
            let mut lines = vec![];
            let points: Vec<Pos2> = (0..len)
                .map(|i| {
                    let w = rect.width() / (len as f32 - 1.0);
                    let x = rect.min.x + w * i as f32;
                    let y = (1.0 - data.data.get(i).unwrap_or(&0.0)) * rect.center().y;
                    if self.setting.oscilloscope.shadow {
                        lines.push(Shape::line_segment(
                            [pos2(x, rect.center().y), pos2(x, y)],
                            Stroke::new(w, self.setting.theme.main.gamma_multiply(0.5)),
                        ));
                    }
                    pos2(x, y)
                })
                .collect();
            lines.push(Shape::line(
                points,
                Stroke::new(2.0, self.setting.theme.main),
            ));
            self.oscilloscope.plot = lines.clone();
            ui.painter().extend(lines);
        }
    }
}
