use fltk::button::Button;

mod card;
mod dealer;
mod header;
pub mod middle;
pub mod player_widget;
pub use fltk::prelude::*;

pub const BUTTON_H: i32 = 80;
pub const WIN_W: i32 = 1000;
pub const WIN_H: i32 = 800;
pub const BORDER: i32 = 20;
pub const PADDING: i32 = 10;

pub const CARD_H: i32 = 60;
pub const CARD_W: i32 = (CARD_H as f32 * CARD_RATIO) as i32;
const CARD_RATIO: f32 = 2.5 / 3.5;
pub const EIGHTH: i32 = WIN_H / 8;

use crate::blackjack::Table;
use crate::constants::TWENTYONE;
pub use crate::gui_classes::card::*;
pub use crate::gui_classes::dealer::GUIDealer;
pub use crate::gui_classes::header::GUIHeader;
use crate::gui_classes::middle::MiddleSection;
use crate::gui_classes::player_widget::GUIPlayer;
use crate::hand::{Action, Hand, HandType};
use crate::player::{Player, Status};
use crate::Message;
use fltk::enums;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::group::{Column, Row};
use num::{Rational64, ToPrimitive, Zero};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct GUIMain {
    pub(crate) header: GUIHeader,
    pub(crate) dealer: GUIDealer,
    pub(crate) message: Frame,
    pub(crate) middle: MiddleSection,
    pub(crate) players_gui: Vec<GUIPlayer>,
    pub(crate) table: Table,
    pub(crate) index: usize,
    pub(crate) active_players: Vec<Rc<RefCell<Player>>>,
    pub(crate) hand_num: usize,
    pub(crate) cont_func: fn(&mut GUIMain),
}

impl GUIMain {
    pub fn new(
        header: GUIHeader,
        dealer: GUIDealer,
        message: Frame,
        middle: MiddleSection,
        players_gui: Vec<GUIPlayer>,
        table: Table,
    ) -> GUIMain {
        GUIMain {
            header,
            dealer,
            message,
            middle,
            players_gui,
            table,
            index: 0,
            active_players: vec![],
            hand_num: 0,
            cont_func: GUIMain::setup_game,
        }
    }

    pub fn setup_game(&mut self) {
        // Clear the gui if needed (has been restarted)
        self.dealer.remove_cards();
        self.middle.remove_cards();
        self.middle.continue_button.hide();
        self.middle.hide_buttons();
        self.middle.insurance.hide();

        // assign players to gui
        for (p, gui) in self.table.player_iter().zip(self.players_gui.iter_mut()) {
            gui.title.set_label(&p.borrow().name());
            gui.points.set_label(&p.borrow().display_money());
            gui.bet.set_label("0");
            gui.insurance.set_label("0");
            gui.player = Some(p);
            gui.redraw();
        }
    }

    pub fn start_round(&mut self) {
        if let Some(_) = self.set_active_players_get_first() {
            if self.table.reshuffle {
                // Do not display shuffling message for single decks
                if self.table.decks() > 1 {
                    self.message.set_label("Shuffling the deck.");
                    self.middle.continue_button.show();
                    self.table.shuffle();
                    self.cont_func = Self::start_betting;
                    return;
                }

                self.table.shuffle()
            }
            self.start_betting()
        } else {
            self.game_over()
        }
    }

    fn start_betting(&mut self) {
        // Set up for betting
        let string = self
            .table
            .player(self.index)
            .unwrap()
            .borrow()
            .name()
            .to_string()
            + ": Place your bet. ";
        self.set_current();
        self.message.set_label(&string);
        self.message.redraw();
        self.middle.bet.show();
    }

    pub fn set_bet(&mut self, str: String) {
        if let Ok(bet) = str.parse() {
            let rc = Rc::clone(self.table.player(self.index).unwrap());

            // Update the player info for the bet. Returns Error if not enough money
            let result = rc
                .deref()
                .borrow_mut()
                .place_bet(Rational64::from_integer(bet));

            match result {
                Err(e) => {
                    self.message
                        .set_label(&format!("{}: {}", self.player_name(), e.to_string()));
                    return;
                }
                _ => (),
            }

            // Update GUI
            self.middle.bet.set_value("0");
            self.players_gui[self.index].bet.set_label(&bet.to_string());
            self.players_gui[self.index]
                .points
                .set_label(&rc.borrow().display_money());

            match self.next_player() {
                Some(_) => self.start_betting(),
                None => self.setup_playing(),
            }
        }
        // else return, and do not set the bet
    }

    /// Set up conditions for playing this round
    fn setup_playing(&mut self) {
        self.table.deal_players();
        self.middle.bet.hide();
        self.dealer.frame.show();

        self.first_player();

        for dcard in self.table.dealer.hand_iter().next().unwrap().card_iter() {
            self.dealer.add_card(dcard);
        }

        for player in self.players_gui.iter_mut() {
            player.redraw();
        }
        self.check_dealer();
    }

    fn check_dealer(&mut self) {
        let dealer_hand = self.table.dealer.get_hand(0).unwrap().clone();
        // Offer insurance if card is an ace
        if dealer_hand.is_first_card_ace() {
            self.middle.hide_buttons();
            self.offer_insurance()
        } else {
            self.peek_for_natural()
        }
    }

    fn peek_for_natural(&mut self) {
        let dealer_hand = self.table.dealer.get_hand(0).unwrap().clone();
        if dealer_hand.peek_for_natural() {
            // Round is over if the dealer has a natural
            self.table.dealer.get_hand_mut(0).unwrap().flip_over();
            self.dealer.flip_over(&dealer_hand);
            self.settle_setup();
        } else {
            self.start_playing();
        }
    }

    fn offer_insurance(&mut self) {
        self.set_current();
        let player = self.active_players[self.index].borrow();
        self.message
            .set_label(&format!("{}, place insurance bet", player.name()));
        self.middle.insurance.show();
    }

    pub fn set_insurance(&mut self, bet_str: String) {
        if let Ok(bet) = bet_str.parse::<f64>() {
            let rc = Rc::clone(&self.active_players[self.index]);

            // Truncate the float at 2 decimal points and convert to rational
            let bet = (bet * 100.0) as i64;
            let bet = Rational64::from((bet, 100));

            // Update the player info for the bet. Returns Error if not enough money
            let result = rc.deref().borrow_mut().set_insurance(bet);

            match result {
                Err(e) => {
                    self.message
                        .set_label(&format!("{}: {}", self.player_name(), e.to_string()));
                    return;
                }
                _ => (),
            }

            // Update GUI
            self.middle.insurance.set_value("0");
            self.players_gui[self.index]
                .insurance
                .set_label(&format!("{:.2}", &bet.to_f64().unwrap_or(0.)));
            self.players_gui[self.index]
                .points
                .set_label(&rc.borrow().display_money());

            match self.next_player() {
                Some(_) => self.offer_insurance(),
                None => {
                    self.middle.insurance.hide();
                    self.first_player();
                    self.peek_for_natural()
                }
            }
        }
    }
    /// This is where each player begins play
    fn start_playing(&mut self) {
        self.set_current();
        self.middle.remove_cards();
        let player = Rc::clone(self.active_players.get(self.index).unwrap());
        let player = player.borrow();
        let str: String = format!("{}'s turn to play", player.name());
        self.message.set_label(&str);

        if let Some(hand) = player.get_hand(self.hand_num) {
            for card in hand.card_iter() {
                self.middle.add_card(card);
            }

            // Check for natural
            match hand.hand_type() {
                HandType::Natural => {
                    self.message.set_label("Blackjack!");
                    self.middle.continue_button.show();
                    return self.inc_turn();
                }
                _ => (),
            }
            let actions = player.actions(self.hand_num);
            self.middle.show_buttons(&actions);

            // This section can be reached after splitting aces
            if actions.len() == 0 {
                self.middle.continue_button.show();
                self.message.set_label(&format!(
                    "{} no further actions available for this hand",
                    player.name()
                ));
                return self.inc_turn();
            }
        }
    }

    /// Increment's the struct's counters for either hand index or current player index. Priority placed
    /// on the current players hands (in case of split) over the next player. Only for use in during the playing phase
    pub fn inc_turn(&mut self) {
        // Increment next available hand to the currently player first
        if self.active_players[self.index].borrow().num_hands() > self.hand_num + 1 {
            self.hand_num += 1;
            self.cont_func = Self::start_playing;
            return;
        }
        // If no extra hand avaiable, increment the next player
        self.hand_num = 0;
        if let Some(_) = self.next_player() {
            self.cont_func = Self::start_playing;
        } else {
            self.cont_func = Self::dealer_play;
        }
    }

    /// Used as a pause button.
    pub fn continue_play(&mut self) {
        self.middle.continue_button.hide();
        (self.cont_func)(self)
    }

    fn current_hand(&mut self) -> Option<Hand> {
        if let Some(hand) = self.active_players[self.index]
            .borrow()
            .get_hand(self.hand_num)
        {
            Some(hand.clone())
        } else {
            None
        }
    }

    pub fn perform_action(&mut self, action: Action) {
        match action {
            Action::Hit => self.hit(),
            Action::Stand => self.stand(),
            Action::Split => self.split(),
            Action::Double => self.double(),
        }
    }

    fn hit(&mut self) {
        {
            let mut player = Rc::clone(&self.active_players[self.index]);
            let mut hand = player.borrow_mut();
            let hand = hand.get_hand_mut(self.hand_num).unwrap();
            // deal a card
            let card = self.table.deal_card(true);
            // update gui
            self.middle.add_card(&card);
            // add to players hand
            hand.insert(card);
        }

        let player = Rc::clone(&self.active_players[self.index]);
        let actions = player.borrow().actions(self.hand_num);
        self.middle.show_buttons(&actions);
        if actions.is_empty() {
            self.middle.continue_button.show();
            // Play the split hand, if avail
            if player.borrow().num_hands() > self.hand_num + 1 {
                self.hand_num += 1;
                self.cont_func = Self::start_playing;
                self.message.set_label("Playing split hand");
                return;
            }

            if player.borrow().get_hand(self.hand_num).unwrap().hand_type() == HandType::Bust {
                self.message.set_label("Bust");
            } else {
                self.message.set_label(&format!(
                    "All available actions are complete for {}",
                    player.borrow().name()
                ));
            }
            // Move on to the next player
            self.inc_turn();
        }
    }

    fn stand(&mut self) {
        let mut player = Rc::clone(&self.active_players[self.index]);
        self.message
            .set_label(&format!("{} stands.", player.borrow().name()));
        self.middle.continue_button.show();
        self.middle.hide_buttons();

        self.inc_turn();
    }

    fn split(&mut self) {
        let mut player = Rc::clone(&self.active_players[self.index]);
        self.message
            .set_label(&format!("{} splits", player.borrow().name()));
        self.middle.hide_buttons();
        self.middle.continue_button.show();

        let card1 = self.table.deal_card(true);
        let card2 = self.table.deal_card(true);
        player.borrow_mut().split_hand(self.hand_num, card1, card2);

        self.players_gui[self.index]
            .points
            .set_label(&player.borrow().display_money());

        self.cont_func = Self::start_playing;
    }

    fn double(&mut self) {
        {
            let mut player = Rc::clone(&self.active_players[self.index]);
            let bet = player
                .borrow()
                .get_hand(self.hand_num)
                .unwrap()
                .bet()
                .unwrap_or(Rational64::zero());

            let mut player = player.borrow_mut();
            let avail_money = player.money();

            let mut hand = player.get_hand_mut(self.hand_num).unwrap();

            let double_bet = min(avail_money, bet);
            if double_bet < bet {
                self.middle.continue_button.show();
                self.middle.hide_buttons();
                self.message.set_label("Doubling for less");
            }

            let hand = hand.double().unwrap();
            let card = self.table.deal_card(true);
            let hand = hand.insert(card);

            player.replace_hand(self.hand_num, hand);

            player.double(double_bet);
            self.middle.add_card(&card);
            self.players_gui[self.index].set_bet(
                &player
                    .get_hand(self.hand_num)
                    .unwrap()
                    .bet()
                    .unwrap_or(Rational64::zero())
                    .to_string(),
            );
            self.players_gui[self.index].set_points(&player.display_money());
        }

        if self.is_bust() {
            self.message.set_label("BUST");
        } else {
            self.message
                .set_label(&format!("No more actions for {}.", self.player_name()));
        }

        self.middle.continue_button.show();
        self.middle.hide_buttons();

        self.inc_turn();
    }

    fn dealer_play(&mut self) {
        self.message.set_label("Dealer's turn");
        self.players_gui[self.index].deactivate_player();
        self.middle.remove_cards();

        let hand = self.table.dealer_mut().get_hand_mut(0).unwrap();
        hand.flip_over();
        self.dealer.flip_over(&hand);

        while self.table.dealer.score(0) < 17 {
            let card = self.table.deal_card(true);
            self.dealer.add_card(&card);
            self.table.dealer.get_hand_mut(0).unwrap().insert(card);
        }

        self.middle.continue_button.show();
        self.cont_func = Self::settle_setup;
    }

    fn settle_setup(&mut self) {
        self.first_player();
        self.hand_num = 0;

        self.settle()
    }

    fn settle(&mut self) {
        {
            self.middle.continue_button.show();

            let mut dealer_hand = self.table.dealer.get_hand(0).unwrap();
            // dealer_hand.flip_over();
            //
            // self.dealer.flip_over(dealer_hand);

            let dealer_handtype = dealer_hand.hand_type();
            match dealer_handtype {
                HandType::Natural => self.settle_with_dealer_natrual(),
                HandType::Bust => self.settle_with_dealer_bust(),
                _ => self.settle_with_dealer_other(),
            }
        }
        self.cont_func = Self::clean_after_settle;
    }

    /// used when the dealer has a natural blackjack.  Everyone loses except for anyone with a natural blackjack.  Insurance is paid out.
    fn settle_with_dealer_natrual(&mut self) {
        let playerrc = Rc::clone(&self.active_players[self.index]);
        let hand = playerrc.borrow().get_hand(self.hand_num).unwrap().clone();
        let mut player = playerrc.borrow_mut();
        self.middle.add_cards(&hand);

        let ins_payout = player.insurance() * Rational64::from_integer(2);

        match hand.hand_type() {
            HandType::Natural => {
                let string = match ins_payout > Rational64::zero() {
                    true => format!(
                        "Tie. Dealer and {} have Blackjacks. Insurance pays {:.2}",
                        player.name(),
                        ins_payout
                    ),
                    false => format!("Tie. Dealer and {} have Blackjacks", player.name()),
                };

                self.message.set_label(&string);
                player.collect(hand.bet().unwrap() + ins_payout);
            }

            _ => {
                let string = match ins_payout > Rational64::zero() {
                    true => format!("Dealer has a blackjack. Insurance pays {:.2}", ins_payout),
                    false => format!("Dealer has a blackjack"),
                };
                self.message.set_label(&string);
                player.collect(ins_payout);
            }
        }
    }

    /// Used if the dealer busted during play.  Everyone wins unless they busted. Natural blackjacks are paid out 1.5 times.
    fn settle_with_dealer_bust(&mut self) {
        let playerrc = Rc::clone(&self.active_players[self.index]);
        let hand = playerrc.borrow().get_hand(self.hand_num).unwrap().clone();
        let mut player = playerrc.borrow_mut();
        self.middle.add_cards(&hand);

        match hand.hand_type() {
            HandType::Bust => {
                // Player still loses due to busting first
                self.message.set_label(&format!("{} Busts", player.name()));
            }

            HandType::Natural => {
                self.message.set_label(&format!("Blackjack!"));
                player.collect(hand.bet().unwrap() * Rational64::from((5, 2)));
            }

            _ => {
                self.message.set_label(&format!(
                    "{} won! Player score: {}, Dealer Bust",
                    player.name(),
                    hand.score(),
                ));
                player.collect(hand.bet().unwrap() * 2);
            }
        }
    }

    /// Used for all other situations where the dealer has not had a natural blackjack or has busted.
    fn settle_with_dealer_other(&mut self) {
        let playerrc = Rc::clone(&self.active_players[self.index]);
        let hand = playerrc.borrow().get_hand(self.hand_num).unwrap().clone();
        let mut player = playerrc.borrow_mut();
        self.middle.add_cards(&hand);

        let dealer_score = self.table.dealer.score(0);
        let player_score = player.score(self.hand_num);

        match hand.hand_type() {
            HandType::Bust => {
                // Player still loses due to busting first
                self.message.set_label(&format!("{} Busts", player.name()));
            }

            HandType::Natural => {
                self.message.set_label(&format!("Blackjack!"));
                player.collect(hand.bet().unwrap() * Rational64::from((5, 2)));
            }

            _ => {
                // If player won
                if player_score > dealer_score {
                    self.message.set_label(&format!(
                        "{} won! Player score: {}, Dealer score {}",
                        player.name(),
                        player_score,
                        dealer_score
                    ));
                    player.collect(hand.bet().unwrap() * 2);
                }
                // If player tied with dealer
                else if player_score == dealer_score {
                    self.message.set_label(&format!(
                        "Tie! Player score: {}, Dealer score {}",
                        player_score, dealer_score
                    ));
                    player.collect(hand.bet().unwrap());
                }
                // If player lost
                else {
                    self.message.set_label(&format!(
                        "Sorry, {} lost. Player score: {}, Dealer score {}",
                        player.name(),
                        player_score,
                        dealer_score
                    ));
                }
            }
        }
    }
    fn clean_after_settle(&mut self) {
        self.players_gui[self.index].set_bet("0");
        self.players_gui[self.index].set_insurance("0");

        {
            let mut player = self.active_players[self.index].borrow_mut();
            player.reset_after_round(self.hand_num);
            self.players_gui[self.index].set_points(&player.display_money());
        }

        if let Some(_) = self.next_hand() {
            self.settle();
        } else {
            self.hand_num = 0;
            if let Some(_) = self.next_player() {
                self.settle()
            } else {
                // Start next round
                self.dealer.remove_cards();
                self.middle.remove_cards();
                self.table.dealer.reset_after_round(0);

                self.start_round();
            }
        }
    }

    pub fn set_current(&mut self) {
        for mut player in self.players_gui.iter_mut() {
            player.deactivate_player();
        }
        let mut player = &mut self.players_gui[self.index];
        player.activate_player();
    }

    /// Sets the active players and returns the first person.  Used at beginning of round.
    fn set_active_players_get_first(&mut self) -> Option<()> {
        self.active_players = self.table.player_iter().collect();
        self.index = 0;
        if let Some(player) = self.active_players.get(0) {
            if player.borrow().status() == Status::Playing {
                return Some(());
            }
        }
        self.next_player()
    }

    /// Gets the first player. Used during play / insurance / settlement as any player can be actively playing, but with no extra money.
    fn first_player(&mut self) -> Option<()> {
        self.index = 0;
        if let Some(player) = self.active_players.get(0) {
            if player.borrow().status() == Status::Playing {
                return Some(());
            }
        }
        self.next_player()
    }

    fn next_player(&mut self) -> Option<()> {
        if let Some((i, _)) = self
            .table
            .player_iter()
            .enumerate()
            .skip(self.index + 1)
            .find(|(_, p)| p.borrow().status() == Status::Playing)
        {
            self.index = i;
            Some(())
        } else {
            None
        }
    }

    fn next_hand(&mut self) -> Option<()> {
        let player = Rc::clone(&self.active_players[self.index]);
        if player.borrow().num_hands() > self.hand_num + 1 {
            self.hand_num += 1;
            Some(())
        } else {
            None
        }
    }

    fn player_available(&self, player: usize) -> bool {
        self.table.player(player).unwrap().borrow().money() > Rational64::zero()
    }

    fn is_bust(&self) -> bool {
        let player = Rc::clone(&self.active_players[self.index]);
        let handtype = player.borrow().get_hand(self.hand_num).unwrap().hand_type();
        handtype == HandType::Bust
    }

    fn player_name(&self) -> String {
        self.table
            .player(self.index)
            .unwrap()
            .clone()
            .borrow()
            .name()
            .clone()
    }

    fn game_over(&mut self) {
        self.middle.hide_buttons();
        self.message
            .set_label("Game over!  All players are out of money.");
    }

    fn player_hand_below(&mut self) {
        let thegui = &self.players_gui[self.index];
        for (num, hand) in self
            .table
            .player(self.index)
            .unwrap()
            .borrow()
            .hand_iter()
            .enumerate()
        {
            let mut row = Row::default()
                .with_label("")
                .with_size(thegui.w(), 20)
                .with_pos(thegui.x(), thegui.insurance.y() + 2 * PADDING)
                .with_align(Align::Center | Align::Inside);

            // row.set_frame(FrameType::RoundedBox);

            let mut col1 = Column::default()
                .with_label("Hand: ")
                .with_size(50, 30)
                .with_align(Align::Center | Align::Inside);
            col1.end();

            // col1.set_frame(FrameType::EngravedFrame);

            let mut col2 = Column::default().size_of_parent();
            for card in hand.card_iter() {
                let card_row = Row::default()
                    .with_pos(0, 10)
                    .with_label(&card.to_string())
                    .with_align(enums::Align::Inside | enums::Align::Center);
                card_row.end()
            }
            col2.set_frame(FrameType::EngravedFrame);
            col2.end();
            row.end();
            //self.players_gui[self.current_player].add(&row);
        }
    }
}
