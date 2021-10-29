use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

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

pub trait BlackJackScore {
    fn score(&self) -> usize;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Denomination {
    Ace,
    King,
    Queen,
    Jack,
    Numerical(usize),
    Extra(&'static str),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
    Pumpkin,
}

// Similar to `Option`, but the `None` equivalent value carrier a card.  Must be flipped to be unwraped.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Visible<T> {
    FacedUp(T),
    FacedDown(T),
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

    pub fn is_10card(self) -> bool {
        use Denomination::*;
        match self.denomination {
            King | Queen | Jack => true,
            Numerical(x) if x == 10 => true,
            _ => false,
        }
    }

    pub fn is_ace(self) -> bool {
        use Denomination::*;
        match self.denomination {
            Ace => true,
            _ => false,
        }
    }
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
            Pumpkin => '\u{f383}',
        };

        write!(f, "{}", char)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.denomination, self.suit)
    }
}

impl BlackJackScore for Card {
    fn score(&self) -> usize {
        use Denomination::*;
        match self.denomination {
            Ace => 11,
            Numerical(v) => v,
            Extra(_) => 0,
            _ => 10,
        }
    }
}

impl<T> Visible<T> {
    pub fn is_faced_up(&self) -> bool {
        match self {
            Visible::FacedUp(_) => true,
            _ => false,
        }
    }

    pub fn is_faced_down(&self) -> bool {
        !self.is_faced_up()
    }

    pub fn flip_up(self) -> Visible<T> {
        match self {
            Visible::FacedUp(_) => self,
            Visible::FacedDown(x) => Visible::FacedUp(x),
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            Visible::FacedUp(val) => val,
            Visible::FacedDown(_) => panic!("called `Visible::unwrap()` on a `FacedDown` value"),
        }
    }
}

impl<T: Display> Display for Visible<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Visible::FacedDown(_) => write!(f, "XXX"),
            Visible::FacedUp(c) => c.fmt(f),
        }
    }
}

impl<T> Deref for Visible<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Visible::FacedDown(x) => x,
            Visible::FacedUp(x) => x,
        }
    }
}

impl<T> DerefMut for Visible<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Visible::FacedDown(x) => x,
            Visible::FacedUp(x) => x,
        }
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
        assert!(!ace.is_10card());

        let eight = Card::new(Denomination::Numerical(8), Suit::Hearts);
        let expected = "8 \u{2661}";
        assert_eq!(format!("{}", eight), expected);
        assert!(!eight.is_10card());

        let four = Card::new(Denomination::Numerical(4), Suit::Diamonds);
        let expected = "4 \u{2662}";
        assert_eq!(format!("{}", four), expected);
        assert!(!four.is_10card());

        let jack = Card::new(Denomination::Jack, Suit::Clubs);
        let expected = "J \u{2663}";
        assert_eq!(format!("{}", jack), expected);
        assert!(jack.is_10card());

        Ok(())
    }

    #[test]
    fn card_get() {
        let eight = Card::new(Denomination::Numerical(8), Suit::Hearts);
        assert_eq!(eight.denom(), Denomination::Numerical(8));
        assert_eq!(eight.suit, Suit::Hearts);
    }
}
