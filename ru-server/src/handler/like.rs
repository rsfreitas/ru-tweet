
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;
use crate::notification::Notify;

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
        code = 1; // invalid fields
    } else if !session.read().unwrap().is_logged_with_id(&message.from) {
        code = 2; // the user is not logged at the moment
    } else {
        let s = session.read().unwrap();
        let session_user = s.get_username(&message.from).unwrap();

        match db.get_username_from_message(&message.id) {
            None => code = 3, // original author not found
            Some(username) => {
                if session_user.eq(&username) {
                    code = 4; // the original author is trying to like his message
                } else if !db.increment_tweet_like(&message.id) {
                    code = 5; // database error
                } else {
                    let id = s.get_id(&username).unwrap();

                    if let Some(token) = s.get_token(&id) {
                        Notify::send(&token, "like");
                    }
                }
            }
        }
    }

    Json(Answer::new(code))
}

