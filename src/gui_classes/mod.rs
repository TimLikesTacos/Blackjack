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
pub use crate::gui_classes::card::*;
pub use crate::gui_classes::dealer::GUIDealer;
pub use crate::gui_classes::header::GUIHeader;
use crate::gui_classes::middle::MiddleSection;
use crate::gui_classes::player_widget::GUIPlayer;
use crate::player::Player;
use crate::Message;
use fltk::enums;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::group::{Column, Row};
use num::{Rational64, Zero};
use std::borrow::BorrowMut;
use std::cell::RefCell;
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
    pub(crate) current: Option<Rc<RefCell<Player>>>,
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
            current: None,
        }
    }

    pub fn setup_game(&mut self) {
        // assign players to gui
        for (player, gui) in self.table.player_iter().zip(self.players_gui.iter_mut()) {
            gui.player = Some(Rc::clone(player));
            gui.title.set_label(player.borrow().name());
            gui.points.set_label(&player.borrow().display_money())
        }
    }
    pub fn start_round(&mut self) {
        self.current = self.table.active_iter().next();

        if self.current.is_none() {
            return self.game_over(); // No players have any money
        }

        self.start_betting()
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

            match self.next_player_betting() {
                Some(_) => self.start_betting(),
                None => self.start_playing(),
            }
        }
        // else return, and do not set the bet
    }

    fn start_playing(&mut self) {
        self.table.deal_players();
        self.middle.bet.hide();
        self.dealer.frame.show();
        for dcard in self.table.dealer.hand_iter().next().unwrap().card_iter() {
            self.dealer.add_card(dcard);
        }

        for player in self.players_gui.iter_mut() {
            player.redraw();
        }

        self.index = 0;
    }
    pub fn set_current(&mut self) {
        for mut player in self.players_gui.iter_mut() {
            player.deactivate_player();
        }
        let mut player = &mut self.players_gui[self.index];
        player.activate_player();

        //
    }

    fn next_player_betting(&mut self) -> Option<()> {
        let players = self.players_gui.len();

        while self.index < players - 1 {
            self.index += 1;
            if self.player_available(self.index) {
                return Some(());
            }
        }
        None
    }

    fn player_available(&self, player: usize) -> bool {
        self.table.player(player).unwrap().borrow().money() > Rational64::zero()
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
        unimplemented!();
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
