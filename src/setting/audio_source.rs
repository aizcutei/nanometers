use egui::Rect;

#[derive(Debug, Clone)]
pub struct AudioSourceSetting {
    pub rect_size: Rect,
    pub waveform: bool,
    pub peak: bool,
    pub stereo: bool,
}

impl AudioSourceSetting {
    pub fn new() -> Self {
        Self {
            rect_size: Rect::from_two_pos([0.0, 0.0].into(), [800.0, 100.0].into()),
            waveform: false,
            peak: false,
            stereo: false,
        }
    }
}
