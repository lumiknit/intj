use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "token/intj.pest"]
struct INTJParser;

pub fn parse_string(source: String) -> Result<String, String> {
	let parsed = INTJParser::parse(Rule::root, &source)
		.unwrap_or_else(|e| panic!("{}", e));
	println!("{}", parsed);
	Ok(String::new())
}
