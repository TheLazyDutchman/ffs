#[cfg(test)]
mod tests {
	use std::{fs, collections::HashMap};

	use ffs::{data::{json::{JSON}, yaml::YAML, Data, tsv::TSV, Row, csv::CSV}, parsing::{AST, token::{Token, Number}}};

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

	#[test]
	fn parse_yaml() {
		let yaml = YAML::parse(fs::read_to_string("tests/test.yaml").unwrap()).unwrap();
		assert_eq!(yaml, YAML { value: Data::Object(HashMap::from([
			(String::from("users"), Data::List(vec![
				Data::Object(HashMap::from([
					(String::from("first"), Data::Object(HashMap::from([(String::from("name"), Data::Immediate(Token::String(String::from("\"test\"")))),(String::from("age"), Data::Immediate(Token::Number(Number::new(10, 10))))])))
				])),
				Data::Object(HashMap::from([
					(String::from("second"), Data::Object(HashMap::from([(String::from("name"), Data::Immediate(Token::String(String::from("\"test2\"")))), (String::from("age"), Data::Immediate(Token::Number(Number::new(11, 10))))])))
				]))
			]))
		])) });
	}

	#[test]
	fn parse_tsv() {
		let tsv = TSV::parse(fs::read_to_string("tests/test.tsv").unwrap()).unwrap();
		assert_eq!(tsv, TSV { values: vec![
			Row{ values: vec![Token::Identifier("name".to_owned()), Token::Identifier("age".to_owned())]},
			Row{ values: vec![Token::Identifier("test".to_owned()), Token::Number(Number::new(10, 10))]},
			Row{ values: vec![Token::Identifier("test2".to_owned()), Token::Number(Number::new(11, 10))]}
		]});
	}

	#[test]
	fn parse_csv() {
		let csv = CSV::parse(fs::read_to_string("tests/test.csv").unwrap()).unwrap();
		assert_eq!(csv, CSV { values: vec![
			Row{ values: vec![Token::Identifier("name".to_owned()), Token::Identifier("age".to_owned())]},
			Row{ values: vec![Token::Identifier("test".to_owned()), Token::Number(Number::new(10, 10))]},
			Row{ values: vec![Token::Identifier("test2".to_owned()), Token::Number(Number::new(11, 10))]}
		]});
	}
}