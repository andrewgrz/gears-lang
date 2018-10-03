use std::io;

#[derive(Debug)]
pub enum GearsError {
    RustIOError(io::Error),
    FunctionNotFound(String),
    InternalCompilerError(String),
    TypeError(String),
}

impl From<io::Error> for GearsError {
    fn from(error: io::Error) -> Self {
        GearsError::RustIOError(error)
    }
}

impl PartialEq for GearsError {
    fn eq(&self, other: &GearsError) -> bool {
        use self::GearsError::*;

        match self {
            FunctionNotFound(l) => {
                match other {
                    FunctionNotFound(r) => l == r,
                    _ => false
                }
            },
            InternalCompilerError(l) => {
                match other {
                    InternalCompilerError(r) => l == r,
                    _ => false
                }
            },
            TypeError(l) => {
                match other {
                    TypeError(r) => l == r,
                    _ => false
                }
            }
            _ => false
        }
    }
}
