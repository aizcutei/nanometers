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
        // Image buffer
        let ppp = ui.ctx().pixels_per_point();
        let width = (rect.width()) as usize;
        let height = (rect.height()) as usize;
        let on_img_frame_width = 16.0;
        let on_img_frame_interval = 2.0;
        let ori_img_frame_width = 20.0;
        let ori_img_frame_interval = 2.5;

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
                        let mut piet_img_full = piet.bitmap_target(width, height, 1.0).unwrap();
                        let mut piet = Device::new().unwrap();
                        let piet_img_frame_width = ori_img_frame_width as usize
                            + (len - 1) * ori_img_frame_interval as usize;
                        let mut piet_img_frame =
                            piet.bitmap_target(piet_img_frame_width, 1024, 1.0).unwrap();
                        {
                            let mut piet_img_frame_ctx = piet_img_frame.render_context();
                            for i in 0..len {
                                for j in 0..data[i].f.len() {
                                    let x = i as f64 * ori_img_frame_interval
                                        + data[i].t[j] as f64 * ori_img_frame_width;
                                    let y = 1024.0 * (1.0 - data[i].f[j]) as f64;
                                    piet_img_frame_ctx.fill(
                                        piet_common::kurbo::Circle::new(
                                            piet_common::kurbo::Point::new(x, y),
                                            1.0,
                                        ),
                                        &piet_common::Color::rgba8(
                                            self.setting.theme.main.r(),
                                            self.setting.theme.main.g(),
                                            self.setting.theme.main.b(),
                                            data[i].i[j],
                                        ),
                                    );
                                }
                            }
                            let update_image = piet_img_frame_ctx
                                .capture_image_area(piet_common::kurbo::Rect::new(
                                    0.0,
                                    0.0,
                                    piet_img_frame_width as f64,
                                    1024.0,
                                ))
                                .unwrap();

                            let mut piet_img_full_ctx = piet_img_full.render_context();
                            if self.spectrogram.last_img.is_some() {
                                piet_img_full_ctx.draw_image_area(
                                    &self.spectrogram.last_img.as_ref().unwrap(),
                                    piet_common::kurbo::Rect::new(
                                        len as f64 * on_img_frame_interval,
                                        0.0,
                                        rect.width() as f64,
                                        rect.height() as f64,
                                    ),
                                    piet_common::kurbo::Rect::new(
                                        0.0,
                                        0.0,
                                        rect.width() as f64 - len as f64 * on_img_frame_interval,
                                        rect.height() as f64,
                                    ),
                                    piet_common::InterpolationMode::NearestNeighbor,
                                );
                            }
                            piet_img_full_ctx.draw_image(
                                &update_image,
                                piet_common::kurbo::Rect::new(
                                    rect.width() as f64 - (len - 1) as f64 * on_img_frame_interval
                                        + on_img_frame_width,
                                    0.0,
                                    rect.width() as f64,
                                    rect.height() as f64,
                                ),
                                piet_common::InterpolationMode::NearestNeighbor,
                            );

                            self.spectrogram.last_img = Some(
                                piet_img_full_ctx
                                    .capture_image_area(piet_common::kurbo::Rect::new(
                                        0.0,
                                        0.0,
                                        rect.width() as f64,
                                        rect.height() as f64,
                                    ))
                                    .unwrap(),
                            );
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
                    SpectrogramOrientation::V => {}
                },
            }
        }

        if self.spectrogram.texture.is_some() {
            ui.painter().image(
                self.spectrogram.texture.as_ref().unwrap().id(),
                rect,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        //
        // match self.setting.spectrogram.mode {
        //     SpectrogramMode::Classic => match self.setting.spectrogram.orientation {
        //         SpectrogramOrientation::H => {
        //             let mut piet = Device::new().unwrap();
        //             let mut piet_img = piet.bitmap_target(width, height, 1.0).unwrap();

        //             if data.is_empty() {
        //             } else {
        //                 let frame_num = data.len();
        //                 {
        //                     let mut piet_ctx = piet_img.render_context();
        //                     if self.spectrogram.last_img.is_some() {
        //                         piet_ctx.draw_image_area(
        //                             &self.spectrogram.last_img.as_ref().unwrap(),
        //                             piet_common::kurbo::Rect::new(
        //                                 frame_num as f64 * update_width,
        //                                 0.0,
        //                                 rect.width() as f64,
        //                                 rect.height() as f64,
        //                             ),
        //                             piet_common::kurbo::Rect::new(
        //                                 0.0,
        //                                 0.0,
        //                                 rect.width() as f64 - frame_num as f64 * update_width,
        //                                 rect.height() as f64,
        //                             ),
        //                             piet_common::InterpolationMode::NearestNeighbor,
        //                         )
        //                     }
        //                     for i in 0..data.len() {
        //                         for j in 0..data[i].f.len() {
        //                             let x = rect.width() as f64 - i as f64 * update_width;
        //                             let y = rect.height() as f64 * (1.0 - data[i].f[j] as f64);
        //                             piet_ctx.fill(
        //                                 piet_common::kurbo::Rect::new(x, y, x + 1.0, y + 1.0),
        //                                 &piet_common::Color::rgba8(
        //                                     self.setting.theme.main.r(),
        //                                     self.setting.theme.main.g(),
        //                                     self.setting.theme.main.b(),
        //                                     data[i].i[j] as u8,
        //                                 ),
        //                             );
        //                         }
        //                     }
        //                     self.spectrogram.last_img = Some(
        //                         piet_ctx
        //                             .capture_image_area(piet_common::kurbo::Rect::new(
        //                                 0.0,
        //                                 0.0,
        //                                 rect.width() as f64,
        //                                 rect.height() as f64,
        //                             ))
        //                             .unwrap(),
        //                     );
        //                 }
        //                 let pixels = piet_img.to_image_buf(ImageFormat::RgbaPremul).unwrap();
        //                 let size = pixels.size();
        //                 let egui_image = ColorImage::from_rgba_premultiplied(
        //                     [size.width as usize, size.height as usize],
        //                     pixels.raw_pixels(),
        //                 );
        //                 let texture =
        //                     ui.ctx()
        //                         .load_texture("spectrogram", egui_image, Default::default());
        //                 self.spectrogram.texture = Some(texture);
        //             }

        //             if self.spectrogram.texture.is_some() {
        //                 ui.painter().image(
        //                     self.spectrogram.texture.as_ref().unwrap().id(),
        //                     rect,
        //                     Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        //                     Color32::WHITE,
        //                 );
        //             }
        //         }
        //         SpectrogramOrientation::V => {}
        //     },
        //     SpectrogramMode::Sharp => {
        //         match self.setting.spectrogram.orientation {
        //             SpectrogramOrientation::H => {
        //                 if !data.is_empty() {
        //                     let frame_num = data.len();
        //                     let frame_width = 3;
        //                     let mut piet = Device::new().unwrap();
        //                     let mut piet_img_full = piet.bitmap_target(width, height, 1.0).unwrap();
        //                     let mut piet = Device::new().unwrap();
        //                     let mut pite_img_update = piet
        //                         .bitmap_target(frame_width * frame_num, 2160, 1.0)
        //                         .unwrap();
        //                     let mut update_img = None;

        //                     {
        //                         {
        //                             // pite_img_update plot
        //                             let mut piet_update_ctx = pite_img_update.render_context();
        //                             for i in 0..data.len() {
        //                                 for j in 0..data[i].f.len() {
        //                                     // println!(
        //                                     //     "{},{},{}",
        //                                     //     data[i].f[j], data[i].t[j], data[i].i[j]
        //                                     // );
        //                                     let x = (i * frame_width) as f64 + data[i].t[j] as f64;
        //                                     let y = 2160.0 * (1.0 - data[i].f[j]) as f64;
        //                                     piet_update_ctx.fill(
        //                                         piet_common::kurbo::Rect::new(
        //                                             x,
        //                                             y,
        //                                             x + 1.0,
        //                                             y + 1.0,
        //                                         ),
        //                                         &piet_common::Color::rgba8(
        //                                             self.setting.theme.main.r(),
        //                                             self.setting.theme.main.g(),
        //                                             self.setting.theme.main.b(),
        //                                             data[i].i[j] as u8,
        //                                         ),
        //                                     );
        //                                 }
        //                             }
        //                             update_img = Some(
        //                                 piet_update_ctx
        //                                     .capture_image_area(piet_common::kurbo::Rect::new(
        //                                         0.0,
        //                                         0.0,
        //                                         (frame_width * frame_num) as f64,
        //                                         2160.0,
        //                                     ))
        //                                     .unwrap(),
        //                             );
        //                             piet_update_ctx.finish().unwrap();
        //                         }

        //                         let mut piet_ctx = piet_img_full.render_context();

        //                         if self.spectrogram.last_img.is_some() {
        //                             piet_ctx.draw_image_area(
        //                                 &self.spectrogram.last_img.as_ref().unwrap(),
        //                                 piet_common::kurbo::Rect::new(
        //                                     frame_num as f64 * update_width,
        //                                     0.0,
        //                                     rect.width() as f64,
        //                                     rect.height() as f64,
        //                                 ),
        //                                 piet_common::kurbo::Rect::new(
        //                                     0.0,
        //                                     0.0,
        //                                     rect.width() as f64 - frame_num as f64 * update_width,
        //                                     rect.height() as f64,
        //                                 ),
        //                                 piet_common::InterpolationMode::NearestNeighbor,
        //                             )
        //                         }
        //                         piet_ctx.draw_image(
        //                             &update_img.unwrap(),
        //                             piet_common::kurbo::Rect::new(
        //                                 rect.width() as f64 - frame_num as f64 * update_width,
        //                                 0.0,
        //                                 rect.width() as f64,
        //                                 rect.height() as f64,
        //                             ),
        //                             piet_common::InterpolationMode::NearestNeighbor,
        //                         );

        //                         self.spectrogram.last_img = Some(
        //                             piet_ctx
        //                                 .capture_image_area(piet_common::kurbo::Rect::new(
        //                                     0.0,
        //                                     0.0,
        //                                     rect.width() as f64,
        //                                     rect.height() as f64,
        //                                 ))
        //                                 .unwrap(),
        //                         );
        //                     }
        //                     let pixels =
        //                         piet_img_full.to_image_buf(ImageFormat::RgbaPremul).unwrap();
        //                     let size = pixels.size();
        //                     let egui_image = ColorImage::from_rgba_premultiplied(
        //                         [size.width as usize, size.height as usize],
        //                         pixels.raw_pixels(),
        //                     );
        //                     let texture = ui.ctx().load_texture(
        //                         "spectrogram",
        //                         egui_image,
        //                         Default::default(),
        //                     );
        //                     self.spectrogram.texture = Some(texture);
        //                 }

        //                 if self.spectrogram.texture.is_some() {
        //                     ui.painter().image(
        //                         self.spectrogram.texture.as_ref().unwrap().id(),
        //                         rect,
        //                         Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        //                         Color32::WHITE,
        //                     );
        //                 }
        //             }
        //             SpectrogramOrientation::V => {
        //                 let mut piet = Device::new().unwrap();
        //                 let mut piet_img = piet.bitmap_target(width, height, 1.0).unwrap();

        //                 if data.is_empty() {
        //                 } else {
        //                     let frame_num = data.len();
        //                     println!("{}", frame_num);
        //                     {
        //                         let mut piet_ctx = piet_img.render_context();
        //                         if self.spectrogram.last_img.is_some() {
        //                             piet_ctx.draw_image_area(
        //                                 &self.spectrogram.last_img.as_ref().unwrap(),
        //                                 piet_common::kurbo::Rect::new(
        //                                     0.0,
        //                                     frame_num as f64 * update_width,
        //                                     rect.width() as f64,
        //                                     rect.height() as f64,
        //                                 ),
        //                                 piet_common::kurbo::Rect::new(
        //                                     0.0,
        //                                     0.0,
        //                                     rect.width() as f64,
        //                                     rect.height() as f64 - frame_num as f64 * update_width,
        //                                 ),
        //                                 piet_common::InterpolationMode::NearestNeighbor,
        //                             )
        //                         }
        //                         for i in 0..data.len() {
        //                             for j in 0..data[i].f.len() {
        //                                 let x = rect.width() as f64 * data[i].f[j] as f64;
        //                                 let y = rect.height() as f64 - i as f64 * update_width
        //                                     + data[i].t[j] as f64;
        //                                 piet_ctx.fill(
        //                                     piet_common::kurbo::Rect::new(x, y, x + 1.0, y + 1.0),
        //                                     &piet_common::Color::rgba8(
        //                                         self.setting.theme.main.r(),
        //                                         self.setting.theme.main.g(),
        //                                         self.setting.theme.main.b(),
        //                                         data[i].i[j] as u8,
        //                                     ),
        //                                 );
        //                             }
        //                         }
        //                         self.spectrogram.last_img = Some(
        //                             piet_ctx
        //                                 .capture_image_area(piet_common::kurbo::Rect::new(
        //                                     0.0,
        //                                     0.0,
        //                                     rect.width() as f64,
        //                                     rect.height() as f64,
        //                                 ))
        //                                 .unwrap(),
        //                         );
        //                     }
        //                     let pixels = piet_img.to_image_buf(ImageFormat::RgbaPremul).unwrap();
        //                     let size = pixels.size();
        //                     let egui_image = ColorImage::from_rgba_premultiplied(
        //                         [size.width as usize, size.height as usize],
        //                         pixels.raw_pixels(),
        //                     );
        //                     let texture = ui.ctx().load_texture(
        //                         "spectrogram",
        //                         egui_image,
        //                         Default::default(),
        //                     );
        //                     self.spectrogram.texture = Some(texture);
        //                 }

        //                 if self.spectrogram.texture.is_some() {
        //                     ui.painter().image(
        //                         self.spectrogram.texture.as_ref().unwrap().id(),
        //                         rect,
        //                         Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        //                         Color32::WHITE,
        //                     );
        //                 }
        //////////
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
        //                 }
        //             };
        //         }
        //     }
        // }
    }
}
