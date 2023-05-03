extern crate rand;
use serde::{Serialize, Deserialize};
use std::fs::File;

use std::io::prelude::*;

use rand::Rng;

use iced::executor;
use iced::widget::{button, checkbox, column, Column, Text, row};
use std::path::{Path, PathBuf};
use iced::{
    Application, Command, Element, Settings, Theme,
};


#[derive(Debug, Clone)]
enum Mode {
    Initial,
    FindFile,
    LoadingFile(String),
    UseQuestions,
    BeforeScoring
}

#[derive(Debug, Clone)]
struct ListOfQuestion {
    questions: Vec<CheckableQuestion>,
    current_page: usize,
}

impl ListOfQuestion {
    fn current_question(&self) -> &CheckableQuestion {
        &(self.questions[self.current_page])
    }

    fn is_first_page(&self) -> bool {
        self.current_page == 0
    }

    fn is_last_page(&self) -> bool {
        self.current_page == self.questions.len() - 1
    }

    fn next_page(&self) -> ListOfQuestion {
        ListOfQuestion {
            questions: self.questions.to_vec(),
            current_page: self.current_page + 1,
        }
    }

    fn prev_page(&self) -> ListOfQuestion {
        ListOfQuestion {
            questions: self.questions.to_vec(),
            current_page: self.current_page - 1,
        }
    }
}

impl Default for Mode {
    fn default() -> Self { Self::Initial }
}

#[derive(Default)]
struct MyApplication {
    mode: Mode,
    checkable_questions: Vec<CheckableQuestion>,
    num_current_page: usize,
}

impl MyApplication {
    fn current_question(&self) -> &CheckableQuestion {
        &(self.checkable_questions[self.num_current_page])
    }

    fn is_last_page(&self) -> bool {
        self.num_current_page == self.checkable_questions.len() - 1
    }

    fn is_first_page(&self) -> bool {
        self.num_current_page == 0
    }

    fn next_page(&mut self) {
        self.num_current_page += 1;
    }

    fn prev_page(&mut self) {
        self.num_current_page -= 1;
    }


}

#[derive(Debug, Clone)]
enum Message {
    FindButtonClicked,
    FileSelected(String),
    QuestionsShuffled(ListOfQuestion),
    OpenPage,
    NextPage,
    PrevPage,
    Toggled(bool),
    OptionMessage(usize, usize, OptionMessage),
    ToBeforeScoring
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
                self.mode = Mode::LoadingFile(file_name.clone());
                let file = File::open(file_name.clone()).unwrap();
                let questions: Vec<Question> = QuestionsCreator::create_from_file(file);
                self.checkable_questions = CheckableQuestionsCreator::from_questions(questions);
                Command::none()
            },
            Message::QuestionsShuffled(use_questions_state) => {
                self.mode = Mode::UseQuestions;
                self.checkable_questions = use_questions_state.questions;
                Command::none()
            },
            Message::OpenPage => {
                self.mode = Mode::UseQuestions;
                Command::none()
            },
            Message::NextPage => {
                dbg!("NextPage");
                self.next_page();
                Command::none()
            },
            Message::PrevPage => {
                dbg!("PrevPage");
                self.prev_page();
                Command::none()
            },
            Message::Toggled(_checked) => {
                dbg!("Toggled");
                Command::none()
            },
            Message::OptionMessage(idx_question, idx_option, checked) => {
                dbg!("BOOL BEFORE {}", self.checkable_questions[idx_question].options_for_select[idx_option].checked);
                self.checkable_questions[idx_question].options_for_select[idx_option].checked ^= true;
                dbg!("BOOL AFTER {}", self.checkable_questions[idx_question].options_for_select[idx_option].checked);
                Command::none()
            },
            Message::ToBeforeScoring => {
                self.mode = Mode::BeforeScoring;
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

                let path_to_dir: PathBuf = Path::new(
                    format!("{}", shellexpand::tilde("~/Documents/kioka")).as_str()
                    ).to_path_buf();
                //let path = path_to_dir.as_path();
                //let p: Option<&Path> = Some(&path);
                //self.current_file_path = Box::new(p);
                let mut col = Column::new();
                col = col.push(
                    Text::new(path_to_dir.as_path().to_string_lossy().to_string())
                    );
                for entry in path_to_dir.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        let button = button(Text::new(
                                entry.path().as_path().to_string_lossy().to_string() 
                                                      ) )
                            .on_press(Message::FileSelected(
                                    entry.path().as_path().to_string_lossy().to_string()  
                                                             ));
                        col = col.push(button);

                    }
                }
                col.into()
            },
            Mode::LoadingFile(file_name) => {
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
                let file = File::open(file_path).unwrap();

                
                //let mut questions: Vec<Question> = 
                //    serde_yaml::from_str(before_serialize.as_str()).unwrap();

                let mut column = Column::new();
                column = column.push(button("Use questions")
                               .on_press(Message::QuestionsShuffled(
                    ListOfQuestion {
                        questions: self.checkable_questions.clone(),
                        current_page: 0,
                    }
                            )));
                column.into()
            },
            Mode::UseQuestions => {
                let mut col = Column::new();
                col = col.push(
                    Text::new(self.current_question().text.as_str())
                    );


                col = col.push(column(
                self.current_question().options_for_select
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        option.view(i).map(move |message| {
                            Message::OptionMessage(self.current_question().idx, i, message)
                        })
                    }).collect()

                    ,));
                //for opt in opts.to_vec().iter() {
                //    col = col.push(*opt);
                //}

                let mut prev_button = button("Prev");
                if !self.is_first_page() {
                    prev_button = 
                        prev_button.on_press(Message::PrevPage);
                }

                let mut next_button = button("Next");
                dbg!("is last page? {}", self.is_last_page());
                if !self.is_last_page() {
                    next_button = 
                        next_button.on_press(Message::NextPage);
                } else {
                    next_button = 
                        next_button.on_press(Message::ToBeforeScoring);
                }

                col = col.push(prev_button).push(next_button);
                col.into()
            },
            Mode::BeforeScoring => {
                let mut col = Column::new();
                col = col.push(Text::new("You have answered all questions."));
                let mut prev_button = button("Prev");
                if !self.is_first_page() {
                    prev_button = 
                        prev_button.on_press(Message::OpenPage);
                }

                let mut scoring_button = button("Scoring");

                col = col.push(prev_button).push(scoring_button);
                col.into()
            }
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuestionsCreator {}

impl QuestionsCreator {
    fn create_from_file(mut file: File) -> Vec<Question> {
        let mut before_serialize = String::new();
        file.read_to_string(&mut before_serialize);
        let mut loaded_questions: Vec<Question> =
            serde_yaml::from_str(before_serialize.as_str()).unwrap();
        for question in loaded_questions.iter_mut() {
            question.shuffle_options();
        }
        shuffle(loaded_questions.to_vec())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Question {
    text: String,
    options_for_select: Vec<OptionForSelect>,
}

impl Question {
    fn shuffle_options(&mut self) {
        self.options_for_select = shuffle(self.options_for_select.to_vec());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OptionForSelect {
    text: String,
    truthy: bool,
}

impl OptionForSelect {
    fn to_checkable(&self) -> CheckableOptionForSelect {
        CheckableOptionForSelect {
            text: self.text.clone(),
            truthy: self.truthy,
            checked: false,
        }

    }
}

#[derive(Debug, Clone)]
struct CheckableQuestion {
    text: String,
    idx: usize,
    options_for_select: Vec<CheckableOptionForSelect>
}

#[derive(Debug, Clone)]
struct CheckableOptionForSelect {
    text: String,
    truthy: bool,
    checked: bool,
}

#[derive(Debug, Clone)]
enum OptionMessage {
    Change(bool),
}

impl CheckableOptionForSelect {
    fn update(&mut self, message: OptionMessage) {
        match message {
            OptionMessage::Change(checked) => {
                dbg!("IN OptionMessage");
                self.checked = checked;
            }
        }
    }

    fn view(&self, i: usize) -> Element<OptionMessage> {
                dbg!("IN view");
        row![checkbox(self.text.clone(), self.checked, OptionMessage::Change)].into()
    }
}

struct CheckableQuestionsCreator {}

impl CheckableQuestionsCreator {
    fn from_questions(questions: Vec<Question>) -> Vec<CheckableQuestion> {
        let mut return_questions: Vec<CheckableQuestion> = vec![];
        for (i, question) in questions.iter().enumerate() {
            let mut options: Vec<CheckableOptionForSelect> = vec![];
            for option in question.options_for_select.iter() {
                options.push(option.to_checkable());
            }
            return_questions.push(
                CheckableQuestion {
                    text: question.text.clone(),
                    idx: i,
                    options_for_select: options
                }
                );
        }
        return_questions
    }
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

fn shuffle<T: std::clone::Clone>(origin: Vec<T>) -> Vec<T> {
    let mut copy = origin.to_vec();
    let mut shuffled = vec![];
    let mut current = random_remove_from(&mut copy);
    while current.is_some() {
        shuffled.push(current.unwrap());
        current = random_remove_from(&mut copy);
    }
    shuffled
}



fn main() -> iced::Result {
    println!("TEST01");
    MyApplication::run(Settings {
        default_font: Some(include_bytes!("./fonts/ipaexgoth.ttf")),
        ..Settings::default()
    })
}
