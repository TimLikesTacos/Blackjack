use crate::deck::Deck;
use crate::player::Player;
use crate::Res;
use num::Integer;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Table {
    dealer: Player,
    players: Vec<Rc<RefCell<Player>>>,
    deck: Deck,
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
        })
    }

    pub fn player(&self, player: usize) -> Option<Rc<RefCell<Player>>> {
        if let Some(pl) = self.players.get(player) {
            Some(Rc::clone(pl))
        } else {
            None
        }
    }

    #[inline]
    pub fn player_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = &'a Rc<RefCell<Player>>> + 'b {
        self.players.iter()
    }
}
