use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ParsingError{
    pub filename: String,
    pub line: usize,
    pub message: String,
}

impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:{}  {}", self.filename, self.line, self.message)
    }
}
