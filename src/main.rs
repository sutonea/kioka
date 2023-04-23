use dirs::home_dir;
use iced::executor;
use iced::widget::{button, Column, Text};
use std::path::{Path, PathBuf};
use iced::{
    Application, Command, Element, Settings, Theme,
};


#[derive(Debug, Clone)]
enum Mode {
    Initial,
    FindFile,
    UseFile,
}

impl Default for Mode {
    fn default() -> Self { Self::Initial }
}

#[derive(Default)]
struct MyApplication {
    mode: Mode,
}

#[derive(Debug, Clone)]
enum Message {
    FindButtonClicked,
    FileSelected,
}

impl Application for MyApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("MyApplication")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::FindButtonClicked => {
                self.mode = Mode::FindFile;
                Command::none()
            },
            Message::FileSelected => {
                self.mode = Mode::UseFile;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.mode {
            Mode::Initial => {
                let button = button("Find file").on_press(Message::FindButtonClicked);
                let column = Column::new().push(button);
                column.into()
            },
            Mode::FindFile => {
                let path_to_dir: PathBuf = Path::new(format!("{}", shellexpand::tilde("~/Documents/kioka")).as_str()).to_path_buf();
                let mut col = Column::new();
                col = col.push(Text::new(path_to_dir.as_path().to_string_lossy().to_string()));
                for entry in path_to_dir.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        let button = button(Text::new(entry.path().as_path().to_string_lossy().to_string() ) ).on_press(Message::FileSelected);
                        col = col.push(button);

                    }
                }
                col.into()
            },
            Mode::UseFile => {

                let button = button("Find file 3").on_press(Message::FindButtonClicked);
                let column = Column::new().push(button);
                column.into()
            },
        }
    }
}


fn main() -> iced::Result {
    MyApplication::run(Settings::default())
}
