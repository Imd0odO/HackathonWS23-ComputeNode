#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use crate::models::table::Table;
use crate::models::bet::Bet;
use crate::logic::strategy::decide;

mod models;
mod logic;

#[post("/", format = "json", data = "<table>")]
fn index(table: Json<Table>) -> Json<Bet> {
    Json(decide(table))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}