extern crate rand;
use serde::{Serialize, Deserialize};
use std::fs::File;

use std::io::prelude::*;

use rand::Rng;

use iced::executor;
use iced::widget::{button, checkbox, column, Column, Text, Row};
use std::path::{Path, PathBuf};
use iced::{
    Application, Command, Element, Settings, Theme,
};


#[derive(Debug, Clone)]
enum Mode {
    FindFile,
    LoadingFile(String),
    UseQuestions,
    BeforeScoring,
    AfterScoring,
}

#[derive(Debug, Clone)]
struct ListOfQuestion {
    questions: Vec<CheckableQuestion>,
}

impl Default for Mode {
    fn default() -> Self { Self::FindFile }
}

#[derive(Default)]
struct MyApplication {
    mode: Mode,
    checkable_questions: Vec<CheckableQuestion>,
    num_current_page: usize,
    is_scored: bool,
    finally_score: usize,
    locale: Locale,
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

    fn to_first_page(&mut self) {
        self.num_current_page = 0;
    }

}

#[derive(Debug, Clone)]
enum Message {
    FileSelected(String),
    QuestionsShuffled(ListOfQuestion),
    OpenPage,
    NextPage,
    PrevPage,
    OptionMessage(usize, usize, OptionMessage),
    ToBeforeScoring,
    Scoring,
    OpenFirstPage,
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
        String::from("KIOKA")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
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
                self.next_page();
                Command::none()
            },
            Message::PrevPage => {
                self.prev_page();
                Command::none()
            },
            Message::OptionMessage(idx_question, idx_option, _checked) => {
                if !self.is_scored {
                    self.checkable_questions[idx_question].options_for_select[idx_option].checked ^= true;
                }
                Command::none()
            },
            Message::ToBeforeScoring => {
                self.mode = Mode::BeforeScoring;
                Command::none()
            },
            Message::Scoring => {
                let count_of_questions = self.checkable_questions.len();
                let mut count_of_correct_answer = 0;
                for question in self.checkable_questions.iter_mut() {
                    if question.is_correct() {
                        count_of_correct_answer += 1;
                    }
                    question.change_to_show_answer();
                }
                self.finally_score = count_of_correct_answer * 100 / count_of_questions;
                self.is_scored = true;
                self.mode = Mode::AfterScoring;
                Command::none()
            },
            Message::OpenFirstPage => {
                self.to_first_page();
                self.mode = Mode::UseQuestions;
                Command::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        match &self.mode {
            Mode::FindFile => {

                let path_to_dir: PathBuf = Path::new(
                    format!("{}", shellexpand::tilde("~/Documents/kioka")).as_str()
                    ).to_path_buf();
                let mut col = Column::new();
                col = col.push(
                    Text::new(path_to_dir.as_path().to_string_lossy().to_string())
                    );
                for entry in path_to_dir.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        let questions = QuestionsCreator::create_from_file(
                            File::open(entry.path().as_path()).unwrap()
                            );
                        let count = questions.len();
                        let button = button(Text::new(
                                format!("{:?}(問題数:{})",entry.path().file_name().unwrap(), count)
                                                      ))
                            .on_press(Message::FileSelected(
                                    entry.path().as_path().to_string_lossy().to_string()  
                                                             ));
                        col = col.push(Column::new().push(button.padding(8)).padding([10, 10, 10, 20]));

                    }
                }
                col.into()
            },
            Mode::LoadingFile(file_name) => {
                let file_path = format!("{}", shellexpand::tilde(file_name));
                let _file = File::open(file_path).unwrap();
                let mut column = Column::new();
                column = column.padding(20).push(button(
                        t("スタート", "Start", self.locale)
                        ).padding(10)
                               .on_press(Message::QuestionsShuffled(
                    ListOfQuestion {
                        questions: self.checkable_questions.clone(),
                    }
                            )).padding(20));
                column.into()
            },
            Mode::UseQuestions => {
                let mut col = Column::new();
                col = col.push(
                    Column::new().push(
                        Text::new(self.current_question().text.as_str())
                    ).padding(20)
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
                if self.is_scored {
                    col = col.push(
                        Column::new().push(
                            Text::new(self.current_question().explanation.clone())
                            ).padding(20)
                        );
                }

                let mut prev_button = button(
                    t("前の問題へ", "Prev question", self.locale)
                    );
                if !self.is_first_page() {
                    prev_button = 
                        prev_button.on_press(Message::PrevPage);
                }

                let mut next_button = button(
                    t("次の問題へ", "Next question", self.locale)
                    );
                if !self.is_last_page() {
                    next_button = 
                        next_button.on_press(Message::NextPage);
                } else {
                    if self.is_scored {

                    next_button = 
                        next_button.on_press(Message::Scoring);

                    } else {
                    next_button = 
                        next_button.on_press(Message::ToBeforeScoring);
                    }
                }

                col = col.push(Row::new().push(Row::new().push(prev_button).padding(10)).push(Row::new().push(next_button).padding(10)));
                col.into()
            },
            Mode::BeforeScoring => {
                let mut col = Column::new();
                col = col.push(Text::new(
                    t("すべての問題に回答しました", "You have answered all questions.", self.locale)
                        ));
                let mut prev_button = button(
                    t("前の問題へ", "Prev question", self.locale)
                    );
                if !self.is_first_page() {
                    prev_button = 
                        prev_button.on_press(Message::OpenPage);
                }

                let scoring_button = button(
                    t("採点する", "Scoring", self.locale)
                    ).
                    on_press(Message::Scoring);

                col = col.push(Column::new().push(prev_button.padding(10)).padding(10)).push(Column::new().push(scoring_button.padding(10)).padding(10));
                col.into()
            },
            Mode::AfterScoring => {
                let mut col = Column::new();
                col = col.push(Text::new(
                    t(
                        format!("正答率 {} ％", self.finally_score).as_str(),
                        format!("Scoring {} %", self.finally_score).as_str(),
                        self.locale
                        ).to_owned()
                    ));
                col = col.push(
                      Column::new().padding(20).push(button(
                    t("最後の問題を確認する", "Last Question", self.locale)
                              ).padding(10).on_press(Message::OpenPage))
                    );

                col = col.push(
                      Column::new().padding(20).push(button(
                    t("最初の問題を確認する", "First Question", self.locale)
                              ).padding(10).on_press(Message::OpenFirstPage))
                    );
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
    explanation: String,
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
            show_answer: false,
        }

    }
}

#[derive(Debug, Clone)]
struct CheckableQuestion {
    text: String,
    idx: usize,
    options_for_select: Vec<CheckableOptionForSelect>,
    explanation: String,
}

impl CheckableQuestion {
    fn is_correct(&self) -> bool {
        for opt in self.options_for_select.iter() {
            if !opt.is_correct() {
                return false
            }
        }
        true
    }

    fn change_to_show_answer(&mut self) {
        for opt in self.options_for_select.iter_mut() {
            opt.change_to_show_answer();
        }
    }
}

#[derive(Debug, Clone)]
struct CheckableOptionForSelect {
    text: String,
    truthy: bool,
    checked: bool,
    show_answer: bool,
}

#[derive(Debug, Clone)]
enum OptionMessage {
    Change(bool),
}

impl CheckableOptionForSelect {
    fn view(&self, _i: usize) -> Element<OptionMessage> {
        if self.show_answer {
            Row::new()
                .push(Text::new(
                        format!(" {}", if self.truthy == self.checked { "OK" } else { "NG" })
                        ))
                .push(
                    checkbox(
                        self.text.clone(), self.checked, OptionMessage::Change
                        )
                    ).padding([10, 10, 10, 20])
                .into()
        } else {
            Row::new()
                .push(
                    checkbox(
                        self.text.clone(), self.checked, OptionMessage::Change
                        )
                    ).padding([10, 10, 30, 20])
                .into()
        }
    }

    fn is_correct(&self) -> bool {
        self.truthy == self.checked
    }

    fn change_to_show_answer(&mut self) {
        self.show_answer = true;
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
                    options_for_select: options,
                    explanation: question.explanation.clone(),
                }
                );
        }
        return_questions
    }
}

fn random_remove_from<T>(from: &mut Vec<T>) -> Option<T> {
    let mut rng = rand::thread_rng();
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
    MyApplication::run(Settings {
        default_font: Some(include_bytes!("./fonts/ipaexgoth.ttf")),
        ..Settings::default()
    })
}

#[derive(Default, Debug, Clone, Copy)]
enum Locale {
    #[default]
    Ja,
    En,
}

fn t<'a>(ja: &'a str, en: &'a str, locale: Locale) -> &'a str {
    match locale {
        Locale::Ja => { ja },
        Locale::En => { en },
    }
}
