#[cfg(test)]
mod tests {
    use std::fs;

    use ffs::{data::json::JSON, parsing::AST};

    #[test]
    fn parse_json() {
        let tokens = JSON::parse(fs::read_to_string("tests/test.json").unwrap()).unwrap();
        println!("{:?}", tokens);
        panic!();
    }
}