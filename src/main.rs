use std::io;

mod lexer;
mod parser;
mod tree;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokens = lexer::tokenize(&input);

    println!("{:?}", tokens);

    parser::parse(&tokens);
}
