mod cli;
mod playlist;

use iced::{
    button, executor, keyboard, Application, Command, Error, Image, Row, Settings, Space, Text,
};
use iced_native::subscription;
use crate::playlist::Playlist;

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
            Message::NextImg => self.playlist.advance(1),
            Message::PrevImg => self.playlist.advance(-1),
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
            .push(Space::with_width(iced::Length::Fill))
            .push(Image::new(self.playlist.current()))
            .push(Space::with_width(iced::Length::Fill))
            .push(
                button::Button::new(&mut self.next_img_button, Text::new(">"))
                    .on_press(Message::NextImg),
            )
            .align_items(iced::Align::Center)
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
