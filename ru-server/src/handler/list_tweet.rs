
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// listTweet handler.
//
// Must receive:
// {
//  "from": string
// }
//
// It will always return a 200 code with an internal code of what really
// happened. It has the following format:
//
// {
//  "code": int,
//  "tweets": array of Tweet,
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;
    let mut tweets = vec![];

    if message.from.is_empty() {
        code = 1;
    } else if !session.read().unwrap().is_logged(&message.from) {
        code = 2;
    } else {
        tweets = db.list_tweet(&message.from);
    }

    let answer = match code {
        0 => Answer::new_with_tweets(code, tweets),
        c @ _ => Answer::new(c)
    };

    Json(answer)
}

