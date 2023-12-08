use std::fs::File;
use std::io::{read_to_string, write};
use toml::de::from_str;
use toml::ser::to_string_pretty;

#[derive(Debug)]
pub struct Settings {
    // Window settings
    pub window_width: u32,
    pub window_height: u32,
    pub window_x: u32,
    pub window_y: u32,
    // Other settings
}

impl Settings {
    pub fn new() -> Self {
        let mut settings = Settings {
            window_width: 800,
            window_height: 600,
            window_x: 100,
            window_y: 100,
        };
        settings
    }
    pub fn save(&self) {
        let contents = to_string_pretty(self).unwrap();
        write("settings.toml", contents).unwrap();
    }
}
