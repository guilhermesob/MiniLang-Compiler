#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    Identifier(String),
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut number_str = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_numeric() || digit == '.' {
                        number_str.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let number: f64 = number_str.parse().unwrap();
                tokens.push(Token::Number(number));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParenthesis);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParenthesis);
                chars.next();
            }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Equal);
                } else {
                    // Handle single equals if needed
                }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::NotEqual);
                } else {
                    // Handle single exclamation if needed
                }
            }
            '<' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::LessThanOrEqual);
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::GreaterThanOrEqual);
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '&' => {
                chars.next();
                if let Some(&'&') = chars.peek() {
                    chars.next();
                    tokens.push(Token::And);
                } else {
                    // Handle single ampersand if needed
                }
            }
            '|' => {
                chars.next();
                if let Some(&'|') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Or);
                } else {
                    // Handle single pipe if needed
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        identifier.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(identifier));
            }
            ' ' => {
                // Ignorar espaços em branco
                chars.next();
            }
            _ => {
                // Ignorar caracteres desconhecidos
                chars.next();
            }
        }
    }

    tokens
}

fn main() {
    let input = "3.14 + (5 * 7.2) - 2 / (1 + 1.1)";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_expression() {
        let input = "3+5*7";
        let expected = vec![
            Token::Number(3.0),
            Token::Plus,
            Token::Number(5.0),
            Token::Multiply,
            Token::Number(7.0),
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_expression_with_parentheses() {
        let input = "3 + (5 * 7) - 2 / (1 + 1)";
        let expected = vec![
            Token::Number(3.0),
            Token::Plus,
            Token::LeftParenthesis,
            Token::Number(5.0),
            Token::Multiply,
            Token::Number(7.0),
            Token::RightParenthesis,
            Token::Minus,
            Token::Number(2.0),
            Token::Divide,
            Token::LeftParenthesis,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(1.0),
            Token::RightParenthesis,
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_with_spaces() {
        let input = " 3 + 4 ";
        let expected = vec![
            Token::Number(3.0),
            Token::Plus,
            Token::Number(4.0),
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_empty_input() {
        let input = "";
        let expected: Vec<Token> = vec![];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_invalid_characters() {
        let input = "3 + a * 7";
        let expected = vec![
            Token::Number(3.0),
            Token::Plus,
            Token::Identifier("a".to_string()),
            Token::Multiply,
            Token::Number(7.0),
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_floats() {
        let input = "3.14 + 2.71 * 1.0";
        let expected = vec![
            Token::Number(3.14),
            Token::Plus,
            Token::Number(2.71),
            Token::Multiply,
            Token::Number(1.0),
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_comparison_operators() {
        let input = "a == b != c < d <= e > f >= g";
        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Equal,
            Token::Identifier("b".to_string()),
            Token::NotEqual,
            Token::Identifier("c".to_string()),
            Token::LessThan,
            Token::Identifier("d".to_string()),
            Token::LessThanOrEqual,
            Token::Identifier("e".to_string()),
            Token::GreaterThan,
            Token::Identifier("f".to_string()),
            Token::GreaterThanOrEqual,
            Token::Identifier("g".to_string()),
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_logical_operators() {
        let input = "a && b || c";
        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::And,
            Token::Identifier("b".to_string()),
            Token::Or,
            Token::Identifier("c".to_string()),
        ];
        assert_eq!(tokenize(input), expected);
    }
}
