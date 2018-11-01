use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum LexicalError {
    UnknownToken(char),
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Span {
    column: usize,
    line: usize,
}

impl Span {
    pub fn new(line: usize, column: usize) -> Span {
        Span {
            column: column,
            line: line,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Char: {}", self.line, self.column)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    // No Data
    Comma,
    LParen,
    RParen,
    LBracket,
    RBracket,
    SemiColon,
    Colon,
    Eq,
    EqEq,
    NotEq,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
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
    While,
}

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

pub fn lex(input: &str) -> Vec<Spanned<Token, Span, LexicalError>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars();
    let mut lookahead = chars.next();
    let mut line = 1;
    let mut column = 1;

    macro_rules! token_data {
        ($tok:expr, $size: expr) => {{
            let start = column;
            column += $size;
            tokens.push(Ok((Span::new(line, start), $tok, Span::new(line, column))));
        }};
    }

    macro_rules! token {
        ($tok:ident, $size: expr) => {{
            let start = column;
            column += $size;
            tokens.push(Ok((
                Span::new(line, start),
                Token::$tok,
                Span::new(line, column),
            )));
        }};
    }

    while let Some(c) = lookahead {
        match c {
            ',' => token!(Comma, 1),
            '(' => token!(LParen, 1),
            ')' => token!(RParen, 1),
            '{' => token!(LBracket, 1),
            '}' => token!(RBracket, 1),
            ';' => token!(SemiColon, 1),
            ':' => token!(Colon, 1),
            '=' => {
                lookahead = chars.next();
                if let Some(c) = lookahead {
                    match c {
                        '=' => token!(EqEq, 2),
                        _ => {
                            token!(Eq, 1);
                            continue;
                        }
                    }
                } else {
                    token!(Eq, 1)
                }
            }
            '!' => {
                lookahead = chars.next();
                if let Some(c) = lookahead {
                    match c {
                        '=' => token!(NotEq, 2),
                        _ => tokens.push(Err(LexicalError::UnknownToken(c))),
                    }
                } else {
                    tokens.push(Err(LexicalError::UnknownToken(c)))
                }
            }
            '<' => {
                lookahead = chars.next();
                if let Some(c) = lookahead {
                    match c {
                        '=' => token!(LessThanEq, 2),
                        _ => {
                            token!(LessThan, 1);
                            continue;
                        }
                    }
                } else {
                    token!(LessThan, 1)
                }
            }
            '>' => {
                lookahead = chars.next();
                if let Some(c) = lookahead {
                    match c {
                        '=' => token!(GreaterThanEq, 2),
                        _ => {
                            token!(GreaterThan, 1);
                            continue;
                        }
                    }
                } else {
                    token!(GreaterThan, 1)
                }
            }
            '+' => token!(Plus, 1),
            '-' => token!(Minus, 1),
            '*' => token!(Star, 1),
            '/' => token!(Slash, 1),

            _ if c.is_alphabetic() || c == '_' => {
                let (tmp, next) = take_while(c, &mut chars, |c| {
                    c.is_alphabetic() || c == '_' || c.is_digit(10)
                });
                lookahead = next;
                let len = tmp.len();

                match tmp.as_str() {
                    "def" => token!(Def, len),
                    "let" => token!(Let, len),
                    "if" => token!(If, len),
                    "else" => token!(Else, len),
                    "true" => token!(True, len),
                    "false" => token!(False, len),
                    "while" => token!(While, len),
                    _ => token_data!(Token::Name(tmp), len),
                }

                continue;
            }

            _ if c.is_digit(10) => {
                let (tmp, next) = take_while(c, &mut chars, |c| c.is_digit(10));
                lookahead = next;
                token_data!(Token::Integer(i64::from_str(&tmp).unwrap()), tmp.len());
                continue;
            }

            ' ' => column += 1,
            '\n' => {
                line += 1;
                column = 1;
            }
            '#' => {
                let (_, next) = take_while(c, &mut chars, |c| c != '\n');
                lookahead = next;
                continue;
            }
            _ => tokens.push(Err(LexicalError::UnknownToken(c))),
        }

        // Advance to next character by default
        lookahead = chars.next();
    }

    tokens
}

fn take_while<C, F>(c0: char, chars: &mut C, f: F) -> (String, Option<char>)
where
    C: Iterator<Item = char>,
    F: Fn(char) -> bool,
{
    let mut buf = String::new();

    buf.push(c0);

    while let Some(c) = chars.next() {
        if !f(c) {
            return (buf, Some(c));
        }

        buf.push(c);
    }

    return (buf, None);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expect {
        ($left:expr, $right:expr) => {{
            let result: Vec<Token> = lex($left).into_iter().map(|r| r.unwrap().1).collect();

            if result.len() != $right.len() {
                panic!(format!(
                    "Uneven length of tokens:\nLeft: {:?}\nRight:{:?}",
                    result, $right
                ));
            }

            for (index, tok) in result.iter().enumerate() {
                if $right[index] != *tok {
                    panic!(format!(
                        "Tokens did not match at index: {}. Found: {:?}, Expected: {:?}",
                        index, tok, $right[index]
                    ));
                }
            }
        }};
    }

    macro_rules! ident_test {
        ($string: expr) => {{
            expect!($string, vec![Token::Name($string.to_string())]);
        }};
    }

    #[test]
    fn test_single_token() {
        use super::Token::*;

        expect!(",", vec![Comma]);
        expect!("(", vec![LParen]);
        expect!(")", vec![RParen]);
        expect!("{", vec![LBracket]);
        expect!("}", vec![RBracket]);
        expect!(";", vec![SemiColon]);
        expect!(":", vec![Colon]);
        expect!("=", vec![Eq]);
        expect!("==", vec![EqEq]);
        expect!("!=", vec![NotEq]);
        expect!("<", vec![LessThan]);
        expect!("<=", vec![LessThanEq]);
        expect!(">", vec![GreaterThan]);
        expect!(">=", vec![GreaterThanEq]);
        expect!("+", vec![Plus]);
        expect!("-", vec![Minus]);
        expect!("*", vec![Star]);
        expect!("/", vec![Slash]);
    }

    #[test]
    fn test_keywords() {
        use super::Token::*;

        expect!("def", vec![Def]);
        expect!("let", vec![Let]);
        expect!("if", vec![If]);
        expect!("else", vec![Else]);
        expect!("true", vec![True]);
        expect!("false", vec![False]);
    }

    #[test]
    fn test_ident() {
        ident_test!("test");
        ident_test!("t");
        ident_test!("_tes");
        ident_test!("_");
        ident_test!("TEST");
        ident_test!("Test");
        ident_test!("tesT");
        ident_test!("abcdefghijklmnopqrstuvwxyz0123456789");
        ident_test!("t3st");
    }

    #[test]
    fn test_comments() {
        use super::Token::*;

        expect!("()#Test", vec![LParen, RParen]);
    }

    #[test]
    fn test_spanning() {
        use super::Token::*;

        fn get_token(
            token: Token,
            line: usize,
            start: usize,
            end: usize,
        ) -> Result<(Span, Token, Span), LexicalError> {
            Ok((Span::new(line, start), token, Span::new(line, end)))
        }

        assert_eq!(
            lex("( )\n() def"),
            vec![
                get_token(LParen, 1, 1, 2),
                get_token(RParen, 1, 3, 4),
                get_token(LParen, 2, 1, 2),
                get_token(RParen, 2, 2, 3),
                get_token(Def, 2, 4, 7),
            ]
        );
    }
}
