use rocket::serde::json::Json;
use crate::logic::gamethread::WinEstimation;
use crate::logic::monte_carlo::monte_carlo;


pub fn decide(table: Json<crate::models::table::Table>) -> crate::models::bet::Bet {
    // clear terminal for stats
    print!("\x1B[2J\x1B[1;1H");

    // start Monte Carlo
    let estimation: WinEstimation = monte_carlo(table);

    // print estimated chance of winning
    println!("-> Estimated chance of winning this hand: {:2.2}%", estimation.chance * 100.0);

    return crate::models::bet::Bet{bet: 0};
 }
