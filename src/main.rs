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

use crate::blackjack::Table;
use crate::card::Visible::{FacedDown, FacedUp};
use crate::card::{Card, Denomination, Suit};
use crate::gui_classes::*;
use crate::hand::Action;
use crate::player::Player;
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::convert::TryFrom;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::mpsc;

mod blackjack;
mod card;
mod constants;
mod deck;
mod deck_traits;
mod errors;
mod gui_classes;
mod hand;
mod player;

// Type alias for Result<T, Box<dyn Error>>
type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub enum Message {
    CurrentPlayer(usize),
    Bet(String),
    Insurance(i32),
    Play(Action),
    Restart,
    Continue,
}

fn main() -> Res<()> {
    const PLAYERS: usize = 5;

    let (s, r) = app::channel::<Message>();

    let mut table = Table::new(PLAYERS as usize, 4)?;

    let mut current_player = 0usize;

    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("Blackjack")
        .with_size(WIN_W, WIN_H)
        .center_screen();

    // Header section. Contains restart button, the title, and my name.
    let mut header = GUIHeader::new(0, 0, WIN_W, EIGHTH, &s);

    // Dealer section.  This is where dealer cards are.
    let mut dealer = GUIDealer::new()
        .with_size(WIN_W - 2 * BORDER, CARD_H)
        .with_pos(BORDER, BORDER + EIGHTH);

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

    let mut middle = MiddleSection::new(
        BORDER,
        BORDER + 2 * EIGHTH + CARD_H,
        WIN_W - 2 * BORDER,
        EIGHTH,
        s.clone(),
    )
    .with_pos(BORDER, BORDER + 2 * EIGHTH + CARD_H)
    .with_size(WIN_W - 2 * BORDER, EIGHTH);

    let mut playerwid = vec![];
    let mut hpack = group::Pack::new(
        BORDER,
        middle.y() + middle.h() + PADDING + 50,
        WIN_W - 2 * BORDER - (PLAYERS as i32 - 1) * PADDING as i32,
        3 * EIGHTH,
        "",
    );

    for pnum in 1..=PLAYERS {
        let mut player = table.player(pnum - 1).unwrap();
        let width = (WIN_W - 2 * BORDER - (PLAYERS as i32 - 1) * PADDING as i32) / PLAYERS as i32;
        let mut playgui = GUIPlayer::new(pnum, width)
            .with_size(hpack.w() / PLAYERS as i32, hpack.h())
            .with_pos(
                hpack.x() + (pnum as i32 - 1) * (hpack.w() / PLAYERS as i32),
                hpack.y(),
            );

        playerwid.push(playgui);
    }

    hpack.end();
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(PADDING);

    wind.make_resizable(false);

    let mut gui = GUIMain::new(header, dealer, message, middle, playerwid, table);

    gui.setup_game();
    gui.start_round();

    wind.end();
    wind.show();

    while app.wait() {
        if let Some(recieved) = r.recv() {
            match recieved {
                Message::Bet(str) => gui.set_bet(str),
                Message::Play(action) => gui.perform_action(action),
                Message::Continue => gui.continue_play(),
                _ => println!("Other"),
            }
        }
    }

    Ok(())
}
