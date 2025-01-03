#![allow(dead_code, unused_imports, unused_variables)]
mod keto;

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Application, Element, Task, Theme};
use std::io::ErrorKind;

// TODO: Add the SQLX for saving the Macro Food
// TODO: Add focusable to input widgets and add keyshortcuts tab and shift tab to move around the form
// TODO: Add keyshortcut C-s to save the form

struct Keto {
    marco_name: String,
    macro_name_is_ok: bool,
    carbohydrates: String,
    carbohydrates_f32: f32,
    carbohydrates_hint: String,
    carbohydrates_is_ok: bool,
    protein: String,
    protein_f32: f32,
    protein_hint: String,
    protein_is_ok: bool,
    fat: String,
    fat_f32: f32,
    fat_hint: String,
    fat_is_ok: bool,
    weight: String,
    weight_f32: f32,
    weight_hint: String,
    weight_is_ok: bool,
    kcal: String,
    kcal_f32: f32,
    kcal_hint: String,
    kcal_is_ok: bool,
}

#[derive(Debug, Clone)]
enum FormError {
    NotAf32,
    NotAi16,
    NotAString,
}

#[derive(Debug, Clone)]
enum Message {
    Focus(&'static str),
    MacroNameOnChange(String),
    CarbohydratesResult(Result<f32, Error>),
    CarbohydratesOnChange(String),
    ProteinOnChange(String),
    ProteinResult(Result<f32, Error>),
    FatOnChange(String),
    FatResult(Result<f32, Error>),
    WeightOnChange(String),
    WeightResult(Result<f32, Error>),
    KcalOnChange(String),
    KcalResult(Result<f32, Error>),
    Save,
    SaveResult(Result<String, Error>),
}

impl Keto {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                marco_name: String::new(),
                macro_name_is_ok: false,
                carbohydrates: String::new(),
                carbohydrates_f32: 0.0,
                carbohydrates_hint: String::new(),
                carbohydrates_is_ok: false,
                protein: String::new(),
                protein_f32: 0.0,
                protein_hint: String::new(),
                protein_is_ok: false,
                fat: String::new(),
                fat_hint: String::new(),
                fat_f32: 0.0,
                fat_is_ok: false,
                weight: String::new(),
                weight_f32: 0.0,
                weight_hint: String::new(),
                weight_is_ok: false,
                kcal: String::new(),
                kcal_f32: 0.0,
                kcal_hint: String::new(),
                kcal_is_ok: false,
            },
            Task::batch(vec![text_input::focus("Name_Of_Macro")]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MacroNameOnChange(text) => {
                self.marco_name = text;
                Task::none()
            }
            Message::CarbohydratesOnChange(text) => {
                self.carbohydrates = text.clone();
                Task::perform(parse_to_number(text.clone()), Message::CarbohydratesResult)
            }
            Message::CarbohydratesResult(result) => {
                if let Ok(result) = result {
                    self.carbohydrates_is_ok = true;
                    self.carbohydrates_f32 = result;
                    self.carbohydrates_hint = String::new();
                } else {
                    // result is Error::NotParseAbleToNumber
                    self.carbohydrates_is_ok = false;
                    self.carbohydrates_hint = String::from("Please provide a number!");
                }
                Task::none()
            }
            Message::ProteinOnChange(text) => {
                self.protein = text.clone();
                Task::perform(parse_to_number(text.clone()), Message::ProteinResult)
            }

            Message::ProteinResult(result) => {
                if let Ok(result) = result {
                    self.protein_f32 = result;
                    self.protein_is_ok = true;
                    self.protein_hint = String::new();
                } else {
                    self.protein_is_ok = false;
                    self.protein_hint = "Please provide a number!".to_string();
                }
                Task::none()
            }

            Message::FatOnChange(text) => {
                self.fat = text.clone();
                Task::perform(parse_to_number(text.clone()), Message::FatResult)
            }
            Message::FatResult(result) => {
                if let Ok(result) = result {
                    self.fat_f32 = result;
                    self.fat_is_ok = true;
                    self.fat_hint = String::new();
                } else {
                    self.fat_is_ok = false;
                    self.fat_hint = String::from("Has to be a number");
                }
                Task::none()
            }
            Message::WeightOnChange(text) => {
                self.weight = text.clone();
                Task::perform(parse_to_number(text.clone()), Message::WeightResult)
            }
            Message::WeightResult(result) => {
                if let Ok(result) = result {
                    self.weight_f32 = result;
                    self.weight_is_ok = true;
                    self.weight_hint = String::new();
                } else {
                    self.weight_is_ok = false;
                    self.weight_hint = String::from("Has to be a number");
                }
                Task::none()
            }
            Message::KcalOnChange(text) => {
                self.kcal = text.clone();
                Task::perform(parse_to_number(text.clone()), Message::KcalResult)
            }
            Message::KcalResult(result) => {
                if let Ok(result) = result {
                    self.kcal_f32 = result;
                    self.kcal_hint = String::new();
                    self.kcal_is_ok = true;
                } else {
                    self.kcal_is_ok = false;
                    self.kcal_hint = String::from("Has to be a number");
                }

                Task::none()
            }
            Message::Save => {
                // Only save if all types of the fields is correct 'ok'
                if self.carbohydrates_is_ok
                    && self.protein_is_ok
                    && self.fat_is_ok
                    && self.weight_is_ok
                    && self.kcal_is_ok
                {
                    //
                    Task::perform(
                        save_macro(
                            self.marco_name.clone(),
                            self.protein_f32,
                            self.carbohydrates_f32,
                            self.fat_f32,
                            self.weight_f32,
                            self.kcal_f32,
                        ),
                        Message::SaveResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SaveResult(result) => {
                todo!();
            }
            Message::Focus(id) => text_input::focus(id),
        }
    }

    fn view(&self) -> Element<Message> {
        let form = column![
            text_input("Name of Macro", &self.marco_name)
                .on_input(Message::MacroNameOnChange)
                .id("Name_Of_Macro")
                .on_submit(Message::Focus("Carbohyrates")),
            text_input("Carbohyrates", &self.carbohydrates)
                .on_input(Message::CarbohydratesOnChange)
                .on_submit(Message::Focus("Protein"))
                .id("Carbohyrates"),
            text(&self.carbohydrates_hint),
            text_input("Protein", &self.protein)
                .id("Protein")
                .on_input(Message::ProteinOnChange)
                .on_submit(Message::Focus("Fat")),
            text(&self.protein_hint),
            text_input("Fat", &self.fat)
                .id("Fat")
                .on_input(Message::FatOnChange)
                .on_submit(Message::Focus("Weight")),
            text(&self.fat_hint),
            text_input("Weight", &self.weight)
                .on_input(Message::WeightOnChange)
                .id("Weight"),
            text(&self.weight_hint),
            button("Save").on_press(Message::Save)
        ]
        .spacing(10);
        form.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> iced::Result {
    iced::application("Experiment", Keto::update, Keto::view)
        .theme(Keto::theme)
        .run_with(Keto::new)
}

async fn parse_to_number(text: String) -> Result<f32, Error> {
    let number = text.parse::<f32>();
    if let Ok(number) = number {
        Ok(number)
    } else {
        Err(Error::NotParseAbleToNumber)
    }
}

async fn save_macro(
    name: String,
    protein: f32,
    carbohydrates: f32,
    fat: f32,
    weight: f32,
    kcal: f32,
) -> Result<String, Error> {
    dbg!(
        "The values passed: {} {} {} {} {} {}",
        &name,
        &protein,
        &carbohydrates,
        &fat,
        &weight,
        &kcal
    );
    Ok("".to_string())
}

#[derive(Debug, Clone)]
enum Error {
    NotParseAbleToNumber,
    ParseError(ErrorKind),
}
