#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    #[cfg(feature = "puffin")]
    start_puffin_server();

    let native_options = eframe::NativeOptions {
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            desired_maximum_frame_latency: Some(1),
            power_preference: eframe::wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        },
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([800.0, 200.0])
            .with_min_inner_size([800.0, 100.0])
            .with_resizable(true)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Nanometers",
        native_options,
        Box::new(|cc| Box::new(nanometers::NanometersApp::new(cc))),
    )
}

#[cfg(feature = "puffin")]
fn start_puffin_server() {
    puffin::set_scopes_on(true); // tell puffin to collect data

    match puffin_http::Server::new("127.0.0.1:8585") {
        Ok(puffin_server) => {
            eprintln!("Run: cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585");

            std::process::Command::new("puffin_viewer")
                .arg("--url")
                .arg("127.0.0.1:8585")
                .spawn()
                .ok();

            // We can store the server if we want, but in this case we just want
            // it to keep running. Dropping it closes the server, so let's not drop it!
            #[allow(clippy::mem_forget)]
            std::mem::forget(puffin_server);
        }
        Err(err) => {
            eprintln!("Failed to start puffin server: {err}");
        }
    };
}
