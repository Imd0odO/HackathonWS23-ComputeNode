use rocket::form::validate::Len;
use crate::models::card::Card;
use crate::models::rank::Rank;
use crate::models::suit::Suit;
use rand::Rng;

// calculate remaining cards
pub fn get_remaining_cards(known_cards: Vec<Card>) -> Vec<Card> {
    // all cards at the start of a game
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

    // remove cards that are known / visible to the player
    for card in known_cards {
        all.remove(all.iter().position(|c| *c == card).unwrap());
    }

    // return cards still in the deck
    return all;
}

// deal cards to players until everybody has 7 cards (5 community cards + 2 hand cards)
pub fn deal_remaining(hands: &mut  Vec<Vec<Card>>, cards: &mut Vec<Card>) {
    // deal missing community cards
    for _ in 0..(5 - hands.last().len()) {
        let card_index: usize = rand::thread_rng().gen_range(0..cards.len());
        for hand in &mut *hands {
            hand.push(cards[card_index]);
        }
        cards.remove(card_index);
    }

    // deal all other player two cards
    for player_index in 1..hands.len() {
        for _ in 0..2 {
            let card_index: usize = rand::thread_rng().gen_range(0..cards.len());
            hands[player_index].append(&mut vec![cards[card_index]]);
        }
    }
}