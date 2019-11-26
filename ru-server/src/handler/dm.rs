
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// dm handler.
//
// Must receive:
// {
//  "from": string,
//  "to": string,
//  "content": string
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
    let mut id = String::new();

    if message.from.is_empty() || message.to.is_empty() || message.content.is_empty() {
        code = 1; // invalid fields
    } else if !session.read().unwrap().is_logged(&message.from) {
        code = 2; // the user is not logged
    } else if !db.user_exists(&message.to) {
        code = 3; // the destination does not exist
    } else {
        if let Some(blocked) = db.get_blocking(&message.to) {
            /*
             * If the sender is blocked in the receiver's list we won't send a
             * DM to him.
             */
            if blocked.contains(&message.from) {
                code = 4; // we're blocked, can't send (should the user know this?)
            }
        }

        if code == 0 {
            match db.add_dm(&message.from, &message.to, &message.content) {
                None => code = 5, // database insertion error
                Some(tid) => id = tid
            };
        }
    }

    Json(Answer::new_with_id(code, &id))
}

