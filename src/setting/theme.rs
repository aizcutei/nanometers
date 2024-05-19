use crate::NanometersApp;
use egui::style::*;
use egui::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub(crate) main: Color32,
    pub(crate) bg: Color32,
    pub(crate) bgaccent: Color32,
    pub(crate) text: Color32,
    pub(crate) accent: Color32,
    pub(crate) frame: Color32,
    pub(crate) selection: Color32,
}

pub const DARK_THEME: Theme = Theme {
    main: Color32::from_rgb(172, 192, 222),
    bg: Color32::from_rgb(43, 48, 55),
    bgaccent: Color32::from_rgb(0x3b, 0x3b, 0x3b),
    text: Color32::from_rgb(0xf0, 0xf0, 0xea),
    accent: Color32::from_rgb(0x6f, 0x6f, 0xff),
    frame: Color32::from_rgb(65, 65, 65),
    selection: Color32::from_rgb(172, 192, 222),
};

pub const LIGHT_THEME: Theme = Theme {
    main: Color32::from_rgb(0x58, 0x56, 0xcf),
    bg: Color32::from_rgb(240, 240, 234),
    bgaccent: Color32::from_rgb(224, 224, 234),
    text: Color32::from_rgb(43, 43, 43),
    accent: Color32::from_rgb(111, 111, 255),
    frame: Color32::from_rgb(190, 190, 190),
    selection: Color32::from_rgb(189, 189, 229),
};

pub const PINK_THEME: Theme = Theme {
    main: Color32::from_rgb(255, 255, 255),
    bg: Color32::from_rgb(255, 192, 203),
    bgaccent: Color32::from_rgb(243, 202, 203),
    text: Color32::from_rgb(255, 255, 255),
    accent: Color32::from_rgb(255, 233, 203),
    frame: Color32::from_rgb(255, 255, 255),
    selection: Color32::from_rgb(233, 233, 203),
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThemeType {
    Light,
    Dark,
    Pink,
    Custom,
}

pub fn set_theme(app: &mut NanometersApp) -> Visuals {
    Visuals {
        dark_mode: false,
        override_text_color: Some(app.setting.theme.text),
        selection: Selection {
            bg_fill: app.setting.theme.selection,
            stroke: Stroke::NONE,
        },
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: app.setting.theme.bgaccent,
                weak_bg_fill: app.setting.theme.bgaccent,
                bg_stroke: Stroke::new(1.0, app.setting.theme.frame),
                rounding: 0.0.into(),
                fg_stroke: Stroke::NONE,
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                bg_fill: app.setting.theme.bgaccent,
                weak_bg_fill: app.setting.theme.bgaccent,
                bg_stroke: Stroke::NONE,
                rounding: 0.0.into(),
                fg_stroke: Stroke::NONE,
                expansion: 0.0,
            },
            active: WidgetVisuals {
                bg_fill: app.setting.theme.selection,
                weak_bg_fill: app.setting.theme.selection,
                bg_stroke: Stroke::NONE,
                rounding: 0.0.into(),
                fg_stroke: Stroke::NONE,
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                bg_fill: app.setting.theme.selection,
                weak_bg_fill: app.setting.theme.selection,
                bg_stroke: Stroke::NONE,
                rounding: 0.0.into(),
                fg_stroke: Stroke::new(1.0, app.setting.theme.text),
                expansion: 0.0,
            },
            open: WidgetVisuals {
                bg_fill: app.setting.theme.bg,
                weak_bg_fill: app.setting.theme.bgaccent,
                bg_stroke: Stroke::NONE,
                rounding: 0.0.into(),
                fg_stroke: Stroke::NONE,
                expansion: 0.0,
            },
        },
        ..Default::default()
    }
}
