use std::num::NonZeroU64;
use std::thread;
use std::time::Instant;
use crossbeam::thread::ScopedJoinHandle;
use crate::logic::deck::deal_remaining;
use crate::models::card::Card;

// specify the thread count that should be used (recommended: cores - 2)
const THREAD_COUNT: usize = 8;

// specify maximum simulation duration
const MAX_SIMULATION_TIME: u128 = 900;

// simulate rounds multithreaded based on known cards and own hands
pub fn simulate(hands: Vec<Vec<Card>>, remaining_cards: Vec<Card>) -> WinEstimation {
    // set global stats to 0
    let mut total_games_played: u128 = 0;
    let mut total_games_won: u64 = 0;

    // encapsulating threads from the other variables
    crossbeam::thread::scope(|scope| {
        // create vector for thread handles
        let mut handles: Vec<ScopedJoinHandle<ThreadResponse>> = Vec::with_capacity(THREAD_COUNT);

        // spawn threads
        for _ in 0..THREAD_COUNT {
            handles.push(scope.spawn(|_| {
                // create thread start timestamp
                let thread_start: Instant = Instant::now();

                // create response
                let mut response: ThreadResponse = ThreadResponse {
                    id: thread::current().id().as_u64(),
                    games_played: 0,
                    games_won: 0,
                    time_spent: 0
                };

                // run simulation for
                while thread_start.elapsed().as_millis() < MAX_SIMULATION_TIME {
                    // clone start conditions
                    let mut round_cards: Vec<Card> = remaining_cards.clone();
                    let mut round_hands: Vec<Vec<Card>> = hands.clone();

                    deal_remaining(&mut round_hands, &mut round_cards);

                    // increment games played for ever round simulated
                    response.games_played += 1;
                }

                // get thread run duration
                response.time_spent = thread_start.elapsed().as_millis();

                // return thread stats
                return response;
            }));
        }

        // print stat end line and clear terminal
        println!("===================================================================");
        print!("\x1B[2J\x1B[1;1H");

        // wait for all threads to finish
        for handle in handles{
            // get thread stats
            let response: ThreadResponse = handle.join().unwrap();

            // add thread stats to global stats
            total_games_played += response.games_played;
            total_games_won += response.games_won;

            // print thread statistics
            println!("Thread: {:10} won {:10} out of {:10} games in {:4}ms" , response.id, response.games_won, response.games_played, response.time_spent)
        }
        // print global statistics
        println!("-------------------------------------------------------------------");
        print!("Total:             won {:10} out of {:10} games in ", total_games_won, total_games_played)
    }).unwrap();

    // return win estimation
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