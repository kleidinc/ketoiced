#![allow(dead_code, unused_variables)]
use crate::Error;
use rust_decimal::Decimal;
use sqlx::postgres::PgPool;

#[derive(Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct MacroFood {
    pub macro_id: uuid::Uuid,
    pub name: String,
    pub protein: Decimal,
    pub carbohydrates: Decimal,
    pub fat: Decimal,
    pub weight: i16,
    pub kcalories: i16,
}

impl MacroFood {
    pub fn new(
        name: String,
        protein: Decimal,
        carbohydrates: Decimal,
        fat: Decimal,
        weight: i16,
        kcalories: i16,
    ) -> Self {
        Self {
            macro_id: uuid::Uuid::new_v4(), // not used anywhere
            name,
            protein,
            carbohydrates,
            fat,
            weight,
            kcalories,
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
