use std::{path::PathBuf, sync::Arc};

use iced::{
    executor,
    widget::{button, column, row, text, text_editor},
    Application, Command, Element, Theme,
};

use crate::{
    file::{default_file, load_file, pick_file},
    Error,
};

#[derive(Debug, Clone)]
pub enum Message {
    Open,
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
}

pub struct BookManagerApp {
    book_path: Option<PathBuf>,
    book_content: text_editor::Content,
    io_error: Option<Error>,
}

impl Application for BookManagerApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                book_path: None,
                book_content: text_editor::Content::new(),
                io_error: None,
            },
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String {
        String::from("Thief Book Manager")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Edit(action) => self.book_content.perform(action),
            Message::FileOpened(result) => match result {
                Ok((path, content)) => {
                    self.book_content = text_editor::Content::with_text(&content);
                    self.book_path = Some(path);
                }
                Err(error) => self.io_error = Some(error),
            },
            Message::Open => {
                return Command::perform(pick_file(), Message::FileOpened);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let hello_world = text("Hello, world!");
        let controls = row![button("Open").on_press(Message::Open)];
        let active_editor = text_editor(&self.book_content).on_action(Message::Edit);

        column![hello_world, controls, active_editor]
            .spacing(10)
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
