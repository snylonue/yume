mod cli;

use std::path::Path;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::WindowBuilder,
};
use yume::player::Player;

const HEIGHT: u32 = 540;
const WIDTH: u32 = 960;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::app().get_matches();

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        // let scaled_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("yume")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pl = pollster::block_on(async {
        Player::new(window, Path::new(args.value_of("image").unwrap())).await
    });

    pl.run(event_loop)
}
