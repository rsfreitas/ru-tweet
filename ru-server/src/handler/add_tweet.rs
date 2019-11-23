
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// addTweet handler.
//
// Must receive:
// {
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

    if message.from.is_empty() || message.content.is_empty() {
        code = 1;
    } else if !session.read().unwrap().is_logged(&message.from) {
        code = 2;
    } else {
        match db.add_tweet(&message.from, &message.content) {
            None => code = 3,
            Some(tid) => id = tid
        };
    }

    Json(Answer::new_with_id(code, &id))
}

