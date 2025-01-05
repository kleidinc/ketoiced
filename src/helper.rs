#![allow(dead_code, unused_variables, unused_imports)]
use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive};

// TODO: improve this helper function
pub fn convert_from_f32_to_bigdecimal(incoming: f32) -> BigDecimal {
    BigDecimal::from_str(&incoming.to_string()).unwrap()
}
