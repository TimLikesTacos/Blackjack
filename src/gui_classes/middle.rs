use crate::card::{Card, Visible};
use crate::gui_classes::{GUICard, CARD_H, CARD_W, EIGHTH};
use crate::player::Player;
use crate::{BORDER, PADDING, WIN_W};
use fltk::button::Button;
use fltk::enums::*;
use fltk::enums::{Color, FrameType};
use fltk::frame::Frame;
use fltk::group::{Group, Pack, PackType, Row};
use fltk::input::Input;
use fltk::prelude::*;
use fltk::widget_extends;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MiddleSection {
    group: Group,
    hit: Button,
    stand: Button,
    double: Button,
    split: Button,
    player: Option<Rc<RefCell<Player>>>,
    num_cards: i32,
}

impl MiddleSection {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> MiddleSection {
        // let mut title = Row::new(x, y, w, 30, "").with_align(Align::Inside | Align::Center);
        // title.end();

        let mut group = Group::default();
        // let mut toprow = Row::new(x, y, w, h / 5, "").with_align(Align::Inside | Align::Center);
        // toprow.set_pad(15);

        let mut hand = Frame::default()
            .with_size(10, 10)
            .with_pos(group.x() + WIN_W / 4, group.y() + PADDING)
            .with_label("Player hand:")
            .with_align(Align::Left);
        hand.set_frame(FrameType::BorderBox);

        let mut bet = Input::default()
            .with_size(80, 60)
            .with_pos(WIN_W / 2 - 40, group.y() + PADDING)
            .with_label("Bet:")
            .with_align(Align::Left);

        bet.hide();

        let mut hit = Button::default()
            // .with_pos(0, 0)
            .with_size(80, 50)
            .with_pos(
                WIN_W / 2 - 80 - PADDING - BORDER,
                group.y() + CARD_H + PADDING,
            )
            .with_align(Align::Inside | Align::Center);
        hit.set_label("Hit");

        let mut split = Button::default()
            // .with_pos(0, 0)
            .with_size(80, 50)
            .left_of(&hit, 2 * PADDING)
            .with_align(Align::Inside | Align::Center);
        split.set_label("Split");

        split.hide();

        let mut stand = Button::default()
            // .with_pos(0, 0)
            .with_size(80, 50)
            .right_of(&hit, 2 * PADDING)
            .with_align(Align::Inside | Align::Center);
        stand.set_label("Stand");

        let mut double = Button::default()
            .with_align(Align::Inside | Align::Center)
            .with_size(80, 50)
            .right_of(&stand, 2 * PADDING);
        double.set_label("Double");

        // group.add(&split);
        // group.add(&hit);
        // group.add(&stand);
        // group.add(&double);
        // toprow.add(&split);
        // toprow.add(&hit);
        // toprow.add(&stand);
        // toprow.add(&double);
        //
        // toprow.end();

        let mut center = Pack::new(x, y, w, h, "middle").with_align(Align::Inside | Align::Center);
        //let mut center = Frame::default().center_of(to_center).with_size(200, 200);
        //center.set_color(Color::Green);
        center.set_label("Middle");
        //center.set_frame(FrameType::EmbossedBox);

        center.set_type(PackType::Horizontal);
        center.end();

        group.end();
        MiddleSection {
            group,
            hit,
            stand,
            double,
            split,
            player: None,
            num_cards: 0,
        }
    }

    pub fn add_card(&mut self, card: &Visible<Card>) {
        let mut acard = GUICard::new(card.clone()).with_pos(
            self.group.x() + WIN_W / 4 + self.num_cards * (PADDING + CARD_W),
            self.group.y(),
        );
        self.num_cards += 1;
        acard.set_size(CARD_W, CARD_H);

        self.group.add(&acard.group);
    }
}

widget_extends!(MiddleSection, Group, group);
