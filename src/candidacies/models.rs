use std::collections::HashMap;

use neo4rs::Relation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_role: String,
    pub email: String,
}

pub struct Candidacy {
    pub uuid: uuid::Uuid,
    pub status: String,
    pub cover_letter_url: String,
    pub resume_url: String,
    pub custom_field: String,
}

impl Candidacy {
    /// Create from a hash map (from a form)
    pub fn from_hash_map(map: HashMap<String, String>, id: uuid::Uuid) -> Self {
        let uuid = id;
        let status = map.get("status").unwrap().clone();
        let cover_letter_url = map.get("cover_letter_url").unwrap().clone();
        let resume_url = map.get("resume_url").unwrap().clone();
        let custom_field = map.get("custom_field").unwrap().clone();

        Candidacy {
            uuid,
            status,
            cover_letter_url,
            resume_url,
            custom_field,
        }
    }

    pub fn from_relation(relation: Relation) -> Self {
        let uuid: uuid::Uuid =
            uuid::Uuid::parse_str(&(String::as_str(&relation.get("uuid").unwrap()))).unwrap();
        let status: String = relation.get("status").unwrap();
        let cover_letter_url: String = relation.get("cover_letter_url").unwrap();
        let resume_url: String = relation.get("resume_url").unwrap();
        let custom_field: String = relation.get("custom_field").unwrap();

        Candidacy {
            uuid,
            status,
            cover_letter_url,
            resume_url,
            custom_field,
        }
    }
}
