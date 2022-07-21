pub mod handler;

use std::path::{Path, PathBuf};
use walkdir::{Error, WalkDir};

pub use handler::Playlist;

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
            }
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

pub fn read_dir(path: &Path, sources: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            sources.push(entry.path().to_owned());
        }
    }
    Ok(())
}
