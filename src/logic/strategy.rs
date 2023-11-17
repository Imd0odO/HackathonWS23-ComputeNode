use rocket::http::Status;
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
    for player_index in 0..table.players.len() {
        if max_bet < table.players[player_index].bet {
            max_bet = table.players[player_index].bet;
        }
    };

    // get min bet
    let min_bet: i32 = table.minimum_bet;

    // start Monte Carlo
    let estimation: WinEstimation = monte_carlo(table, active_player);

    // print estimated chance of winning
    println!("-> Estimated chance of winning this hand: {:2.2}%", estimation.chance * 100.0);

    // return minimum bet if first player
    if active_player == 0 {
        return Bet { bet: min_bet }
    }

    // return bet based on estimation
    return match estimation {
        // check if chance of winning < 50 % and max bet > 50 % of stack
        e if e.chance >= 0.20 && e.chance < 0.5 && max_bet < (stack / 2) => {
            return Bet { bet: max_bet };
        }

        // check if chance of winning > 50 % and < 75 %
        e if e.chance < 0.75 && e.chance > 0.50 => {
            if max_bet > stack {
                return Bet {bet: stack};
            }
            return Bet { bet: max_bet };
        }

        // raise 25 % if chance of winning > 75 & and < 90 %
        e if e.chance < 0.90 && e.chance >= 0.75 => {
            if max_bet > (5 * stack / 4) {
                return Bet {bet: stack};
            }
            return Bet { bet: 5 * max_bet / 4 };
        }

        // all in if chance of winning > 90 %
        e if e.chance >= 0.90 => {
            return Bet { bet: stack };
        }

        // fold
        _ => {
            Bet{bet: 0}
        }
    };
 }
