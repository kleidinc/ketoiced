#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
pub struct MacroFood {
    pub id: Option<String>,
    pub name: String,
    pub protein: f32,
    pub carbohydrates: f32,
    pub fat: f32,
    pub weight_gram: f32,
    pub kcal: f32,
}
