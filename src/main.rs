extern crate rand;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

use rand::Rng;
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
    UseFile(String),
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
    FileSelected(String),
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
            Message::FileSelected(file_name) => {
                self.mode = Mode::UseFile(file_name);
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
                //let path = path_to_dir.as_path();
                //let p: Option<&Path> = Some(&path);
                //self.current_file_path = Box::new(p);
                let mut col = Column::new();
                col = col.push(Text::new(path_to_dir.as_path().to_string_lossy().to_string()));
                for entry in path_to_dir.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        let button = button(Text::new(entry.path().as_path().to_string_lossy().to_string() ) )
                            .on_press(Message::FileSelected( entry.path().as_path().to_string_lossy().to_string()  ));
                        col = col.push(button);

                    }
                }
                col.into()
            },
            Mode::UseFile(file_name) => {
                //let mut questions = vec![
                //    Question {
                //        text: "Sample question 1".to_string(),
                //        options_for_select: vec![
                //            OptionForSelect { text: "A".to_string(), truthy: true },
                //            OptionForSelect { text: "B".to_string(), truthy: false },
                //        ]
                //    },
                //    Question {
                //        text: "Sample question 2".to_string(),
                //        options_for_select: vec![
                //            OptionForSelect { text: "X".to_string(), truthy: true },
                //            OptionForSelect { text: "Y".to_string(), truthy: false },
                //        ]
                //    },
                //];
                //let out_str = serde_yaml::to_string(&questions).unwrap();
                //let mut file = File::create(format!("{}", shellexpand::tilde("/tmp/kioka_example.yml"))).unwrap();
                //file.write_all(out_str.as_bytes()).unwrap();

                let file_path = format!("{}", shellexpand::tilde(file_name));
                let mut file = File::open(file_path).unwrap();
                let mut before_serialize = String::new();
                file.read_to_string(&mut before_serialize);

                let mut questions: Vec<Question> = serde_yaml::from_str(before_serialize.as_str()).unwrap();
                let mut next_question: Option<Question> = random_remove_from(&mut questions);
                let mut column = Column::new();
                while next_question.is_some() {
                    column = column.push(Text::new(next_question.unwrap().text));
                    next_question = random_remove_from(&mut questions);
                }
                column.into()
            },
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Question {
    text: String,
    options_for_select: Vec<OptionForSelect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OptionForSelect {
    text: String,
    truthy: bool,
}

fn random_remove_from<T>(from: &mut Vec<T>) -> Option<T> {
    let mut rng = rand::thread_rng();
    //dbg!("dbg before");
    //dbg!(from.len());
    //dbg!("dbg after");
    if from.len() == 0 {
        return None;
    } else {
        let random_number = rng.gen_range(0..(from.len()));
        Some(from.remove(random_number))
    }
}



fn main() -> iced::Result {
    MyApplication::run(Settings::default())
}
