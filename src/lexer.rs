#[derive(Debug, Clone, PartialEq, Eq, derive_new::new)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(String),
    Literal(String),
}

// Function tokenizes input string into separate tokens, returning a vector of those
// tokens as strings, excluding empty tokens.
pub fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
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
                tokens.push(Token::Literal(number_string));
            }
            '(' | ')' | '+' | '-' | '*' | '/' => tokens.push(Token::Operator(char.to_string())),
            _ => {
                panic!("Invalid syntax!");
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_equation() {
        let test_str = "132 + 3 * (18 / 2) + 12";
        insta::assert_debug_snapshot!(tokenize(test_str), @r###"
        [
            Literal(
                "132",
            ),
            Operator(
                "+",
            ),
            Literal(
                "3",
            ),
            Operator(
                "*",
            ),
            Operator(
                "(",
            ),
            Literal(
                "18",
            ),
            Operator(
                "/",
            ),
            Literal(
                "2",
            ),
            Operator(
                ")",
            ),
            Operator(
                "+",
            ),
            Literal(
                "12",
            ),
        ]
        "###)
    }
}
