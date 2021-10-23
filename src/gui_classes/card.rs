use crate::card::{Card, Denomination, Suit, Visible};
use crate::gui_classes::{CARD_H, CARD_W};
use fltk::button::ButtonType;
use fltk::enums::FrameType::{FreeBoxType, RoundedBox, RoundedFrame, UpBox};
use fltk::enums::{Align, Color, FrameType};
use fltk::frame::*;
use fltk::group::Group;
use fltk::input::InputType;
use fltk::prelude::*;
use fltk::widget::Widget;
use fltk::{image, widget_extends};
use std::ops::{Deref, DerefMut};

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

    // pub fn right_of<W: WidgetExt>(mut self, w: &'a GUICard, padding: i32) -> GUICard<'a> {
    //     let newgroup = self.group.right_of(&w.group, padding);
    //     GUICard {
    //         group: newgroup,
    //         card: self.card,
    //     }
    // }
}

// impl Deref for GUICard<'_> {
//     type Target = Group;
//
//     fn deref(&self) -> &Self::Target {
//         &self.group
//     }
// }
//
// impl DerefMut for GUICard<'_> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.group
//     }
// }

widget_extends!(GUICard, Group, group);
