
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
//  "name": string,
//  "from": string (optional)
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

    if message.name.is_empty() {
        code = 1; // invalid fields
    } else if !session.read().unwrap().is_logged(&message.name) {
        code = 2; // the user is not logged at the moment
    } else {
        if !message.from.is_empty() {
            /*
             * We can list messages from another user on if he is not blocking
             * us.
             */
            if let Some(blocked) = db.get_blocking(&message.from) {
                if blocked.contains(&message.name) {
                    code = 3; // we're blocked, can't list messages (should the user know this?)
                }
            }
        }

        /*
         * We can list the tweets of the own user or tweets from another one. To
         * do this we must receive "from" with the name of this user.
         */
        if code == 0 {
            let s = if !message.from.is_empty() { &message.from } else { &message.name };
            tweets = db.list_tweet(&s);
        }
    }

    let answer = match code {
        0 => Answer::new_with_tweets(code, tweets),
        c @ _ => Answer::new(c)
    };

    Json(answer)
}

