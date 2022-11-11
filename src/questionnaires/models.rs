use bcrypt::DEFAULT_COST;
use neo4rs::Node;
use std::collections::HashMap;

pub struct QuestionnaireQuestionAnswer {
    pub answer: String,
    pub is_valid: bool,
}

pub struct QuestionnaireQuestion {
    pub question: String,
    pub answers: Vec<QuestionnaireQuestionAnswer>,
}

pub struct Questionnaire {
    pub name: String,
    pub questions: Vec<QuestionnaireQuestion>,
}

impl Questionnaire {
    /// Create from a hash map (from a form)
    pub fn from_hash_map(map: HashMap<String, String>) -> Self {
        let name = map.get("name").unwrap().clone();

        let mut questions = Vec::new();

        let email = map.get("email").unwrap().clone();
        let password = map.get("password").unwrap().clone();
        let age = map.get("age").unwrap().clone().parse::<i64>().unwrap();
        let photo_url = map.get("photo_url").unwrap().clone();
        let description = map.get("description").unwrap().clone();

        Questionnaire { name, questions }
    }

    pub fn from_node(node: Node) -> Self {
        let name: String = node.get("name").unwrap();
        let email: String = node.get("email").unwrap();
        let password: String = node.get("password").unwrap();
        let age: i64 = node.get("age").unwrap();
        let photo_url: String = node.get("photo_url").unwrap();
        let description: String = node.get("description").unwrap();

        Candidate {
            name,
            email,
            password,
            age,
            photo_url,
            description,
        }
    }
}
