use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(800, 600);
        WindowBuilder::new()
            .with_title("Nanometers")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(800, 600, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(scale_factor) = input.scale_factor() {
                todo!("Scale factor")
            }

            if let Some(size) = input.window_resized() {}
        }

        match event {
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            }
            Event::WindowEvent { event, .. } => {}
            _ => {}
        }
    });
}
