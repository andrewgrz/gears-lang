
use std::io;

#[derive(Debug)]
pub enum GearsError {
    RustIOError(io::Error)
}

impl From<io::Error> for GearsError {
    fn from(error: io::Error) -> Self {
        GearsError::RustIOError(error)
    }
}