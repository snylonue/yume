pub mod playlist;
pub mod renderer;

use clap::ArgMatches;
use playlist::Playlist;
use renderer::{texture::Rgba8Image, Pan, Renderer};
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use winit_input_helper::WinitInputHelper;

pub struct Player {
    renderer: Renderer,
    playlist: Playlist,
    window: Window,
    input: WinitInputHelper,
}

impl Player {
    pub async fn new(window: Window, arg: ArgMatches<'_>) -> Self {
        let (playlist, renderer) = match arg.value_of("image") {
            Some(p) => {
                let mut sources = Vec::new();
                playlist::read_dir(p.as_ref(), &mut sources).unwrap();
                let playlist = Playlist::new(sources);
                let img = playlist.current_image().unwrap();
                let renderer = Renderer::new(&window, &img).await;
                (playlist, renderer)
            }
            None => {
                let playlist = Playlist::new(vec![]);
                (playlist, Renderer::idle(&window).await)
            }
        };

        Self {
            renderer,
            playlist,
            window,
            input: WinitInputHelper::new(),
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>) -> ! {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match &event {
                Event::RedrawRequested(_) => match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.size()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                },
                Event::WindowEvent {
                    event: WindowEvent::DroppedFile(path),
                    ..
                } => {
                    self.playlist.load_path(path).unwrap();
                    self.update_image();
                }
                _ => {}
            }

            if self.input.update(&event) {
                if self.input.window_resized().is_some()
                    || self.input.scale_factor_changed().is_some()
                {
                    self.resize(self.window.inner_size());
                }

                if self.input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                self.handle_input();

                self.window.request_redraw();
            }
        })
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render()
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.renderer.scale = 1.0;
        self.renderer.resize(size);
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.renderer.size
    }

    pub fn handle_input(&mut self) {
        if self.input.key_pressed(VirtualKeyCode::Left) {
            self.playlist.advance(-1);
            self.update_image();
        }

        if self.input.key_pressed(VirtualKeyCode::Right) {
            self.playlist.advance(1);
            self.update_image();
        }

        if self.input.key_pressed(VirtualKeyCode::S) {
            self.handle_scale_to_fit();
        }

        if self.input.key_pressed(VirtualKeyCode::H) {
            self.renderer.pan.increase_width(1);
            self.renderer.reconfigure_vertex_buffer();
        }

        if self.input.key_pressed(VirtualKeyCode::L) {
            self.renderer.pan.increase_width(-1);
            self.renderer.reconfigure_vertex_buffer();
        }

        if self.input.key_pressed(VirtualKeyCode::J) {
            self.renderer.pan.increase_height(1);
            self.renderer.reconfigure_vertex_buffer();
        }

        if self.input.key_pressed(VirtualKeyCode::K) {
            self.renderer.pan.increase_height(-1);
            self.renderer.reconfigure_vertex_buffer();
        }

        if self.input.mouse_held(0) {
            let (dx, dy) = self.input.mouse_diff();
            self.renderer.pan.increase_width(-dx as i32);
            self.renderer.pan.increase_height(-dy as i32);
            self.renderer.reconfigure_vertex_buffer();
        }

        let scroll_diff = self.input.scroll_diff();
        if scroll_diff.abs() >= f32::EPSILON {
            // todo: multiply a factor set by user
            self.renderer.add_scale(scroll_diff * 0.1);
        }
    }

    fn update_image(&mut self) {
        let img = match self.playlist.current_image() {
            Ok(img) => img,
            _ => Rgba8Image::new(1, 1),
        };
        self.renderer.pan = Pan::default();
        self.renderer.update_image(&img);
        self.renderer.set_scale(self.scale_to_fit().min(1f32))
    }

    fn scale_to_fit(&self) -> f32 {
        let src = self.renderer.surface_size();
        let dst = self.renderer.texture_size();

        (src.width as f32 / dst.width as f32).min(src.height as f32 / dst.height as f32)
    }

    pub fn handle_scale_to_fit(&mut self) {
        self.renderer.set_scale(self.scale_to_fit());
    }
}
