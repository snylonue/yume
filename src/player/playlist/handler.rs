use std::{path::{PathBuf, Path}, fmt::Debug};
use walkdir::WalkDir;
use crate::Rgba8Image;
use super::Pos;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Playlist {
    pub(crate) items: Vec<Box<dyn Handler>>,
    pub(crate) pos: Pos
}

impl Playlist {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        fn new_boxed_handler(p: PathBuf) -> Box<dyn Handler> {
            Box::new(p)
        }
        let items: Vec<Box<dyn Handler>> = paths.into_iter().map(new_boxed_handler).collect();
        let pos = if items.is_empty() {
            Pos::Start
        } else {
            Pos::Normal(0)
        };
        Self { items, pos }
    }

    fn current(&self) -> Option<&Box<dyn Handler>> {
        self.items.get(self.pos.to_index()?)
    }

    pub fn current_image(&self) -> Result<Rgba8Image> {
        self.current().ok_or("no image")?.handle()
    }

    pub fn advance(&mut self, d: isize) {
        self.pos.advance(d, self.items.len())
    }

    pub fn load_path(&mut self, p: &Path) -> Result<()> {
        self.items = p.parse()?;
        self.pos = Pos::Normal(0);
        Ok(())
    }
}

pub trait Handler: Debug {
    fn handle(&self) -> Result<Rgba8Image>;
}

impl Handler for PathBuf {
    fn handle(&self) -> Result<Rgba8Image> {
        Ok(image::open(self)?.to_rgba8())
    }
}

impl Handler for Path {
    fn handle(&self) -> Result<Rgba8Image> {
        Ok(image::open(self)?.to_rgba8())
    }
}

pub trait Parser {
    fn parse(&self) -> Result<Vec<Box<dyn Handler>>>;
}

impl Parser for Path {
    fn parse(&self) -> Result<Vec<Box<dyn Handler>>> {
        if self.is_file() {
            return Ok(vec![Box::new(self.to_path_buf())]);
        }

        let mut p: Vec<Box<dyn Handler>> = Vec::new();
        for entry in WalkDir::new(self).max_depth(1).min_depth(1) {
            let entry = entry?;
            if entry.file_type().is_file() {
                p.push(Box::new(entry.path().to_path_buf()));
            }
            if entry.file_type().is_dir() {
                p.extend(entry.path().parse()?)
            }
        }
        Ok(p)
    }
}