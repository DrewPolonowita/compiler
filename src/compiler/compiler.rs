use crate::lexer::tokens::Token;
use crate::parser::parser::ParseTree;


pub fn compile(tree: ParseTree) -> String {
    use ParseTree::*;

    match tree {
        Program(program) => program_to_string(*program),
        Statements(stmts) => statements_to_string(stmts),
        Statement(stmt) => statement_to_string(*stmt),
        Assignment(id, typ, expr) => assignment_to_string(*id, *typ, *expr),
        Expression(l_tree, token, r_tree) => expression_to_string(*l_tree, token, *r_tree),
        Term(l_tree, token, r_tree) => expression_to_string(*l_tree, token, *r_tree),
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
fn expression_to_string(l_tree: ParseTree, operator: Token, r_tree: ParseTree) -> String {
    format!("{} {} {}", compile(l_tree), token_to_string(operator), compile(r_tree))
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

        Times => "*",
        Plus => "+",
        Subtract => "-",
        Divide => "/",


        _ => unreachable!("THIS IS A BUG")
    }.to_string()
}
fn str_to_string(str: &str) -> String {
    format!("std::string({})", str.to_string())
}