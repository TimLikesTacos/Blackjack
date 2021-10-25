//! The standard deck has 52 cards in each.  One of each demonination and suit.  
//! However, decks can be made from multiple decks, containing identical cards.

use crate::card::{Card, Denomination, Denomination::*, Suit, Visible};
use crate::errors::BlJaError;
use std::error::Error;
use std::ops::{Deref, DerefMut};

const NUM_CARD_IN_DECK: usize = 52;
const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
const DEMONS: [Denomination; 13] = [
    Ace,
    King,
    Queen,
    Jack,
    Numerical(10),
    Numerical(9),
    Numerical(8),
    Numerical(7),
    Numerical(6),
    Numerical(5),
    Numerical(4),
    Numerical(3),
    Numerical(2),
];
/// A deck of cards can be represented just like a stack (a Vec in Rust).  
/// Cards are stacked on top of each other, and the cards are dealt from the top (pop method).  
/// The deck can be `cut` by placing the first section to the left of the cut to the right of
/// the right section of the cut.
/// This struct is designed around the deck structure of a blackjack deck, not necessarily
/// a generic deck for any game.  Due to this, the generic methods such as shuffling and cutting
/// which can be implemented by any type of deck for any game will be done by traits.
#[derive(Clone, Debug, PartialEq)]
pub struct Deck {
    pub(crate) deck: Vec<Card>,
    pub(crate) reshuffle: bool,
}

/// Deref and DerefMut is implmented as Deck is a simple wrapper for a Vec, and we still want
/// the functionality of a vec for other uses.
impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.deck
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deck
    }
}

impl Deck {
    /// Create a new deck using a number of nominal 52 card decks.
    /// // todo add the plastic card
    pub fn new(number_of_decks: usize) -> Result<Deck, Box<dyn Error>> {
        // A value of 1 will be added to the deck.  This is used to add the 'plastic card' when playing
        // blackjack
        if let Some(cap) = number_of_decks
            .checked_mul(NUM_CARD_IN_DECK)
            .and_then(|r| r.checked_add(1))
        {
            let mut deck = Vec::with_capacity(cap);
            // Add cards to the deck
            for _ in 0..number_of_decks {
                for s in SUITS.iter() {
                    for d in DEMONS.iter() {
                        deck.push(Card::new(*d, *s));
                    }
                }
            }

            Ok(Deck {
                deck,
                reshuffle: false,
            })
        } else {
            // Checked multiplication overflowed.  Not a reasonable deck
            Err(Box::new(BlJaError::TooManyDecks))
        }
    }

    /// Just a blackjack related alias for `.pop()`
    #[inline]
    pub fn deal(&mut self, faced_up: bool) -> Visible<Card> {
        if let Some(card) = self.pop() {
            if faced_up {
                Visible::FacedUp(card)
            } else {
                Visible::FacedDown(card)
            }
        } else {
            if self.reshuffle {
                panic!("Out of cards");
            }
            self.reshuffle = true;
            self.deal(faced_up)
        }
    }
}

#[cfg(test)]
mod decktests {
    use super::*;

    #[test]
    fn newdeck() -> Result<(), Box<dyn Error>> {
        let onedeck = Deck::new(1)?;
        assert_eq!(onedeck.len(), 52);
        assert_eq!(onedeck.capacity(), 53);

        let sixdeck = Deck::new(6)?;
        assert_eq!(sixdeck.len(), 312);
        assert_eq!(sixdeck.capacity(), 313);

        let expecterr = Deck::new(usize::MAX / 50);
        assert!(expecterr.is_err());
        Ok(())
    }

    #[test]
    fn dealtest() -> Result<(), Box<dyn Error>> {
        let mut thedeck = Deck::new(1)?;
        let mut card1 = thedeck.deal(false);
        assert!(card1.is_faced_down());
        card1 = card1.flip_up();
        assert!(card1.is_faced_up());
        assert!(card1.denom() == Denomination::Numerical(2));
        assert_eq!(thedeck.len(), 51);

        let card2 = thedeck.deal(true);

        assert!(card2.is_faced_up());
        assert!(card2.denom() == Denomination::Numerical(3));
        assert_eq!(thedeck.len(), 50);

        Ok(())
    }
}
