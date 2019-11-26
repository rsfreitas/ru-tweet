
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DM {
    from: String,
    to: String,
    content: String,
    timestamp: String,
    id: String,
}

impl DM {
    pub fn new(from: &str, to: &str, content: &str, timestamp: &str, id: &str) -> DM {
        DM{
            from: from.to_string(),
            to: to.to_string(),
            content: content.to_string(),
            timestamp: timestamp.to_string(),
            id: id.to_string(),
        }
    }
}

