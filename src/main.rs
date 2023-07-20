use std::num::ParseIntError;


fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

fn exercise2_should_work() {
    assert_eq!(parse_number("42"), Ok(42));
    assert_eq!(
        parse_number("invalid").map_err(|e| e.to_string()),
        Err("invalid digit found in string".parse().unwrap())
    );
}

fn main() {
    exercise2_should_work();
}
