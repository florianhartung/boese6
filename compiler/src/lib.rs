use crate::lexer::Lexer;
use crate::parser::Parser;

mod ast;
mod codegen;
mod lexer;
mod parser;
mod token;

pub fn compile_to_nasm(src_code: &str) -> String {
    let mut lexer = Lexer::new(&src_code);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token().expect("Lexer failed to read token") {
        tokens.push(token);
    }

    println!("--- TOKENS ---");
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast_root = parser.parse().expect("Failed to parse tokens");

    println!("--- AST ---");
    println!("{:?}", ast_root);

    let nasm_code = codegen::generate_nasm_from_ast(ast_root);

    println!("--- NASM CODE ---");
    println!("{:?}", nasm_code);
    nasm_code
}
