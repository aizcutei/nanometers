// use crate::setting::*;
use crate::NanometersApp;
use egui::*;

impl NanometersApp {
    pub fn spectrogram_frame(&mut self, rect: eframe::epaint::Rect, ui: &mut Ui) {
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, Color32::RED);
    }

    pub fn spectrum_frame(&mut self, rect: eframe::epaint::Rect, ui: &mut Ui) {
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, Color32::KHAKI);
    }
}
