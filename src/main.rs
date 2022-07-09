mod cli;

use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Icon, WindowBuilder},
};
use yume::player::Player;

const HEIGHT: u32 = 540;
const WIDTH: u32 = 960;
const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::app().get_matches();

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("yume")
            .with_inner_size(size)
            .with_window_icon(Some({
                let icon = image::load_from_memory(ICON_BYTES)?.to_rgba8();
                Icon::from_rgba(icon.to_vec(), icon.width(), icon.height())?
            }))
            .build(&event_loop)?
    };

    let pl = pollster::block_on(async { Player::new(window, args).await });

    pl.run(event_loop)
}
