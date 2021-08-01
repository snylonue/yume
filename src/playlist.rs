use std::path::PathBuf;

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
    pub fn advance(&mut self, d: isize) {
        self.pos = match self.sources.len() {
            0 => 0,
            len => (self.pos as isize + d) as usize % len,
        };
    }
}
