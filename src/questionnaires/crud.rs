use neo4rs::{query, Node};

use super::models::Questionnaire;
use crate::{
    questionnaires::models::{QuestionnaireQuestion, QuestionnaireQuestionAnswer},
    SharedState,
};

/// Create a new candidate in the database and return it
pub async fn create_questionnaire(
    questionnaire: Questionnaire,
    company_email: String,
    state: SharedState,
) -> Result<Questionnaire, neo4rs::Error> {
    tracing::info!("Creating questionnaire: {}", &questionnaire.name);

    // Insert questionnaire in the database
    let questionnaire_id = uuid::Uuid::new_v4();
    let mut result = state
        .graph
        .execute(
            query(
                r#"
            CREATE (c:Questionnaire {
                name: $name,
                uuid: $uuid
            })
            RETURN c
        "#,
            )
            .param("uuid", questionnaire_id.to_string())
            .param("name", questionnaire.name.clone()),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Created questionnaire: {name}");
    }

    // Create relationship between questionnaire and company
    let mut result_relationship_questionnaire_company = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Company {email: $company_email})
            MATCH (q:Questionnaire {uuid: $questionnaire_id})
            CREATE (c)-[:HAS_QUESTIONNAIRE]->(q)
        "#,
            )
            .param("company_email", company_email)
            .param("questionnaire_id", questionnaire_id.to_string()),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(_)) = result_relationship_questionnaire_company.next().await {
        tracing::info!("Created relation between {} and ?", questionnaire_id);
    }

    // Insert questions in the database
    let slice = &questionnaire.questions[..];
    for question in slice.iter().clone() {
        let question_id = uuid::Uuid::new_v4();
        let mut result_question = state
            .graph
            .execute(
                query(
                    r#"
                CREATE (c:Question {
                    uuid: $uuid,
                    question: $question
                })
                RETURN c
            "#,
                )
                .param("uuid", question_id.to_string())
                .param("question", question.question.clone()),
            )
            .await?;

        // Check if created, and log the name
        while let Ok(Some(row)) = result_question.next().await {
            let node: Node = row.get("c").unwrap();
            let question: String = node.get("question").unwrap();
            tracing::info!("Created question: {question}");
        }

        // Create relationship between questionnaire and question
        let mut result_question = state
            .graph
            .execute(
                query(
                    r#"
                MATCH (q:Questionnaire {uuid: $id_questionnaire})
                MATCH (c:Question {uuid: $id_question})
                CREATE (q)-[:HAS_QUESTION]->(c)
                RETURN q, c
            "#,
                )
                .param("id_questionnaire", questionnaire_id.to_string())
                .param("id_question", question_id.to_string()),
            )
            .await?;

        // Check if created, and log the name
        while let Ok(Some(_)) = result_question.next().await {
            tracing::info!("Created relation between {questionnaire_id} and {question_id}");
        }

        // Insert answers in the database
        let slice = &question.answers[..];
        for answer in slice.iter().clone() {
            let answer_id = uuid::Uuid::new_v4();
            let mut result_answer = state
                .graph
                .execute(
                    query(
                        r#"
                    CREATE (c:Answer {
                        uuid: $uuid,
                        answer: $answer,
                        is_valid: $is_valid
                    })
                    RETURN c
                "#,
                    )
                    .param("uuid", answer_id.to_string())
                    .param("answer", answer.answer.clone())
                    .param("is_valid", answer.is_valid.to_string()),
                )
                .await?;

            // Check if created, and log the name
            while let Ok(Some(row)) = result_answer.next().await {
                let node: Node = row.get("c").unwrap();
                let answer: String = node.get("answer").unwrap();
                tracing::info!("Created answer: {answer}");
            }

            // Create relationship between question and answer
            let mut result_answer = state
                .graph
                .execute(
                    query(
                        r#"
                    MATCH (q:Question {uuid: $id_question})
                    MATCH (c:Answer {uuid: $id_answer})
                    CREATE (q)-[:HAS_ANSWER]->(c)
                    RETURN q, c
                "#,
                    )
                    .param("id_question", question_id.to_string())
                    .param("id_answer", answer_id.to_string()),
                )
                .await?;

            // Check if created, and log the name
            while let Ok(Some(_)) = result_answer.next().await {
                tracing::info!("Created relation between {question_id} and {answer_id}");
            }
        }
    }

    Ok(questionnaire)
}

/// Get a questionnaires by company
pub async fn get_questionnaires_by_company_email(
    email: String,
    state: SharedState,
) -> Result<Vec<Questionnaire>, neo4rs::Error> {
    tracing::info!("Getting questionnaires by company email: {}", email);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (c:Company {email:$email})
            MATCH (c)-[POSTED]-(q:Questionnaire)
            RETURN q
        "#,
            )
            .param("email", email.to_string()),
        )
        .await?;

    let mut questionnaires: Vec<Questionnaire> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("q").unwrap();
        let uuid: String = node.get("uuid").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Found questionnaire: {name}");
        questionnaires.push(get_questionnaire_by_id(uuid, state.clone()).await?.unwrap());
    }

    Ok(questionnaires)
}

/// Delete a questionnaire by id
pub async fn delete_questionnaire_by_id(
    questionnaire_id: String,
    state: SharedState,
) -> Result<(), neo4rs::Error> {
    tracing::info!("Deleting questionnaire by id: {}", questionnaire_id);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (q:Questionnaire {uuid:$uuid})
            DETACH DELETE q
            RETURN q
        "#,
            )
            .param("uuid", questionnaire_id.to_string()),
        )
        .await?;

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("q").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Deleted questionnaire: {name}");
    }

    Ok(())
}

/// Get a questionnaire by id
pub async fn get_questionnaire_by_id(
    questionnaire_id: String,
    state: SharedState,
) -> Result<Option<Questionnaire>, neo4rs::Error> {
    tracing::info!("Getting questionnaire by id: {}", questionnaire_id);

    let mut result = state
        .graph
        .execute(
            query(
                r#"
            MATCH (q:Questionnaire {uuid:$uuid})
            RETURN q
        "#,
            )
            .param("uuid", questionnaire_id.to_string()),
        )
        .await?;

    if let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("q").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Found questionnaire: {name} Get questions by questionnaire id");

        let uuid: String = node.get("uuid").unwrap();
        let name: String = node.get("name").unwrap();

        let mut questions: Vec<QuestionnaireQuestion> = Vec::new();

        // Get questions of the questionnaire
        let mut result_question = state
            .graph
            .execute(
                query(
                    r#"
                MATCH (q:Questionnaire {uuid:$uuid})
                MATCH (q)-[HAS_QUESTION]-(c:Question)
                RETURN c
            "#,
                )
                .param("uuid", questionnaire_id.to_string()),
            )
            .await?;

        while let Ok(Some(row)) = result_question.next().await {
            let node: Node = row.get("c").unwrap();
            let question: String = node.get("question").unwrap();
            tracing::info!("Found question: {question}");

            let uuid: String = node.get("uuid").unwrap();
            let question: String = node.get("question").unwrap();

            let mut answers: Vec<QuestionnaireQuestionAnswer> = Vec::new();

            // Get answers of the question
            let mut result_answer = state
                .graph
                .execute(
                    query(
                        r#"
                    MATCH (q:Question {uuid:$uuid})
                    MATCH (q)-[HAS_ANSWER]-(c:Answer)
                    RETURN c
                "#,
                    )
                    .param("uuid", uuid.to_string()),
                )
                .await?;

            while let Ok(Some(row)) = result_answer.next().await {
                let node: Node = row.get("c").unwrap();
                let answer: String = node.get("answer").unwrap();
                tracing::info!("Found answer: {answer}");

                let uuid: String = node.get("uuid").unwrap();
                let answer: String = node.get("answer").unwrap();
                let is_valid_string: String = node.get("is_valid").unwrap();

                let mut is_valid: bool = false;
                if is_valid_string == "true" {
                    is_valid = true;
                }

                answers.push(QuestionnaireQuestionAnswer {
                    uuid,
                    answer,
                    is_valid,
                });
            }

            questions.push(QuestionnaireQuestion {
                uuid,
                question,
                answers,
            });
        }

        return Ok(Some(Questionnaire {
            uuid,
            name,
            questions,
        }));
    }

    Ok(None)
}

/// Compute questionnaire score
pub async fn compute_questionnaire_score(
    input: Vec<String>,
    state: SharedState,
) -> Result<u64, neo4rs::Error> {
    tracing::info!("Computing questionnaire score");

    let mut questions: u64 = 0;
    let mut valid_answers: u64 = 0;

    for answer_id in input {
        tracing::info!("Answer id: {}", answer_id);

        questions += 1;

        let mut result = state
            .graph
            .execute(
                query(
                    r#"
                MATCH (a:Answer {uuid:$answer_id})
                RETURN a
            "#,
                )
                .param("answer_id", answer_id.to_string()),
            )
            .await?;

        while let Ok(Some(row)) = result.next().await {
            let node: Node = row.get("a").unwrap();
            let is_valid_answer: String = node.get("is_valid").unwrap();

            let mut is_valid: bool = false;
            if is_valid_answer == "true" {
                is_valid = true;
            }

            if is_valid {
                valid_answers += 1;
            }
        }
    }

    if questions == 0 {
        return Ok(0);
    }
    Ok(((valid_answers as f64 / questions as f64) * 100f64) as u64)
}
