use rocket::serde::json::Json;
use crate::models::table::Table;
use std::time::Instant;
use crate::logic::deck::get_remaining_cards;
use crate::logic::gamethread::{simulate, WinEstimation};
use crate::models::card::Card;


pub fn monte_carlo(mut table: Json<Table>) -> WinEstimation {
    // create start Timestamp for stats
    let simulation_start: Instant = Instant::now();

    // generate known hands for every player, hand[0] is the own hand
    let mut hands: Vec<Vec<Card>> = Vec::with_capacity(10);
    (0..table.players.len()).for_each(|i| {
        hands.push(table.community_cards.clone());
    });
    hands[0].append(&mut table.players[table.active_player as usize].cards.as_mut().unwrap());

    // calculate remaining cards based on the known cards (own hand)
    let remaining_cards: Vec<Card> = get_remaining_cards(hands[0].clone());

    // get an estimation by multithreading
    let estimation: WinEstimation = simulate(hands, remaining_cards);

    // print total duration for Monte Carlo
    println!("{:4}ms", simulation_start.elapsed().as_millis());
    return estimation;
}