use std::error::Error;
use std::thread::current;

use fltk::enums::{Color, FrameType};
use fltk::group::{Column, Flex, Row};
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

use gui_classes::middle::*;
use gui_classes::player_widget::PlayerWid;

use crate::table::Table;
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::ops::Deref;

mod card;
mod constants;
mod deck;
mod deck_traits;
mod errors;
mod gui_classes;
mod hand;
mod player;
mod table;

// Type alias for Result<T, Box<dyn Error>>
type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Increment(usize),
    Decrement(usize),
}

fn main() -> Res<()> {
    let win_w = 1000;
    let win_h = 800;
    let border = 20;

    const PLAYERS: usize = 4;
    let player_box_h = win_h - border - 200;

    let hspacing = 10;
    let button_width = (win_w - 2 * border) / 4;

    let mut table = Table::new(PLAYERS as usize, 4)?;

    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("Blackjack")
        .with_size(win_w, win_h)
        .center_screen();

    let mut vpack = group::Pack::new(border, border, win_w - 2 * border, win_w - border, "vapck1");
    let mut hpdealer = Pack::new(border, border, win_w - border, 120, "");
    let mut dealer = Frame::default().with_size(wind.w() - 2 * border, win_h / 4);

    dealer.set_frame(FrameType::EmbossedBox);
    dealer.set_label("Dealer");
    dealer.set_color(Color::Red);

    hpdealer.end();
    hpdealer.set_type(PackType::Horizontal);

    let mut middle = MiddleSection::build(0, 50, win_w - 2 * border, win_h / 4);

    let mut playerwid = vec![];
    let mut hpack = group::Pack::new(0, 0, win_w - 2 * border, 300, "");
    for pnum in 1..=PLAYERS {
        let mut player = table.player(pnum - 1).unwrap();
        playerwid.push(PlayerWid::new(player, pnum));
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

    let (s, r) = app::channel::<Message>();

    // Just to testing to make sure functions are working.
    std::thread::spawn(move || loop {
        app::sleep(1.);
        s.send(Message::Increment(1));
    });

    let mut current_player = 0usize;
    while app.wait() {
        if let Some(Message::Increment(step)) = r.recv() {
            playerwid[current_player].deactivate_player();
            //PlayerWid::deactivate_player(&mut playerwid[current_player]);
            current_player += step;
            current_player %= PLAYERS;
            playerwid[current_player].activate_player();
        }
    }

    // app.run()?;
    Ok(())
}
