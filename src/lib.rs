mod ast;
mod generator;
mod parser;
mod scanner;
mod values;

use parser::Parser;
use scanner::Scanner;

pub fn to_html(haml: &str) -> String {
    let mut scanner = Scanner::new(haml);
    let tokens = scanner.get_tokens();
    let mut parser = Parser::new(tokens);
    let parsed_values = parser.parse();
    generator::to_html(&parsed_values)
}
