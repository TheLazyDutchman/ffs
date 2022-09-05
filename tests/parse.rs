#[cfg(test)]
mod tests {
    use std::{fs, collections::HashMap};

    use ffs::{data::json::{JSON, Data}, parsing::{AST, token::{Token, Number}}};

    #[test]
    fn parse_json() {
        let json = JSON::parse(fs::read_to_string("tests/test.json").unwrap()).unwrap();
        assert_eq!(json, JSON{ value: Data::List(
            vec![
                Data::Object(HashMap::from([(String::from("\"name\""), Data::Immediate(Token::String(String::from("\"test\"")))), (String::from("\"age\""), Data::Immediate(Token::Number(Number::new(10, 10))))])),
                Data::Object(HashMap::from([(String::from("\"name\""), Data::Immediate(Token::String(String::from("\"test2\"")))), (String::from("\"age\""), Data::Immediate(Token::Number(Number::new(11, 10))))]))
            ]
        ) });
    }
}