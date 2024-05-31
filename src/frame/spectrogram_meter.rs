use std::collections::VecDeque;

// use crate::setting::*;
use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
use emath::TSTransform;

impl NanometersApp {
    pub fn spectrogram_meter(
        &mut self,
        image: Vec<Color32>,
        rect: eframe::epaint::Rect,
        ui: &mut Ui,
    ) {
        // Image buffer
        let ppp = ui.ctx().pixels_per_point();
        let width = (rect.width()) as usize;
        let height = (rect.height()) as usize;

        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        // Calculate image
        match self.setting.spectrogram.mode {
            SpectrogramMode::Classic => match self.setting.spectrogram.curve {
                SpectrogramCurve::Linear => match self.setting.spectrogram.orientation {
                    SpectrogramOrientation::H => {}
                    SpectrogramOrientation::V => {}
                },
                SpectrogramCurve::Logarithmic => match self.setting.spectrogram.orientation {
                    SpectrogramOrientation::H => {}
                    SpectrogramOrientation::V => {}
                },
            },
            SpectrogramMode::Sharp => match self.setting.spectrogram.curve {
                SpectrogramCurve::Linear => match self.setting.spectrogram.orientation {
                    SpectrogramOrientation::H => {}
                    SpectrogramOrientation::V => {
                        if !image.is_empty() {
                            let colorimage = ColorImage {
                                size: [2048, height],
                                pixels: image[(3840 - height) * 2048..3840 * 2048].to_owned(),
                            };
                            let texture = ui.ctx().load_texture(
                                "spectrogram",
                                colorimage,
                                Default::default(),
                            );
                            self.spectrogram.texture = Some(texture);
                            ui.painter().image(
                                self.spectrogram.texture.as_ref().unwrap().id(),
                                rect,
                                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
                        } else {
                            if self.spectrogram.texture.is_some() {
                                ui.painter().image(
                                    self.spectrogram.texture.as_ref().unwrap().id(),
                                    rect,
                                    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                    Color32::WHITE,
                                );
                            }
                        }
                    }
                },
                SpectrogramCurve::Logarithmic => match self.setting.spectrogram.orientation {
                    SpectrogramOrientation::H => {}
                    SpectrogramOrientation::V => {}
                },
            },
        }
    }
}
