use crate::gui_classes::{BUTTON_H, PADDING};
use crate::Message;
use fltk::app::Sender;
use fltk::button::Button;
use fltk::enums::Align;
use fltk::frame::Frame;
use fltk::group::{Pack, PackType, Row};
use fltk::prelude::*;

#[allow(dead_code)]
pub struct GUIHeader {
    restart: Button,
}

impl GUIHeader {
    pub fn new(x: i32, y: i32, w: i32, h: i32, s: &Sender<Message>) -> GUIHeader {
        let mut row = Row::new(x, y, w, h, "");
        row.set_margin(PADDING);

        let mut butg = Pack::default().with_size(50, h);
        let mut button = Button::default()
            .with_size(BUTTON_H, BUTTON_H)
            .with_label("Restart")
            .with_align(Align::Inside | Align::Center);
        button.emit(s.clone(), Message::Restart);

        butg.set_type(PackType::Horizontal);
        butg.end();
        // Add empty space

        let mut title = Frame::default()
            .with_label("BLACKJACK")
            .with_align(Align::Inside | Align::Center);

        title.set_label_size(title.label_size() * 2);

        Frame::default()
            .with_label("Tim Reed\nTake Home Assignment")
            .with_align(Align::Inside | Align::Right | Align::Top);

        row.end();

        GUIHeader { restart: button }
    }
}
