#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_equation() {
        assert_eq!(
            tokenize("132 + 3 * (18 / 2) + 12"),
            vec![
                (Type::Literal, String::from("132")),
                (Type::Operator, String::from("+")),
                (Type::Literal, String::from("3")),
                (Type::Operator, String::from("*")),
                (Type::Operator, String::from("(")),
                (Type::Literal, String::from("18")),
                (Type::Operator, String::from("/")),
                (Type::Literal, String::from("2")),
                (Type::Operator, String::from(")")),
                (Type::Operator, String::from("+")),
                (Type::Literal, String::from("12")),
            ],
        );
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Identifier,
    Keyword,
    Operator,
    Literal,
}

// Function tokenizes input string into separate tokens, returning a vector of those
// tokens as strings, excluding empty tokens.
pub fn tokenize(line: &str) -> Vec<(Type, String)> {
    let mut tokens: Vec<(Type, String)> = vec![];
    let mut iter = line.chars().peekable();

    while let Some(char) = iter.next() {
        match char {
            char if char.is_whitespace() => continue,
            '1'..='9' => {
                let mut number_string = char.to_string();

                while let Some(&next_char) = iter.peek() {
                    if next_char.is_ascii_digit() {
                        number_string.push(next_char);
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push((Type::Literal, number_string));
            }
            '(' | ')' | '+' | '-' | '*' | '/' => tokens.push((Type::Operator, char.to_string())),
            _ => {
                panic!("Invalid syntax!");
            }
        }
    }

    tokens
}
