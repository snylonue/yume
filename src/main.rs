mod cli;

use iced::{button, executor, keyboard, Application, Command, Error, Image, Row, Settings, Text};
use iced_native::subscription;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct Yume {
    playlist: Playlist,
    next_img_button: button::State,
    prev_img_button: button::State,
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
        &self.sources[self.pos % self.sources.len()]
    }
    pub fn current(&self) -> &Path {
        &self.sources[self.pos]
    }
    pub fn pos_delta(&mut self, d: isize) {
        self.pos = (self.pos as isize + d) as usize % self.sources.len();
    }
}

impl Application for Yume {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = Playlist;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                playlist: flags,
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("yume")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::NextImg => self.playlist.pos_delta(1),
            Message::PrevImg => self.playlist.pos_delta(-1),
        };
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // todo: move keyboard input handling to `Message`
        subscription::events_with(|ev, _| match ev {
            iced_native::Event::Keyboard(keyboard::Event::CharacterReceived(',')) => {
                Some(Message::PrevImg)
            }
            iced_native::Event::Keyboard(keyboard::Event::CharacterReceived('.')) => {
                Some(Message::NextImg)
            }
            _ => None,
        })
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Row::new()
            .push(
                button::Button::new(&mut self.prev_img_button, Text::new("<"))
                    .on_press(Message::PrevImg),
            )
            .push(Image::new(self.playlist.current()))
            .push(
                button::Button::new(&mut self.next_img_button, Text::new(">"))
                    .on_press(Message::NextImg),
            )
            .into()
    }
}

fn main() -> Result<(), Error> {
    let images = cli::app()
        .get_matches()
        .values_of("image")
        .unwrap()
        .map(Into::into)
        .collect();
    let playlist = Playlist::new(images);
    Yume::run(Settings::with_flags(playlist))
}
