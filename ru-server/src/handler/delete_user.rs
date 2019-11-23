
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

    if message.name.is_empty() {
        code = 1;
    } else {
        /* Gets the user session ID, if it's logged */
        let _ = match session.read().unwrap().get_id(&message.name) {
            None => (),
            Some(id) => {
                /* Deletes the user session */
                session.write().unwrap().delete(id);
            }
        };

        if !db.delete_user(&message.name) {
            code = 2;
        }
    }

    Json(Answer::new(code))
}

