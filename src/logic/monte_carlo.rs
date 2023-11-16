use rocket::serde::json::Json;
use crate::models::table::Table;
use std::thread;
use std::time::Instant;
use crate::logic::get_remaining_cards::get_remaining_cards;
use crate::models::card::Card;

const THREADCOUNT: usize = 8;

pub fn monte_carlo(table: Json<Table>) -> f64 {



    let mut handles = Vec::with_capacity(THREADCOUNT);

    let player_cnt = table.players.len();

    let mut hands: Vec<Vec<&Card>> = Vec::with_capacity(player_cnt);

    // give every player the community cards
    (0..player_cnt).for_each(|_| {
        hands.push(vec![&table.community_cards[0],&table.community_cards[1],&table.community_cards[2]])
    });

    // give yourself your cards
    hands[0].append(&mut vec![&table.players[table.active_player as usize].cards.as_ref().unwrap()[0], &table.players[table.active_player as usize].cards.as_ref().unwrap()[1]]);

    let unknown: Vec<Card> = get_remaining_cards(&hands[0]);

    (0..THREADCOUNT).for_each(|_| {
        handles.push(thread::spawn(move || {
            let total_games: u64 = 0;
            let wins: u64 = 0;

            let start: Instant = Instant::now();

            while start.elapsed().as_millis() < 450 {

            }
        }))
    });

    return 0.0
}