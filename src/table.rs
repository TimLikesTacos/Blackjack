use crate::card::{Card, Denomination, Suit, Visible};
use crate::deck::Deck;
use crate::deck_traits::Shufflable;
use crate::player::Player;
use crate::Res;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

/// A struct where the players, dealer, and deck is located.
#[derive(Debug)]
pub struct Table {
    pub(crate) dealer: Player,
    players: Vec<Rc<RefCell<Player>>>,
    deck: Deck,
    num_of_decks: usize,
    pub(crate) reshuffle: bool,
}

impl Table {
    /// Creates a new table, with initialized dealer, players, and a shuffled deck.
    pub fn new(num_players: usize, num_decks: usize) -> Res<Table> {
        let mut players = vec![];
        for n in 1..=num_players {
            let name = format!("Player {}", n);
            players.push(Rc::new(RefCell::new(Player::new(name))));
        }
        let mut table = Table {
            dealer: Player::new("Dealer".to_string()),
            players,
            deck: Deck::new(num_decks)?,
            num_of_decks: num_decks,
            reshuffle: true,
        };
        table.shuffle();
        dbg!(&table.deck);
        Ok(table)
    }

    pub fn player(&self, player: usize) -> Option<&Rc<RefCell<Player>>> {
        self.players.get(player)
    }

    /// Only used for initial card dealing at the beginning of round.
    pub fn deal_players(&mut self) {
        //todo: This section has a lot of repetition when it comes to getting a new card.  This
        // was due to avoiding issues with the borrow checker. Find more efficient way.

        for player in self.players.iter() {
            // Each player is guaranteed to have one hand at beginning of play
            let player = Rc::clone(player);
            let mut hand = player.borrow_mut();
            let hand = hand
                .hand_iter_mut()
                .next()
                .expect("Every player should have one hand");

            let card = self.deck.deal(true);
            let card = match card.denom() {
                Denomination::Extra(s) if s == "shuffle" => {
                    self.reshuffle = true;
                    self.deck.deal(true)
                }
                _ => card,
            };

            hand.insert(card);
        }

        // Give the dealer one card faced up.
        let card = self.deck.deal(true);
        let card = match card.denom() {
            Denomination::Extra(s) if s == "shuffle" => {
                self.reshuffle = true;
                self.deck.deal(true)
            }
            _ => card,
        };
        self.dealer.hand_iter_mut().next().unwrap().insert(card);

        // Give out second cards
        for player in self.players.iter() {
            // Each player is guaranteed to have one hand at beginning of play
            let player = Rc::clone(player);
            let mut hand = player.borrow_mut();
            let hand = hand
                .hand_iter_mut()
                .next()
                .expect("Every player should have one hand");
            let card = self.deck.deal(true);
            let card = match card.denom() {
                Denomination::Extra(s) if s == "shuffle" => {
                    self.reshuffle = true;
                    self.deck.deal(true)
                }
                _ => card,
            };
            hand.insert(card);
        }

        // Give dealer one card faced down
        let card = self.deck.deal(false);
        let card = match card.denom() {
            Denomination::Extra(s) if s == "shuffle" => {
                self.reshuffle = true;
                self.deck.deal(false)
            }
            _ => card,
        };
        self.dealer.hand_iter_mut().next().unwrap().insert(card);
    }

    #[inline]
    pub fn player_iter<'a>(&'a self) -> impl Iterator<Item = Rc<RefCell<Player>>> + 'a {
        self.players.iter().map(|rc| Rc::clone(rc))
    }

    #[inline]
    pub fn deal_card(&mut self, facedup: bool) -> Visible<Card> {
        let card = self.deck.deal(facedup);
        match card.denom() {
            Denomination::Extra(s) if s == "shuffle" => {
                self.reshuffle = true;
                self.deck.deal(facedup)
            }
            _ => card,
        }
    }

    #[inline]
    pub fn dealer_mut(&mut self) -> &mut Player {
        &mut self.dealer
    }

    pub fn shuffle(&mut self) {
        let mut deck = Deck::new(self.num_of_decks).unwrap();

        // // todo put this back to push
        // deck.push(Card::new(Denomination::Extra("rules"), Suit::Pumpkin));
        // for _ in 0..2 {
        //     deck.push(Card::new(Denomination::Extra("joker"), Suit::Pumpkin));
        // }


        deck.shuffle();
        // Place the cut card 60-75 cards from the back. Not done for single deck
        if self.num_of_decks > 1 {
            let mut rng = rand::thread_rng();
            let position = rng.gen_range(60..=75);
            let cut_card = Card::new(Denomination::Extra("shuffle"), Suit::Clubs);
            let len = deck.len();
            deck.insert(len - position, cut_card);
            self.reshuffle = false;
        }


        self.deck = deck;
    }

    #[inline]
    pub fn decks(&self) -> usize {
        self.num_of_decks
    }
}
