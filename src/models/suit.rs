use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    HEARTS,
    SPADES,
    CLUBS,
    DIAMONDS
}