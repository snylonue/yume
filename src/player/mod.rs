pub mod renderer;
pub mod playlist;

use std::path::Path;
use playlist::Playlist;
use renderer::Renderer;
use winit::{window::Window, dpi::PhysicalSize};

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
        self.renderer.resize(size);
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.renderer.size
    }
}