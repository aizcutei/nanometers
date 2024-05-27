use crate::utils::*;

pub fn multiband_color(data: Vec<f32>) -> [f32; 3] {
    let low = data
        .iter()
        .take(128)
        .zip(MB_LOW.iter())
        .map(|(a, b)| a * b)
        .sum::<f32>()
        / 256.0;
    let mid = data
        .iter()
        .take(436)
        .zip(MB_MID.iter())
        .map(|(a, b)| a * b)
        .sum::<f32>()
        / (436.0 * 2.0);
    let high = data
        .iter()
        .take(512)
        .zip(MB_HIGH.iter())
        .map(|(a, b)| a * b)
        .sum::<f32>()
        / 1024.0;
    [if low > 1.0 { 1.0 } else { low }, mid, high]
}

pub fn full_brightness_color(rgb: &[f32; 3]) -> egui::Color32 {
    let max = rgb.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    egui::Color32::from_rgb(
        (rgb[0] * (1.0 / max) * 255.0) as u8,
        (rgb[1] * (1.0 / max) * 255.0) as u8,
        (rgb[2] * (1.0 / max) * 255.0) as u8,
    )
}

pub fn color_lut_129() -> Vec<egui::Color32> {
    (0..129)
        .into_iter()
        .map(|i| {
            let h = i as f64 * 360.0 / 129.0;
            let s = 1.0;
            let v = 1.0;
            hsv_to_rgb(h as u16, s, v).unwrap()
        })
        .collect()
}

pub fn hsl_to_rgb(h: u16, s: f64, l: f64) -> Option<egui::Color32> {
    match h {
        0..=60 => {
            let c = color_hsl_c(l, s);
            let x = color_hsl_x(c, h as f64);
            let m = color_hsl_m(l, c);
            Some(egui::Color32::from_rgb(
                ((c + m) * 255.0) as u8,
                ((x + m) * 255.0) as u8,
                (m * 255.0) as u8,
            ))
        }
        61..=120 => {
            let c = color_hsl_c(l, s);
            let x = color_hsl_x(c, h as f64);
            let m = color_hsl_m(l, c);
            Some(egui::Color32::from_rgb(
                ((x + m) * 255.0) as u8,
                ((c + m) * 255.0) as u8,
                (m * 255.0) as u8,
            ))
        }
        121..=180 => {
            let c = color_hsl_c(l, s);
            let x = color_hsl_x(c, h as f64);
            let m = color_hsl_m(l, c);
            Some(egui::Color32::from_rgb(
                (m * 255.0) as u8,
                ((c + m) * 255.0) as u8,
                ((x + m) * 255.0) as u8,
            ))
        }
        181..=240 => {
            let c = color_hsl_c(l, s);
            let x = color_hsl_x(c, h as f64);
            let m = color_hsl_m(l, c);
            Some(egui::Color32::from_rgb(
                (m * 255.0) as u8,
                ((x + m) * 255.0) as u8,
                ((c + m) * 255.0) as u8,
            ))
        }
        241..=300 => {
            let c = color_hsl_c(l, s);
            let x = color_hsl_x(c, h as f64);
            let m = color_hsl_m(l, c);
            Some(egui::Color32::from_rgb(
                ((x + m) * 255.0) as u8,
                (m * 255.0) as u8,
                ((c + m) * 255.0) as u8,
            ))
        }
        301..=360 => {
            let c = color_hsl_c(l, s);
            let x = color_hsl_x(c, h as f64);
            let m = color_hsl_m(l, c);
            Some(egui::Color32::from_rgb(
                ((c + m) * 255.0) as u8,
                (m * 255.0) as u8,
                ((x + m) * 255.0) as u8,
            ))
        }
        _ => None,
    }
}

fn color_hsl_c(l: f64, s: f64) -> f64 {
    (1.0 - (2.0 * l - 1.0).abs()) * s
}

fn color_hsl_x(c: f64, h: f64) -> f64 {
    c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs())
}

fn color_hsl_m(l: f64, c: f64) -> f64 {
    l - c / 2.0
}

pub fn hsv_to_rgb(h: u16, s: f64, v: f64) -> Option<egui::Color32> {
    match h {
        0..=60 => {
            let c = color_hsv_c(v, s);
            let x = color_hsv_x(c, h as f64);
            let m = color_hsv_m(v, c);
            Some(egui::Color32::from_rgb(
                ((c + m) * 255.0) as u8,
                ((x + m) * 255.0) as u8,
                (m * 255.0) as u8,
            ))
        }
        61..=120 => {
            let c = color_hsv_c(v, s);
            let x = color_hsv_x(c, h as f64);
            let m = color_hsv_m(v, c);
            Some(egui::Color32::from_rgb(
                ((x + m) * 255.0) as u8,
                ((c + m) * 255.0) as u8,
                (m * 255.0) as u8,
            ))
        }
        121..=180 => {
            let c = color_hsv_c(v, s);
            let x = color_hsv_x(c, h as f64);
            let m = color_hsv_m(v, c);
            Some(egui::Color32::from_rgb(
                (m * 255.0) as u8,
                ((c + m) * 255.0) as u8,
                ((x + m) * 255.0) as u8,
            ))
        }
        181..=240 => {
            let c = color_hsv_c(v, s);
            let x = color_hsv_x(c, h as f64);
            let m = color_hsv_m(v, c);
            Some(egui::Color32::from_rgb(
                (m * 255.0) as u8,
                ((x + m) * 255.0) as u8,
                ((c + m) * 255.0) as u8,
            ))
        }
        241..=300 => {
            let c = color_hsv_c(v, s);
            let x = color_hsv_x(c, h as f64);
            let m = color_hsv_m(v, c);
            Some(egui::Color32::from_rgb(
                ((x + m) * 255.0) as u8,
                (m * 255.0) as u8,
                ((c + m) * 255.0) as u8,
            ))
        }
        301..=360 => {
            let c = color_hsv_c(v, s);
            let x = color_hsv_x(c, h as f64);
            let m = color_hsv_m(v, c);
            Some(egui::Color32::from_rgb(
                ((c + m) * 255.0) as u8,
                (m * 255.0) as u8,
                ((x + m) * 255.0) as u8,
            ))
        }
        _ => None,
    }
}

fn color_hsv_c(v: f64, s: f64) -> f64 {
    v * s
}

fn color_hsv_x(c: f64, h: f64) -> f64 {
    c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs())
}

fn color_hsv_m(v: f64, c: f64) -> f64 {
    v - c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_to_rgb() {
        assert_eq!(
            hsl_to_rgb(0, 0.0, 0.0),
            Some(egui::Color32::from_rgb(0, 0, 0))
        );
        assert_eq!(
            hsl_to_rgb(0, 0.0, 1.0),
            Some(egui::Color32::from_rgb(255, 255, 255))
        );
        assert_eq!(
            hsl_to_rgb(0, 1.0, 0.5),
            Some(egui::Color32::from_rgb(255, 0, 0))
        );
        assert_eq!(
            hsl_to_rgb(120, 1.0, 0.5),
            Some(egui::Color32::from_rgb(0, 255, 0))
        );
        assert_eq!(
            hsl_to_rgb(240, 1.0, 0.5),
            Some(egui::Color32::from_rgb(0, 0, 255))
        );
    }

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(
            hsv_to_rgb(0, 0.0, 0.0),
            Some(egui::Color32::from_rgb(0, 0, 0))
        );
        assert_eq!(
            hsv_to_rgb(0, 0.0, 1.0),
            Some(egui::Color32::from_rgb(255, 255, 255))
        );
        assert_eq!(
            hsv_to_rgb(0, 1.0, 0.5),
            Some(egui::Color32::from_rgb(127, 0, 0))
        );
        assert_eq!(
            hsv_to_rgb(120, 1.0, 0.5),
            Some(egui::Color32::from_rgb(0, 127, 0))
        );
        assert_eq!(
            hsv_to_rgb(240, 1.0, 0.5),
            Some(egui::Color32::from_rgb(0, 0, 127))
        );
    }
}
