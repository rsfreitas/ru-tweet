
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::data::tweet::Tweet;
use crate::database::Database;

//
// getTweet handler.
//
// Must receive:
// {
//  "from": string,
//  "name": string,
//  "id": string
// }
//
// It will always return a 200 code with an internal code of what really
// happened. It has the following format:
//
// {
//  "code": int,
//  "tweet": Tweet,
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;
    let mut tweet = Tweet::new_empty();

    if message.from.is_empty() || message.id.is_empty() {
        code = 1;
    } else if !session.read().unwrap().is_logged_with_id(&message.from) {
        code = 2;
    } else {
        tweet = match db.get_tweet(&message.name, &message.id) {
            None => {
                code = 3;
                Tweet::new_empty()
            },
            Some(t) => t
        }
    }

    let answer = match code {
        0 => Answer::new_with_tweet(code, tweet),
        c @ _ => Answer::new(c)
    };

    Json(answer)
}

