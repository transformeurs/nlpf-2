use std::collections::HashMap;

use async_session::chrono::{self, NaiveDate};
use neo4rs::Node;

#[derive(Debug)]
pub struct Offer {
    pub title: String,
    pub uuid: uuid::Uuid,
    pub description: String,
    pub created_at: NaiveDate,
    pub skills: Vec<String>,
    pub location: String,
    pub salary: i64,
    pub job_duration: String,
    pub job_start: String,
    pub questionnaire_id: Option<uuid::Uuid>,
}

impl Offer {
    /// Create from a hash map (from a form)
    pub fn from_hash_map(
        map: HashMap<String, String>,
        skills: Vec<String>,
        id: uuid::Uuid,
    ) -> Self {
        let title = map.get("title").unwrap().clone();
        let uuid = id;
        let description = map.get("description").unwrap().clone();
        let created_at = chrono::Utc::now().date_naive();
        let skills = skills;
        let location = map.get("location").unwrap().clone();
        let salary = map.get("salary").unwrap().clone().parse::<i64>().unwrap();
        let job_duration = map.get("job_duration").unwrap().clone();
        let job_start = map.get("job_start").unwrap().clone();

        let questionnaire_id = map.get("questionnaire").unwrap().clone();
        let questionnaire_id = if questionnaire_id == "0" {
            None
        } else {
            Some(uuid::Uuid::parse_str(&questionnaire_id).unwrap())
        };

        Offer {
            title,
            uuid,
            description,
            created_at,
            skills,
            location,
            salary,
            job_duration,
            job_start,
            questionnaire_id,
        }
    }

    pub fn from_node(node: Node) -> Offer {
        let title: String = node.get("title").unwrap();
        let uuid: uuid::Uuid =
            uuid::Uuid::parse_str(String::as_str(&node.get("uuid").unwrap())).unwrap();
        let description: String = node.get("description").unwrap();
        let created_at: NaiveDate = node.get("created_at").unwrap();
        let skills: Vec<String> = node.get("skills").unwrap();
        let location: String = node.get("location").unwrap();
        let salary: i64 = node.get("salary").unwrap();
        let job_duration: String = node.get("job_duration").unwrap();
        let job_start: String = node.get("job_start").unwrap();

        let questionnaire_id: String = node.get("questionnaire_id").unwrap();
        let questionnaire_id = if questionnaire_id == "0" {
            None
        } else {
            Some(uuid::Uuid::parse_str(&questionnaire_id).unwrap())
        };

        Offer {
            title,
            uuid,
            description,
            created_at,
            skills,
            location,
            salary,
            job_duration,
            job_start,
            questionnaire_id,
        }
    }
}
