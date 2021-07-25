use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct Playlist {
    pub sources: Vec<PathBuf>,
    pub pos: usize,
}

impl Playlist {
    pub fn new(sources: Vec<PathBuf>) -> Self {
        Self { sources, pos: 0 }
    }
    pub fn next(&mut self) -> &Path {
        self.pos = (self.pos + 1) % self.sources.len();
        &self.sources[self.pos % self.sources.len()]
    }
    pub fn current(&self) -> &Path {
        &self.sources[self.pos]
    }
    pub fn advance(&mut self, d: isize) {
        self.pos = (self.pos as isize + d) as usize % self.sources.len();
    }
}
