use crate::card::Visible::{FacedDown, FacedUp};
use crate::card::{Card, Denomination, Suit, Visible};
use crate::gui_classes::{GUICard, CARD_H, CARD_W};
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
        }
    }

    pub fn add_card(&mut self, card: &Visible<Card>) {
        let mut acard = GUICard::new(card.clone()).with_pos(
            self.group.x() + 50 + self.group.children() * (PADDING + CARD_W),
            self.group.y(),
        );

        acard.set_size(CARD_W, CARD_H);

        self.group.add(&acard.group);
    }
}
widget_extends!(GUIDealer, Group, group);
