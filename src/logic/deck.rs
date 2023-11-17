use crate::models::card::Card;
use crate::models::rank::Rank;
use crate::models::suit::Suit;

pub fn get_remaining_cards(known_cards: Vec<Card>) -> Vec<Card> {
    let mut all: Vec<Card> = vec![
        Card { rank: Rank::A, suit: Suit::HEARTS },
        Card { rank: Rank::K, suit: Suit::HEARTS },
        Card { rank: Rank::Q, suit: Suit::HEARTS },
        Card { rank: Rank::J, suit: Suit::HEARTS },
        Card { rank: Rank::_10, suit: Suit::HEARTS },
        Card { rank: Rank::_9, suit: Suit::HEARTS },
        Card { rank: Rank::_8, suit: Suit::HEARTS },
        Card { rank: Rank::_7, suit: Suit::HEARTS },
        Card { rank: Rank::_6, suit: Suit::HEARTS },
        Card { rank: Rank::_5, suit: Suit::HEARTS },
        Card { rank: Rank::_4, suit: Suit::HEARTS },
        Card { rank: Rank::_3, suit: Suit::HEARTS },
        Card { rank: Rank::_2, suit: Suit::HEARTS },

        Card { rank: Rank::A, suit: Suit::DIAMONDS },
        Card { rank: Rank::K, suit: Suit::DIAMONDS },
        Card { rank: Rank::Q, suit: Suit::DIAMONDS },
        Card { rank: Rank::J, suit: Suit::DIAMONDS },
        Card { rank: Rank::_10, suit: Suit::DIAMONDS },
        Card { rank: Rank::_9, suit: Suit::DIAMONDS },
        Card { rank: Rank::_8, suit: Suit::DIAMONDS },
        Card { rank: Rank::_7, suit: Suit::DIAMONDS },
        Card { rank: Rank::_6, suit: Suit::DIAMONDS },
        Card { rank: Rank::_5, suit: Suit::DIAMONDS },
        Card { rank: Rank::_4, suit: Suit::DIAMONDS },
        Card { rank: Rank::_3, suit: Suit::DIAMONDS },
        Card { rank: Rank::_2, suit: Suit::DIAMONDS },

        Card { rank: Rank::A, suit: Suit::CLUBS },
        Card { rank: Rank::K, suit: Suit::CLUBS },
        Card { rank: Rank::Q, suit: Suit::CLUBS },
        Card { rank: Rank::J, suit: Suit::CLUBS },
        Card { rank: Rank::_10, suit: Suit::CLUBS },
        Card { rank: Rank::_9, suit: Suit::CLUBS },
        Card { rank: Rank::_8, suit: Suit::CLUBS },
        Card { rank: Rank::_7, suit: Suit::CLUBS },
        Card { rank: Rank::_6, suit: Suit::CLUBS },
        Card { rank: Rank::_5, suit: Suit::CLUBS },
        Card { rank: Rank::_4, suit: Suit::CLUBS },
        Card { rank: Rank::_3, suit: Suit::CLUBS },
        Card { rank: Rank::_2, suit: Suit::CLUBS },

        Card { rank: Rank::A, suit: Suit::SPADES },
        Card { rank: Rank::K, suit: Suit::SPADES },
        Card { rank: Rank::Q, suit: Suit::SPADES },
        Card { rank: Rank::J, suit: Suit::SPADES },
        Card { rank: Rank::_10, suit: Suit::SPADES },
        Card { rank: Rank::_9, suit: Suit::SPADES },
        Card { rank: Rank::_8, suit: Suit::SPADES },
        Card { rank: Rank::_7, suit: Suit::SPADES },
        Card { rank: Rank::_6, suit: Suit::SPADES },
        Card { rank: Rank::_5, suit: Suit::SPADES },
        Card { rank: Rank::_4, suit: Suit::SPADES },
        Card { rank: Rank::_3, suit: Suit::SPADES },
        Card { rank: Rank::_2, suit: Suit::SPADES },
    ];

    for card in known_cards {
        all.remove(all.iter().position(|c| *c == card).unwrap());
    }

    return all;
}

pub fn deal_remaining(hands: &mut Vec<Vec<&Card>>, remaining: &mut Vec<Card>) {


}