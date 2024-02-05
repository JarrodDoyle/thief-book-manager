mod application;
mod file;
mod project;

use std::io;

use application::BookManagerApp;
use iced::{Application, Settings};

fn main() -> iced::Result {
    BookManagerApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}
