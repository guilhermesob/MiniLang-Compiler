#[derive(Debug)]
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
