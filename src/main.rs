use std::env;
use crate::generate_ir::ir::IRLine;

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

    println!("{:#?}", &parse_tree);
    let mut n = 0;
    for line in parse_tree.generate_ir() {
        if matches!(line, IRLine::Label(_)) && n > 0 {
            n -= 1;
        }

        println!("{}{}", "   ".repeat(n), &line);

        if matches!(line, IRLine::Label(_) | IRLine::CmpSingleAddress(_) | IRLine::CmpThreeAddress(_)) {
            n += 1
        } else if matches!(line, IRLine::Goto(_)) && n > 0 {
            n -= 1
        }
    }
}