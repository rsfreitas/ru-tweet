
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// deleteTweet handler.
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
//  "code": int
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, session: State<RwLock<Session>>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;

    if message.from.is_empty() || message.id.is_empty() || message.name.is_empty() {
        code = 1; // invalid fields
    } else if !session.read().unwrap().is_logged_with_id(&message.from) {
        code = 2; // the user is not logged at the moment
    } else if !session.read().unwrap().is_id_from_user(&message.from, &message.name) {
        code = 3; // ID is not from user (name)
    } else if !db.delete_tweet(&message.name, &message.id) {
        code = 4; // database error
    }

    Json(Answer::new(code))
}

