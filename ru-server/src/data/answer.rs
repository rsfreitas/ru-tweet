
use super::tweet::Tweet;

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub code: u32,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub tweets: Vec<Tweet>,
}

impl Answer {
    pub fn new(code: u32) -> Answer {
        Answer{
            code: code,
            id: "".to_string(),
            tweets: Vec::new(),
        }
    }

    pub fn new_with_id(code: u32, id: &str) -> Answer {
        Answer{
            code: code,
            id: id.to_string(),
            tweets: Vec::new(),
        }
    }

    pub fn new_with_tweets(code: u32, tweets: Vec<Tweet>) -> Answer {
        Answer{
            code: code,
            id: "".to_string(),
            tweets: tweets,
        }
    }
}

