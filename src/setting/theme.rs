use egui::Color32;
use serde::{de, Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Theme {
    name: &'static str,
    main: Color32,
    bg: Color32,
    bgaccent: Color32,
    text: Color32,
    accent: Color32,
    selection: Color32,
}

pub const DARK_THEME: Theme = Theme {
    name: "Dark",
    main: Color32::from_rgb(0x2b, 0x2b, 0x2b),
    bg: Color32::from_rgb(0x2b, 0x2b, 0x2b),
    bgaccent: Color32::from_rgb(0x3b, 0x3b, 0x3b),
    text: Color32::from_rgb(0xf0, 0xf0, 0xea),
    accent: Color32::from_rgb(0x6f, 0x6f, 0xff),
    selection: Color32::from_rgb(0x6f, 0x6f, 0xff),
};

pub const LIGHT_THEME: Theme = Theme {
    name: "Light",
    main: Color32::from_rgb(0xf0, 0xf0, 0xea),
    bg: Color32::from_rgb(0xf0, 0xf0, 0xea),
    bgaccent: Color32::from_rgb(0xe0, 0xe0, 0xea),
    text: Color32::from_rgb(0x2b, 0x2b, 0x2b),
    accent: Color32::from_rgb(0x6f, 0x6f, 0xff),
    selection: Color32::from_rgb(0x6f, 0x6f, 0xff),
};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum ThemeType {
    Light,
    Dark,
    Custom,
}
