use std::num::NonZeroU64;
use std::thread;
use std::time::Instant;
use crossbeam::thread::ScopedJoinHandle;
use crate::logic::deck::deal_remaining;
use crate::logic::winning_combinations::{BestHand, evaluate};
use crate::logic::winning_combinations::BestHand::{Flush, FourOfAKind, FullHouse, HighCard, Pair, RoyalFlush, Straight, StraightFlush, ThreeOfAKind, TwoPair};
use crate::models::card::Card;

// specify the thread count that should be used (recommended: cores - 2)
const THREAD_COUNT: usize = 20;

// specify maximum simulation duration
const MAX_SIMULATION_TIME: u128 = 750;

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

                    // deal everybody cards until everyone has seven
                    deal_remaining(&mut round_hands, &mut round_cards);

                    // evaluate hands
                    let mut best_hands: Vec<BestHand> = Vec::with_capacity(round_hands.len());
                    for hand in round_hands {
                        best_hands.push(evaluate(&hand));
                    }

                    // get won hand
                    let player_best_hand: BestHand = best_hands[0];

                    // sort all hands
                    best_hands.sort();

                    // evaluate best two hands
                    let best_hand: BestHand;
                    match (best_hands[0], best_hands[1]) {
                        (RoyalFlush, RoyalFlush) => {
                            best_hand = RoyalFlush
                        }
                        (StraightFlush(r1), StraightFlush(r2)) => {
                            if r1 < r2 {
                                best_hand = StraightFlush(r1)
                            }
                            else {
                                best_hand = StraightFlush(r2)
                            }
                        }
                        (FourOfAKind(r1), FourOfAKind(r2)) => {
                            if r1 < r2 {
                                best_hand = FourOfAKind(r1)
                            }
                            else {
                                best_hand = FourOfAKind(r2)
                            }

                        }
                        (FullHouse(r1, r2), FullHouse(r3, r4)) => {
                            let mut v1 = vec![r1, r2];
                            let mut v2 = vec![r3, r4];
                            v1.sort();
                            v2.sort();
                            if v1[0] < v2[0] {
                                best_hand = FullHouse(r1, r2)
                            }
                            else {
                                best_hand = FullHouse(r3, r4)
                            }
                        }
                        (Flush(r1), Flush(r2)) => {
                            if r1 < r2 {
                                best_hand = Flush(r1)
                            }
                            else {
                                best_hand = Flush(r2)
                            }
                        }
                        (Straight(r1), Straight(r2)) => {
                            if r1 < r2 {
                                best_hand = Straight(r1)
                            }
                            else {
                                best_hand = Straight(r2)
                            }
                        }
                        (ThreeOfAKind(r1), ThreeOfAKind(r2)) => {
                            if r1 < r2 {
                                best_hand = ThreeOfAKind(r1)
                            }
                            else {
                                best_hand = ThreeOfAKind(r2)
                            }
                        }
                        (TwoPair(r1, r2), TwoPair( r3, r4)) => {
                            let mut v1 = vec![r1, r2];
                            let mut v2 = vec![r3, r4];
                            v1.sort();
                            v2.sort();
                            if v1[0] < v2[0] {
                                best_hand = TwoPair(r1, r2)
                            }
                            else {
                                best_hand = TwoPair(r3, r4)
                            }
                        }
                        (Pair(r1), Pair(r2)) => {
                            if r1 < r2 {
                                best_hand = Pair(r1)
                            }
                            else {
                                best_hand = Pair(r2)
                            }
                        }
                        (HighCard(r1), HighCard(r2)) => {
                            if r1 < r2 {
                                best_hand = HighCard(r1)
                            }
                            else {
                                best_hand = HighCard(r2)
                            }
                        }
                        _ => {
                            best_hand = best_hands[0]
                        }
                    };

                    // increment games won if own hand won in the simulated round
                    if player_best_hand == best_hand {
                        response.games_won += 1;
                    }

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
            println!("Thread: {:10} won {:10} out of {:10} games in {:4}ms" , response.id.get(), response.games_won, response.games_played, response.time_spent)
        }
        // print global statistics
        println!("-------------------------------------------------------------------");
        print!("Total:             won {:10} out of {:10} games in ", total_games_won, total_games_played)
    }).unwrap();

    // return win estimation
    let chance: f64 = total_games_won as f64 / total_games_played as f64;
    return WinEstimation {chance, normalized: chance * hands.len() as f64}
}

pub struct ThreadResponse {
    id: NonZeroU64,
    games_played: u128,
    games_won: u64,
    time_spent: u128
}

pub struct WinEstimation {
    pub(crate) chance: f64,
    pub(crate) normalized: f64,
}