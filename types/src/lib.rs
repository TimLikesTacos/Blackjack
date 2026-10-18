pub mod card;
pub mod constants;
pub mod deck;
pub mod deck_traits;
pub mod errors;
pub mod hand;
pub mod player;
pub mod table;
pub type Res<T> = Result<T, Box<dyn std::error::Error>>;
