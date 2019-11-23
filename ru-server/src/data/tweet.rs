
#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub from: String,
    pub content: String,

    id: String,
    timestamp: String,
}

impl Tweet {
    pub fn new(from: &str, content: &str, id: &str) -> Tweet {
        Tweet{
            from: from.to_string(),
            content: content.to_string(),
            id: id.to_string(),
            timestamp: "".to_string(),
        }
    }

    pub fn new_empty() -> Tweet {
        Tweet{
            from: "".to_string(),
            content: "".to_string(),
            id: "".to_string(),
            timestamp: "".to_string(),
        }
    }
}

