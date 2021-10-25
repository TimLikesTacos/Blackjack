use crate::deck::Deck;
use crate::errors::BlJaError;
use rand::Rng;
use std::error::Error;

/// Trait that the object can by shuffled
pub trait Shufflable {
    fn shuffle(&mut self);
}

/// Trait that the object but by `cut` as in a deck of card.
pub trait Cuttable {
    fn cut(&mut self, at_index: usize) -> Result<(), Box<dyn Error>>;
}

impl Shufflable for Deck {
    fn shuffle(&mut self) {
        // Requires using random numbers.
        // Start from the bottom of the deck, swap with one randomly located in the rest of the deck.
        // Move down the line, swapping cards at random with the i..vec.len() range.
        let mut rng = rand::thread_rng();
        let len = self.len();
        for i in 0..(len - 2) {
            let other = rng.gen_range(i + 1..len);
            self.swap(i, other);
        }
    }
}

impl Cuttable for Deck {
    fn cut(&mut self, at_index: usize) -> Result<(), Box<dyn Error>> {
        if at_index > self.len() {
            return Err(Box::new(BlJaError::InvalidIndex(at_index)));
        }
        let (left, right) = self.split_at(at_index);
        self.deck = right.into_iter().chain(left.into_iter()).cloned().collect();
        Ok(())
    }
}

#[cfg(test)]
mod trait_tests {
    use super::*;
    #[test]
    fn shuffle() -> Result<(), Box<dyn Error>> {
        let mut onedeck = Deck::new(1)?;
        let orig = onedeck.clone();
        let first = onedeck.get(0).unwrap().clone();
        let last = onedeck.get(onedeck.len() - 1).unwrap().clone();

        assert_eq!(onedeck.get(0).unwrap(), &first);
        assert_eq!(onedeck.get(onedeck.len() - 1).unwrap(), &last);

        onedeck.shuffle();
        assert_ne!(onedeck, orig);
        assert_eq!(onedeck.len(), 52);
        let mut count = 0;
        let mut same = true;
        // First card can never be the same.  Last card may be.
        assert_ne!(onedeck.get(0).unwrap(), &first);
        while count < 4 {
            if onedeck.get(onedeck.len() - 1).unwrap() == &last {
                same = true;
            } else {
                same = false;
                break;
            }
            count += 1;
        }
        assert!(!same);

        let mut sixdeck = Deck::new(6)?;
        let orig = sixdeck.clone();
        let first = sixdeck.get(0).unwrap().clone();
        let last = sixdeck.get(sixdeck.len() - 1).unwrap().clone();

        assert_eq!(sixdeck.get(0).unwrap(), &first);
        assert_eq!(sixdeck.get(sixdeck.len() - 1).unwrap(), &last);

        sixdeck.shuffle();
        assert_ne!(sixdeck, orig);
        assert_eq!(sixdeck.len(), 312);
        let mut count = 0;
        let mut same = true;
        // First card can never be the same.  Last card may be.
        assert_ne!(sixdeck.get(0).unwrap(), &first);
        while count < 4 {
            if sixdeck.get(sixdeck.len() - 1).unwrap() == &last {
                same = true;
            } else {
                same = false;
                break;
            }
            count += 1;
        }
        assert!(!same);
        Ok(())
    }

    #[test]
    fn cut() -> Result<(), Box<dyn Error>> {
        let mut deck = Deck::new(1)?;
        let orig = deck.clone();

        // Cutting beyond the length of the deck should be an error
        assert!(deck.cut(53).is_err());
        // Cutting at the beginning should not do anything
        deck.cut(0)?;
        assert_eq!(deck.deck, orig.deck);
        // cutting after the last card should not do anything.
        deck.cut(52)?;
        assert_eq!(deck.deck, orig.deck);

        deck.cut(2)?;
        assert_eq!(deck.get(0).unwrap(), orig.get(2).unwrap());
        assert_eq!(deck.get(deck.len() - 2).unwrap(), orig.get(0).unwrap());
        assert_eq!(deck.len(), 52);

        Ok(())
    }
}
