use crate::gui_classes::{BUTTON_H, PADDING};
use crate::Message;
use fltk::app::Sender;
use fltk::button::Button;
use fltk::enums::Align;
use fltk::frame::Frame;
use fltk::group::{Column, Group, Pack, PackType, Row};
use fltk::prelude::*;

pub struct GUIHeader {
    restart: Button,
}

impl GUIHeader {
    pub fn new(x: i32, y: i32, w: i32, h: i32, s: &Sender<Message>) -> GUIHeader {
        let mut row = Row::new(x, y, w, h, "");
        row.set_margin(PADDING);

        // let mut button = Button::new(x + PADDING, y + PADDING, BUTTON_H, BUTTON_H + BUTTON_H / 2, "Restart")
        //     .with_align(Align::Inside | Align::Center);

        let mut butg = Pack::default().with_size(50, h);
        let mut button = Button::default()
            .with_size(BUTTON_H, BUTTON_H)
            .with_label("Restart")
            .with_align(Align::Inside | Align::Center);
        button.emit(s.clone(), Message::Restart);

        butg.set_type(PackType::Horizontal);
        butg.end();
        // Add empty space
        //let empty_spacing = Frame::default().with_size(40, 0);

        let mut title = Frame::default()
            // .with_size(BUTTON_H, BUTTON_H * 3)
            .with_label("BLACKJACK")
            .with_align(Align::Inside | Align::Center);

        title.set_label_size(title.label_size() * 2);

        // Add empty space
        //  empty_spacing.clone();

        let author = Frame::default()
            .with_label("Tim Reed\nTake Home Assignment")
            .with_align(Align::Inside | Align::Right | Align::Top);

        row.end();

        GUIHeader { restart: button }
    }
}
