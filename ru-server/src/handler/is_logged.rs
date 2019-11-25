
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;

//
// is_logged handler.
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
        code = 1;
    }

    if !session.read().unwrap().is_logged_with_id(&message.id) {
        code = 2;
    }

    Json(Answer::new(code))
}

