use crate::player::Player;
use fltk::button::Button;
use fltk::enums::FrameType;
use fltk::frame::Frame;
use fltk::group::{Column, Row};
use fltk::prelude::*;
use fltk::{enums, group};
use num::{Rational64, Zero};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct PlayerWid {
    id: usize,
    player: Rc<RefCell<Player>>,
    pack: group::Pack,
    title: Frame,
    points: Frame,
    bet: Frame,
    insurance: Frame,
}

impl PlayerWid {
    pub fn new(player: Rc<RefCell<Player>>, id: usize) -> PlayerWid {
        // let player = table.player(pnum - 1).unwrap();
        let mut playervpac = group::Pack::default()
            .with_size(200, 100)
            .with_align(enums::Align::Inside | enums::Align::Center);
        // let playertitle = Row::default_fill().size_of_parent();

        let mut title = Frame::default()
            .with_size(0, 00)
            .with_label(player.borrow().name().as_str())
            .with_align(enums::Align::Center);

        title.set_color(enums::Color::from_u32(0x7FFFD4));
        title.set_frame(FrameType::PlasticDownBox);

        // playertitle.end();

        let row1 = Row::default_fill().with_label("row1").with_pos(0, 10);
        let mut col1 = Column::default_fill().with_label("col1").with_size(50, 30);
        col1.set_pad(5);

        //let row2 = Row::default_fill().with_label("row2");
        Frame::default()
            .with_size(0, 0)
            .with_label("Points: ")
            .with_align(enums::Align::Inside | enums::Align::Center);
        // Frame::default()
        //     .with_size(0, 0)
        //     .with_label("Bet: ")
        //     .with_align(enums::Align::Inside | enums::Align::Center);
        Frame::default()
            .with_size(0, 0)
            .with_label("Insurance: ")
            .with_align(enums::Align::Inside | enums::Align::Center);
        col1.end();
        let col2 = Column::default_fill().with_label("col2").size_of_parent();
        let mut points = Frame::default()
            .with_size(0, 0)
            .with_label(&player.borrow().money().to_string())
            .with_align(enums::Align::Inside | enums::Align::Center);
        let mut bet = Frame::default()
            .with_size(0, 0)
            .with_label("0")
            .with_align(enums::Align::Inside | enums::Align::Center);
        let mut insurance = Frame::default()
            .with_size(0, 0)
            .with_label(&player.borrow().insurance().to_string())
            .with_align(enums::Align::Inside | enums::Align::Center);
        col1.end();
        col2.end();
        row1.end();
        playervpac.auto_layout();
        playervpac.end();
        PlayerWid {
            id,
            player,
            pack: playervpac,
            title,
            points,
            bet,
            insurance,
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

    pub fn set_points(&mut self) {
        self.points
            .set_label(&self.player.borrow().money().to_string());
        self.points.redraw()
    }

    pub fn set_bet(&mut self) {
        self.bet.set_label(
            &self
                .player
                .borrow()
                .hand_iter()
                .next()
                .unwrap()
                .bet()
                .unwrap_or(Rational64::zero())
                .to_string(),
        );
        self.bet.redraw()
    }

    pub fn set_insurance(&mut self) {
        self.insurance
            .set_label(&self.player.borrow().insurance().to_string());
        self.insurance.redraw()
    }
}
