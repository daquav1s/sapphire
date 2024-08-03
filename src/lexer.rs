use std::{default, iter::Peekable};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_equation() {
        assert_eq!(
            tokenize("var my_variable = 1 * 6 + another_variable"),
            vec![
                Type::Keyword(String::from("var")),
                Type::Identifier(String::from("my_variable")),
                Type::Operator('='),
                Type::Int(1),
                Type::Operator('*'),
                Type::Int(6),
                Type::Operator('+'),
                Type::Identifier(String::from("another_variable")),
            ],
        );
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Identifier(String),
    Keyword(String),
    Operator(char),
    Int(i32),
    String(String),
    EOF,
}

// Processes characters sequentially to form and yield a string or numeric sequence
// based on a specified criterion.
pub fn consume(
    char: char,
    iter: &mut Peekable<impl Iterator<Item = char>>,
    predicate: &mut impl FnMut(char) -> bool,
) -> String {
    let mut result = String::from(char.to_string());
    while let Some(&next_char) = iter.peek() {
        if predicate(next_char) {
            result.push(next_char);
            iter.next();
            if char == '"' && next_char == '"' {
                break;
            }
        } else {
            break;
        }
    }
    result
}

// Function tokenizes input string into separate tokens, returning a vector of those
// tokens as strings, excluding empty tokens.
pub fn tokenize(line: &str) -> Vec<Type> {
    let mut tokens: Vec<Type> = vec![];
    let mut iter = line.chars().peekable();

    while let Some(char) = iter.next() {
        match char {
            char if char.is_whitespace() => continue,
            '"' => {
                tokens.push(Type::String(consume(char, &mut iter, &mut |char: char| {
                    char.is_ascii()
                })));
            }
            char if char.is_ascii_alphabetic() => {
                let contents = consume(char, &mut iter, &mut |char: char| {
                    char.is_ascii_alphabetic() || char == '_'
                });

                if matches!(contents.as_str(), "if" | "else" | "for" | "var") {
                    tokens.push(Type::Keyword(contents));
                } else {
                    tokens.push(Type::Identifier(contents));
                }
            }
            '1'..='9' => {
                tokens.push(Type::Int(
                    consume(char, &mut iter, &mut |char: char| char.is_ascii_digit())
                        .parse()
                        .expect("Failed to parse string"),
                ));
            }
            '(' | ')' | '+' | '-' | '*' | '/' | '%' | '=' => tokens.push(Type::Operator(char)),
            _ => {
                panic!("Invalid syntax!");
            }
        }
    }
    tokens.push(Type::EOF); // temp EOF

    tokens
}
