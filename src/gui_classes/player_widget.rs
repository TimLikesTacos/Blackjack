use crate::gui_classes::PADDING;
use crate::hand::Hand;
use crate::player::Player;
use fltk::enums;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::group::{Column, Group, Row};
use fltk::prelude::*;
use fltk::widget_extends;
use std::cell::RefCell;
use std::rc::Rc;

const TEXT_H: i32 = 20;

#[derive(Debug)]
pub struct GUIPlayer {
    id: usize,
    pack: Group,
    pub(crate) title: Frame,
    pub(crate) points: Frame,
    pub(crate) bet: Frame,
    pub(crate) insurance: Frame,
    pub(crate) player: Option<Rc<RefCell<Player>>>,
}

impl GUIPlayer {
    pub fn new(id: usize, width: i32) -> GUIPlayer {
        let mut playervpac = Group::default().with_align(Align::Center | Align::Inside);

        playervpac.set_frame(FrameType::EmbossedFrame);
        playervpac.set_size(width, playervpac.h());

        let mut title = Frame::default()
            .with_pos(playervpac.x(), playervpac.y())
            .with_size(playervpac.w(), 50)
            .with_label("player")
            .with_align(Align::Inside | Align::Center);

        title.set_color(enums::Color::from_u32(0xff8566));
        title.set_frame(FrameType::PlasticDownBox);

        let text = Frame::default()
            .with_size(title.w() / 2, 10)
            //.below_of(&title, 5)
            .with_pos(title.x(), title.y() + 55)
            .with_label("Points: ")
            .with_align(enums::Align::Inside | enums::Align::Right);

        Frame::default()
            .with_size(text.w(), 10)
            .with_pos(text.x(), text.y() + TEXT_H)
            .with_label("Bet: ")
            .with_align(enums::Align::Inside | enums::Align::Right);

        Frame::default()
            .with_size(text.w(), 10)
            .with_pos(text.x(), text.y() + 2 * TEXT_H)
            .with_label("Insurance: ")
            .with_align(enums::Align::Inside | enums::Align::Right);

        // The values for the above entries
        let points = Frame::default()
            .with_size(text.w(), 10)
            .with_pos(text.x() + text.w(), text.y())
            .with_label("0")
            .with_align(enums::Align::Inside | enums::Align::Left);

        let bet = Frame::default()
            .with_size(points.w(), 10)
            .with_pos(points.x(), points.y() + TEXT_H)
            .with_label("0")
            .with_align(enums::Align::Inside | enums::Align::Left);

        let insurance = Frame::default()
            .with_size(points.w(), 10)
            .with_pos(points.x(), points.y() + 2 * TEXT_H)
            .with_label("0")
            .with_align(enums::Align::Inside | enums::Align::Left);

        playervpac.end();

        GUIPlayer {
            id,
            pack: playervpac,
            title,
            points,
            bet,
            insurance,
            player: None,
        }
    }

    pub fn activate_player(&mut self) {
        self.title.set_frame(FrameType::PlasticUpBox);
        self.title.redraw()
    }

    pub fn deactivate_player(&mut self) {
        self.title.set_frame(FrameType::PlasticDownBox);
        self.title.redraw()
    }

    // todo allow implementation of displaying the completed hands in the player section
    #[allow(dead_code)]
    pub fn add_hand(&mut self, hand: &Hand) {
        let thisgroup = Group::default()
            .with_size(self.pack.w(), TEXT_H)
            .with_pos(self.pack.x(), self.insurance.y() + 2 * PADDING);

        let row = Row::default()
            .with_label("")
            .with_size(self.pack.w(), TEXT_H)
            .with_pos(self.pack.x(), self.insurance.y() + 2 * PADDING)
            .with_align(Align::Center | Align::Inside);

        let col1 = Column::default()
            .with_label("Hand: ")
            .with_size(50, 30)
            .with_align(Align::Center | Align::Inside);
        col1.end();

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
        thisgroup.end();

        self.pack.add(&thisgroup);
    }

    pub fn set_points(&mut self, points: &str) {
        self.points.set_label(points);
        self.points.redraw()
    }

    pub fn set_bet(&mut self, bet: &str) {
        self.bet.set_label(bet);
        self.bet.redraw()
    }

    pub fn set_insurance(&mut self, insurance: &str) {
        self.insurance.set_label(insurance);
        self.insurance.redraw()
    }
}

widget_extends!(GUIPlayer, Group, pack);
