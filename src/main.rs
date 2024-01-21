use std::{io, path::Path, sync::Arc};

use iced::{
    executor,
    widget::{button, column, row, text, text_editor},
    Application, Command, Element, Settings, Theme,
};

#[derive(Debug, Clone)]
enum Message {
    Open,
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, Error>),
}

struct BookManagerApp {
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
                book_content: text_editor::Content::new(),
                io_error: None,
            },
            Command::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Thief Book Manager")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Edit(action) => self.book_content.perform(action),
            Message::FileOpened(result) => match result {
                Ok(content) => self.book_content = text_editor::Content::with_text(&content),
                Err(error) => self.io_error = Some(error),
            },
            Message::Open => {
                return Command::perform(pick_file(), Message::FileOpened);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let controls = row![button("Open").on_press(Message::Open)];
        let hello_world = text("Hello, world!");
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

fn main() -> iced::Result {
    BookManagerApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)
}

async fn pick_file() -> Result<Arc<String>, Error> {
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    load_file(file_handle.path()).await
}
