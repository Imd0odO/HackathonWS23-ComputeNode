use rocket::serde::json::Json;
use crate::models::table::Table;
use std::time::Instant;
use crate::logic::deck::get_remaining_cards;
use crate::logic::gamethread::{simulate, WinEstimation};
use crate::models::card::Card;


pub fn monte_carlo(mut table: Json<Table>) -> f64 {
    let simulation_start: Instant = Instant::now();

    let player_cnt: usize = table.players.len();
    let active_player: usize = table.active_player as usize;

    let mut hands: Vec<Vec<Card>> = Vec::with_capacity(10);

    (0..player_cnt).for_each(|i| {
        hands.push(table.community_cards.clone());
    });

    hands[0].append(&mut table.players[active_player].cards.as_mut().unwrap());

    let remaining_cards: Vec<Card> = get_remaining_cards(hands[0].clone());

    let estimation: WinEstimation = simulate(hands, remaining_cards);

    print!("{:4}ms\n", simulation_start.elapsed().as_millis());
    return 0.0
}