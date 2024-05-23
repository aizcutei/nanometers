use crate::NanometersApp;
use egui::*;

impl NanometersApp {
    pub fn oscilloscope_frame(&mut self, rect: eframe::epaint::Rect, ui: &mut Ui) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
    }
}
