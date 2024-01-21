use iced::{widget::text, Element, Result, Sandbox, Settings};

#[derive(Debug)]
enum Message {}

struct Application;

impl Sandbox for Application {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Thief Book Manager")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, world!").into()
    }
}

fn main() -> Result {
    Application::run(Settings::default())
}
