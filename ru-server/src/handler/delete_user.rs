
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::message::Message;
use crate::data::session::Session;
use crate::data::answer::Answer;
use crate::database::Database;

//
// deleteUser handler.
//
// Must receive:
// {
//  "from": string,
//  "name": string
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

    if message.name.is_empty() || message.from.is_empty() {
        code = 1;
    } else if !session.read().unwrap().is_id_from_user(&message.from, &message.name) {
        code = 2;
    } else {
        /* Deletes the user session */
        session.write().unwrap().delete(&message.from);

        if !db.delete_user(&message.name) {
            code = 3;
        }
    }

    Json(Answer::new(code))
}

