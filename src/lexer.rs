use errors::GearsError;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Span {
    column: usize,
    line: usize,
}

impl Span {
    fn new(line: usize, column: usize) -> Span {
        Span {
            column: column,
            line: line,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    start: Span,
    end: Span,
    tok_type: TokType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokType {
    // No Data
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    SemiColon,
    Colon,
    Eq,
    Plus,
    Minus,
    Star,
    Slash,

    // Data
    Name(String),
    Integer(i64),

    // Keywords
    Def,
    Let,
    If,
    Else,
    True,
    False,
}

pub fn lex(input: &str) -> Result<Vec<Token>, GearsError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars();
    let mut lookahead = chars.next();
    let mut line = 1;
    let mut column = 1;

    macro_rules! token {
        ($tok:ident, $size: expr) => {{
            let start = column;
            column += $size;
            tokens.push(Token {
                tok_type: TokType::$tok,
                start: Span::new(line, start),
                end: Span::new(line, column),
            });
        }};
    }

    while let Some(c) = lookahead {
        match c {
            ',' => token!(Comma, 1),
            '(' => token!(LParen, 1),
            ')' => token!(RParen, 1),
            '{' => token!(LBrace, 1),
            '}' => token!(RBrace, 1),
            ';' => token!(SemiColon, 1),
            ':' => token!(Colon, 1),
            '=' => token!(Eq, 1),
            '+' => token!(Plus, 1),
            '-' => token!(Minus, 1),
            '*' => token!(Star, 1),
            '/' => token!(Slash, 1),

            ' ' => column += 1,
            '\n' => {
                line += 1;
                column = 1;
            }
            _ => return Err(GearsError::UnrecognizedToken(Span::new(line, column))),
        }

        // Advance to next character by default
        lookahead = chars.next();
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expect {
        ($left:expr, $right:expr) => {{
            let result = lex($left).unwrap();
            
            for (index, tok) in result.iter().enumerate() {
                if $right[index] != tok.tok_type {
                    panic!(format!("Tokens did not match at index: {}. Found: {:?}, Expected: {:?}", index, tok, $right[index]));
                }
            }
        }};
    }

    #[test]
    fn test_single_char() {
        use super::TokType::*;
        expect!(",", vec![Comma]);
        expect!("(", vec![LParen]);
        expect!(")", vec![RParen]);
        expect!("{", vec![LBrace]);
        expect!("}", vec![RBrace]);
        expect!(";", vec![SemiColon]);
        expect!(":", vec![Colon]);
        expect!("=", vec![Eq]);
        expect!("+", vec![Plus]);
        expect!("-", vec![Minus]);
        expect!("*", vec![Star]);
        expect!("/", vec![Slash]);
    }

    #[test]
    fn test_spanning() {
        assert_eq!(
            lex("( )\n()").unwrap(),
            vec![
                Token {
                    tok_type: TokType::LParen,
                    start: Span::new(1, 1),
                    end: Span::new(1, 2),
                },
                Token {
                    tok_type: TokType::RParen,
                    start: Span::new(1, 3),
                    end: Span::new(1, 4),
                },
                Token {
                    tok_type: TokType::LParen,
                    start: Span::new(2, 1),
                    end: Span::new(2, 2),
                },
                Token {
                    tok_type: TokType::RParen,
                    start: Span::new(2, 2),
                    end: Span::new(2, 3),
                }
            ]
        );
    }
}
