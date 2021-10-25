use crate::card::{Card, Visible};
use crate::gui_classes::{CARD_H, CARD_W};
use fltk::enums::{Align, Color, FrameType};
use fltk::frame::*;
use fltk::group::Group;
use fltk::prelude::*;
use fltk::widget_extends;

#[allow(dead_code)]
pub struct GUICard {
    pub(crate) group: Group,
    card: Visible<Card>,
}

impl GUICard {
    pub fn new(card: Visible<Card>) -> GUICard {
        let mut group = Group::default().with_size(CARD_W, CARD_H);
        let (facedup, tcard) = match card {
            Visible::FacedUp(c) => (true, c),
            Visible::FacedDown(c) => (false, c),
        };

        let mut suit = Frame::default()
            .with_size(CARD_W, CARD_H)
            .with_align(Align::Inside | Align::Center);
        let mut den = Frame::default()
            .with_size(CARD_W, CARD_H)
            .with_align(Align::TopLeft | Align::Inside);
        let mut denb = Frame::default()
            .with_size(CARD_W, CARD_H)
            .with_align(Align::BottomRight | Align::Inside);

        if facedup {
            suit.set_label(&tcard.suit().to_string());
            den.set_label(&tcard.denom().to_string());
            denb.set_label(&tcard.denom().to_string());
            group.set_color(Color::White);
        } else {
            group.set_color(Color::Blue);
        }

        group.set_frame(FrameType::BorderBox);

        group.end();

        GUICard { group, card }
    }
}

widget_extends!(GUICard, Group, group);
