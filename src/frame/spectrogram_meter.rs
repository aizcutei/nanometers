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
        data: &[SpectrogramFrame],
        image: Vec<Color32>,
        rect: eframe::epaint::Rect,
        ui: &mut Ui,
    ) {
        // Image buffer
        let ppp = ui.ctx().pixels_per_point();
        let width = (rect.width()) as usize;
        let height = (rect.height()) as usize;
        if self.spectrogram.texture_size != [width, height]
            || self.spectrogram.texture_raw.is_empty()
        {
            self.spectrogram.texture_size = [width, height];
            self.spectrogram.texture_raw = vec![Color32::TRANSPARENT; width * height].into();
        }
        // Prepare
        let fft_size = 1025;
        // let points_history = 96000;
        let speed = 5.0;
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
                    SpectrogramOrientation::H => {
                        // let block_height = rect.height() / fft_size as f32;
                        // let mut points = vec![];
                        // let mut shapes = vec![];
                        // // Move old points
                        // let ts = TSTransform::new([-(speed * data.len() as f32), 0.0].into(), 1.0);
                        // for i in 0..self.spectrogram.points.len() {
                        //     let pos = ts.mul_pos(self.spectrogram.points[i].pos);
                        //     self.spectrogram.points[i].pos = pos;
                        //     shapes.push(Shape::rect_filled(
                        //         Rect::from_points(&[pos, pos2(pos.x + 1.0, pos.y + 1.0)]),
                        //         0.0,
                        //         Color32::from_rgba_unmultiplied(
                        //             self.setting.theme.main.r(),
                        //             self.setting.theme.main.g(),
                        //             self.setting.theme.main.b(),
                        //             (self.spectrogram.points[i].color as f64
                        //                 * self.setting.spectrogram.brightness_boost)
                        //                 as u8,
                        //         ),
                        //     ));
                        // }
                        // if data.is_empty() {
                        //     if self.spectrogram.points.len() >= points_history {
                        //         self.spectrogram.points.drain(0..2048);
                        //     }
                        //     ui.painter().extend(shapes);
                        // } else {
                        //     // Add new points
                        //     // println!("{:?}", data[0].tc);
                        //     for (i, frame) in data.iter().enumerate() {
                        //         for (j, cc) in frame.cc.iter().enumerate() {
                        //             if cc > &0.0 {
                        //                 let pos = Pos2::new(
                        //                     rect.right() - speed * i as f32
                        //                         + frame.tc[j] * speed * 50.0,
                        //                     (1.0 - frame.fc[j] / 24000.0) * rect.height(),
                        //                 );
                        //                 let color = (cc * 255.0 * 0.3) as u8;
                        //                 points.push(pos);
                        //                 shapes.push(Shape::rect_filled(
                        //                     Rect::from_points(&[
                        //                         pos,
                        //                         pos2(pos.x + 1.0, pos.y + 1.0),
                        //                     ]),
                        //                     0.0,
                        //                     Color32::from_rgba_unmultiplied(
                        //                         self.setting.theme.main.r(),
                        //                         self.setting.theme.main.g(),
                        //                         self.setting.theme.main.b(),
                        //                         (color as f64
                        //                             * self.setting.spectrogram.brightness_boost)
                        //                             as u8,
                        //                     ),
                        //                 ));
                        //                 self.spectrogram
                        //                     .points
                        //                     .push(SpectrogramPoints::new(pos, color));
                        //             }
                        //         }
                        //     }
                        //     if self.spectrogram.points.len() >= points_history {
                        //         self.spectrogram.points.drain(0..2048);
                        //     }
                        //     ui.painter().extend(shapes);
                        // }
                    }
                    SpectrogramOrientation::V => {
                        // self.spectrogram
                        //     .texture_raw
                        //     .drain(0..width.wrapping_mul(speed as usize));
                        // self.spectrogram.texture_raw.extend(vec![
                        //     Color32::TRANSPARENT;
                        //     width.wrapping_mul(speed as usize)
                        // ]);
                        // if data.is_empty() {
                        //     self.spectrogram.waiting_index += 1;
                        // } else {
                        //     for i in 0..data.len() {
                        //         for j in 0..data[i].cc.len() {
                        //             if data[i].cc[j] > 0.0 {
                        //                 let color = (data[i].cc[j]
                        //                     * 255.0
                        //                     * self.setting.spectrogram.brightness_boost as f32)
                        //                     as u8;
                        //                 let x = (data[i].fc[j] / 24000.0 * width as f32).round()
                        //                     as usize;
                        //                 let y: usize = height
                        //                     - self.spectrogram.waiting_index * 7
                        //                     - (data[i].tc[j] * speed).round() as usize;

                        //                 let index = (y - 1) * width + x;
                        //                 self.spectrogram.texture_raw[index] =
                        //                     Color32::from_rgba_unmultiplied(
                        //                         self.setting.theme.main.r(),
                        //                         self.setting.theme.main.g(),
                        //                         self.setting.theme.main.b(),
                        //                         color,
                        //                     );
                        //             }
                        //         }
                        //         if self.spectrogram.waiting_index > 0 {
                        //             self.spectrogram.waiting_index -= 1;
                        //         }
                        //     }
                        //     self.spectrogram.waiting_index = 0;
                        // }

                        // let colorimage = ColorImage {
                        //     size: self.spectrogram.texture_size,
                        //     pixels: self.spectrogram.texture_raw.make_contiguous().to_owned(),
                        // };

                        // let texture = ui.ctx().load_texture(
                        //     "spectrogram",
                        //     colorimage,
                        //     // colorimage.region(
                        //     //     &Rect::from_min_max(
                        //     //         pos2(0.0, 2048.0 - rect.width()),
                        //     //         pos2(2048.0, 2048.0),
                        //     //     ),
                        //     //     Some(1.0),
                        //     // ),
                        //     Default::default(),
                        // );

                        // ui.painter().image(
                        //     texture.id(),
                        //     rect,
                        //     Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                        //     Color32::WHITE,
                        // );
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
                            ui.painter().image(
                                texture.id(),
                                rect,
                                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
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
