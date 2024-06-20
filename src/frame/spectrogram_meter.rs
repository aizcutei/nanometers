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
        // let resolution = 2048;
        // let speed = 1;
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        // Calculate image
        match self.setting.spectrogram.mode {
            SpectrogramMode::Classic => match self.setting.spectrogram.orientation {
                SpectrogramOrientation::H => {}
                SpectrogramOrientation::V => {}
            },
            SpectrogramMode::Sharp => {
                match self.setting.spectrogram.orientation {
                    SpectrogramOrientation::H => {
                        // if !image.is_empty() {
                        //     let colorimage = ColorImage {
                        //         size: [resolution, (width / speed) as usize],
                        //         pixels: image[(1920 - (width / speed) as usize) * resolution
                        //             ..1920 * resolution]
                        //             .to_owned(),
                        //     };
                        //     let texture = ui.ctx().load_texture(
                        //         "spectrogram",
                        //         colorimage,
                        //         Default::default(),
                        //     );
                        //     self.spectrogram.texture = Some(texture);
                        // }
                        // if self.spectrogram.texture.is_some() {
                        //     Image::from_texture(self.spectrogram.texture.as_ref().unwrap())
                        //         .maintain_aspect_ratio(false)
                        //         .fit_to_exact_size(vec2(rect.height(), rect.width()))
                        //         .rotate(-90.0_f32.to_radians(), vec2(1.0, 0.0))
                        //         .paint_at(
                        //             ui,
                        //             Rect::from_min_size(
                        //                 pos2(rect.min.x - rect.height(), rect.min.y),
                        //                 vec2(rect.height(), rect.width()),
                        //             ),
                        //         );
                        // }
                    }
                    SpectrogramOrientation::V => {
                        // if !image.is_empty() {
                        //     let colorimage = ColorImage {
                        //         size: [resolution, (height / speed) as usize],
                        //         pixels: image[(1920 - (height / speed) as usize) * resolution
                        //             ..1920 * resolution]
                        //             .to_owned(),
                        //     };
                        //     let texture = ui.ctx().load_texture(
                        //         "spectrogram",
                        //         colorimage,
                        //         Default::default(),
                        //     );
                        //     self.spectrogram.texture = Some(texture);
                        // }
                        // if self.spectrogram.texture.is_some() {
                        //     ui.painter().image(
                        //         self.spectrogram.texture.as_ref().unwrap().id(),
                        //         rect,
                        //         Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                        //         Color32::WHITE,
                        //     );
                        // }
                    }
                };
            }
        }
    }
}
