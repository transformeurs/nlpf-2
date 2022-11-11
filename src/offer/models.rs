use std::collections::HashMap;

use async_session::chrono::{self, NaiveDate};
use neo4rs::Node;

// Maybe include the company in the offer?
pub struct Offer {
    title: String,
    description: String,
    created_at: NaiveDate,
    skills: Vec<String>,
    location: String,
    salary: i64,
    job_duration: String,
    job_start: String,
}

impl Offer {
    /// Create from a hash map (from a form)
    pub fn from_hash_map(map: HashMap<String, String>, skills: Vec<String>) -> Self {
        let title = map.get("title").unwrap().clone();
        let description = map.get("description").unwrap().clone();
        let created_at = chrono::Utc::now().date_naive();
        let skills = skills;
        let location = map.get("location").unwrap().clone();
        let salary = map.get("salary").unwrap().clone().parse::<i64>().unwrap();
        let job_duration = map.get("job_duration").unwrap().clone();
        let job_start = map.get("job_start").unwrap().clone();

        Offer {
            title,
            description,
            created_at,
            skills,
            location,
            salary,
            job_duration,
            job_start,
        }
    }

    // Didn't understand thw purpose of this function

    // pub fn from_node(node: Node) -> Self {
    //     let name: String = node.get("name").unwrap();
    //     let email: String = node.get("email").unwrap();
    //     let password: String = node.get("password").unwrap();
    //     let age: i64 = node.get("age").unwrap();
    //     let photo_url: String = node.get("photo_url").unwrap();
    //     let description: String = node.get("description").unwrap();

    //     Offer {
    //         name,
    //         email,
    //         password,
    //         age,
    //         photo_url,
    //         description,
    //     }
    // }
}
