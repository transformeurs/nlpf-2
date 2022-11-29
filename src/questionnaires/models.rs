use std::{collections::HashMap, num};

use bcrypt::DEFAULT_COST;
use neo4rs::Node;

pub struct QuestionnaireQuestionAnswer {
    pub uuid: String,
    pub answer: String,
    pub is_valid: bool,
}

pub struct QuestionnaireQuestion {
    pub uuid: String,
    pub question: String,
    pub answers: Vec<QuestionnaireQuestionAnswer>,
}

pub struct Questionnaire {
    pub uuid: String,
    pub name: String,
    pub questions: Vec<QuestionnaireQuestion>,
}

impl Questionnaire {
    /// Create from a hash map (from a form)
    pub fn from_hash_map(map: HashMap<String, String>) -> Self {
        let name = map.get("name").unwrap().clone();

        let mut questions = Vec::new();

        // Get the questions
        for (key, value) in map.iter() {
            if key.starts_with("question") {
                let question_id: i32 = key.split("-").collect::<Vec<&str>>()[1].parse().unwrap();
                let mut question = QuestionnaireQuestion {
                    uuid: String::new(),
                    question: value.clone(),
                    answers: Vec::new(),
                };

                // Get the answers
                for (key, value) in map.iter() {
                    let start_with = format!("answer-{}", question_id);
                    if key.starts_with(start_with.as_str()) {
                        let answer_id: i32 =
                            key.split("-").collect::<Vec<&str>>()[2].parse().unwrap();

                        // Get the answer validity
                        let mut answer_validity_bool: bool = false;
                        let answer_validity =
                            map.get(format!("validity-{}-{}", question_id, answer_id).as_str());

                        if let Some(answer_validity) = answer_validity {
                            answer_validity_bool = answer_validity == "on";
                        }

                        let answer = QuestionnaireQuestionAnswer {
                            uuid: String::new(),
                            answer: value.clone(),
                            is_valid: answer_validity_bool,
                        };
                        question.answers.push(answer);
                    }
                }

                questions.push(question);
            }
        }

        Questionnaire {
            uuid: String::new(),
            name,
            questions,
        }
    }
}
