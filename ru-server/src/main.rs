#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use(bson, doc)] extern crate mongodb;
extern crate uuid;

use std::sync::RwLock;

mod data;
mod handler;
mod database;

use data::session::Session;
use database::Database;

fn main() {
    /* Starts the database */
    let db = Database::create("localhost", 27017).unwrap();

    /* Creates the container to hold active users */
    let s = Session::create();

    /* Puts the server to run */
    rocket::ignite().mount("/addUser", routes![handler::add_user::handler])
                    .mount("/deleteUser", routes![handler::delete_user::handler])
                    .mount("/login", routes![handler::login::handler])
                    .mount("/logout", routes![handler::logout::handler])
                    .mount("/isLogged", routes![handler::is_logged::handler])
                    .mount("/addTweet", routes![handler::add_tweet::handler])
                    .mount("/deleteTweet", routes![handler::delete_tweet::handler])
                    .mount("/listTweet", routes![handler::list_tweet::handler])
                    .mount("/getTweet", routes![handler::get_tweet::handler])
                    .manage(RwLock::new(s))
                    .manage(db)
                    .launch();
}
