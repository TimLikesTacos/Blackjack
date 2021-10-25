use crate::card::{Card, Visible};
use crate::deck::Deck;
use crate::player::{Player, Status};
use crate::{Message, Message::*, Res};
use fltk::app::Receiver;
use num::{Integer, Rational64, Zero};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::SyncSender;

#[derive(Debug)]
pub struct Table {
    pub(crate) dealer: Player,
    players: Vec<Rc<RefCell<Player>>>,
    deck: Deck,
    reshuffle: bool,
}

impl Table {
    pub fn new(num_players: usize, num_decks: usize) -> Res<Table> {
        let mut players = vec![];
        for n in 1..=num_players {
            let name = format!("Player {}", n);
            players.push(Rc::new(RefCell::new(Player::new(name))));
        }
        Ok(Table {
            dealer: Player::new("Dealer".to_string()),
            players,
            deck: Deck::new(num_decks)?,
            reshuffle: false,
        })
    }

    pub fn player(&self, player: usize) -> Option<&Rc<RefCell<Player>>> {
        self.players.get(player)
    }

    // pub fn player_mut(&mut self, player: usize) -> Option<&mut Player> {
    //     self.players.get_mut(player)
    // }

    // #[inline]
    // pub fn player_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = &'a Rc<RefCell<Player>>> + 'b
    // where
    //     'a: 'b,
    // {
    //     self.players.iter()
    // }

    /// Only used for initial card dealing at the beginning of round.
    pub fn deal_players(&mut self) {
        for player in self.players.iter() {
            // Each player is guaranteed to have one hand at beginning of play
            let player = Rc::clone(player);
            let mut hand = player.borrow_mut();
            let hand = hand
                .hand_iter_mut()
                .next()
                .expect("Every player should have one hand");
            hand.insert(self.deck.deal(true));
        }

        // Give the dealer one card faced up.
        self.dealer
            .hand_iter_mut()
            .next()
            .unwrap()
            .insert(self.deck.deal(true));

        // Give out second cards
        for player in self.players.iter() {
            // Each player is guaranteed to have one hand at beginning of play
            let player = Rc::clone(player);
            let mut hand = player.borrow_mut();
            let hand = hand
                .hand_iter_mut()
                .next()
                .expect("Every player should have one hand");
            hand.insert(self.deck.deal(true));
        }

        // Give dealer one card faced down
        self.dealer
            .hand_iter_mut()
            .next()
            .unwrap()
            .insert(self.deck.deal(false));
    }

    #[inline]
    pub fn player_iter<'a>(&'a self) -> impl Iterator<Item = Rc<RefCell<Player>>> + 'a {
        self.players.iter().map(|rc| Rc::clone(rc))
    }

    #[inline]
    pub fn deal_card(&mut self, facedup: bool) -> Visible<Card> {
        self.deck.deal(facedup)
    }

    #[inline]
    pub fn dealer_mut(&mut self) -> &mut Player {
        &mut self.dealer
    }
}
