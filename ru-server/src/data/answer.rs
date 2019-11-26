
use super::tweet::Tweet;
use super::dm::DM;

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub code: u32,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub tweets: Vec<Tweet>,

    #[serde(default)]
    pub following: Vec<String>,

    #[serde(default)]
    pub blocked: Vec<String>,

    #[serde(default)]
    pub dms: Vec<DM>,
}

impl Answer {
    pub fn new(code: u32) -> Answer {
        Answer {
            code: code,
            id: "".to_string(),
            tweets: Vec::new(),
            following: Vec::new(),
            blocked: Vec::new(),
            dms: Vec::new(),
        }
    }

    pub fn new_with_id(code: u32, id: &str) -> Answer {
        Answer {
            code: code,
            id: id.to_string(),
            tweets: Vec::new(),
            following: Vec::new(),
            blocked: Vec::new(),
            dms: Vec::new(),
        }
    }

    pub fn new_with_tweets(code: u32, tweets: Vec<Tweet>) -> Answer {
        Answer {
            code: code,
            id: "".to_string(),
            tweets: tweets,
            following: Vec::new(),
            blocked: Vec::new(),
            dms: Vec::new(),
        }
    }

    pub fn new_with_tweet(code: u32, tweet: Tweet) -> Answer {
        Answer {
            code: code,
            id: "".to_string(),
            tweets: vec![tweet],
            following: Vec::new(),
            blocked: Vec::new(),
            dms: Vec::new(),
        }
    }

    pub fn new_info(id: &str, code: u32, following: Vec<String>, blocked: Vec<String>) -> Answer {
        Answer {
            code: code,
            id: id.to_string(),
            tweets: Vec::new(),
            following: following,
            blocked: blocked,
            dms: Vec::new(),
        }
    }

    pub fn new_with_dms(code: u32, dms: Vec<DM>) -> Answer {
        Answer {
            code: code,
            id: "".to_string(),
            tweets: Vec::new(),
            following: Vec::new(),
            blocked: Vec::new(),
            dms: dms,
        }
    }
}

