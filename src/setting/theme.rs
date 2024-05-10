use egui::Color32;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub(crate) main: Color32,
    pub(crate) bg: Color32,
    pub(crate) bgaccent: Color32,
    pub(crate) text: Color32,
    pub(crate) accent: Color32,
    pub(crate) selection: Color32,
}

pub const DARK_THEME: Theme = Theme {
    main: Color32::from_rgb(172, 192, 222),
    bg: Color32::from_rgb(43, 48, 55),
    bgaccent: Color32::from_rgb(0x3b, 0x3b, 0x3b),
    text: Color32::from_rgb(0xf0, 0xf0, 0xea),
    accent: Color32::from_rgb(0x6f, 0x6f, 0xff),
    selection: Color32::from_rgb(0x6f, 0x6f, 0xff),
};

pub const LIGHT_THEME: Theme = Theme {
    main: Color32::from_rgb(23, 23, 44),
    bg: Color32::from_rgb(240, 240, 234),
    bgaccent: Color32::from_rgb(0xe0, 0xe0, 0xea),
    text: Color32::from_rgb(0x2b, 0x2b, 0x2b),
    accent: Color32::from_rgb(0x6f, 0x6f, 0xff),
    selection: Color32::from_rgb(0x6f, 0x6f, 0xff),
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThemeType {
    Light,
    Dark,
    Custom,
}
