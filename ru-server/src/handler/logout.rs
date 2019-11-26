
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;

//
// logout handler.
//
// Must receive:
// {
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
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>) -> Json<Answer> {
    let mut code = 0;

    if message.id.is_empty() {
        code = 1; // invalid fields
    } else  if !session.write().unwrap().delete(&message.id) {
        code = 2; // the user is not logged at the moment
    }

    Json(Answer::new(code))
}

