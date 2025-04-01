use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::Thing;

fn deserialize_id<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let thing = Thing::deserialize(deserializer)?;
    if let surrealdb::sql::Id::Number(num) = thing.id {
        Ok(num)
    } else {
        Err(serde::de::Error::custom("Expected a numeric ID"))
    }
}

/// when deserializing and the id is not exist the default function will handle it!
///
/// but when deserializing and the id exists the default functionality does not execute

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OChat {
    #[serde(deserialize_with = "deserialize_id")]
    id: i64,
    name: String,
    #[serde(skip_deserializing)]
    summary: String,
}

impl OChat {
    pub fn new() -> Self {
        Self {
            id: Local::now().timestamp_millis(),
            name: "new chat".to_owned(),
            summary: "".to_owned(),
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn get_time_creation(&self) -> String {
        let t: DateTime<Local> = Local.timestamp_opt(self.id, 0).unwrap();
        t.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OMessage {
    #[serde(deserialize_with = "deserialize_id")]
    id: i64,
    message: String,
    response: String,
}
impl OMessage {
    pub fn new() -> Self {
        Self {
            id: Local::now().timestamp(),
            message: "".to_string(),
            response: "".to_owned(),
        }
    }
    pub fn add_message(&mut self, m: &str) {
        self.message = m.to_owned();
    }
    pub fn add_response(&mut self, r: &str) {
        self.response = r.to_owned();
    }
    pub fn get_time_creation(&self) -> String {
        let t: DateTime<Local> = Local.timestamp_opt(self.id, 0).unwrap();
        t.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn response(&self) -> &str {
        &self.response
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
}

//------------------------tests--------------

#[cfg(test)]
mod quick_test {
    use chrono::Local;

    use super::{OChat, OMessage};

    #[test]
    fn id_when_deserialize() {
        let s = serde_json::from_str::<OChat>(
            r#"{"id":5,"name": "Rust Chat", "summary": "A discussion about Rust"}"#,
        )
        .unwrap();
        assert_eq!(s.id, 5)
    }
    #[test]
    fn no_id_when_deserialize() {
        let t = Local::now().timestamp_millis();
        let s = serde_json::from_str::<OChat>(
            r#"{"name": "Rust Chat", "summary": "A discussion about Rust"}"#,
        )
        .unwrap();
        //dbg!(&t);
        //dbg!(&s.id);
        assert!(t <= s.id)
    }

    #[test]
    fn check_message_creation_date() {
        let m = OMessage {
            id: 1743073118,
            message: "".to_owned(),
            response: "".to_owned(),
        };
        assert_eq!(m.get_time_creation(), "2025-03-27 13:58:38")
    }
}
