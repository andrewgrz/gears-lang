use lalrpop_util::ParseError;
use lexer::{LexicalError, Span, Token};
use std::io;

#[derive(Debug)]
pub enum InterOpErrorType {
    TooManyArgs,
    TooFewArgs,
}

#[derive(Debug)]
pub enum GearsError {
    RustIOError(io::Error),
    FunctionNotFound(String),
    InternalCompilerError(String),
    TypeError(String),
    SymbolNotFound(String),
    InterOpError {
        error: InterOpErrorType,
        message: String,
    },
    ParseError {
        location: Span,
        message: String,
    },
}

impl From<io::Error> for GearsError {
    fn from(error: io::Error) -> Self {
        GearsError::RustIOError(error)
    }
}

impl<'a> From<ParseError<Span, Token, LexicalError>> for GearsError {
    fn from(error: ParseError<Span, Token, LexicalError>) -> Self {
        match error {
            ParseError::InvalidToken { location } => GearsError::ParseError {
                message: format!("Invalid token found at {}", &location),
                location: location,
            },
            ParseError::UnrecognizedToken { token, expected } => {
                let (mut token_msg, location) = match token {
                    Some(tok) => (
                        format!("Unexpected token ({:?}) at {}", tok.1, tok.0),
                        tok.0,
                    ),
                    None => (format!("Unexpected EOF token"), Span::new(0, 0)),
                };

                token_msg += &format!("Expected one of: {:?}", expected);

                GearsError::ParseError {
                    location: location,
                    message: token_msg,
                }
            }
            ParseError::ExtraToken { token } => GearsError::ParseError {
                message: format!("Extra token ({:?}) at {}", token.1, token.0),
                location: token.0,
            },
            ParseError::User { error } => GearsError::ParseError {
                location: Span::new(0, 0),
                message: format!("{:?}", error),
            },
        }
    }
}

impl PartialEq for GearsError {
    fn eq(&self, other: &GearsError) -> bool {
        use self::GearsError::*;

        match self {
            FunctionNotFound(l) => match other {
                FunctionNotFound(r) => l == r,
                _ => false,
            },
            InternalCompilerError(l) => match other {
                InternalCompilerError(r) => l == r,
                _ => false,
            },
            TypeError(l) => match other {
                TypeError(r) => l == r,
                _ => false,
            },
            _ => false,
        }
    }
}
