use crate::player::renderer::texture::Rgba8Image;
use std::path::{Path, PathBuf};
use walkdir::{Error, WalkDir};

#[derive(Debug, Clone, Default)]
pub struct Playlist {
    pub sources: Vec<PathBuf>,
    pub pos: usize,
}

impl Playlist {
    pub fn new(sources: Vec<PathBuf>) -> Self {
        Self { sources, pos: 0 }
    }

    pub fn current(&self) -> Option<&PathBuf> {
        self.sources.get(self.pos)
    }

    pub fn current_image(&self) -> Option<Rgba8Image> {
        self.current().map(|p| image::open(p).unwrap().to_rgba8())
    }

    pub fn advance(&mut self, d: isize) {
        self.pos = match self.sources.len() {
            0 => 0,
            len => (self.pos as isize + d + len as isize) as usize % len,
        };
    }
}

pub fn read_dir(path: &Path, sources: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in WalkDir::new(path).min_depth(1).into_iter() {
        let entry = entry?;
        if entry.file_type().is_file() {
            sources.push(entry.path().to_owned());
        }
    }
    Ok(())
}
