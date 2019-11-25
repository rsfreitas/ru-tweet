
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// like handler.
//
// Must receive:
// {
//  "from": string,
//  "id": string
// }
//
// It will always return a 200 code with an internal code of what really
// happened. It has the following format:
//
// {
//  "code": int
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;

    if message.from.is_empty() || message.id.is_empty() {
        code = 1;
    } else if !session.read().unwrap().is_logged_with_id(&message.from) {
        code = 2;
    } else if !db.increment_tweet_like(&message.id) {
        code = 3;
    }

    Json(Answer::new(code))
}

