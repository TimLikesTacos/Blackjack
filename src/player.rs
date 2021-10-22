use crate::errors::BlJaError;
use crate::hand::Hand;
use crate::Res;
use num::rational::Ratio;
use num::{Rational64, Zero};

#[derive(Debug)]
pub struct Player {
    name: String,
    hands: Vec<Hand>, // a player can have multiple hands following a split
    money: Rational64,
    insurance: Rational64,
}

impl Player {
    /// Create a new player
    pub fn new(name: String) -> Player {
        Player {
            name,
            hands: vec![Hand::new()],
            money: Rational64::from_integer(500),
            insurance: Rational64::zero(),
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
        Ok(())
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
    pub fn money(&self) -> Rational64 {
        self.money
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
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
