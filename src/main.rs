use iced::{
    executor,
    widget::{column, text, text_editor},
    Application, Command, Element, Result, Settings, Theme,
};

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

struct BookManagerApp {
    book_content: text_editor::Content,
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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Thief Book Manager")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Edit(action) => self.book_content.perform(action),
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let hello_world = text("Hello, world!");
        let active_editor = text_editor(&self.book_content).on_action(Message::Edit);

        column![hello_world, active_editor]
            .spacing(10)
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> Result {
    BookManagerApp::run(Settings::default())
}
