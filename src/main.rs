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

    let c = compile(parse_tree);

    let filepath = format!("test_executables/{}", filename);

    let c = format!(
        "#include <iostream>

        int main() {{
            {}
            return 0;
        }}",
        c
    );

    std::fs::write("file.cpp", c).unwrap();

    let result = Command::new("g++")
        .arg("file.cpp")
        .arg("-o")
        .arg(&filepath)
        .output()
        .expect("failed to run g++");

    if !result.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
        return;
    }

    std::fs::remove_file("file.cpp").unwrap();
}