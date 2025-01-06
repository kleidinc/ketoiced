#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::needless_return,
    clippy::large_enum_variant
)]
mod helper;
mod keto;

use anyhow::{Context, Error as AnyhowError};
use bigdecimal::BigDecimal;
use iced::event::{self, Event};
use iced::keyboard;
use iced::keyboard::key;
use iced::widget::{self, button, column, container, row, text, text_input};
use iced::Error as IcedError;
use iced::{Application, Element, Subscription, Task, Theme};
use rust_decimal::Decimal;
use sqlx::error::Error as SQLXError;
use sqlx::postgres::PgConnection;
use sqlx::postgres::PgPool;

use helper::parse_to_number;

use std::io::ErrorKind;

fn main() -> Result<(), IcedError> {
    iced::application("Experiment", Keto::update, Keto::view)
        .subscription(Keto::subscription)
        .theme(Keto::theme)
        .run_with(Keto::new)
}

static HELPER_TXT_NUM: &str = "Please provide a number in the form 22.5 in grams.";

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
    SaveResult(Result<uuid::Uuid, Error>),
    ShowResult(Result<keto::MacroFood, Error>),
    Event(Event),
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
        match message {
            Message::MacroNameOnChange(text) => {
                // Add a check if the Macro name exists already!
                // check_macro_name
                self.macro_name_is_ok = true;
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
                    self.carbohydrates_hint = HELPER_TXT_NUM.to_string()
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
                    self.protein_hint = HELPER_TXT_NUM.to_string();
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
                    self.fat_hint = HELPER_TXT_NUM.to_string();
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
                    self.weight_hint = HELPER_TXT_NUM.to_string();
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
                    // widget::focus_next();
                } else {
                    self.kcalories_is_ok = false;
                    self.kcalories_hint = HELPER_TXT_NUM.to_string();
                }

                Task::none()
            }
            Message::Save => {
                // Only save if all types of the fields is correct 'ok'
                if self.carbohydrates_is_ok
                    && self.macro_name_is_ok
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
            // The result in this case is the uuid of the saved macro food
            // TODO: use the id to get the saved macro food and show it below the form
            Message::SaveResult(result) => Task::none(),
            Message::ShowResult(result) => {
                if let Ok(result) = result {
                    dbg!(result);
                    Task::none()
                } else {
                    println!("WE didnt get anything");
                    Task::none()
                }
            }
            Message::Focus(id) => return text_input::focus(id),
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Control),
                    modifiers,
                    ..
                }) => {
                    if modifiers.alt() {
                        // TODO: run a Task
                        Task::none()
                    } else {
                        Task::none()
                    }
                }
                _ => Task::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        //
        // save can only be done when all fields are the correct type
        let save_button = if self.macro_name_is_ok
            && self.protein_is_ok
            && self.fat_is_ok
            && self.carbohydrates_is_ok
            && self.weight_is_ok
            && self.kcalories_is_ok
        {
            button("Save").on_press_maybe(Some(Message::Save))
        } else {
            // TODO: change the style of the button
            button("Inactive Save - make sure all fields are correct")
        };
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
                .id("Weight")
                .on_input(Message::WeightOnChange)
                .on_submit(Message::Focus("Kcal")),
            text(&self.weight_hint),
            text_input("KCalories", &self.kcalories)
                .id("Kcal")
                .on_input(Message::KcalOnChange)
                .on_submit(Message::Focus("Name_Of_Macro")),
            text(&self.kcalories_hint),
            save_button
        ]
        .spacing(10);
        form.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
}

// When the macro is saved correctly we will get back a uuid
async fn save_macro(
    name: String,
    protein: f32,
    carbohydrates: f32,
    fat: f32,
    weight: f32,
    kcalories: f32,
) -> Result<uuid::Uuid, Error> {
    let result = keto::MacroFood::new(name, protein, carbohydrates, fat, weight, kcalories)
        .save()
        .await;
    if let Ok(uuid) = result {
        dbg!(uuid);
        Ok(uuid)
    } else {
        dbg!("There is an error");
        Err(Error::DBErrorCannotSave)
    }
}

async fn get_macro_by_uuid(macro_id: uuid::Uuid) -> Option<Keto> {
    // get the keto::MacroFood
    // convert it to Macro
    // from BigDecimal to str
    let result = keto::MacroFood::get_macro_food_by_id(macro_id).await;
    if let Ok(result) = result {
        //
    } else {
        //
    };
    todo!();
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    DBErrorCannotSave,
    NotParseAbleToNumber,
    ParseError(ErrorKind),
    IcedError,
    SQLXError,
}
