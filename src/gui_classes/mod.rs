use fltk::button::Button;

mod card;
mod dealer;
mod header;
pub mod middle;
pub mod player_widget;

pub const BUTTON_H: i32 = 80;
pub const WIN_W: i32 = 1000;
pub const WIN_H: i32 = 800;
pub const BORDER: i32 = 20;
pub const PADDING: i32 = 10;

pub const CARD_H: i32 = 60;
pub const CARD_W: i32 = (CARD_H as f32 * CARD_RATIO) as i32;
const CARD_RATIO: f32 = 2.5 / 3.5;
pub const EIGHTH: i32 = WIN_H / 8;

pub use crate::gui_classes::card::*;
pub use crate::gui_classes::dealer::GUIDealer;
pub use crate::gui_classes::header::GUIHeader;
