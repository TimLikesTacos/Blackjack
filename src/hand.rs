use crate::card::Denomination::*;
use crate::card::{BlackJackScore, Card, Denomination, Visible};
use crate::constants::{DOUBLECARDCOUNT, SPLITCARDCOUNT, TWENTYONE};
use crate::errors::BlJaError;
use crate::Res;
use num::Rational64;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HandType {
    Normal,      // normal hand
    Soft,        // 'soft', as in a Ace is present and 11, not one is present
    Natural,     // Natural blackjack, only 2 cards
    Split,       // The hand has been split
    SplitSoft,   // The hand has been split, and is a soft hand
    SplitAces,   // Split a hand with two aces
    Doubled,     // The hand has been doubled down on
    DoubledSoft, // Doubled with ace
    Bust,        // Busted.
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Action {
    Hit,
    Stand,
    Split,
    Double,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Hand {
    cards: Vec<Visible<Card>>,
    htype: HandType,
    score: usize,
    bet: Option<Rational64>,
}

#[derive(Debug)]
/// A wrapper to Hand to represent that the hand is set for doubling, but needs a new card.
/// This keeps the only action that can occur to the hand to an insert. Once the insert occurs, the hand is returned
pub struct Doubled(Hand);

impl Doubled {
    pub fn insert(mut self, card: Visible<Card>) -> Hand {
        self.0.insert(card);
        self.0
    }
}

#[derive(Debug)]
/// A wrapper to Hand to represent that the hand has been split, but needs a new card
/// /// This keeps the only action that can occur to the hand to an insert. Once the insert occurs, the hand is returned
pub struct BeenSplit(Hand);

impl BeenSplit {
    pub fn insert(mut self, card: Visible<Card>) -> Hand {
        self.0.insert(card);
        self.0
    }
}

impl Hand {
    #[inline]
    pub fn new() -> Hand {
        Hand {
            cards: vec![],
            htype: HandType::Normal,
            score: 0,
            bet: None,
        }
    }

    /// add a dealt card to the hand.  Adjusts score and handtype.
    #[inline]
    pub fn insert(&mut self, card: Visible<Card>) -> HandType {
        // add card to the hand
        self.cards.push(card);
        // No further actions are needed if the card is facedown.
        if card.is_faced_down() {
            return self.htype;
        }
        self.add_card_to_score(card.unwrap())
    }

    /// Action to add any flipped over card to the score and adjust the handtype
    pub fn flip_over(&mut self) -> HandType {
        // Only the last card in the hand should be faced down.
        if self.cards.len() == 0 {
            return self.htype;
        }
        if let Some(last) = self.cards.get(self.cards.len() - 1) {
            match last {
                Visible::FacedUp(_) => self.htype, // already face up
                Visible::FacedDown(c) => {
                    last.flip_up();
                    self.add_card_to_score(*c)
                }
            }
        } else {
            self.htype
        }
    }

    /// Get the actions available for the hand
    pub fn actions(&self) -> HashSet<Action> {
        use HandType::*;

        match self.htype {
            Normal if self.score == TWENTYONE => HashSet::with_capacity(0), // No actions for score of 21.
            Normal | Soft | Split | SplitSoft => {
                let mut set = HashSet::with_capacity(4);
                // check if splitable
                if self.splitable() {
                    set.insert(Action::Split);
                }
                if self.doubleable() {
                    set.insert(Action::Double);
                }
                set.insert(Action::Hit);
                set.insert(Action::Stand);
                set
            }
            _ => HashSet::with_capacity(0), // Bust, Natural, Doubled<...>, SplitAces
        }
    }

    /// Performs a split, returns two single card hands.
    pub fn split_hand(self) -> Res<(BeenSplit, BeenSplit)> {
        if !self.splitable() || self.bet.is_none() {
            return Err(Box::new(BlJaError::ImproperAction(
                "Tried to split cards that are not the same denomination or has no bet",
            )));
        }
        let mut cards = self.cards;
        let card1 = cards.pop().ok_or_else(|| "Not enough cards to split")?; // splittable() checks for proper card count
        let card2 = cards.pop().ok_or_else(|| "Not enough cards to split")?;
        // Get the appropriate split type
        let thetype = if card1.denom() == Denomination::Ace {
            HandType::SplitAces
        } else {
            HandType::Split
        };

        let mut hand1 = Hand::new();
        let mut hand2 = Hand::new();
        hand1.insert(card1);
        hand2.insert(card2);
        hand1.htype = thetype;
        hand2.htype = thetype;
        hand1.bet = self.bet;
        hand2.bet = self.bet;

        Ok((BeenSplit(hand1), BeenSplit(hand2)))
    }

    ///Performs actions before adding a card
    pub fn double(&mut self) -> Res<Doubled> {
        if !self.doubleable() {
            return Err(Box::new(BlJaError::ImproperAction(
                "Cannot double this hand",
            )));
        }
        // Important! update handtype first before adding card. This is needed to prevent changing Soft to Doubled instead of DoubledSoft
        let newtype = match self.htype {
            HandType::Normal => HandType::Doubled,
            HandType::Soft => HandType::DoubledSoft,
            HandType::Split => HandType::Doubled,
            HandType::SplitSoft => HandType::DoubledSoft, // at this point, a split hand does not matter towards other rules.
            _ => {
                return Err(Box::new(BlJaError::ImproperAction(
                    "Tried to double an initial hand type that does not allow doubling",
                )));
            }
        };

        // Double the bet
        self.bet = Some(self.bet.ok_or_else(|| "Doubling a hand that has no bet")? * 2);
        self.htype = newtype;
        Ok(Doubled(self.clone()))
    }

    fn splitable(&self) -> bool {
        if self.cards.len() == SPLITCARDCOUNT {
            if let Some(first) = self.cards.get(0) {
                // Return true if all cards are eqwual to the denomination of the first card.
                return self.cards.iter().all(|c| first.denom() == c.denom());
            }
        }
        false
    }

    #[inline]
    fn doubleable(&self) -> bool {
        match self.htype {
            HandType::Bust | HandType::Natural => false,
            _ => self.cards.len() == DOUBLECARDCOUNT,
        }
    }

    #[inline]
    pub fn set_bet(&mut self, bet: Rational64) {
        self.bet = Some(bet)
    }

    #[inline]
    pub fn bet(&self) -> Option<Rational64> {
        self.bet
    }

    #[inline]
    pub fn score(&self) -> usize {
        self.score
    }

    #[inline]
    pub fn hand_type(&self) -> HandType {
        self.htype
    }

    #[inline]
    pub fn num_cards(&self) -> usize {
        self.cards.len()
    }

    #[inline]
    pub fn card_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = &'a Visible<Card>> + 'b {
        self.cards.iter()
    }

    /// Local function that does the grunt work to add the card to the score and adjust the handtype.
    fn add_card_to_score(&mut self, card: Card) -> HandType {
        use HandType::*;
        // Get the score
        let cardscore = card.score();

        // Upgrade to a 'soft' hand if not already
        if card.denom() == Ace {
            let newh = match self.htype {
                Normal => Soft,
                Split => SplitSoft,
                Doubled => DoubledSoft,
                x => x, // No other type needs to change due to an ace.
            };
            self.htype = newh;
        }

        // Go from soft to hard if needed
        self.score += cardscore;

        // Adjust Soft/hard and score if needed, or mark as bust
        self.hard_or_bust();
        // Check if a natural occurred.
        self.check_natural();

        self.htype
    }

    fn hard_or_bust(&mut self) {
        use HandType::*;
        if self.score > TWENTYONE {
            self.htype = match self.htype {
                Soft => {
                    self.score -= 10;
                    Normal
                }
                SplitSoft => {
                    self.score -= 10;
                    Split
                }
                DoubledSoft => {
                    self.score -= 10;
                    Doubled
                }
                x => x,
            };
            // if still > 21, bust
            if self.score > TWENTYONE {
                self.htype = Bust;
            }
        }
    }

    fn check_natural(&mut self) {
        // Check for natural blackjack
        // This also checks that if the split was originally `normal` and 21 obtained with two
        // cards, that it will not be a `natural` type which increases payout.  It will stay `normal`
        // This is per the Bicycle rules for paying 1.0X for a natural after a split of a 10 card.
        use HandType::*;
        if self.cards.len() == 2 && self.score == TWENTYONE {
            self.htype = match self.htype {
                Soft | SplitAces => Natural,
                x => x,
            }
        }
    }

    pub fn is_first_card_ace(&self) -> bool {
        if self.cards.len() < 2 {
            return false;
        }
        self.cards[0].is_ace()
    }

    pub fn is_first_card_10card(&self) -> bool {
        if self.cards.len() < 2 {
            return false;
        }
        self.cards[0].is_10card()
    }

    pub fn peek_for_natural(&self) -> bool {
        if self.cards.len() != 2 {
            return false;
        }
        (self.cards[0].is_ace() && self.cards[1].is_10card())
            || (self.cards[1].is_ace() && self.cards[0].is_10card())
    }
}

#[cfg(test)]
mod handtests {
    use super::*;
    use crate::card::Visible::FacedUp;
    use crate::card::{Denomination, Suit};
    use crate::deck::Deck;
    use crate::hand::Action::{Double, Hit, Stand};
    use crate::hand::HandType::SplitAces;
    use crate::Res;
    use std::error::Error;

    #[test]
    fn handtest() -> Res<()> {
        let mut hand = Hand::new();
        let mut deck = Deck::new(1)?;
        hand.insert(deck.deal(true).unwrap());
        assert_eq!(hand.score, 2);
        assert_eq!(hand.htype, HandType::Normal);

        // add card face down
        hand.insert(deck.deal(false).unwrap());
        assert_eq!(hand.score, 2);
        assert_eq!(hand.htype, HandType::Normal);

        // flip over card
        hand.flip_over();
        assert_eq!(hand.score, 5);
        assert_eq!(hand.htype, HandType::Normal);

        let king = Card::new(Denomination::King, Suit::Hearts);
        let queen = Card::new(Denomination::Queen, Suit::Spades);

        hand.insert(Visible::FacedUp(king));
        assert_eq!(hand.score, 15);
        assert_eq!(hand.htype, HandType::Normal);

        hand.insert(Visible::FacedUp(queen));
        assert_eq!(hand.score, 25);
        assert_eq!(hand.htype, HandType::Bust);
        Ok(())
    }

    #[test]
    fn handtest2() -> Res<()> {
        let mut hand = Hand::new();
        let mut deck = Deck::new(1)?;

        let king = Card::new(Denomination::King, Suit::Hearts);
        let queen = Card::new(Denomination::Queen, Suit::Spades);

        let ace1 = Card::new(Denomination::Ace, Suit::Hearts);
        let ace2 = Card::new(Denomination::Ace, Suit::Diamonds);

        let nine = Card::new(Denomination::Numerical(9), Suit::Diamonds);

        hand.insert(Visible::FacedUp(king));
        assert_eq!(hand.score, 10);
        assert_eq!(hand.htype, HandType::Normal);

        hand.insert(Visible::FacedUp(ace1));
        assert_eq!(hand.score, 21);
        assert_eq!(hand.htype, HandType::Natural);

        let mut hand = Hand::new();

        hand.insert(Visible::FacedUp(ace1));
        assert_eq!(hand.score, 11);
        assert_eq!(hand.htype, HandType::Soft);

        hand.insert(Visible::FacedUp(ace2));
        assert_eq!(hand.score, 12);
        assert_eq!(hand.htype, HandType::Normal);

        hand.insert(Visible::FacedUp(nine));
        assert_eq!(hand.score, 21);
        assert_eq!(hand.htype, HandType::Normal);

        Ok(())
    }

    #[test]
    fn actions() -> Res<()> {
        use Action::*;

        let mut hand = Hand::new();

        let king = Card::new(Denomination::King, Suit::Hearts);
        let queen = Card::new(Denomination::Queen, Suit::Spades);
        let jack = Card::new(Denomination::Jack, Suit::Spades);
        let ace1 = Card::new(Denomination::Ace, Suit::Hearts);
        let ace2 = Card::new(Denomination::Ace, Suit::Diamonds);

        let nine = Card::new(Denomination::Numerical(9), Suit::Diamonds);
        let four = Card::new(Denomination::Numerical(4), Suit::Spades);
        let three = Card::new(Denomination::Numerical(3), Suit::Hearts);
        let two = Card::new(Denomination::Numerical(2), Suit::Clubs);

        //empty hand
        let actions = hand.actions();
        assert!(actions.contains(&Hit));
        assert!(actions.contains(&Stand));
        assert_eq!(actions.len(), 2);

        hand.insert(Visible::FacedUp(nine.clone()));

        // One card
        let actions = hand.actions();
        assert!(actions.contains(&Hit));
        assert!(actions.contains(&Stand));
        assert_eq!(actions.len(), 2);
        assert_eq!(hand.score(), 9);
        assert_eq!(hand.hand_type(), HandType::Normal);

        // add same card (possible in multiple decks), check split avail
        hand.insert(Visible::FacedUp(nine.clone()));
        let actions = hand.actions();
        assert!(actions.contains(&Hit));
        assert!(actions.contains(&Stand));
        assert!(actions.contains(&Split));
        assert!(actions.contains(&Double));
        assert_eq!(actions.len(), 4);
        assert_eq!(hand.score(), 18);
        assert_eq!(hand.hand_type(), HandType::Normal);

        // Add a three card.  Make sure 21 is score, no actions available, and normal type.
        hand.insert(Visible::FacedUp(three.clone()));
        let actions = hand.actions();
        assert_eq!(actions.len(), 0);
        assert_eq!(hand.score(), 21);
        assert_eq!(hand.hand_type(), HandType::Normal);

        Ok(())
    }

    #[test]
    fn actions2() -> Res<()> {
        /*
        Reset hand and try other combos
         */

        let jack = Card::new(Denomination::Jack, Suit::Spades);
        let ace1 = Card::new(Denomination::Ace, Suit::Hearts);
        let three = Card::new(Denomination::Numerical(3), Suit::Hearts);

        let mut splitaces = Hand::new();
        splitaces.insert(FacedUp(ace1.clone()));
        splitaces.htype = SplitAces; // simulate that a split has occurred

        let mut splitaces2 = splitaces.clone();

        // Check that split aces can form a natural.
        splitaces.insert(FacedUp(jack));
        assert_eq!(splitaces.hand_type(), HandType::Natural);
        let actions = splitaces.actions();
        assert_eq!(actions.len(), 0);

        splitaces2.insert(FacedUp(three));
        assert_eq!(splitaces2.hand_type(), SplitAces);
        let actions = splitaces2.actions();
        assert_eq!(actions.len(), 0);
        assert_eq!(splitaces2.score(), 14);

        Ok(())
    }

    #[test]
    // Test for actions with Ace in the first set of cards
    fn actions3() -> Res<()> {
        let queen = Card::new(Denomination::Queen, Suit::Spades);
        let ace1 = Card::new(Denomination::Ace, Suit::Hearts);
        let three = Card::new(Denomination::Numerical(3), Suit::Hearts);
        let nine = Card::new(Denomination::Numerical(9), Suit::Diamonds);

        let mut hand = Hand::new();
        hand.insert(FacedUp(three));
        hand.insert(FacedUp(ace1));
        assert_eq!(hand.score(), 14);
        assert_eq!(hand.hand_type(), HandType::Soft);
        let actions = hand.actions();
        assert_eq!(actions.len(), 3);
        assert!(actions.contains(&Hit));
        assert!(actions.contains(&Stand));
        assert!(actions.contains(&Double));

        // Remove the `soft` from the hand
        hand.insert(FacedUp(queen));
        assert_eq!(hand.score(), 14);
        assert_eq!(hand.hand_type(), HandType::Normal);
        let actions = hand.actions();
        assert_eq!(actions.len(), 2);
        assert!(actions.contains(&Hit));
        assert!(actions.contains(&Stand));

        // Bust the hand
        hand.insert(FacedUp(nine));
        assert_eq!(hand.score(), 23);
        assert_eq!(hand.hand_type(), HandType::Bust);
        let actions = hand.actions();
        assert_eq!(actions.len(), 0);

        Ok(())
    }

    #[test]
    // Test when an ace comes later and cannot be `soft`
    fn late_ace() -> Res<()> {
        let queen = FacedUp(Card::new(Denomination::Queen, Suit::Spades));
        let ace1 = FacedUp(Card::new(Denomination::Ace, Suit::Hearts));
        let three = FacedUp(Card::new(Denomination::Numerical(3), Suit::Hearts));

        let mut hand = Hand::new();

        hand.insert(queen);
        hand.insert(three);
        assert_eq!(hand.hand_type(), HandType::Normal);

        hand.insert(ace1);
        assert_eq!(hand.hand_type(), HandType::Normal);
        assert_eq!(hand.score(), 14);

        Ok(())
    }

    #[test]
    fn double() -> Res<()> {
        use HandType::*;

        let mut hand = Hand::new();
        hand.set_bet(100.into());
        let queen = Card::new(Denomination::Queen, Suit::Spades);
        let ace1 = Card::new(Denomination::Ace, Suit::Hearts);
        let three = Card::new(Denomination::Numerical(3), Suit::Hearts);
        let four = Card::new(Denomination::Numerical(4), Suit::Spades);
        let nine = Card::new(Denomination::Numerical(9), Suit::Diamonds);

        // Test a sucessfull double
        hand.insert(FacedUp(three));
        hand.insert(FacedUp(four.clone()));
        assert_eq!(hand.hand_type(), Normal);
        let hand = hand.double()?.insert(FacedUp(queen));
        assert_eq!(hand.hand_type(), Doubled);

        assert_eq!(hand.score(), 17);
        assert_eq!(hand.hand_type(), Doubled);
        let actions = hand.actions();
        assert_eq!(actions.len(), 0);
        assert_eq!(hand.bet().unwrap(), 200.into());

        // Check doubling a soft hand
        let mut hand = Hand::new();
        hand.set_bet(125.into());
        hand.insert(FacedUp(ace1));
        hand.insert(FacedUp(four));
        assert_eq!(hand.hand_type(), Soft);

        let hand = hand.double()?.insert(FacedUp(nine));
        assert_eq!(hand.score(), 14);
        assert_eq!(hand.hand_type(), Doubled);
        let actions = hand.actions();
        assert_eq!(actions.len(), 0);
        assert_eq!(hand.bet().unwrap(), 250.into());

        Ok(())
    }

    #[test]
    fn split() -> Res<()> {
        let queen = FacedUp(Card::new(Denomination::Queen, Suit::Spades));
        let ace1 = FacedUp(Card::new(Denomination::Ace, Suit::Hearts));
        let three = FacedUp(Card::new(Denomination::Numerical(3), Suit::Hearts));
        let three2 = FacedUp(Card::new(Denomination::Numerical(3), Suit::Diamonds));
        let nine = FacedUp(Card::new(Denomination::Numerical(9), Suit::Diamonds));

        let mut hand = Hand::new();
        hand.set_bet(100.into());
        hand.insert(three.clone());
        hand.insert(three2);
        assert_eq!(hand.hand_type(), HandType::Normal);

        let (mut hand1s, mut hand2s) = hand.split_hand()?;
        let hand1s = hand1s.insert(queen);
        let hand2s = hand2s.insert(ace1);
        assert_eq!(hand1s.hand_type(), HandType::Split);
        assert_eq!(hand2s.hand_type(), HandType::SplitSoft);

        let actions = hand1s.actions();
        assert_eq!(actions.len(), 3);
        assert!(actions.contains(&Hit));
        assert!(actions.contains(&Stand));
        assert!(actions.contains(&Double));

        let actions = hand2s.actions();
        assert_eq!(actions.len(), 3);
        assert!(actions.contains(&Hit));
        assert!(actions.contains(&Stand));
        assert!(actions.contains(&Double));

        assert_eq!(hand1s.score(), 13);
        assert_eq!(hand2s.score(), 14);

        let hand = hand1s.double()?.insert(three);
        assert_eq!(hand.hand_type(), HandType::Doubled);

        let hand = hand2s.double()?.insert(nine);
        assert_eq!(hand.hand_type(), HandType::Doubled);
        assert_eq!(hand.score(), 13);
        let actions = hand.actions();
        assert_eq!(actions.len(), 0);

        Ok(())
    }
}
