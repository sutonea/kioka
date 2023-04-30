extern crate rand;
use serde::{Serialize, Deserialize};
use std::fs::File;

use std::io::prelude::*;

use rand::Rng;

use iced::executor;
use iced::widget::{button, checkbox, Column, Text};
use std::path::{Path, PathBuf};
use iced::{
    Application, Command, Element, Settings, Theme,
};


#[derive(Debug, Clone)]
enum Mode {
    Initial,
    FindFile,
    LoadingFile(String),
    UseQuestions(UseQuestionsState),
}

#[derive(Debug, Clone)]
struct UseQuestionsState {
    questions: Vec<Question>,
    current_page: usize,
}

impl UseQuestionsState {
    fn current_question(&self) -> Question {
        self.questions[self.current_page].clone()
    }

    fn is_first_page(&self) -> bool {
        self.current_page == 0
    }

    fn is_last_page(&self) -> bool {
        self.current_page == self.questions.len() - 1
    }

    fn next_page(&self) -> UseQuestionsState {
        UseQuestionsState {
            questions: self.questions.to_vec(),
            current_page: self.current_page + 1,
        }
    }

    fn prev_page(&self) -> UseQuestionsState {
        UseQuestionsState {
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
}

#[derive(Debug, Clone)]
enum Message {
    FindButtonClicked,
    FileSelected(String),
    QuestionsShuffled(UseQuestionsState),
    OpenPage(UseQuestionsState),
    Toggled(bool)
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
                self.mode = Mode::LoadingFile(file_name);
                Command::none()
            },
            Message::QuestionsShuffled(use_questions_state) => {
                self.mode = Mode::UseQuestions(use_questions_state);
                Command::none()
            },
            Message::OpenPage(use_questions_state) => {
                self.mode = Mode::UseQuestions(use_questions_state);
                Command::none()
            },
            Message::Toggled(_checked) => {
                Command::none()
            },

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

                let questions: Vec<Question> = QuestionsCreator::create_from_file(file);
                let shuffled_questions: Vec<Question> = shuffle(questions);
                let mut column = Column::new();
                column = column.push(button("Use questions")
                               .on_press(Message::QuestionsShuffled(
                    UseQuestionsState {
                        questions: shuffled_questions,
                        current_page: 0,
                    }
                            )));
                column.into()
            },
            Mode::UseQuestions(use_questions_state) => {
                let mut column = Column::new();
                column = column.push(
                    Text::new(use_questions_state.current_question().text)
                    );

                let options_for_select = 
                    use_questions_state
                        .current_question()
                            .options_for_select.to_vec();

                let shuffled_options = shuffle(options_for_select);

                for opt in shuffled_options.iter() {
                    column = column.push(
                        checkbox(opt.clone().text, false, Message::Toggled)
                        );
                }


                let mut prev_button = button("Prev");
                if !use_questions_state.is_first_page() {
                    prev_button = 
                        prev_button.on_press(
                            Message::OpenPage(
                                use_questions_state.prev_page()
                                )
                            );
                }
                let mut next_button = button("Next");
                if !use_questions_state.is_last_page() {
                    next_button = 
                        next_button.on_press(
                            Message::OpenPage(
                                use_questions_state.next_page()
                                )
                            )
                }
                column = column.push(prev_button).push(next_button);
                column.into()
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
        serde_yaml::from_str(before_serialize.as_str()).unwrap()
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
    MyApplication::run(Settings::default())
}
