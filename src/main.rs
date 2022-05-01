mod cli;

use std::path::Path;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use yume::player::Player;

const HEIGHT: u32 = 540;
const WIDTH: u32 = 960;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::app().get_matches();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        // let scaled_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("yume")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pl = pollster::block_on(async {
        Player::new(&window, Path::new(args.value_of("image").unwrap())).await
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::RedrawRequested(_) = event {
            match pl.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => pl.resize(pl.size()),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }

        if input.update(&event) {
            if let Some(_) = input.window_resized() {
                let size = window.inner_size();
                pl.resize(size);
            }

            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                pl.handle_playlist_change(VirtualKeyCode::Left);
            }

            if input.key_pressed(VirtualKeyCode::Right) {
                pl.handle_playlist_change(VirtualKeyCode::Right);
            }

            if input.key_pressed(VirtualKeyCode::S) {
                pl.handle_scale_to_fit();
            }

            window.request_redraw();
        }
    })
}
