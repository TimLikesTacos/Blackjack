mod card;
mod constants;
mod deck;
mod deck_traits;
mod errors;
mod hand;
mod player;

use fltk::enums::{Color, FrameType};
use fltk::group::Flex;
use fltk::{
    app, button,
    button::Button,
    enums,
    frame::Frame,
    group,
    group::{Pack, PackType},
    output::Output,
    prelude::*,
    text, window,
    window::Window,
};
use std::error::Error;

// Type alias for Result<T, Box<dyn Error>>
type Res<T> = Result<T, Box<dyn Error>>;

fn main() {
    let win_w = 1000;
    let win_h = 800;
    let border = 20;

    let players = 4;
    let player_box_h = win_h - border - 200;

    let hspacing = 10;
    let button_width = (win_w - 2 * border) / 4;

    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("Blackjack")
        .with_size(win_w, win_h)
        .center_screen();

    let mut vpack = group::Pack::new(border, border, win_w - 2 * border, win_w - border, "vapck1");
    let mut hpdealer = Pack::new(border, border, win_w - border, 120, "");
    let mut dealer = Frame::default().with_size(wind.w() - 2 * border, player_box_h);

    dealer.set_frame(FrameType::EmbossedBox);
    dealer.set_label("Dealer");
    dealer.set_color(Color::Red);

    hpdealer.end();
    hpdealer.set_type(PackType::Horizontal);

    let mut center = Frame::default().center_of(&wind).with_size(200, 200);
    center.set_color(Color::Green);
    center.set_label("Middle");
    center.set_frame(FrameType::EmbossedBox);

    let mut hpack = group::Pack::new(0, 0, 100, 300, "");
    for pnum in 1..=players {
        let mut frame = Frame::default().with_size(
            (wind.w() - 2 * border - hspacing * (players - 1)) / players,
            300,
        );
        frame.set_frame(FrameType::PlasticUpBox);
        frame.set_label("Player");
        frame.set_color(enums::Color::from_u32(0x7FFFD4));
        frame.set_label_size(14);
    }
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(hspacing);
    vpack.end();
    vpack.set_type(PackType::Vertical);
    vpack.set_spacing(15);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
