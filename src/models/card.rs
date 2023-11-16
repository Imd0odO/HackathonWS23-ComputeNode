use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Card {
    pub rank: crate::models::rank::Rank,
    pub suit: crate::models::suit::Suit,
}