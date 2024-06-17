#[derive(Debug, PartialEq)]
enum AstNode {
    Number(f64),
    Identifier(String),
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}

#[derive(Debug, PartialEq)]
enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

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

#[derive(Debug)]
enum ParseError {
    UnexpectedToken,
    UnexpectedEndOfInput,
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, position: 0 }
    }

    fn parse(&mut self) -> Result<AstNode, ParseError> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<AstNode, ParseError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<AstNode, ParseError> {
        let mut node = self.parse_logical_and()?;

        while self.match_token(&Token::Or) {
            self.position += 1;
            let right = self.parse_logical_and()?;
            node = AstNode::BinaryOp {
                op: BinaryOperator::Or,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_logical_and(&mut self) -> Result<AstNode, ParseError> {
        let mut node = self.parse_equality()?;

        while self.match_token(&Token::And) {
            self.position += 1;
            let right = self.parse_equality()?;
            node = AstNode::BinaryOp {
                op: BinaryOperator::And,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_equality(&mut self) -> Result<AstNode, ParseError> {
        let mut node = self.parse_comparison()?;

        while self.match_token(&Token::Equal) || self.match_token(&Token::NotEqual) {
            let op = if self.match_token(&Token::Equal) {
                self.position += 1;
                BinaryOperator::Equal
            } else {
                self.position += 1;
                BinaryOperator::NotEqual
            };
            let right = self.parse_comparison()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_comparison(&mut self) -> Result<AstNode, ParseError> {
        let mut node = self.parse_term()?;

        while self.match_token(&Token::LessThan)
            || self.match_token(&Token::GreaterThan)
            || self.match_token(&Token::LessThanOrEqual)
            || self.match_token(&Token::GreaterThanOrEqual)
        {
            let op = if self.match_token(&Token::LessThan) {
                self.position += 1;
                BinaryOperator::LessThan
            } else if self.match_token(&Token::GreaterThan) {
                self.position += 1;
                BinaryOperator::GreaterThan
            } else if self.match_token(&Token::LessThanOrEqual) {
                self.position += 1;
                BinaryOperator::LessThanOrEqual
            } else {
                self.position += 1;
                BinaryOperator::GreaterThanOrEqual
            };
            let right = self.parse_term()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> Result<AstNode, ParseError> {
        let mut node = self.parse_factor()?;

        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let op = if self.match_token(&Token::Plus) {
                self.position += 1;
                BinaryOperator::Plus
            } else {
                self.position += 1;
                BinaryOperator::Minus
            };
            let right = self.parse_factor()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<AstNode, ParseError> {
        let mut node = self.parse_unary()?;

        while self.match_token(&Token::Multiply) || self.match_token(&Token::Divide) {
            let op = if self.match_token(&Token::Multiply) {
                self.position += 1;
                BinaryOperator::Multiply
            } else {
                self.position += 1;
                BinaryOperator::Divide
            };
            let right = self.parse_unary()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<AstNode, ParseError> {
        if self.match_token(&Token::Minus) {
            self.position += 1;
            let expr = self.parse_primary()?;
            return Ok(AstNode::BinaryOp {
                op: BinaryOperator::Minus,
                left: Box::new(AstNode::Number(0.0)),
                right: Box::new(expr),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<AstNode, ParseError> {
        if let Some(token) = self.tokens.get(self.position) {
            match token {
                Token::Number(n) => {
                    self.position += 1;
                    Ok(AstNode::Number(*n))
                }
                Token::Identifier(id) => {
                    self.position += 1;
                    Ok(AstNode::Identifier(id.clone()))
                }
                Token::LeftParenthesis => {
                    self.position += 1;
                    let expr = self.parse_expression()?;
                    if self.match_token(&Token::RightParenthesis) {
                        self.position += 1;
                        Ok(expr)
                    } else {
                        Err(ParseError::UnexpectedToken)
                    }
                }
                _ => Err(ParseError::UnexpectedToken),
            }
        } else {
            Err(ParseError::UnexpectedEndOfInput)
        }
    }

    fn match_token(&self, token: &Token) -> bool {
        if let Some(t) = self.tokens.get(self.position) {
            t == token
        } else {
            false
        }
    }
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
    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();
    println!("{:?}", ast);
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

    #[test]
    fn test_parse_simple_expression() {
        let tokens = vec![
            Token::Number(3.0),
            Token::Plus,
            Token::Number(5.0),
            Token::Multiply,
            Token::Number(7.0),
        ];
        let mut parser = Parser::new(&tokens);
        let ast = parser.parse().unwrap();
        let expected = AstNode::BinaryOp {
            op: BinaryOperator::Plus,
            left: Box::new(AstNode::Number(3.0)),
            right: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(AstNode::Number(5.0)),
                right: Box::new(AstNode::Number(7.0)),
            }),
        };
        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parse_expression_with_parentheses() {
        let tokens = vec![
            Token::Number(3.0),
            Token::Plus,
            Token::LeftParenthesis,
            Token::Number(5.0),
            Token::Multiply,
            Token::Number(7.0),
            Token::RightParenthesis,
        ];
        let mut parser = Parser::new(&tokens);
        let ast = parser.parse().unwrap();
        let expected = AstNode::BinaryOp {
            op: BinaryOperator::Plus,
            left: Box::new(AstNode::Number(3.0)),
            right: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(AstNode::Number(5.0)),
                right: Box::new(AstNode::Number(7.0)),
            }),
        };
        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parse_complex_expression() {
        let tokens = vec![
            Token::Number(3.0),
            Token::Plus,
            Token::Number(5.0),
            Token::Multiply,
            Token::Number(7.0),
            Token::Minus,
            Token::Number(2.0),
            Token::Divide,
            Token::Number(1.0),
        ];
        let mut parser = Parser::new(&tokens);
        let ast = parser.parse().unwrap();
        let expected = AstNode::BinaryOp {
            op: BinaryOperator::Minus,
            left: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Plus,
                left: Box::new(AstNode::Number(3.0)),
                right: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(AstNode::Number(5.0)),
                    right: Box::new(AstNode::Number(7.0)),
                }),
            }),
            right: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Divide,
                left: Box::new(AstNode::Number(2.0)),
                right: Box::new(AstNode::Number(1.0)),
            }),
        };
        assert_eq!(ast, expected);
    }
}
