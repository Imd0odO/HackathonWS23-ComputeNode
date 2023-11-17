use rocket::serde::json::Json;
use crate::logic::monte_carlo::monte_carlo;


pub fn decide(table: Json<crate::models::table::Table>) -> crate::models::bet::Bet {
    // TODO: Add Poker Logic Here... :)
    monte_carlo(table);

    return crate::models::bet::Bet{bet: 0}
 }
