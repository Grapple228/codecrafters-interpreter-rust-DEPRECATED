use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone, thiserror::Error)]
pub enum ScannerError{
    UnexpectedCharacter(usize, char),
    UnterminatedString(usize),
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::UnexpectedCharacter(line, character) => write!(f, "[line {}] Error: Unexpected character: {}", line, character),
            Self::UnterminatedString(line) => write!(f, "[line {}] Error: Unterminated string.", line),            
        }
    }
}