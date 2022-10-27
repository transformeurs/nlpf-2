use std::collections::HashMap;

use bcrypt::DEFAULT_COST;

pub struct Candidate {
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: i64,
    pub photo_url: String,
    pub description: String,
}

impl Candidate {
    /// Create from a hash map (from a form)
    pub fn from_hash_map(map: HashMap<String, String>) -> Self {
        let name = map.get("username").unwrap().clone();
        let email = map.get("email").unwrap().clone();
        let password = map.get("password").unwrap().clone();
        let age = map.get("age").unwrap().clone().parse::<i64>().unwrap();
        let photo_url = map.get("photo_url").unwrap().clone();
        let description = map.get("description").unwrap().clone();

        // Hash the password
        let password = bcrypt::hash(&password, DEFAULT_COST).unwrap();

        Candidate {
            name,
            email,
            password,
            age,
            photo_url,
            description,
        }
    }

    // TODO methods for password comparison
}
