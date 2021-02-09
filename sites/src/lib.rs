#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

mod errors;
mod routes;

use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api/v1",
            routes![
                routes::users::post_users,
            ],
        )
        .attach(config::AppState::manage())
        .register(catchers![not_found])
}