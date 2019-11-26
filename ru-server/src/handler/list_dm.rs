
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// listDm handler.
//
// Must receive:
// {
//  "from": string,
//  "to": string,
// }
//
// It will always return a 200 code with an internal code of what really
// happened. It has the following format:
//
// {
//  "code": int
//  "dms": array of DMs
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;
    let mut dms = vec![];

    if message.from.is_empty() || message.to.is_empty() {
        code = 1; // invalid fields
    } else if !session.read().unwrap().is_logged(&message.from) {
        code = 2; // the user is not logged at the moment
    } else if !db.user_exists(&message.to) {
        code = 3; // the destination user does not exist
    } else {
        let from = db.list_dm(&message.from, &message.to);
        let to = db.list_dm(&message.to, &message.from);
        dms = [&from[..], &to[..]].concat();
    }

    let answer = match code {
        0 => Answer::new_with_dms(code, dms),
        c @ _ => Answer::new(c)
    };

    Json(answer)
}

