use crate::player::Player;
use fltk::button::Button;
use fltk::enums::*;
use fltk::enums::{Color, FrameType};
use fltk::frame::Frame;
use fltk::group::{Pack, PackType, Row};
use fltk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MiddleSection {
    title: Row,
    hit: Button,
    stand: Button,
    double: Button,
    split: Button,
    message: String,
    player: Option<Rc<RefCell<Player>>>,
}

impl MiddleSection {
    pub fn build(x: i32, y: i32, w: i32, h: i32) -> MiddleSection {
        let message = String::new();
        let mut title = Row::new(x, y, w, 30, &message).with_align(Align::Inside | Align::Center);
        title.end();

        let mut toprow = Row::new(x, y, w, h / 5, "").with_align(Align::Inside | Align::Center);
        toprow.set_pad(15);

        let mut split = Button::default()
            // .with_pos(0, 0)
            .with_size(30, 80)
            .with_align(Align::Inside | Align::Center);
        split.set_label("Split");

        let mut hit = Button::default()
            // .with_pos(0, 0)
            .with_size(30, 80)
            .with_align(Align::Inside | Align::Center);
        hit.set_label("Hit");

        let mut stand = Button::default()
            // .with_pos(0, 0)
            .with_size(30, 80)
            .with_align(Align::Inside | Align::Center);
        stand.set_label("Stand");

        let mut double = Button::default()
            // .with_pos(0, 0)
            .with_size(30, 80)
            .with_align(Align::Inside | Align::Center);
        double.set_label("Double");

        toprow.add(&split);
        toprow.add(&hit);
        toprow.add(&stand);
        toprow.add(&double);

        toprow.end();

        let mut center = Pack::new(x, y, w, h, "middle").with_align(Align::Inside | Align::Center);
        //let mut center = Frame::default().center_of(to_center).with_size(200, 200);
        //center.set_color(Color::Green);
        center.set_label("Middle");
        //center.set_frame(FrameType::EmbossedBox);

        center.set_type(PackType::Horizontal);
        center.end();

        MiddleSection {
            title,
            hit,
            stand,
            double,
            split,
            message,
            player: None,
        }
    }
}
