use std::env;
use std::process::Command;

mod lexer;
mod parser;
mod error;
mod interfaces;
mod semantics;
mod generate_ir;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(filename) = args.get(1) else {
        println!("Failed to get args");
        return;
    };

    let filepath = format!("src/resources/test_files/{}", filename.as_str());
    let contents = std::fs::read_to_string(filepath).unwrap();
    let lexer = lexer::lexer::Lexer::new(contents);

    use crate::parser::parse_tree::ParseTree;

    let parse_tree = match ParseTree::parse(lexer) {
        Ok(tree) => tree,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(0);
        },
    };

    println!("{:#?}", parse_tree);
}