use crate::audio::{plugin_client, PluginClient, SystemCapture};
use crate::AudioSource;
use eframe::egui::{self, ViewportCommand};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NanometersApp {
    #[serde(skip)]
    audio_source: Option<Box<dyn AudioSource>>,

    #[serde(skip)]
    raw_left: Arc<Mutex<[f32]>>,

    #[serde(skip)]
    raw_right: Arc<Mutex<[f32]>>,

    #[serde(skip)]
    stft_bufer: Vec<Vec<f32>>,

    #[serde(skip)]
    db: f64,
}

impl Default for NanometersApp {
    fn default() -> Self {
        let raw_left = Arc::new(Mutex::new([0.0; 48000 * 5 + 2]));
        let raw_right = Arc::new(Mutex::new([0.0; 48000 * 5 + 2]));
        let mut plugin_client = PluginClient::new(|data| {
            // let mut left = raw_left.lock().unwrap();
            // let mut right = raw_right.lock().unwrap();
            println!("{}", data[0].len());
        });
        plugin_client.start();

        let audio_source = Some(Box::new(plugin_client) as Box<dyn AudioSource>);

        Self {
            audio_source,
            raw_left,
            raw_right,
            stft_bufer: vec![vec![0.0; 0]; 2],
            db: 0.0,
        }
    }
}

impl NanometersApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for NanometersApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        custom_window_frame(ctx, "drag to move", |ui| {
            ui.label("This is just the contents of the window.");
            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
                if ui.button("exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Interact with the title bar (drag to move window):

    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    // ui.allocate_ui_at_rect(title_bar_rect, |ui| {
    //     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
    //         ui.spacing_mut().item_spacing.x = 0.0;
    //         ui.visuals_mut().button_frame = false;
    //         ui.add_space(8.0);
    //     });
    // });
}
