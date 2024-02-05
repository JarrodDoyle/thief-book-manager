use std::{path::PathBuf, sync::Arc};

use iced::{
    executor,
    widget::{button, column, row, scrollable, text, text_editor, vertical_rule, Column},
    Application, Command, Element, Length, Theme,
};

use crate::{
    file::{default_file, load_file, load_folder},
    project::ProjectState,
    Error,
};

#[derive(Debug, Clone)]
pub enum Message {
    Open,
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    FolderSelected(Result<PathBuf, Error>),
}

pub struct BookManagerApp {
    project: Option<ProjectState>,
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
                project: None,
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
            Message::FolderSelected(result) => match result {
                Ok(path) => self.project = Some(ProjectState::new(path)),
                Err(error) => self.io_error = Some(error),
            },
            Message::Open => {
                return Command::perform(load_folder(), Message::FolderSelected);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let hello_world = text("Hello, world!");
        let controls = row![button("Open").on_press(Message::Open)];
        let active_editor = text_editor(&self.book_content).on_action(Message::Edit);

        let mut books: Vec<Element<'_, Self::Message>> = vec![];
        if let Some(project) = &self.project {
            for book in project.books.iter() {
                books.push(text(book.file_name.clone()).into());
            }
        }

        let left_panel = scrollable(Column::with_children(books)).width(Length::Fixed(256.0));
        let right_panel = column![hello_world, controls, active_editor]
            .spacing(10)
            .padding(10);

        row![left_panel, vertical_rule(2), right_panel].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
