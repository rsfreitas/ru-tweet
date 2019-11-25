
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// login handler.
//
// Must receive:
// {
//  "name": string,
//  "password": string
// }
//
// It will always return a 200 code with an internal code of what really
// happened. It has the following format:
//
// {
//  "code": int
//  "id": string
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;
    let mut id = String::new();
    let mut session = session.write().unwrap();

    if message.name.is_empty() || !db.check_user_and_password(&message.name, &message.password) {
        code = 1;
    } else {
        /* If the user is already logged we return 0 and its ID */
        if session.is_logged(&message.name) {
            match session.get_id(&message.name) {
                Some(sid) => id = sid,
                None => code = 2
            };
        } else {
            match session.add(&message.name) {
                Some(sid) => id = sid,
                None => code = 3
            };
        }
    }

    Json(Answer::new_with_id(code, &id))
}

