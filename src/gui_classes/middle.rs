use crate::card::{Card, Visible};
use crate::gui_classes::{GUICard, CARD_H, CARD_W};
use crate::hand::Action::{Double, Hit, Split, Stand};
use crate::hand::{Action, Hand};
use crate::Message::{Bet, Insurance};
use crate::{Message, BORDER, PADDING, WIN_W};
use fltk::app::Sender;
use fltk::button::Button;
use fltk::enums::FrameType;
use fltk::enums::*;
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::input::{FloatInput, IntInput};
use fltk::prelude::*;
use fltk::widget_extends;
use std::collections::HashSet;

pub struct MiddleSection {
    group: Group,
    pub(crate) hit: Button,
    pub(crate) stand: Button,
    pub(crate) double: Button,
    pub(crate) split: Button,
    pub(crate) bet: IntInput,
    pub(crate) insurance: FloatInput,
    pub(crate) continue_button: Button,
    num_cards: i32,
}

impl MiddleSection {
    pub fn new(s: Sender<Message>) -> MiddleSection {
        let group = Group::default();

        let mut hand = Frame::default()
            .with_size(10, 10)
            .with_pos(group.x() + WIN_W / 4, group.y() + PADDING)
            .with_label("Player hand:")
            .with_align(Align::Left);
        hand.set_frame(FrameType::BorderBox);

        hand.hide();

        let mut bet = IntInput::default()
            .with_size(80, 60)
            .with_pos(WIN_W / 2 - 40, group.y() + PADDING)
            .with_label("Bet:")
            .with_align(Align::Left);
        bet.set_value("0");
        bet.set_callback({
            let sender = s.clone();
            move |i| {
                let value = i.value();
                sender.send(Bet(value));
            }
        });

        bet.hide();

        let mut insurance = FloatInput::default()
            .with_size(80, 60)
            .with_pos(WIN_W / 2 - 40, group.y() + PADDING)
            .with_label("Insurance Bet:")
            .with_align(Align::Left);
        insurance.set_value("0");
        insurance.set_callback({
            let sender = s.clone();
            move |i| {
                let value = i.value();
                sender.send(Insurance(value));
            }
        });

        insurance.hide();

        let mut hit = Button::default()
            // .with_pos(0, 0)
            .with_size(80, 50)
            .with_pos(
                WIN_W / 2 - 80 - PADDING - BORDER,
                group.y() + CARD_H + PADDING,
            )
            .with_align(Align::Inside | Align::Center);
        hit.set_label("Hit");
        hit.emit(s.clone(), Message::Play(Hit));

        hit.hide();

        let mut split = Button::default()
            // .with_pos(0, 0)
            .with_size(80, 50)
            .left_of(&hit, 2 * PADDING)
            .with_align(Align::Inside | Align::Center);
        split.set_label("Split");
        split.emit(s.clone(), Message::Play(Split));

        split.hide();

        let mut stand = Button::default()
            // .with_pos(0, 0)
            .with_size(80, 50)
            .right_of(&hit, 2 * PADDING)
            .with_align(Align::Inside | Align::Center);
        stand.set_label("Stand");
        stand.emit(s.clone(), Message::Play(Stand));

        stand.hide();

        let mut double = Button::default()
            .with_align(Align::Inside | Align::Center)
            .with_size(80, 50)
            .right_of(&stand, 2 * PADDING);
        double.set_label("Double");
        double.emit(s.clone(), Message::Play(Double));

        double.hide();

        let mut continue_button = Button::default()
            .with_align(Align::Inside | Align::Center)
            .with_size(80, 50)
            .with_pos(
                WIN_W / 2 - 2 * PADDING - 2 * BORDER,
                group.y() + CARD_H + PADDING,
            );
        continue_button.set_label("Continue");
        continue_button.emit(s, Message::Continue);

        continue_button.hide();

        group.end();
        MiddleSection {
            group,
            hit,
            stand,
            double,
            split,
            bet,
            insurance,
            num_cards: 0,
            continue_button,
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
        self.group.redraw();
    }

    pub fn add_cards(&mut self, hand: &Hand) {
        self.remove_cards();
        for card in hand.card_iter() {
            self.add_card(card);
        }
    }

    pub fn hide_buttons(&mut self) {
        self.hit.hide();
        self.stand.hide();
        self.split.hide();
        self.double.hide();
    }

    pub fn show_buttons(&mut self, actions: &HashSet<Action>) {
        self.hide_buttons();
        for action in actions.iter() {
            match action {
                Action::Hit => self.hit.show(),
                Action::Split => self.split.show(),
                Action::Stand => self.stand.show(),
                Action::Double => self.double.show(),
            }
        }
    }

    pub fn remove_cards(&mut self) {
        let mut last = self.group.children() - 1;
        let first = last - self.num_cards;
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

widget_extends!(MiddleSection, Group, group);
