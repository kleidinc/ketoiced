#![allow(dead_code, unused_variables, unused_imports)]
use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive};
use sqlx::postgres::PgPool;

pub async fn parse_to_number(text: String) -> Result<f32, crate::Error> {
    let number = text.parse::<f32>();
    if let Ok(number) = number {
        Ok(number)
    } else {
        Err(crate::Error::NotParseAbleToNumber)
    }
}

pub fn convert_from_f32_to_bigdecimal(incoming: f32) -> BigDecimal {
    BigDecimal::from_str(&incoming.to_string()).unwrap()
}

pub fn convert_from_bigdecimal_to_f32(incoming: BigDecimal) -> f32 {
    todo!();
}

pub async fn connect_to_db() -> PgPool {
    PgPool::connect("postgres://alex:1234@localhost/ketoiced")
        .await
        .expect("couldn't connect to db")
}
