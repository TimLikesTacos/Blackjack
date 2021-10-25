use crate::card::Visible::{FacedDown, FacedUp};
use crate::card::{Card, Denomination, Suit, Visible};
use crate::gui_classes::{GUICard, CARD_H, CARD_W};
use crate::hand::Hand;
use crate::player::Player;
use crate::PADDING;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::group::Column;
use fltk::group::Group;
use fltk::group::Row;
use fltk::prelude::*;
use fltk::widget::Widget;
use fltk::widget_extends;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GUIDealer {
    group: Group,
    pub(crate) frame: Frame,
    num_cards: i32,
}

impl GUIDealer {
    pub fn new() -> GUIDealer {
        let mut group = Group::default();
        let mut dealer = Frame::default()
            .with_size(CARD_H, CARD_H)
            .with_label("DEALER:")
            .with_align(Align::Inside | Align::Left | Align::Center);
        group.add(&dealer);

        dealer.hide();
        group.end();
        group.set_frame(FrameType::BorderBox);
        GUIDealer {
            group,
            frame: dealer,
            num_cards: 0,
        }
    }

    pub fn add_card(&mut self, card: &Visible<Card>) {
        let mut acard = GUICard::new(card.clone()).with_pos(
            self.group.x() + 50 + self.group.children() * (PADDING + CARD_W),
            self.group.y(),
        );

        acard.set_size(CARD_W, CARD_H);
        self.num_cards += 1;

        self.group.add(&acard.group);
    }

    pub fn flip_over(&mut self, hand: &Hand) {
        self.remove_cards();
        for card in hand.card_iter() {
            self.add_card(&card.flip_up());
        }
    }

    pub fn remove_cards(&mut self) {
        let mut last = self.group.children() - 1;
        let mut first = last - self.num_cards;
        // the cards are the last widgets, remove all cards
        // remove from the last to the first
        while last > first {
            self.group.remove_by_index(last);
            last -= 1;
        }
        self.num_cards = 0;
        self.group.hide();
        self.group.show();
    }
}
widget_extends!(GUIDealer, Group, group);
