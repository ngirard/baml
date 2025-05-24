use super::*;

const QUESTION_FILE: &str = r#"
enum Scope {
  GRANULAR
  ENCOMPASSING
}

class GeneratedQuestion {
  conceptualScope Scope
}

class QuestionGenerationResult {
  potentialQuestions GeneratedQuestion[]
}
"#;

test_deserializer!(
    test_question_generation_valid,
    QUESTION_FILE,
    r#"{"potentialQuestions": [{"conceptualScope": "GRANULAR"}]}"#,
    FieldType::Class("QuestionGenerationResult".to_string()),
    {
        "potentialQuestions": [{ "conceptualScope": "GRANULAR" }]
    }
);

test_failing_deserializer!(
    test_question_generation_invalid_enum,
    QUESTION_FILE,
    r#"{"potentialQuestions": [{"conceptualScope": "overview"}]}"#,
    FieldType::Class("QuestionGenerationResult".to_string())
);
