#[cfg(test)]
mod tests {
    #[test]
    fn parse_json() {
        let parser = ffs::parsing::Parser::new();
        let json = parser.parse("tests/test.json").unwrap();
        
        println!("ast: {:?}", json);
    }
}