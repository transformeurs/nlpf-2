use neo4rs::{query, Node};

use crate::SharedState;

use super::models::Questionnaire;

/// Create a new candidate in the database and return it
pub async fn create_questionnaire(
    questionnaire: Questionnaire,
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
                id: $id
            })
            RETURN c
        "#,
            )
            .param("id", questionnaire_id.to_string())
            .param("name", questionnaire.name.clone()),
        )
        .await?;

    // Check if created, and log the name
    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("c").unwrap();
        let name: String = node.get("name").unwrap();
        tracing::info!("Created questionnaire: {name}");
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
                    id: $id,
                    question: $question
                })
                RETURN c
            "#,
                )
                .param("id", question_id.to_string())
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
                MATCH (q:Questionnaire {id: $id_questionnaire})
                MATCH (c:Question {id: $id_question})
                CREATE (q)-[:HAS_QUESTION]->(c)
                RETURN q, c
            "#,
                )
                .param("id_questionnaire", questionnaire_id.to_string())
                .param("id_question", question_id.to_string()),
            )
            .await?;

        // Check if created, and log the name
        while let Ok(Some(row)) = result_question.next().await {
            let node: Node = row.get("q").unwrap();
            let name: String = node.get("name").unwrap();
            let node: Node = row.get("c").unwrap();
            let question: String = node.get("question").unwrap();
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
                        id: $id,
                        answer: $answer,
                        is_valid: $is_valid
                    })
                    RETURN c
                "#,
                    )
                    .param("id", answer_id.to_string())
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
                    MATCH (q:Question {id: $id_question})
                    MATCH (c:Answer {id: $id_answer})
                    CREATE (q)-[:HAS_ANSWER]->(c)
                    RETURN q, c
                "#,
                    )
                    .param("id_question", question_id.to_string())
                    .param("id_answer", answer_id.to_string()),
                )
                .await?;

            // Check if created, and log the name
            while let Ok(Some(row)) = result_answer.next().await {
                let node: Node = row.get("q").unwrap();
                let question: String = node.get("question").unwrap();
                let node: Node = row.get("c").unwrap();
                let answer: String = node.get("answer").unwrap();
                tracing::info!("Created relation between {question_id} and {answer_id}");
            }
        }
    }

    Ok(questionnaire)
}
