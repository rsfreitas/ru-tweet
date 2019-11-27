
use serde_json;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

const HOST: &'static str = "https://fcm.googleapis.com/fcm/send";
const KEY: &'static str = "put-yout-FCM-key-here";

pub struct Notify {
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Android {
    priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NotifyData {
    to: String,
    priority: u32,
    android: Android,
    data: Data,
}

impl Notify {
    pub fn send(token: &str, command: &str) -> bool {
        let client = reqwest::blocking::Client::new();
        let body = NotifyData{
            to: token.to_string(),
            priority: 10,
            android: Android{
                priority: "high".to_string(),
            },
            data: Data{
                command: command.to_string(),
            },
        };

        let key = format!("key={}", KEY);
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&key).unwrap());
        let res = client.post(HOST)
                        .body(serde_json::to_string(&body).unwrap())
                        .headers(headers)
                        .send();

        match res {
            Err(_) => false,
            Ok(r) => r.status().is_success()
        }
    }
}

