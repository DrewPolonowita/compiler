use std::env;
use std::process::Command;
use crate::compiler::compiler::compile;
use crate::parser::parser::ParseTree;

mod lexer;
mod parser;
mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(filename) = args.get(1) else {
        println!("Failed to get args");
        return;
    };

    let filepath = format!("src/resources/test_files/{}", filename.as_str());
    let contents = std::fs::read_to_string(filepath).unwrap();
    let lexer = lexer::lexer::Lexer::new(contents);

    let parse_tree = ParseTree::parse(lexer);
    //println!("{:#?}", parse_tree);

    let rust = compile(parse_tree);

    let rust = format!("fn main() {{{}}}", rust);

    std::fs::write("output.rs", rust).unwrap();

    Command::new("rustc")
        .arg("output.rs")
        .arg("-O") // optional optimization
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(format!("test_executables/{}.exe", filename).as_str())
        .status()
        .expect("failed to run rustc");

    //std::fs::remove_file("output.rs").unwrap();

}
