use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BlJaError {
    TooManyDecks,
    InvalidIndex(usize),
}

impl Error for BlJaError {}

impl Display for BlJaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlJaError::TooManyDecks => write!(f, "Too many decks.  Recommend between 1 and 8"),
            BlJaError::InvalidIndex(i) => write!(f, "Invalid Index: {}", i),
        }
    }
}
