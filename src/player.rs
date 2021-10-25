use crate::card::{Card, Visible};
use crate::errors::BlJaError;
use crate::hand::{Action, Hand};
use crate::Res;
use num::rational::Ratio;
use num::{One, Rational64, ToPrimitive, Zero};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Playing,
    Out,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    hands: Vec<Hand>, // a player can have multiple hands following a split
    money: Rational64,
    insurance: Rational64,
    status: Status,
}

impl Player {
    /// Create a new player
    pub fn new(name: String) -> Player {
        Player {
            name,
            hands: vec![Hand::new()],
            money: Rational64::from_integer(500),
            insurance: Rational64::zero(),
            status: Status::Playing,
        }
    }

    pub fn place_bet(&mut self, bet: Rational64) -> Res<()> {
        if self.hands.len() > 1
            || self
                .hands
                .get(0)
                .ok_or_else(|| "Player hand not initialized")?
                .num_cards()
                > 0
        {
            return Err(Box::new(BlJaError::ImproperAction(
                "Trying to place bet on a hand with cards or after split",
            )));
        }
        let newbalance = self.money - bet;
        if newbalance < Rational64::zero() {
            return Err(Box::new(BlJaError::NotEnoughMoney));
        }
        self.hands[0].set_bet(bet.into());
        self.money = newbalance;

        Ok(())
    }

    /// Sets insurance
    pub fn set_insurance(&mut self, insurance_bet: Rational64) -> Res<()> {
        let newbalance = self.money - insurance_bet;
        if newbalance < Rational64::zero() {
            return Err(Box::new(BlJaError::NotEnoughMoney));
        }
        let bet = self
            .hands
            .get(0)
            .ok_or_else(|| "Player hand not initialized")?
            .bet()
            .ok_or_else(|| "Bet is not set")?;
        if insurance_bet * 2 as i64 > bet {
            return Err(Box::new(BlJaError::ExcessiveInsurance));
        }

        self.insurance = insurance_bet;
        self.money = newbalance;
        Ok(())
    }

    /// Performs actions of splitting the hand, and adding cards to the hands in order.
    pub fn split_hand(
        &mut self,
        hand_num: usize,
        newcard1: Visible<Card>,
        newcard2: Visible<Card>,
    ) {
        let (mut new1, mut new2) = self.hands.remove(hand_num).split_hand().unwrap();
        let new1 = new1.insert(newcard1);
        let new2 = new2.insert(newcard2);

        let bet = new2.bet().unwrap();

        self.hands.insert(hand_num, new2);
        self.hands.insert(hand_num, new1);

        self.money -= bet;
    }

    /// Helper function to check if the player has enough money to split or double.
    fn can_split_or_double(&self, hand: &Hand) -> bool {
        let bet = hand.bet().unwrap();
        bet <= self.money
    }

    /// Get the actions available for the applicable hand.  This makes sure that the player has enough money
    /// to support splitting or doubling.
    pub fn actions(&self, hand: usize) -> HashSet<Action> {
        let thehand = &self.hands[hand];
        let mut actions = thehand.actions();
        if !self.can_split_or_double(thehand) {
            actions.remove(&Action::Double);
            actions.remove(&Action::Split);
        }
        actions
    }

    #[inline]
    pub fn insurance(&self) -> Rational64 {
        self.insurance
    }

    #[inline]
    pub fn collect(&mut self, amount: Rational64) {
        self.money += amount
    }

    #[inline]
    pub fn hand_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = &'a Hand> + 'b {
        self.hands.iter()
    }

    #[inline]
    pub fn hand_iter_mut<'b, 'a: 'b>(&'a mut self) -> impl Iterator<Item = &'a mut Hand> + 'b {
        self.hands.iter_mut()
    }

    #[inline]
    pub fn get_hand(&self, index: usize) -> Option<&Hand> {
        self.hands.get(index)
    }

    #[inline]
    pub fn num_hands(&self) -> usize {
        self.hands.len()
    }

    #[inline]
    pub fn get_hand_mut(&mut self, index: usize) -> Option<&mut Hand> {
        self.hands.get_mut(index)
    }

    #[inline]
    pub fn money(&self) -> Rational64 {
        self.money
    }

    #[inline]
    pub fn display_money(&self) -> String {
        // Include rounding to nearest hundreds place.
        let float = self.money.to_f64().unwrap_or(0.0);
        format!("{:.2}", float)
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn status(&self) -> Status {
        self.status
    }

    #[inline]
    pub fn set_if_out(&mut self) {
        if self.money <= Rational64::one() {
            self.status = Status::Out;
        }
    }

    #[inline]
    pub fn score(&self, hand: usize) -> usize {
        self.hands[hand].score()
    }

    #[inline]
    pub fn double(&mut self, bet: Rational64) {
        self.money -= bet;
    }

    #[inline]
    pub fn replace_hand(&mut self, index: usize, newhand: Hand) {
        self.hands.remove(index);
        self.hands.insert(index, newhand);
    }

    /// Clears the players hand, sets the player to Out status if does not have any playable amount of money.
    pub fn reset_after_round(&mut self, hand: usize) {
        if self.hands.len() - 1 > hand {
            return; // Do not reset, still have hands to check
        }
        self.hands = vec![Hand::new()];
        self.insurance = Rational64::zero();
        if self.money() < Rational64::one() {
            self.status = Status::Out;
        }
    }
}

#[cfg(test)]
mod playertests {
    use super::*;
    use num::ToPrimitive;
    use std::any::Any;

    fn player() -> Player {
        Player::new("p1".to_string())
    }

    #[test]
    fn placebet() -> Res<()> {
        let mut aplayer = player();
        aplayer.place_bet(200.into())?;
        assert_eq!(aplayer.money, 300.into());
        assert_eq!(aplayer.hands[0].bet().unwrap(), 200.into());

        let mut aplayer = player();
        assert!(aplayer.place_bet(600.into()).is_err());
        Ok(())
    }

    #[test]
    fn insurance() -> Res<()> {
        // Max
        let mut aplayer = player();
        aplayer.place_bet(200.into())?;
        aplayer.set_insurance(100.into())?;
        assert_eq!(aplayer.insurance(), 100.into());

        // One
        let mut aplayer = player();
        aplayer.place_bet(200.into())?;
        aplayer.set_insurance(1.into())?;
        assert_eq!(aplayer.insurance(), 1.into());

        // Zero
        let mut aplayer = player();
        aplayer.place_bet(200.into())?;
        aplayer.set_insurance(0.into())?;
        assert_eq!(aplayer.insurance(), 0.into());

        // Errors - Excessive insurance bet
        let mut aplayer = player();
        aplayer.place_bet(200.into())?;
        let res = aplayer.set_insurance(Rational64::from((10001, 100))); // $100.01
        match res {
            Err(e) if e.is::<BlJaError>() => assert!(true),
            _ => assert!(false),
        }
        let res = aplayer.set_insurance(Rational64::from((9999, 100))); // $99.99
        assert!(res.is_ok());
        assert!((aplayer.insurance().to_f64().unwrap() - 99.99f64).abs() < 1e-10);

        // Errors - Not enough money
        let mut aplayer = player();
        aplayer.place_bet(400.into())?;
        let res = aplayer.set_insurance(200.into());
        match res {
            Err(e) if e.is::<BlJaError>() => assert!(true),
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn collect() -> Res<()> {
        let mut aplayer = player();
        assert_eq!(aplayer.money(), 500.into());
        aplayer.collect(300.into());
        assert_eq!(aplayer.money(), 800.into());

        Ok(())
    }
}
