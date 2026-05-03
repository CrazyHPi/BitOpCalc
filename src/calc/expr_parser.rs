/// Token types for the expression parser
#[derive(Debug, Clone)]
enum Token {
    Number(u64),
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,
    LParen,
    RParen,
}

/// Tokenize an expression string into tokens
fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            '0'..='9' | 'a'..='f' | 'A'..='F' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_hexdigit() || c == 'x' || c == 'X' || c == 'b' || c == 'B' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let value = parse_number(&num_str)?;
                tokens.push(Token::Number(value));
            }
            '&' => {
                chars.next();
                tokens.push(Token::And);
            }
            '|' => {
                chars.next();
                tokens.push(Token::Or);
            }
            '^' => {
                chars.next();
                tokens.push(Token::Xor);
            }
            '~' => {
                chars.next();
                tokens.push(Token::Not);
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'<') {
                    chars.next();
                    tokens.push(Token::ShiftLeft);
                } else {
                    return Err("Expected '<<' for left shift".into());
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::ShiftRight);
                } else {
                    return Err("Expected '>>' for right shift".into());
                }
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            _ => {
                return Err(format!("Unexpected character: '{}'", ch));
            }
        }
    }

    Ok(tokens)
}

fn parse_number(s: &str) -> Result<u64, String> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).map_err(|e| format!("Invalid hex number: {}", e))
    } else if s.starts_with("0b") || s.starts_with("0B") {
        u64::from_str_radix(&s[2..], 2).map_err(|e| format!("Invalid binary number: {}", e))
    } else {
        s.parse::<u64>().map_err(|e| format!("Invalid number: {}", e))
    }
}

/// AST nodes for the parsed expression
#[derive(Debug)]
enum Expr {
    Value(u64),
    UnaryNot(Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
}

/// Pratt parser for parsing expressions with correct operator precedence
///
/// Precedence (low to high):
///   | (OR)          -> 1
///   ^ (XOR)         -> 2
///   & (AND)         -> 3
///   << >> (shifts)  -> 4
///   ~ (NOT)         -> 5 (prefix unary)
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expression(0)?;
        if self.pos < self.tokens.len() {
            return Err(format!(
                "Unexpected token after expression: {:?}",
                self.tokens[self.pos]
            ));
        }
        Ok(expr)
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<Expr, String> {
        // Parse left-hand side
        let mut lhs = self.parse_primary()?;

        while self.pos < self.tokens.len() {
            let op_prec = self.current_op_prec();
            if op_prec < min_prec {
                break;
            }

            // Handle binary operators
            let op = match &self.tokens[self.pos] {
                Token::Or => BinaryOp::Or,
                Token::Xor => BinaryOp::Xor,
                Token::And => BinaryOp::And,
                Token::ShiftLeft => BinaryOp::ShiftLeft,
                Token::ShiftRight => BinaryOp::ShiftRight,
                _ => break,
            };

            self.pos += 1;

            // Right-associative for shifts, left for others
            let next_prec = if matches!(op, BinaryOp::ShiftLeft | BinaryOp::ShiftRight) {
                op_prec
            } else {
                op_prec + 1
            };

            let rhs = self.parse_expression(next_prec)?;
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        if self.pos >= self.tokens.len() {
            return Err("Unexpected end of expression".into());
        }

        match &self.tokens[self.pos] {
            Token::Number(n) => {
                let val = *n;
                self.pos += 1;
                Ok(Expr::Value(val))
            }
            Token::LParen => {
                self.pos += 1;
                let expr = self.parse_expression(0)?;
                if self.pos < self.tokens.len() && matches!(self.tokens[self.pos], Token::RParen) {
                    self.pos += 1;
                    Ok(expr)
                } else {
                    Err("Mismatched parentheses".into())
                }
            }
            Token::Not => {
                self.pos += 1;
                let expr = self.parse_expression(5)?;
                Ok(Expr::UnaryNot(Box::new(expr)))
            }
            _ => Err(format!(
                "Unexpected token: {:?}",
                self.tokens[self.pos]
            )),
        }
    }

    fn current_op_prec(&self) -> u8 {
        if self.pos >= self.tokens.len() {
            return 0;
        }
        match &self.tokens[self.pos] {
            Token::Or => 1,
            Token::Xor => 2,
            Token::And => 3,
            Token::ShiftLeft | Token::ShiftRight => 4,
            _ => 0,
        }
    }
}

/// Evaluate an AST to a u64 value
fn eval(expr: &Expr) -> u64 {
    match expr {
        Expr::Value(v) => *v,
        Expr::UnaryNot(e) => !eval(e),
        Expr::Binary(lhs, op, rhs) => {
            let l = eval(lhs);
            let r = eval(rhs);
            match op {
                BinaryOp::And => l & r,
                BinaryOp::Or => l | r,
                BinaryOp::Xor => l ^ r,
                BinaryOp::ShiftLeft => l << r,
                BinaryOp::ShiftRight => l >> r,
            }
        }
    }
}

/// Parse and evaluate an expression string
pub fn parse(expr: &str) -> Result<u64, String> {
    let trimmed = expr.trim();
    if trimmed.is_empty() {
        return Err("Empty expression".into());
    }

    let tokens = tokenize(trimmed)?;
    if tokens.is_empty() {
        return Err("Empty expression".into());
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    Ok(eval(&ast))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_hex() {
        assert_eq!(parse("0xFF").unwrap(), 255);
        assert_eq!(parse("0x0F").unwrap(), 15);
        assert_eq!(parse("0xABCDEF").unwrap(), 0xABCDEF);
    }

    #[test]
    fn test_single_binary() {
        assert_eq!(parse("0b1010").unwrap(), 10);
        assert_eq!(parse("0b11111111").unwrap(), 255);
    }

    #[test]
    fn test_single_decimal() {
        assert_eq!(parse("255").unwrap(), 255);
        assert_eq!(parse("0").unwrap(), 0);
    }

    #[test]
    fn test_and() {
        assert_eq!(parse("0xFF & 0x0F").unwrap(), 0x0F);
        assert_eq!(parse("0xF0 & 0x0F").unwrap(), 0x00);
    }

    #[test]
    fn test_or() {
        assert_eq!(parse("0xF0 | 0x0F").unwrap(), 0xFF);
        assert_eq!(parse("0x01 | 0x02").unwrap(), 0x03);
    }

    #[test]
    fn test_xor() {
        assert_eq!(parse("0xFF ^ 0x0F").unwrap(), 0xF0);
        assert_eq!(parse("0x0F ^ 0x0F").unwrap(), 0x00);
    }

    #[test]
    fn test_not() {
        assert_eq!(parse("~0x00").unwrap(), 0xFFFFFFFFFFFFFFFF);
        assert_eq!(parse("~0xFF").unwrap(), 0xFFFFFFFFFFFFFF00);
    }

    #[test]
    fn test_shift_left() {
        assert_eq!(parse("1 << 8").unwrap(), 256);
        assert_eq!(parse("0x01 << 4").unwrap(), 0x10);
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(parse("256 >> 8").unwrap(), 1);
        assert_eq!(parse("0xF0 >> 4").unwrap(), 0x0F);
    }

    #[test]
    fn test_precedence_shift_over_bitwise() {
        // 0xFF & 0xF0 << 2 should be 0xFF & (0xF0 << 2)
        let result = parse("0xFF & 0xF0 << 2").unwrap();
        assert_eq!(result, 0xFF & (0xF0 << 2));
        assert_ne!(result, (0xFF & 0xF0) << 2);
    }

    #[test]
    fn test_precedence_and_over_or() {
        // 0xF0 | 0x0F & 0xFF should be 0xF0 | (0x0F & 0xFF)
        let result = parse("0xF0 | 0x0F & 0xFF").unwrap();
        assert_eq!(result, 0xF0 | (0x0F & 0xFF));
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(parse("(0xFF & 0xF0) << 2").unwrap(), (0xFF & 0xF0) << 2);
        assert_eq!(parse("(0x01 | 0x02) ^ 0x03").unwrap(), (0x01 | 0x02) ^ 0x03);
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(parse("((0xFF & 0xF0) | 0x0F)").unwrap(), 0xFF);
    }

    #[test]
    fn test_complex_expression() {
        // (0xFF ^ 0x0F) << 4 & 0xFF0
        let result = parse("(0xFF ^ 0x0F) << 4 & 0xFF0").unwrap();
        let expected = ((0xFF ^ 0x0F) << 4) & 0xFF0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_not_with_parens() {
        assert_eq!(parse("~(0xFF & 0x0F)").unwrap(), !(0xFF & 0x0F));
    }

    #[test]
    fn test_invalid_expression() {
        assert!(parse("").is_err());
        assert!(parse("   ").is_err());
        assert!(parse("0xFF &").is_err());
        assert!(parse("& 0xFF").is_err());
    }

    #[test]
    fn test_mismatched_parens() {
        assert!(parse("(0xFF").is_err());
        assert!(parse("0xFF)").is_err());
        assert!(parse("((0xFF)").is_err());
    }
}
