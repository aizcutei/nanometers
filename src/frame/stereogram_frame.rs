use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
use rayon::iter::IntoParallelRefIterator;

const SQRT2_2: f32 = 0.7071067811865476;
const SQRT2_4: f32 = 0.3535533905932738;
const SQRT2_2_SUB1: f32 = 0.2928932188134524;
const SQRT2_2_ADD1: f32 = 1.7071067811865476;

impl NanometersApp {
    pub fn stereogram_frame(
        &mut self,
        data: &StereoSendData,
        rect: eframe::epaint::Rect,
        ui: &mut Ui,
    ) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        self.setting.stereogram.point_size = rect.max.y * 0.01;
        match self.setting.stereogram.mode {
            StereogramMode::Logarithmic => {
                match self.setting.stereogram.polarity {
                    StereogramPolarity::Uni => {
                        // Guide lines
                        if self.setting.stereogram.guides {
                            let mut shapes = vec![];
                            shapes.push(Shape::circle_stroke(
                                rect.center_bottom(),
                                rect.max.y,
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::circle_stroke(
                                rect.center_bottom(),
                                rect.max.y * 0.86,
                                Stroke::new(2.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::circle_stroke(
                                rect.center_bottom(),
                                rect.max.y * 0.73,
                                Stroke::new(1.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::circle_stroke(
                                rect.center_bottom(),
                                rect.max.y * 0.60,
                                Stroke::new(1.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line(
                                vec![
                                    [
                                        rect.center().x - SQRT2_2 * rect.max.y,
                                        SQRT2_2_SUB1 * rect.max.y,
                                    ]
                                    .into(),
                                    rect.center_bottom(),
                                    [
                                        rect.center().x + SQRT2_2 * rect.max.y,
                                        SQRT2_2_SUB1 * rect.max.y,
                                    ]
                                    .into(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            ui.painter().extend(shapes);
                        }
                        // Point plot
                        if data.linear.is_empty() {
                            ui.painter().extend(self.stereo.plot.clone());
                        } else {
                            let transform = emath::TSTransform::new(
                                [rect.center().x, rect.max.y].into(),
                                if self.setting.stereogram.normalize {
                                    if data.max > 0.001 {
                                        rect.max.y / (1.0 + data.max.log10() / 3.0)
                                    } else {
                                        rect.max.y
                                    }
                                } else {
                                    rect.max.y
                                },
                            );
                            let shapes: Vec<_> = data
                                .log
                                .iter()
                                .map(|p| {
                                    Shape::circle_filled(
                                        transform.mul_pos(Pos2::new(
                                            if p.y > 0.0 { -p.x } else { p.x },
                                            if p.y > 0.0 { -p.y } else { p.y },
                                        )),
                                        self.setting.stereogram.point_size,
                                        self.setting.theme.main,
                                    )
                                })
                                .collect();
                            self.stereo.plot = shapes.clone();
                            ui.painter().extend(shapes);
                        }
                    }
                    StereogramPolarity::Bi => {
                        // Guide lines
                        if self.setting.stereogram.guides {
                            let mut shapes = vec![];
                            shapes.push(Shape::circle_stroke(
                                rect.center(),
                                rect.center().y,
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::circle_stroke(
                                rect.center(),
                                rect.center().y * 0.86,
                                Stroke::new(2.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::circle_stroke(
                                rect.center(),
                                rect.center().y * 0.73,
                                Stroke::new(1.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::circle_stroke(
                                rect.center(),
                                rect.center().y * 0.60,
                                Stroke::new(1.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line_segment(
                                [
                                    [
                                        rect.center().x - rect.center().y * SQRT2_2,
                                        rect.center().y * SQRT2_2_SUB1,
                                    ]
                                    .into(),
                                    [
                                        rect.center().x + rect.center().y * SQRT2_2,
                                        rect.center().y * SQRT2_2_ADD1,
                                    ]
                                    .into(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line_segment(
                                [
                                    [
                                        rect.center().x - rect.center().y * SQRT2_2,
                                        rect.center().y * SQRT2_2_ADD1,
                                    ]
                                    .into(),
                                    [
                                        rect.center().x + rect.center().y * SQRT2_2,
                                        rect.center().y * SQRT2_2_SUB1,
                                    ]
                                    .into(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            ui.painter().extend(shapes);
                        }
                        // Point
                        if data.linear.is_empty() {
                            ui.painter().extend(self.stereo.plot.clone());
                        } else {
                            let transform = emath::TSTransform::new(
                                [rect.center().x, rect.center().y].into(),
                                if self.setting.stereogram.normalize {
                                    if data.max > 0.001 {
                                        rect.center().y / (1.0 + data.max.log10() / 3.0)
                                    } else {
                                        rect.center().y
                                    }
                                } else {
                                    rect.center().y
                                },
                            );
                            let shapes: Vec<_> = data
                                .log
                                .iter()
                                .map(|p| {
                                    Shape::circle_filled(
                                        transform.mul_pos(p.to_owned()),
                                        self.setting.stereogram.point_size * 0.5,
                                        self.setting.theme.main,
                                    )
                                })
                                .collect();
                            self.stereo.plot = shapes.clone();
                            ui.painter().extend(shapes);
                        }
                    }
                }
            }
            StereogramMode::Linear => {
                match self.setting.stereogram.polarity {
                    StereogramPolarity::Uni => {
                        // Guide lines
                        if self.setting.stereogram.guides {
                            let mut shapes = vec![];
                            shapes.push(Shape::line(
                                vec![
                                    [rect.center_bottom().x - rect.max.y, rect.max.y].into(),
                                    rect.center_top(),
                                    [rect.center_bottom().x + rect.max.y, rect.max.y].into(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line(
                                vec![
                                    [rect.center_bottom().x - rect.center().y, rect.max.y].into(),
                                    rect.center(),
                                    [rect.center_bottom().x + rect.center().y, rect.max.y].into(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line(
                                vec![
                                    [rect.center_bottom().x - rect.center().y, rect.center().y]
                                        .into(),
                                    rect.center_bottom(),
                                    [rect.center_bottom().x + rect.center().y, rect.center().y]
                                        .into(),
                                ],
                                Stroke::new(2.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line_segment(
                                [
                                    [rect.center().x, 0.0].into(),
                                    [rect.center().x, rect.max.y].into(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            ui.painter().extend(shapes);
                        }
                        // Points
                        if data.linear.is_empty() {
                            ui.painter().extend(self.stereo.plot.clone());
                        } else {
                            let transform = emath::TSTransform::new(
                                [rect.center().x, rect.max.y].into(),
                                if self.setting.stereogram.normalize {
                                    0.717067812 * rect.max.y / data.max
                                } else {
                                    0.717067812 * rect.center().y
                                },
                            );
                            let shapes: Vec<_> = data
                                .linear
                                .iter()
                                .map(|p| {
                                    Shape::circle_filled(
                                        transform.mul_pos(Pos2::new(
                                            if p.y > 0.0 { -p.x } else { p.x },
                                            if p.y > 0.0 { -p.y } else { p.y },
                                        )),
                                        self.setting.stereogram.point_size,
                                        self.setting.theme.main,
                                    )
                                })
                                .collect();
                            self.stereo.plot = shapes.clone();
                            ui.painter().extend(shapes);
                        }
                    }
                    StereogramPolarity::Bi => {
                        // Guide lines
                        if self.setting.stereogram.guides {
                            let mut shapes = vec![];
                            shapes.push(Shape::line(
                                vec![
                                    rect.center_top(),
                                    [rect.center().x + rect.center().y, rect.center().y].into(),
                                    rect.center_bottom(),
                                    [rect.center().x - rect.center().y, rect.center().y].into(),
                                    rect.center_top(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line(
                                vec![
                                    [rect.center().x, rect.center().y / 2.0].into(),
                                    [rect.center().x + rect.center().y / 2.0, rect.center().y]
                                        .into(),
                                    [rect.center().x, rect.center().y * 1.5].into(),
                                    [rect.center().x - rect.center().y / 2.0, rect.center().y]
                                        .into(),
                                    [rect.center().x, rect.center().y / 2.0].into(),
                                ],
                                Stroke::new(2.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line_segment(
                                [
                                    [rect.center().x, 0.0].into(),
                                    [rect.center().x, rect.max.y].into(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line_segment(
                                [
                                    [rect.center().x + rect.center().y, rect.center().y].into(),
                                    [rect.center().x - rect.center().y, rect.center().y].into(),
                                ],
                                Stroke::new(3.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line_segment(
                                [
                                    [
                                        rect.center().x - rect.center().y / 2.0,
                                        rect.center().y / 2.0,
                                    ]
                                    .into(),
                                    [
                                        rect.center().x + rect.center().y / 2.0,
                                        rect.center().y * 1.5,
                                    ]
                                    .into(),
                                ],
                                Stroke::new(2.0, self.setting.theme.frame),
                            ));
                            shapes.push(Shape::line_segment(
                                [
                                    [
                                        rect.center().x + rect.center().y / 2.0,
                                        rect.center().y / 2.0,
                                    ]
                                    .into(),
                                    [
                                        rect.center().x - rect.center().y / 2.0,
                                        rect.center().y * 1.5,
                                    ]
                                    .into(),
                                ],
                                Stroke::new(2.0, self.setting.theme.frame),
                            ));
                            ui.painter().extend(shapes);
                        }
                        // Points
                        if data.linear.is_empty() {
                            ui.painter().extend(self.stereo.plot.clone());
                        } else {
                            let transform = emath::TSTransform::new(
                                [rect.center().x, rect.center().y].into(),
                                if self.setting.stereogram.normalize {
                                    0.3535533906 * rect.max.y / data.max
                                } else {
                                    0.3535533906 * rect.center().y
                                },
                            );
                            let shapes: Vec<_> = data
                                .linear
                                .iter()
                                .map(|p| {
                                    Shape::circle_filled(
                                        transform.mul_pos(p.clone()),
                                        self.setting.stereogram.point_size * 0.5,
                                        self.setting.theme.main,
                                    )
                                })
                                .collect();
                            self.stereo.plot = shapes.clone();
                            ui.painter().extend(shapes);
                        }
                    }
                }
            }
            StereogramMode::Lissajous => match self.setting.stereogram.color {
                StereogramColor::Static => {
                    if !data.lissa.is_empty() {
                        let transform = emath::TSTransform::new(
                            [rect.center().x, rect.center().y].into(),
                            if self.setting.stereogram.normalize {
                                rect.center().y / data.max
                            } else {
                                rect.center().y
                            },
                        );
                        let shapes: Vec<_> = data
                            .lissa
                            .iter()
                            .map(|p| {
                                Shape::circle_filled(
                                    transform.mul_pos(p.to_owned()).to_owned(),
                                    self.setting.stereogram.point_size,
                                    self.setting.theme.main,
                                )
                            })
                            .collect();
                        self.stereo.plot = shapes.clone();
                        ui.painter().extend(shapes);
                    } else {
                        ui.painter().extend(self.stereo.plot.clone());
                    }
                }
                StereogramColor::RGB => {}
                StereogramColor::MultiBand => {}
            },
        }
    }
}
