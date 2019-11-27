
use std::sync::RwLock;

use rocket::State;
use rocket_contrib::json::Json;

use crate::data::session::Session;
use crate::data::message::Message;
use crate::data::answer::Answer;

//
// token handler.
//
// Must receive:
// {
//  "from": string,
//  "token:": string
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

    if message.from.is_empty() || message.token.is_empty() {
        code = 1; // invalid fields
    } else if !session.read().unwrap().is_logged_with_id(&message.from) {
        code = 2; // the user is not logged at the moment
    } else {
        let name = session.read().unwrap().get_username(&message.from).unwrap();
        session.write().unwrap().set_user_token(&message.from, &name, &message.token);
    }

    Json(Answer::new(code))
}

