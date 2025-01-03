#![allow(dead_code, unused_imports, unused_variables, clippy::needless_return)]
mod keto;

use anyhow::{Context, Error as AnyhowError};
use iced::widget::{button, column, container, row, text, text_input};
use iced::Error as IcedError;
use iced::{Application, Element, Task, Theme};
use sqlx::error::Error as SQLXError;
use sqlx::postgres::PgConnection;
use sqlx::postgres::PgPool;

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
    kcalories: String,
    kcalories_f32: f32,
    kcalories_hint: String,
    kcalories_is_ok: bool,
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
                kcalories: String::new(),
                kcalories_f32: 0.0,
                kcalories_hint: String::new(),
                kcalories_is_ok: false,
            },
            Task::batch(vec![text_input::focus("Name_Of_Macro")]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        // let pool =
        //     if let Ok(pool) = PgPool::connect("postgres://alex:1234@localhost/ketoiced").await {
        //         pool
        //     } else {
        //         // TODO: handle this error better
        //         panic!();
        //     };

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
                self.kcalories = text.clone();
                Task::perform(parse_to_number(text.clone()), Message::KcalResult)
            }
            Message::KcalResult(result) => {
                if let Ok(result) = result {
                    self.kcalories_f32 = result;
                    self.kcalories_hint = String::new();
                    self.kcalories_is_ok = true;
                } else {
                    self.kcalories_is_ok = false;
                    self.kcalories_hint = String::from("Has to be a number");
                }

                Task::none()
            }
            Message::Save => {
                // Only save if all types of the fields is correct 'ok'
                if self.carbohydrates_is_ok
                    && self.protein_is_ok
                    && self.fat_is_ok
                    && self.weight_is_ok
                    && self.kcalories_is_ok
                {
                    //
                    Task::perform(
                        save_macro(
                            self.marco_name.clone(),
                            self.protein_f32,
                            self.carbohydrates_f32,
                            self.fat_f32,
                            self.weight_f32,
                            self.kcalories_f32,
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
        //
        // Activate the connectiontionpool here
        //
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
            button("Save").on_press(Message::Save),
        ]
        .spacing(10);
        form.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> Result<(), IcedError> {
    // we need to activate the db pool
    // let pool = PgPool::connect("postgres://alex:1234@localhost/kedoiced")
    //     .await
    //     .unwrap();
    //
    //
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

// When the macro is saved correctly we will get back a uuid
async fn save_macro(
    name: String,
    protein: f32,
    carbohydrates: f32,
    fat: f32,
    weight: i16,
    kcalories: i16,
) -> Result<String, Error> {
    // BUG: redo the types so the types correspond to the Postgres/sqlx
    let result = keto::MacroFood::new(name, protein, carbohydrates, fat, weight, kcalories)
        .save()
        .await;
    dbg!(
        "The values passed: {} {} {} {} {} {}",
        &name,
        &protein,
        &carbohydrates,
        &fat,
        &weight,
        &kcalories
    );
    Ok("".to_string())
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    DBErrorCannotSave,
    NotParseAbleToNumber,
    ParseError(ErrorKind),
    IcedError,
    SQLXError,
}
