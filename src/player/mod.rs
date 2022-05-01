pub mod playlist;
pub mod renderer;

use playlist::Playlist;
use renderer::Renderer;
use std::path::Path;
use winit::{dpi::PhysicalSize, event::VirtualKeyCode, window::Window};

pub struct Player {
    renderer: Renderer,
    playlist: Playlist,
}

impl Player {
    pub async fn new(window: &Window, p: &Path) -> Self {
        let mut sources = Vec::new();
        playlist::read_dir(p, &mut sources).unwrap();
        let playlist = Playlist::new(sources);
        let init_image = playlist.current().unwrap();
        let img = image::open(init_image).unwrap().to_rgba8();
        let renderer = Renderer::new(window, &img).await;

        Self { renderer, playlist }
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

    pub fn handle_playlist_change(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Left => self.playlist.advance(-1),
            VirtualKeyCode::Right => self.playlist.advance(1),
            _ => {}
        }
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
