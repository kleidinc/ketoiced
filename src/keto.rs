#![allow(dead_code, unused_variables)]
use std::str::FromStr;

use crate::{helper, Error};
use bigdecimal::BigDecimal;
use sqlx::postgres::PgPool;

#[derive(Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct MacroFood {
    pub macro_id: uuid::Uuid,
    pub name: String,
    pub protein: BigDecimal,
    pub carbohydrates: BigDecimal,
    pub fat: BigDecimal,
    pub weight: BigDecimal,
    pub kcalories: BigDecimal,
}

impl MacroFood {
    pub fn new(
        name: String,
        protein: f32,
        carbohydrates: f32,
        fat: f32,
        weight: f32,
        kcalories: f32,
    ) -> Self {
        Self {
            macro_id: uuid::Uuid::new_v4(), // not used anywhere
            name,
            protein: helper::convert_from_f32_to_bigdecimal(protein),
            carbohydrates: helper::convert_from_f32_to_bigdecimal(carbohydrates),
            fat: helper::convert_from_f32_to_bigdecimal(fat),
            weight: helper::convert_from_f32_to_bigdecimal(weight),
            kcalories: helper::convert_from_f32_to_bigdecimal(kcalories),
        }
    }

    pub async fn save(&self) -> Result<uuid::Uuid, anyhow::Error> {
        let pool = PgPool::connect("postgres://alex:1234@localhost/ketoiced")
            .await
            .unwrap();
        let rec = sqlx::query!(
            r#"
INSERT INTO "macro_food"(name, protein, carbohydrates, fat, weight, kcalories)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING macro_id
"#,
            self.name,
            self.protein,
            self.carbohydrates,
            self.fat,
            self.weight,
            self.kcalories
        )
        .fetch_one(&pool)
        .await?;
        Ok(rec.macro_id)
    }

    pub async fn get_all(pool: PgPool) -> Result<Vec<MacroFood>, anyhow::Error> {
        todo!();
    }
}
