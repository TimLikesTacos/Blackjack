use std::fmt::{Display, Formatter};

/// The most basic representation of a playing card.    
/// The `denomination` is the number of letter
/// The `suit` is the basic four suits.
/// Values for cards, or if values are greater than or less than others
/// depends on the specific game, so implementations of those attributes
/// will be handled in respective modules by use of traits.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Card {
    denomination: Denomination,
    suit: Suit,
}

impl Card {
    pub fn new(denomination: Denomination, suit: Suit) -> Card {
        Card { denomination, suit }
    }

    pub fn denom(&self) -> Denomination {
        self.denomination
    }
    pub fn suit(&self) -> Suit {
        self.suit
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Denomination {
    Ace,
    King,
    Queen,
    Jack,
    Numerical(i32),
    Extra(&'static str),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

impl Display for Denomination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Denomination::*;

        let str = match self {
            Ace => "A".to_owned(),
            King => "K".to_owned(),
            Queen => "Q".to_owned(),
            Jack => "J".to_owned(),
            Numerical(v) => v.to_string(),
            Extra(str) => str.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Suit::*;

        let char = match self {
            Clubs => '\u{2663}',
            Spades => '\u{2660}',
            Hearts => '\u{2661}',
            Diamonds => '\u{2662}',
        };

        write!(f, "{}", char)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.denomination, self.suit)
    }
}

#[cfg(test)]
mod cardtests {
    use super::*;
    use std::error::Error;

    #[test]
    fn card_disp() -> Result<(), Box<dyn Error>> {
        let ace = Card::new(Denomination::Ace, Suit::Spades);
        let expected = "A \u{2660}";
        assert_eq!(format!("{}", ace), expected);

        let eight = Card::new(Denomination::Numerical(8), Suit::Hearts);
        let expected = "8 \u{2661}";
        assert_eq!(format!("{}", eight), expected);

        let four = Card::new(Denomination::Numerical(4), Suit::Diamonds);
        let expected = "4 \u{2662}";
        assert_eq!(format!("{}", four), expected);

        let jack = Card::new(Denomination::Jack, Suit::Clubs);
        let expected = "J \u{2663}";
        assert_eq!(format!("{}", jack), expected);

        Ok(())
    }

    #[test]
    fn card_get() {
        let eight = Card::new(Denomination::Numerical(8), Suit::Hearts);
        assert_eq!(eight.denom(), Denomination::Numerical(8));
        assert_eq!(eight.suit, Suit::Hearts);
    }
}
