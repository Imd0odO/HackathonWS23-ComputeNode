use rocket::serde::json::Json;
use crate::models::table::Table;
use std::thread;
use std::time::Instant;
use crate::logic::get_remaining_cards::get_remaining_cards;
use crate::models::card::Card;
use crate::models::rank::Rank::_2;
use crate::models::suit::Suit::HEARTS;

const THREADCOUNT: usize = 8;

pub fn monte_carlo(mut table: Json<Table>) -> f64 {
    let a = Instant::now();

    let player_cnt: usize = table.players.len();
    let active_player: usize = table.active_player as usize;

    let mut hands: Vec<Vec<Card>> = Vec::with_capacity(10);

    (0..player_cnt).for_each(|i| {
        hands.push(table.community_cards.clone());
    });

    hands[0].append(&mut table.players[active_player].cards.as_mut().unwrap());

    let remaining_cards: Vec<Card> = get_remaining_cards(hands[0].clone());

    let v = &remaining_cards;

    let mut total_games_played: u128 = 0;

    crossbeam::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(THREADCOUNT);
        for _ in 0..THREADCOUNT {
            handles.push(scope.spawn(|_| {
                let thread_start: Instant = Instant::now();
                let mut games_played :u128 = 0;
                let mut games_won :u64 = 0;
                let mut round_cards: Vec<Card> = remaining_cards.clone();
                let mut round_hands: Vec<Vec<Card>> = hands.clone();

                while thread_start.elapsed().as_millis() < 900 {
                    games_played += 1;
                }

                return games_played;
            }));
        }
        print!("\x1B[2J\x1B[1;1H");
        for handle in handles{
            let games_played = handle.join().unwrap();
            total_games_played += games_played;
            println!("Thread: {} games" , games_played)
        }
    }) // All spawned threads are auto-joined here, no need for join_handles
    .unwrap();


    println!("{} {}", total_games_played, a.elapsed().as_micros());
    return 0.0
}