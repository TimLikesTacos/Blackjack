use std::error::Error;
use std::thread::current;

use fltk::enums::{Align, Color, FrameType};
use fltk::group::{Column, Flex, Row};
use fltk::{
    app, button,
    button::Button,
    enums,
    frame::Frame,
    group,
    group::{Pack, PackType},
    image,
    output::Output,
    prelude::*,
    text, window,
    window::Window,
};

use gui_classes::middle::*;
use gui_classes::player_widget::GUIPlayer;

use crate::card::Visible::{FacedDown, FacedUp};
use crate::card::{Card, Denomination, Suit};
use crate::gui_classes::*;
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
    const PLAYERS: usize = 4;

    let mut table = Table::new(PLAYERS as usize, 4)?;

    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("Blackjack")
        .with_size(WIN_W, WIN_H)
        .center_screen();

    // Header section. Contains restart button, the title, and my name.
    let mut header = GUIHeader::new(0, 0, WIN_W, EIGHTH);

    // Dealer section.  This is where dealer cards are.
    let mut dealer = GUIDealer::new(table.player(1).unwrap())
        .with_size(WIN_W - 2 * BORDER, CARD_H)
        .with_pos(BORDER, BORDER + EIGHTH);
    let card = FacedUp(Card::new(Denomination::King, Suit::Clubs));
    let card2 = FacedUp(Card::new(Denomination::Numerical(4), Suit::Hearts));
    dealer.add_card(&card);
    dealer.add_card(&card2);

    let mut message = Frame::new(
        BORDER,
        BORDER + EIGHTH + CARD_H,
        WIN_W - 2 * BORDER,
        EIGHTH,
        "Messages go here",
    )
    .with_align(Align::Center);
    message.set_label_size(30);
    message.set_frame(FrameType::BorderBox);

    // let mut vpack = group::Pack::new(
    //     BORDER,
    //     BORDER + BUTTON_H,
    //     WIN_W - 2 * BORDER,
    //     WIN_W - BORDER,
    //     "vapck1",
    // );
    //
    // //.with_align(Align::Inside | Align::Left);
    //
    let mut middle = MiddleSection::new(
        BORDER,
        BORDER + 2 * EIGHTH + CARD_H,
        WIN_W - 2 * BORDER,
        EIGHTH,
    )
    .with_pos(BORDER, BORDER + 2 * EIGHTH + CARD_H)
    .with_size(WIN_W - 2 * BORDER, EIGHTH);
    middle.set_frame(FrameType::BorderBox);
    middle.add_card(&card2);
    middle.add_card(&card);

    let mut playerwid = vec![];
    let mut hpack = group::Pack::new(
        BORDER,
        middle.y() + middle.h() + PADDING + 50,
        WIN_W - 2 * BORDER,
        2 * EIGHTH,
        "",
    );

    for pnum in 1..=PLAYERS {
        let mut player = table.player(pnum - 1).unwrap();
        let mut playgui = GUIPlayer::new(player, pnum);

        playerwid.push(playgui);
    }

    hpack.end();
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(PADDING);
    // vpack.end();
    // vpack.set_type(PackType::Vertical);
    // vpack.set_spacing(15);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    // let (s, r) = app::channel::<Message>();
    //
    // // Just to testing to make sure functions are working.
    // std::thread::spawn(move || loop {
    //     app::sleep(1.);
    //     s.send(Message::Increment(1));
    // });
    //
    // let mut current_player = 0usize;
    // while app.wait() {
    //     if let Some(Message::Increment(step)) = r.recv() {
    //         playerwid[current_player].deactivate_player();
    //         //PlayerWid::deactivate_player(&mut playerwid[current_player]);
    //         current_player += step;
    //         current_player %= PLAYERS;
    //         playerwid[current_player].activate_player();
    //     }
    // }

    app.run()?;
    Ok(())
}
