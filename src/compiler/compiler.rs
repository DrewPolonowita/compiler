use crate::lexer::tokens::Token;
use crate::ParseTree;


pub fn compile(tree: ParseTree) -> String {
    use ParseTree::*;

    match tree {
        Program(program) => program_to_string(*program),
        Statements(stmts) => statements_to_string(stmts),
        Statement(stmt) => statement_to_string(*stmt),
        Assignment(id, typ, expr) => assignment_to_string(*id, *typ, *expr),
        Expression(vec_tree, vec_token) => expression_to_string(vec_tree, vec_token),
        Term(vec_tree, vec_token) => expression_to_string(vec_tree, vec_token),
        Factor(tree) => factor_to_string(*tree),
        Println(tree) => print_to_string(*tree),

        Type(token) => token_to_string(token),

        Arithmetic(token) => arithmetic_to_string(&token),
        Number(Token::Number(num)) => num,
        String(Token::String(str)) => str_to_string(&str),
        Identifier(Token::Identifier(id)) => id,
        _ => todo!()
    }
}

fn arithmetic_to_string(token: &Token) -> String {
    use Token::*;

    match token {
        Plus => "+".to_string(),
        Subtract => "-".to_string(),
        Times => "*".to_string(),
        Divide => "/".to_string(),
        _ => todo!()
    }
}

fn print_to_string(tree: ParseTree) -> String {format!("std::cout << {} << std::endl", compile(tree)) }
fn factor_to_string(tree: ParseTree) -> String {format!("({})", compile(tree)) }
fn expression_to_string(trees: Vec<ParseTree>, operators: Vec<Token>) -> String {
    let mut trees = trees.into_iter();
    let mut operators = operators.into_iter();

    let Some(tree) = trees.next() else { todo!() };

    let mut final_string = String::from(compile(tree));

    while let Some(tree) = trees.next() {
        let Some(op) = operators.next() else { todo!() };

        final_string = format!("{} {} {}", final_string, arithmetic_to_string(&op), compile(tree))
    }
    final_string
}
fn assignment_to_string(id: ParseTree, expr_type: ParseTree, expr: ParseTree) -> String {
    format!("{} {} = {}", compile(expr_type), compile(id), compile(expr))
}
fn statement_to_string(stmt: ParseTree) -> String {
    format!("{};", compile(stmt))
}
fn statements_to_string(stmts: Vec<ParseTree>) -> String {
    let mut stmts = stmts.into_iter();
    let mut final_string = String::new();
    while let Some(stmt) = stmts.next() {
        final_string += &compile(stmt);
    }

    final_string
}
fn program_to_string(program: ParseTree) -> String {
    compile(program)
}
fn token_to_string(token: Token) -> String {
    use Token::*;
    match token {
        IntType => "int",
        StringType => "std::string",
        BoolType => "bool",
        _ => todo!()
    }.to_string()
}
fn str_to_string(str: &str) -> String {
    format!("std::string({})", str.to_string())
}