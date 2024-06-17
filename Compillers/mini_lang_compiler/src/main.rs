#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut number = 0;
                while let Some(&digit) = chars.peek() {
                    if digit.is_numeric() {
                        number = number * 10 + digit.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
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
    let input = "3 + (5 * 7) - 2 / (1 + 1)";
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
            Token::Number(3),
            Token::Plus,
            Token::Number(5),
            Token::Multiply,
            Token::Number(7),
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_expression_with_parentheses() {
        let input = "3 + (5 * 7) - 2 / (1 + 1)";
        let expected = vec![
            Token::Number(3),
            Token::Plus,
            Token::LeftParenthesis,
            Token::Number(5),
            Token::Multiply,
            Token::Number(7),
            Token::RightParenthesis,
            Token::Minus,
            Token::Number(2),
            Token::Divide,
            Token::LeftParenthesis,
            Token::Number(1),
            Token::Plus,
            Token::Number(1),
            Token::RightParenthesis,
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_with_spaces() {
        let input = " 3 + 4 ";
        let expected = vec![
            Token::Number(3),
            Token::Plus,
            Token::Number(4),
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
            Token::Number(3),
            Token::Plus,
            Token::Multiply,
            Token::Number(7),
        ];
        assert_eq!(tokenize(input), expected);
    }
}
