use std::{char, iter::Peekable};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_equation() {
        assert_eq!(
            tokenize("132 + 3 * (18 / 2) + 12"),
            vec![
                Type::Int(132),
                Type::Operator('+'),
                Type::Int(3),
                Type::Operator('*'),
                Type::Operator('('),
                Type::Int(18),
                Type::Operator('/'),
                Type::Int(2),
                Type::Operator(')'),
                Type::Operator('+'),
                Type::Int(12),
            ],
        );
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Identifier,
    Keyword(String),
    Operator(char),
    Int(i32),
    String(String),
}

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
            char if char.is_ascii_alphabetic() => {
                tokens.push(Type::String(consume(char, &mut iter, &mut |char: char| {
                    char.is_ascii_alphabetic() || char == '_'
                })));
            }
            '1'..='9' => {
                tokens.push(Type::Int(
                    consume(char, &mut iter, &mut |char: char| char.is_ascii_digit())
                        .parse()
                        .expect("Failed to parse string"),
                ));
            }
            '"' => continue,
            '(' | ')' | '+' | '-' | '*' | '/' | '%' => tokens.push(Type::Operator(char)),
            _ => {
                panic!("Invalid syntax!");
            }
        }
    }

    tokens
}
