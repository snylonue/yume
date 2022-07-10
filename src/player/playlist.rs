use crate::player::renderer::texture::Rgba8Image;
use std::path::{Path, PathBuf};
use walkdir::{Error, WalkDir};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pos {
    Start,
    Normal(usize),
    End,
}

impl Pos {
    pub const fn is_normal(&self) -> bool {
        matches!(self, Pos::Normal(_))
    }

    pub fn advance(&mut self, d: isize, len: usize) {
        match self {
            Self::Start if d > 0 => *self = Self::Normal(d as usize - 1),
            Self::End if d < 0 => *self = Self::Normal((len as isize - d) as usize),
            Self::Normal(pos) => {
                let new_pos = *pos as isize + d;
                *self = if new_pos < 0 {
                    Self::Start
                } else if new_pos >= len as isize {
                    Self::End
                } else {
                    Self::Normal(new_pos as usize)
                };
            },
            _ => {}
        }
    }
    pub const fn to_index(&self) -> Option<usize> {
        match self {
            Self::Start | Self::End => None,
            Self::Normal(p) => Some(*p),
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self::Start
    }
}

pub enum Entry {
    Item(PathBuf),
    SubList(Playlist)
}

#[derive(Debug, Clone, Default)]
pub struct Playlist {
    pub sources: Vec<PathBuf>,
    pub pos: Pos,
}

impl Playlist {
    pub fn new(sources: Vec<PathBuf>) -> Self {
        Self { sources, pos: Pos::Start }
    }

    pub fn current(&self) -> Option<&PathBuf> {
        self.sources.get(self.pos.to_index()?)
    }

    pub fn current_image(&self) -> Option<Rgba8Image> {
        self.current().map(|p| image::open(p).unwrap().to_rgba8())
    }

    pub fn advance(&mut self, d: isize) {
        self.pos.advance(d, self.sources.len())
    }

    pub fn load(&mut self, p: &Path) -> Result<(), Error> {
        self.sources.clear();
        read_dir(p, &mut self.sources)?;
        self.pos = if self.sources.is_empty() {
            Pos::Start
        } else {
            Pos::Normal(0)
        };
        Ok(())
    }
}

pub fn read_dir(path: &Path, sources: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            sources.push(entry.path().to_owned());
        }
    }
    Ok(())
}
