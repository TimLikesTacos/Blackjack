use crate::gui_classes::{PADDING, WIN_W};
use crate::hand::Hand;
use crate::player::Player;
use fltk::button::Button;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::group::{Column, Group, Row};
use fltk::prelude::*;
use fltk::widget_extends;
use fltk::{enums, group};
use num::{Rational64, Zero};
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
        // let player = table.player(pnum - 1).unwrap();
        let mut playervpac = Group::default().with_align(Align::Center | Align::Inside);
        // let playertitle = Row::default_fill().size_of_parent();
        playervpac.set_frame(FrameType::EmbossedFrame);
        playervpac.set_size(width, playervpac.h());

        let mut title = Frame::default()
            .with_pos(playervpac.x(), playervpac.y())
            .with_size(playervpac.w(), 50)
            .with_label("player")
            .with_align(Align::Inside | Align::Center);

        title.set_color(enums::Color::from_u32(0x7FFFD4));
        title.set_frame(FrameType::PlasticDownBox);

        // playertitle.end();

        // let row1 = Row::default()
        //     .with_label("row1")
        //     .with_pos(title.x(), title.y() + title.h() + PADDING);
        // let mut col1 = Column::default()
        //     .with_label("col1")
        //     .with_size(row1.w() / 2, 30);
        // col1.set_pad(5);

        //let row2 = Row::default_fill().with_label("row2");
        let mut text = Frame::default()
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

        // col1.end();
        // let col2 = Column::default()
        //     .with_label("col2")
        //     .with_size(row1.w() / 2, 30);

        // The values for the above entries
        let mut points = Frame::default()
            .with_size(text.w(), 10)
            .with_pos(text.x() + text.w(), text.y())
            .with_label("0")
            .with_align(enums::Align::Inside | enums::Align::Left);

        let mut bet = Frame::default()
            .with_size(points.w(), 10)
            .with_pos(points.x(), points.y() + TEXT_H)
            .with_label("0")
            .with_align(enums::Align::Inside | enums::Align::Left);

        let mut insurance = Frame::default()
            .with_size(points.w(), 10)
            .with_pos(points.x(), points.y() + 2 * TEXT_H)
            .with_label("0")
            .with_align(enums::Align::Inside | enums::Align::Left);

        // col2.end();
        // row1.end();

        // for (num, hand) in player.borrow().hand_iter().enumerate() {
        //     let mut row = Row::default()
        //         .with_label("")
        //         .with_size(playervpac.w(), TEXT_H)
        //         .with_pos(playervpac.x(), insurance.y() + 2 * PADDING)
        //         .with_align(Align::Center | Align::Inside);
        //
        //     // row.set_frame(FrameType::RoundedBox);
        //
        //     let mut col1 = Column::default()
        //         .with_label("Hand: ")
        //         .with_size(50, 30)
        //         .with_align(Align::Center | Align::Inside);
        //     col1.end();
        //
        //     // col1.set_frame(FrameType::EngravedFrame);
        //
        //     let mut col2 = Column::default().size_of_parent();
        //     for card in hand.card_iter() {
        //         let card_row = Row::default()
        //             .with_pos(0, 10)
        //             .with_label(&card.to_string())
        //             .with_align(enums::Align::Inside | enums::Align::Center);
        //         card_row.end()
        //     }
        //     col2.set_frame(FrameType::EngravedFrame);
        //     col2.end();
        //     row.end()
        // }

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

    pub fn add_hand(&mut self, hand: &Hand) {
        let thisgroup = Group::default()
            .with_size(self.pack.w(), TEXT_H)
            .with_pos(self.pack.x(), self.insurance.y() + 2 * PADDING);

        let mut row = Row::default()
            .with_label("")
            .with_size(self.pack.w(), TEXT_H)
            .with_pos(self.pack.x(), self.insurance.y() + 2 * PADDING)
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
