
#[derive(Debug)]
pub struct User {
    pub name: String,
    pub token: String,
}

impl User {
    pub fn new(name: &str) -> User {
        User{
            name: name.to_string(),
            token: "".to_string(),
        }
    }

    pub fn new_with_token(name: &str, token: &str) -> User {
        User{
            name: name.to_string(),
            token: token.to_string(),
        }
    }
}

