use crate::models::card::Card;
use crate::models::rank::Rank;
use crate::models::suit::Suit;

pub enum BestHand {
    // Royal Flush
    RoyalFlush,
    // Straight Flush contains its highest card
    StraightFlush(Rank),
    // Four of a Kind contains its rank
    FourOfAKind(Rank),
    // Full House contains both of its ranks
    FullHouse(Rank, Rank),
    // Flush contains its highest card
    Flush(Rank),
    // Straight contains its highest card
    Straight(Rank),
    // Three of a kind contains its rank
    ThreeOfAKind(Rank),
    // Two Pairs contains the rank of the Pairs
    TwoPair(Rank, Rank),
    // Pair contains its rank
    Pair(Rank),
    // High Card contains its rank
    HighCard(Rank),
}

// evaluate all cards
pub fn evaluate(hand: &Vec<Card>) -> BestHand {
    return evaluateFlushes(hand)
}


// calculate every suit based winning hand
fn evaluateFlushes(cards: &Vec<Card>) -> BestHand {

    // create a vector for each suit
    let mut suits: Vec<Vec<&Card>> = vec![vec![], vec![], vec![], vec![]];

    //sort cards for suit
    for card in cards {
        match card.suit {
            Suit::HEARTS => suits[0].push(card),
            Suit::DIAMONDS => suits[1].push(card),
            Suit::CLUBS => suits[2].push(card),
            Suit::SPADES => suits[3].push(card),
        }
    }

    // get best hand
    for suit in suits {
        // min Flush if 5 times same suit
        if suit.len() >= 5 {
            // min Straight Flush if its a straight
            // TODO: implement for more than 5 cards
            if suit[0].rank as u8 + 1 == suit[1].rank as u8 && suit[1].rank as u8 + 1 == suit[2].rank as u8 && suit[2].rank as u8 + 1 == suit[3].rank as u8 && suit[3].rank as u8 + 1 == suit[4].rank as u8 {
                // Royal Flush if highest card in Straight Flush is an ace
                if suit[0].rank == Rank::A {
                    return BestHand::RoyalFlush;
                }
                return BestHand::StraightFlush(suit[0].rank);
            }
            return BestHand::Flush(suit[0].rank);
        }
    }
    // if its not suit based best hand -> high card
    return BestHand::HighCard(cards[0].rank);
}