
use rocket::State;
use rocket_contrib::json::Json;

use crate::data::message::Message;
use crate::data::answer::Answer;
use crate::database::Database;

//
// addUser handler.
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
// }
//
#[post("/", format = "application/json", data = "<message>")]
pub fn handler(message: Json<Message>, db: State<Database>) -> Json<Answer> {
    let mut code = 0;

    if message.name.is_empty() || message.password.is_empty() {
        code = 1; // invalid fields
    } else {
        match db.user_exists(&message.name) {
            true => code = 2, // the user already exists
            false => db.add_user(&message.name, &message.password)
        };
    }

    Json(Answer::new(code))
}

