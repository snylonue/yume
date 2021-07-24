use iced::{executor, Application, Command, Error};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct Yume {
    playlist: Playlist,
}

#[derive(Debug, Clone)]
pub enum Message {
    NextImg,
    PrevImg,
}

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
        &self.sources[self.pos]
    }
    pub fn current(&self) -> &Path {
        &self.sources[self.pos]
    }
}

impl Application for Yume {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("yume")
    }

    fn update(
        &mut self,
        _: Self::Message,
        _: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        // todo
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}

fn main() -> Result<(), Error> {
    Yume::run(Default::default())
}
