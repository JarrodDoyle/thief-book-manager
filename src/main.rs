use iced::{
    widget::{column, text, text_editor},
    Element, Result, Sandbox, Settings,
};

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

struct Application {
    book_content: text_editor::Content,
}

impl Sandbox for Application {
    type Message = Message;

    fn new() -> Self {
        Self {
            book_content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Thief Book Manager")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => self.book_content.perform(action),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let hello_world = text("Hello, world!");
        let active_editor = text_editor(&self.book_content).on_action(Message::Edit);

        column![hello_world, active_editor]
            .spacing(10)
            .padding(10)
            .into()
    }
}

fn main() -> Result {
    Application::run(Settings::default())
}
