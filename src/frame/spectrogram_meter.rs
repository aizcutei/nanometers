use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
use emath::TSTransform;
use piet_common::*;
use std::collections::VecDeque;

impl NanometersApp {
    pub fn spectrogram_meter(
        &mut self,
        data: Vec<SpectrogramSendFrame>,
        rect: eframe::epaint::Rect,
        ui: &mut Ui,
    ) {
        let mut ppp = ui.ctx().pixels_per_point();
        // let mut ppp = 2.0;
        let frame_width = 14.0;
        let frame_interval = 3.0;

        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);

        if !data.is_empty() {
            match self.setting.spectrogram.mode {
                SpectrogramMode::Classic => match self.setting.spectrogram.orientation {
                    SpectrogramOrientation::H => {}
                    SpectrogramOrientation::V => {}
                },
                SpectrogramMode::Sharp => match self.setting.spectrogram.orientation {
                    SpectrogramOrientation::H => {
                        let len = data.len();
                        let mut piet = Device::new().unwrap();
                        let mut piet_img_full = piet
                            .bitmap_target(
                                (rect.width() * ppp) as usize,
                                (rect.height() * ppp) as usize,
                                1.0,
                            )
                            .unwrap();
                        {
                            let mut piet_img_ctx = piet_img_full.render_context();
                            // Draw last image
                            if self.spectrogram.last_img.is_some() {
                                piet_img_ctx.draw_image_area(
                                    &self.spectrogram.last_img.as_ref().unwrap(),
                                    piet_common::kurbo::Rect::new(
                                        len as f64 * frame_interval * ppp as f64,
                                        0.0,
                                        (rect.width() * ppp) as f64,
                                        (rect.height() * ppp) as f64,
                                    ),
                                    piet_common::kurbo::Rect::new(
                                        0.0,
                                        0.0,
                                        (rect.width() * ppp) as f64
                                            - len as f64 * frame_interval * ppp as f64,
                                        (rect.height() * ppp) as f64,
                                    ),
                                    piet_common::InterpolationMode::NearestNeighbor,
                                );
                            }
                            // Draw new image
                            for i in 0..len {
                                for j in 0..data[i].f.len() {
                                    let x = (rect.width() as f64
                                        - (len - 1 - i) as f64 * frame_interval
                                        - frame_width
                                        + data[i].t[j] as f64 * frame_width)
                                        * ppp as f64;
                                    let y = (rect.height() * (1.0 - data[i].f[j]) * ppp) as f64;
                                    let draw_circle = piet_common::kurbo::Circle::new(
                                        piet_common::kurbo::Point::new(x, y),
                                        ppp as f64,
                                    );
                                    // let draw_rect = piet_common::kurbo::Rect::new(
                                    //     x - ppp as f64,
                                    //     y - ppp as f64,
                                    //     x + ppp as f64,
                                    //     y + ppp as f64,
                                    // );
                                    piet_img_ctx.fill(
                                        draw_circle,
                                        &piet_common::Color::rgba8(
                                            self.setting.theme.main.r(),
                                            self.setting.theme.main.g(),
                                            self.setting.theme.main.b(),
                                            data[i].i[j],
                                        ),
                                    );
                                }
                            }

                            self.spectrogram.last_img = match piet_img_ctx.capture_image_area(
                                piet_common::kurbo::Rect::new(
                                    0.0,
                                    0.0,
                                    (rect.width() * ppp) as f64,
                                    (rect.height() * ppp) as f64,
                                ),
                            ) {
                                Ok(img) => Some(img),
                                Err(_) => None,
                            };
                        }

                        let pixels = piet_img_full.to_image_buf(ImageFormat::RgbaPremul).unwrap();
                        let size = pixels.size();
                        let egui_image = ColorImage::from_rgba_premultiplied(
                            [size.width as usize, size.height as usize],
                            pixels.raw_pixels(),
                        );
                        let texture =
                            ui.ctx()
                                .load_texture("spectrogram", egui_image, Default::default());
                        self.spectrogram.texture = Some(texture);
                    }
                    SpectrogramOrientation::V => {
                        // if rect.width() <= 400.0 {
                        //     ppp = 2.0;
                        // }
                        let len = data.len();
                        let mut piet = Device::new().unwrap();
                        let mut piet_img_full = piet
                            .bitmap_target(
                                (rect.width() * ppp) as usize,
                                (rect.height() * ppp) as usize,
                                1.0,
                            )
                            .unwrap();
                        {
                            let mut piet_img_ctx = piet_img_full.render_context();
                            // Draw last image
                            if self.spectrogram.last_img.is_some() {
                                piet_img_ctx.draw_image_area(
                                    &self.spectrogram.last_img.as_ref().unwrap(),
                                    piet_common::kurbo::Rect::new(
                                        0.0,
                                        len as f64 * frame_interval * ppp as f64,
                                        (rect.width() * ppp) as f64,
                                        (rect.height() * ppp) as f64,
                                    ),
                                    piet_common::kurbo::Rect::new(
                                        0.0,
                                        0.0,
                                        (rect.width() * ppp) as f64,
                                        (rect.height() * ppp) as f64
                                            - len as f64 * frame_interval * ppp as f64,
                                    ),
                                    piet_common::InterpolationMode::NearestNeighbor,
                                );
                            }
                            // Draw new image
                            for i in 0..len {
                                for j in 0..data[i].f.len() {
                                    let y = (rect.height() as f64
                                        - (len - 1 - i) as f64 * frame_interval
                                        - frame_width
                                        + data[i].t[j] as f64 * frame_width)
                                        * ppp as f64;
                                    let x = (rect.width() * data[i].f[j] * ppp) as f64;
                                    let draw_circle = piet_common::kurbo::Circle::new(
                                        piet_common::kurbo::Point::new(x, y),
                                        ppp as f64,
                                    );
                                    // let draw_rect = piet_common::kurbo::Rect::new(
                                    //     x - ppp as f64,
                                    //     y - ppp as f64,
                                    //     x + ppp as f64,
                                    //     y + ppp as f64,
                                    // );
                                    piet_img_ctx.fill(
                                        draw_circle,
                                        &piet_common::Color::rgba8(
                                            self.setting.theme.main.r(),
                                            self.setting.theme.main.g(),
                                            self.setting.theme.main.b(),
                                            data[i].i[j],
                                        ),
                                    );
                                }
                            }

                            self.spectrogram.last_img = match piet_img_ctx.capture_image_area(
                                piet_common::kurbo::Rect::new(
                                    0.0,
                                    0.0,
                                    (rect.width() * ppp) as f64,
                                    (rect.height() * ppp) as f64,
                                ),
                            ) {
                                Ok(img) => Some(img),
                                Err(_) => None,
                            };
                        }

                        let pixels = piet_img_full.to_image_buf(ImageFormat::RgbaPremul).unwrap();
                        let size = pixels.size();
                        let egui_image = ColorImage::from_rgba_premultiplied(
                            [size.width as usize, size.height as usize],
                            pixels.raw_pixels(),
                        );
                        let texture =
                            ui.ctx()
                                .load_texture("spectrogram", egui_image, Default::default());
                        self.spectrogram.texture = Some(texture);
                    }
                },
            }
        }

        // Draw Texture
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
