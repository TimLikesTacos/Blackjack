use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum BlJaError {
    TooManyDecks,
    InvalidIndex(usize),
    ImproperAction(&'static str),
    NotEnoughMoney,
    ExcessiveInsurance,
}

impl Error for BlJaError {}

impl Display for BlJaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlJaError::TooManyDecks => write!(f, "Too many decks.  Recommend between 1 and 8"),
            BlJaError::InvalidIndex(i) => write!(f, "Invalid Index: {}", i),
            BlJaError::ImproperAction(str) => write!(f, "Improper Action: {}", str.to_string()),
            BlJaError::NotEnoughMoney => write!(f, "Player does not have enough money"),
            BlJaError::ExcessiveInsurance => {
                write!(f, "Excessive insurance bet.  Must be <= 1/2 of bet")
            }
        }
    }
}
