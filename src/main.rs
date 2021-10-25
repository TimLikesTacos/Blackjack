use fltk::enums::{Align, FrameType};
use fltk::{
    app,
    frame::Frame,
    group::{Pack, PackType},
    prelude::*,
    window::Window,
};
use std::error::Error;

use crate::gui_classes::*;
use crate::hand::Action;
use crate::table::Table;
use clap::{App, Arg};
use gui_classes::middle::*;
use gui_classes::player_widget::GUIPlayer;
use std::process::exit;

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

#[derive(Debug, Clone)]
pub enum Message {
    CurrentPlayer(usize),
    Bet(String),
    Insurance(String),
    Play(Action),
    Restart,
    Continue,
}

fn main() -> Res<()> {
    // CLI options
    let matches = App::new("BlackJack")
        .version("0.1.0")
        .author("Tim Reed <thetimmyreed@gmail.com")
        .about("Blackjack game for take home assignment")
        .arg(
            Arg::with_name("players")
                .short("p")
                .long("players")
                .help("Sets the number of players fo the game. Minimum of 1, maximum of 5.")
                .takes_value(true)
                .default_value("4"),
        )
        .arg(
            Arg::with_name("decks")
                .help("Sets the amount of 52 card decks used. Minimum of 1, Maxmimum of 8")
                .short("d")
                .long("decks")
                .takes_value(true)
                .default_value("6"),
        )
        .get_matches();

    // Get options from CLI or use defaults
    let players: usize = matches.value_of("players").unwrap_or_default().parse()?;
    let decks: usize = matches.value_of("decks").unwrap_or_default().parse()?;

    if players > 5 || players < 1 || decks < 1 || decks > 8 {
        eprintln!("Invalid player or deck parameters. Run 'blackjack --help' for usage details");
        exit(1);
    }

    // Channel for sending messages from GUI to the rest of the app.
    let (s, r) = app::channel::<Message>();

    // Table to be used for game.  Needed before the GUI is built so that the gui knows the players involved.
    let table = Table::new(players, decks)?;

    // Create the application.  All items between here and `wind.end()` are part of the gui.
    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("Blackjack")
        .with_size(WIN_W, WIN_H)
        .center_screen();

    fltk::app::set_background_color(121, 210, 121);
    // Header section. Contains restart button, the title, and my name.
    let header = GUIHeader::new(0, 0, WIN_W, EIGHTH, &s);

    // Dealer section.  This is where dealer cards are.
    let dealer = GUIDealer::new()
        .with_size(WIN_W - 2 * BORDER, CARD_H)
        .with_pos(BORDER, BORDER + EIGHTH);

    // Main way to communicate with the player.
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

    // Where player actions occur
    let middle = MiddleSection::new(s.clone())
        .with_pos(BORDER, BORDER + 2 * EIGHTH + CARD_H)
        .with_size(WIN_W - 2 * BORDER, EIGHTH);

    let mut playerwid = vec![];
    let mut hpack = Pack::new(
        BORDER,
        middle.y() + middle.h() + PADDING + 50,
        WIN_W - 2 * BORDER - (players as i32 - 1) * PADDING as i32,
        3 * EIGHTH,
        "",
    );

    // Create the player sectons.
    for pnum in 1..=players {
        let width = (WIN_W - 2 * BORDER - (players as i32 - 1) * PADDING as i32) / players as i32;
        let playgui = GUIPlayer::new(pnum, width)
            .with_size(hpack.w() / players as i32, hpack.h())
            .with_pos(
                hpack.x() + (pnum as i32 - 1) * (hpack.w() / players as i32),
                hpack.y(),
            );

        playerwid.push(playgui);
    }

    hpack.end();
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(PADDING);

    wind.make_resizable(false);

    // Merge all the created subsections above into one central control struct.
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
                Message::Insurance(insurance_bet) => gui.set_insurance(insurance_bet),
                Message::Restart => {
                    let table = Table::new(players, decks)?;
                    gui = GUIMain::new(
                        gui.header,
                        gui.dealer,
                        gui.message,
                        gui.middle,
                        gui.players_gui,
                        table,
                    );
                    gui.setup_game();
                    gui.start_round();
                }
                _ => println!("Other"),
            }
        }
    }

    Ok(())
}
