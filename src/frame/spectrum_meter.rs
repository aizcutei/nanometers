use crate::setting::SpectrumFreqLine;
use crate::setting::SpectrumMode;
// use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
use rustfft::num_complex::ComplexFloat;
use text::Fonts;

impl NanometersApp {
    pub fn spectrum_meter(&mut self, data: &RawData, rect: Rect, ui: &mut Ui) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);
        if self.spectrum.last_rect.is_none() {
            self.spectrum.last_rect = Some(rect);
            self.spectrum.pos = freq_to_pos(rect);
        } else {
            if self.spectrum.last_rect.unwrap() != rect {
                self.spectrum.last_rect = Some(rect);
                self.spectrum.pos = freq_to_pos(rect);
            }
        }
        if self.spectrum.ch0.is_empty() {
            self.spectrum.ch0 = vec![0.0; 2049];
        }
        if self.spectrum.ch1.is_empty() {
            self.spectrum.ch1 = vec![0.0; 2049];
        }

        // Ref lines
        match self.setting.spectrum.freq_line {
            SpectrumFreqLine::Off => {}
            SpectrumFreqLine::On => {
                if self.spectrum.last_rect.is_none() {
                    self.spectrum.last_rect = Some(rect);
                    self.spectrum.lines = ref_lines(ui, rect, 0.5, self.setting.theme.frame);
                    self.spectrum.line_brightness = false;
                } else {
                    if self.spectrum.last_rect.unwrap() != rect {
                        self.spectrum.last_rect = Some(rect);
                        self.spectrum.lines = ref_lines(ui, rect, 0.5, self.setting.theme.frame);
                        self.spectrum.line_brightness = false;
                    } else {
                        if self.spectrum.line_brightness {
                            self.spectrum.lines =
                                ref_lines(ui, rect, 0.5, self.setting.theme.frame);
                            self.spectrum.line_brightness = false;
                        }
                    }
                }

                ui.painter().extend(self.spectrum.lines.clone());
            }
            SpectrumFreqLine::Bright => {
                if self.spectrum.last_rect.is_none() {
                    self.spectrum.last_rect = Some(rect);
                    self.spectrum.lines = ref_lines(ui, rect, 1.0, self.setting.theme.frame);
                    self.spectrum.line_brightness = false;
                } else {
                    if self.spectrum.last_rect.unwrap() != rect {
                        self.spectrum.last_rect = Some(rect);
                        self.spectrum.lines = ref_lines(ui, rect, 1.0, self.setting.theme.frame);
                        self.spectrum.line_brightness = false;
                    } else {
                        if !self.spectrum.line_brightness {
                            self.spectrum.lines =
                                ref_lines(ui, rect, 1.0, self.setting.theme.frame);
                            self.spectrum.line_brightness = false;
                        }
                    }
                }
                ui.painter().extend(self.spectrum.lines.clone());
            }
        }
        let ref_line_x =
            0.2991878257 * (self.setting.spectrum.ref_line.log10() as f32 - 1.0) * rect.width()
                + rect.left();
        ui.painter().line_segment(
            [pos2(ref_line_x, 0.0), pos2(ref_line_x, rect.bottom())],
            Stroke::new(1.0, self.setting.theme.main),
        );

        // Main
        let mut max_index = 0;
        match self.setting.spectrum.mode {
            SpectrumMode::FFT => {
                let mut wave_0_points = Vec::new();
                let mut wave_1_points = Vec::new();
                if data.l.is_empty() {
                    wave_0_points.extend(
                        self.spectrum
                            .pos
                            .iter()
                            .zip(self.spectrum.ch0.iter())
                            .take(2050)
                            .map(|(&pos, &ch0)| pos2(pos, (1.0 - ch0) * rect.height())),
                    );
                    wave_1_points.extend(
                        self.spectrum
                            .pos
                            .iter()
                            .zip(self.spectrum.ch1.iter())
                            .take(2049)
                            .map(|(&pos, &ch1)| pos2(pos, (1.0 - ch1) * rect.height())),
                    );
                } else {
                    for i in 0..2049 {
                        if data.l[i] >= data.l[max_index] {
                            max_index = i;
                        }
                        if data.l[i] > self.spectrum.ch0[i] || self.spectrum.ch0[i].is_nan() {
                            self.spectrum.ch0[i] = data.l[i];
                            wave_0_points.push(pos2(
                                self.spectrum.pos[i],
                                (1.0 - data.l[i]) * rect.height(),
                            ));
                        } else {
                            self.spectrum.ch0[i] = self.spectrum.ch0[i]
                                * self.setting.spectrum.smoothing
                                + data.l[i] * (1.0 - self.setting.spectrum.smoothing);
                            wave_0_points.push(pos2(
                                self.spectrum.pos[i],
                                (1.0 - self.spectrum.ch0[i]) * rect.height(),
                            ));
                        }
                        if data.r[i] > self.spectrum.ch1[i] || self.spectrum.ch1[i].is_nan() {
                            self.spectrum.ch1[i] = data.r[i];
                            wave_1_points.push(pos2(
                                self.spectrum.pos[i],
                                (1.0 - data.r[i]) * rect.height(),
                            ));
                        } else {
                            self.spectrum.ch1[i] = self.spectrum.ch1[i]
                                * self.setting.spectrum.smoothing
                                + data.r[i] * (1.0 - self.setting.spectrum.smoothing);
                            wave_1_points.push(pos2(
                                self.spectrum.pos[i],
                                (1.0 - self.spectrum.ch1[i]) * rect.height(),
                            ));
                        }
                    }
                }

                let wave_0 = Shape::line(wave_0_points, Stroke::new(2.0, self.setting.theme.main));
                let wave_1 = Shape::line(wave_1_points, Stroke::new(2.0, self.setting.theme.text));
                ui.painter().add(wave_0);
                ui.painter().add(wave_1);
            }
            SpectrumMode::ColorBar => {}
            SpectrumMode::Both => {}
        }
        if max_index > 0 && max_index < 2048 {
            let delta = (data.l[max_index + 1].abs() - data.l[max_index - 1].abs())
                / (2.0
                    * (2.0 * data.l[max_index].abs()
                        - data.l[max_index - 1].abs()
                        - data.l[max_index + 1].abs()));
            let freq = (max_index as f32 + delta) * 11.7130307467;
            let freq_str = format!("{:.1} Hz", freq);
        }
    }
}

fn ref_lines(ui: &mut Ui, rect: Rect, width: f32, color: Color32) -> Vec<Shape> {
    let mut lines = vec![];
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.0900645099 * rect.width(), 0.0),
            pos2(rect.left() + 0.0900645099 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 20
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.1427488708 * rect.width(), 0.0),
            pos2(rect.left() + 0.1427488708 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 30
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.1801290197 * rect.width(), 0.0),
            pos2(rect.left() + 0.1801290197 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 40
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2091233158 * rect.width(), 0.0),
            pos2(rect.left() + 0.2091233158 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 50
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2328133807 * rect.width(), 0.0),
            pos2(rect.left() + 0.2328133807 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 60
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2528430451 * rect.width(), 0.0),
            pos2(rect.left() + 0.2528430451 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 70
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2701935296 * rect.width(), 0.0),
            pos2(rect.left() + 0.2701935296 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 80
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2854977416 * rect.width(), 0.0),
            pos2(rect.left() + 0.2854977416 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 90
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2991878257 * rect.width(), 10.0),
            pos2(rect.left() + 0.2991878257 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 100
    ui.fonts(|fonts| {
        lines.push(Shape::text(
            fonts,
            pos2(rect.left() + 0.2991878257 * rect.width(), 5.0),
            Align2::CENTER_CENTER,
            "100Hz",
            FontId::monospace(9.0),
            color,
        ));
    });
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.3892523356 * rect.width(), 0.0),
            pos2(rect.left() + 0.3892523356 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 200
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.4419366965 * rect.width(), 0.0),
            pos2(rect.left() + 0.4419366965 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 300
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.4793168454 * rect.width(), 0.0),
            pos2(rect.left() + 0.4793168454 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 400
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5083111415 * rect.width(), 0.0),
            pos2(rect.left() + 0.5083111415 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 500
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5320012064 * rect.width(), 0.0),
            pos2(rect.left() + 0.5320012064 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 600
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5520308708 * rect.width(), 0.0),
            pos2(rect.left() + 0.5520308708 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 700
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5693813553 * rect.width(), 0.0),
            pos2(rect.left() + 0.5693813553 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 800
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5846855673 * rect.width(), 0.0),
            pos2(rect.left() + 0.5846855673 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 900
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5983756514 * rect.width(), 10.0),
            pos2(rect.left() + 0.5983756514 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 1000
    ui.fonts(|fonts| {
        lines.push(Shape::text(
            fonts,
            pos2(rect.left() + 0.5983756514 * rect.width(), 5.0),
            Align2::CENTER_CENTER,
            "1kHz",
            FontId::monospace(9.0),
            color,
        ));
    });
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.6884401613 * rect.width(), 0.0),
            pos2(rect.left() + 0.6884401613 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 2000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.7411245222 * rect.width(), 0.0),
            pos2(rect.left() + 0.7411245222 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 3000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.7785046711 * rect.width(), 0.0),
            pos2(rect.left() + 0.7785046711 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 4000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.8074989672 * rect.width(), 0.0),
            pos2(rect.left() + 0.8074989672 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 5000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.8311890321 * rect.width(), 0.0),
            pos2(rect.left() + 0.8311890321 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 6000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.8512186965 * rect.width(), 0.0),
            pos2(rect.left() + 0.8512186965 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 7000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.868569181 * rect.width(), 0.0),
            pos2(rect.left() + 0.868569181 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 8000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.883873393 * rect.width(), 0.0),
            pos2(rect.left() + 0.883873393 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 9000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.8975634771 * rect.width(), 10.0),
            pos2(rect.left() + 0.8975634771 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 10000
    ui.fonts(|fonts| {
        lines.push(Shape::text(
            fonts,
            pos2(rect.left() + 0.8975634771 * rect.width(), 5.0),
            Align2::CENTER_CENTER,
            "10kHz",
            FontId::monospace(9.0),
            color,
        ));
    });
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.987627987 * rect.width(), 0.0),
            pos2(rect.left() + 0.987627987 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 20000
    lines
}

fn freq_to_pos(rect: Rect) -> Vec<f32> {
    let mut pos = Vec::with_capacity(2049);
    for i in 0..2049 {
        if i == 0 {
            pos.push(rect.left());
        } else {
            pos.push(
                rect.left() + 0.2991878257 * ((i as f32 * 11.71875).log10() - 1.0) * rect.width(),
            );
        }
    }
    pos
}
