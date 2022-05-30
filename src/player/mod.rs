pub mod playlist;
pub mod renderer;

use playlist::Playlist;
use renderer::{Renderer, Pan};
use winit_input_helper::WinitInputHelper;
use std::path::Path;
use winit::{dpi::PhysicalSize, event::{VirtualKeyCode, Event}, window::Window, event_loop::{EventLoop, ControlFlow}};

pub struct Player {
    renderer: Renderer,
    playlist: Playlist,
    window: Window,
    input: WinitInputHelper,
}

impl Player {
    pub async fn new(window: Window, p: &Path) -> Self {
        let mut sources = Vec::new();
        playlist::read_dir(p, &mut sources).unwrap();
        let playlist = Playlist::new(sources);
        let init_image = playlist.current().unwrap();
        let img = image::open(init_image).unwrap().to_rgba8();
        let renderer = Renderer::new(&window, &img).await;

        Self { renderer, playlist, window, input: WinitInputHelper::new() }
    }

    pub fn run(mut self, event_loop: EventLoop<()>) -> ! {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            if let Event::RedrawRequested(_) = event {
                match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.size()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
    
            if self.input.update(&event) {
                if self.input.window_resized().is_some() || self.input.scale_factor_changed().is_some() {
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
            self.renderer.pan.decrease_width(1);
            self.renderer.reconfigure_vertex_buffer();
        }

        if self.input.key_pressed(VirtualKeyCode::J) {
            self.renderer.pan.increase_height(1);
            self.renderer.reconfigure_vertex_buffer();
        }

        if self.input.key_pressed(VirtualKeyCode::K) {
            self.renderer.pan.decrease_height(1);
            self.renderer.reconfigure_vertex_buffer();
        }
    }

    fn update_image(&mut self) {
        let img = self.playlist.current_image().unwrap();
        self.renderer.scale = 1.0;
        self.renderer.update_image(&img);
    }

    pub fn handle_scale_to_fit(&mut self) {
        let src = self.renderer.surface_size();
        let dst = self.renderer.texture_size();

        let f = (src.width as f32 / dst.width as f32).min(src.height as f32 / dst.height as f32);
        self.renderer.scale = f;
        self.renderer.reconfigure_vertex_buffer();
    }
}
