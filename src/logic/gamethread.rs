use std::num::NonZeroU64;
use std::thread;
use std::time::Instant;
use crate::models::card::Card;

const THREADCOUNT: usize = 8;

pub fn simulate(hands: Vec<Vec<Card>>, remaining_cards: Vec<Card>) -> WinEstimation {
    let mut total_games_played: u128 = 0;
    let mut total_games_won: u64 = 0;
    crossbeam::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(THREADCOUNT);
        for _ in 0..THREADCOUNT {
            handles.push(scope.spawn(|_| {
                let thread_start: Instant = Instant::now();
                let mut response: ThreadResponse = ThreadResponse {
                    id: thread::current().id().as_u64(),
                    games_played: 0,
                    games_won: 0,
                    time_spent: 0
                };
                let mut round_cards: Vec<Card> = remaining_cards.clone();
                let mut round_hands: Vec<Vec<Card>> = hands.clone();

                while thread_start.elapsed().as_millis() < 900 {


                    response.games_played += 1;
                }

                response.time_spent = thread_start.elapsed().as_millis();
                return response;
            }));
        }
        print!("\x1B[2J\x1B[1;1H");
        for handle in handles{
            let response: ThreadResponse = handle.join().unwrap();
            total_games_played += response.games_played;
            total_games_won += response.games_won;
            println!("Thread: {:2} won {:10} out of {:10} games in {:4}ms" , response.id, response.games_won, response.games_played, response.time_spent)
        }
        println!("-----------------------------------------------------------");
        print!("Total:     won {:10} out of {:10} games in ", total_games_won, total_games_played)
    }) // All spawned threads are auto-joined here, no need for join_handles
        .unwrap();

    return WinEstimation { chance: 0f64 }
}

pub struct ThreadResponse {
    id: NonZeroU64,
    games_played: u128,
    games_won: u64,
    time_spent: u128
}

pub struct WinEstimation {
    chance: f64
}