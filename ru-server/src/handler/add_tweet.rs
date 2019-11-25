
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::data::tweet::Kind;
use crate::database::Database;

//
// addTweet handler.
//
// Must receive:
// {
//  "name": string,
//  "from": string,
//  "content": string,
// }
//
// It will always return a 200 code with an internal code of what really
// happened. It has the following format:
//
// {
//  "code": int,
//  "id": string
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;
    let mut id = String::new();

    if message.name.is_empty() || message.content.is_empty() || message.from.is_empty() {
        code = 1; // invalid fields
    } else if !session.read().unwrap().is_id_from_user(&message.from, &message.name) {
        code = 2; // ID is not from user (name)
    } else if !session.read().unwrap().is_logged(&message.name) {
        code = 3; // the user is not logged
    } else {
        match db.add_tweet(&message.name, &message.content, Kind::Simple) {
            None => code = 4, // database insertion error
            Some(tid) => id = tid
        };
    }

    Json(Answer::new_with_id(code, &id))
}

