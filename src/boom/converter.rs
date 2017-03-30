use std::num::ParseIntError;

pub fn parse_int(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}
