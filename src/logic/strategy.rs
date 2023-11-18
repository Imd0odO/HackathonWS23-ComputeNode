use rocket::serde::json::Json;
use crate::logic::gamethread::WinEstimation;
use crate::logic::monte_carlo::monte_carlo;
use crate::models::bet::Bet;
use crate::models::player::PlayerStatusEnum;


pub fn decide(mut table: Json<crate::models::table::Table>) -> crate::models::bet::Bet {
    // get player count
    let player_count : usize = table.players.len();

    // remove inactive players
    table.players.retain(|player| player.status != PlayerStatusEnum::OUT);

    // get active player
    let active_player: usize = (table.active_player - ((player_count - table.players.len())) as i32) as usize;

    // get own stack
    let stack: i32 = table.players[active_player].stack;

    // get max bet
    let mut max_bet: i32 = 0;
    let mut all_in: bool = false;
    for player_index in 0..table.players.len() {
        if max_bet < table.players[player_index].bet {
            max_bet = table.players[player_index].bet;
        }
        if table.players[player_index].bet >= stack && table.players.len() > 5 {
            all_in = true
        }
    };
    if all_in {max_bet = 0}

    // get min bet
    let min_bet: i32 = table.minimum_bet;

    // start Monte Carlo
    let estimation: WinEstimation = monte_carlo(table, active_player);

    // print estimated chance of winning
    println!("-> Estimated chance of winning this hand: {:2.2}%", estimation.chance * 100.0);
    println!("-> Relative chance of winning this hand:  {:2.2}", estimation.normalized);

    // return minimum bet if first player
    if active_player == 0 {
        return Bet { bet: min_bet }
    }

    // return bet based on estimation
    if player_count > 3 {
        return match estimation.normalized {
            // all in
            e if e >= 2.5 => {
                return Bet{bet: stack}
            }

            // raise
            e if e >= 1.5 && e < 2.5 => {
                if max_bet as f64 * 1.25 < stack as f64 {max_bet = (max_bet as f64 * 1.25) as i32}
                return Bet{bet: max_bet}
            }

            // check
            e if e < 1.5 && e >= 0.75 => {
                return Bet{bet: max_bet}
            }

            // fold
            _ => {
                Bet{bet: 0}
            }
        }
    }
    else if player_count == 3 {
        return match estimation.chance {
            // all in
            e if e >= 0.80 => {
                return Bet{bet: stack}
            }

            // raise
            e if e >= 0.60 && e < 0.80 => {
                if max_bet as f64 * 1.25 < stack as f64 {max_bet = (max_bet as f64 * 1.25) as i32}
                return Bet{bet: max_bet}
            }

            // check
            e if e < 0.60 && e >= 0.30 => {
                return Bet{bet: max_bet}
            }

            // fold
            _ => {
                Bet{bet: 0}
            }
        }
    }
    else {
        return match estimation.chance {
            // all in
            e if e >= 0.80 => {
                return Bet{bet: stack}
            }

            // raise
            e if e >= 0.55 && e < 0.80 => {
                if max_bet as f64 * 1.25 < stack as f64 {max_bet = (max_bet as f64 * 1.25) as i32}
                return Bet{bet: max_bet}
            }

            // check
            e if e < 0.55 && e >= 0.45 => {
                return Bet{bet: max_bet}
            }

            // fold
            _ => {
                Bet{bet: 0}
            }
        }
    }
 }
