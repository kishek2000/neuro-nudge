//! This module defines the types used by the recommendation engine, NeuroNudge.
//!
//! The overall interaction between the domains is as follows - a learner attempts
//! and completes their lesson plan, with a certain level of success/progress. Based
//! on their performance in this lesson plan, their future lesson plans and scores for
//! different areas of study are determined. The goal is to master areas of study, from
//! basic to advanced, and to do so in a way that is sensitive to the learner's needs.
//!
//! For example, if in a lesson about Shapes a child is able to identify a square but
//! not a circle, then the next lesson for Shapes in the next lesson plan should focus
//! on circles but also ensure that a certain level of reinforcement is provided for
//! squares. This is a very simple example, but the idea is that the engine should be
//! able to determine the next lesson plan based on the learner's performance in the
//! current lesson plan.
//!
//! The way that the engine takes this information and curates a recommendation is not
//! the concern of this module.
//!
//! Note - difficulty level is a key concept. The way the content domain is setup is that
//! there are known modules such as Shapes. In a known module, there are known lessons.
//! These are pre-defined. Each lesson has a particular difficulty level. For now, the
//! concept of difficulty is split into 8 qualitative categories: Very Easy, Easy, Medium,
//! Hard, Very Hard, Expert, Master, and Grandmaster. The engine will determine the
//! difficulty level of a lesson plan based on the learner's performance in the previous
//! lesson plan.
//!

use content::*;

pub mod content;
pub mod learner;

/// The following is an example lesson with 5 questions about Basic Shapes: Recognising Squares.
/// There are 5 total questions. The first few questions only provide 1 option which is an image URL
/// of a square. Then, by the 5th question, we have at most 3 options provided where only 1 is the square.
/// The learner is expected to identify the square in each question.
///
/// The difficulty level of this lesson is Easy.

pub fn simulate_basic_lesson() -> LessonResult {
    let mut example_module: ContentModule = ContentModule::new("Shapes".to_string());

    let example_lesson: Lesson = Lesson::new(
        "Recognising Squares".to_string(),
        vec![
            Question::new(
                Prompt::new(PromptType::Text, "Select the square!".to_string()),
                Some(vec![QuestionOption::new(SQUARE_IMAGE.to_string(), true)]),
                None,
                Answer::Integer(0),
            ),
            Question::new(
                Prompt::new(PromptType::Text, "Select the square!".to_string()),
                Some(vec![QuestionOption::new(SQUARE_IMAGE.to_string(), true)]),
                None,
                Answer::Integer(0),
            ),
            Question::new(
                Prompt::new(PromptType::Text, "Select the square!".to_string()),
                Some(vec![QuestionOption::new(SQUARE_IMAGE.to_string(), true)]),
                None,
                Answer::Integer(0),
            ),
            // More difficult question where there are now more than 1 shape to choose from
            Question::new(
                Prompt::new(PromptType::Text, "Select the square!".to_string()),
                Some(vec![
                    QuestionOption::new(SQUARE_IMAGE.to_string(), true),
                    QuestionOption::new(CIRCLE_IMAGE.to_string(), true),
                    QuestionOption::new(TRIANGLE_IMAGE.to_string(), true),
                ]),
                None,
                Answer::Integer(0),
            ),
            Question::new(
                Prompt::new(PromptType::Text, "Select the square!".to_string()),
                Some(vec![
                    QuestionOption::new(TRIANGLE_IMAGE.to_string(), true),
                    QuestionOption::new(CIRCLE_IMAGE.to_string(), true),
                    QuestionOption::new(SQUARE_IMAGE.to_string(), true),
                ]),
                None,
                Answer::Integer(2),
            ),
        ],
        DifficultyLevel::Easy,
        "Shapes".to_string(),
    );

    example_module.add_lesson(example_lesson);

    // Example lesson attempted questions
    let mut lesson_attempted_questions: Vec<QuestionAttempt> = vec![];
    let mut curr_question = 1;
    for question in example_module.get_lessons()[0].get_questions() {
        let question_attempt: QuestionAttempt = if curr_question <= 3 {
            QuestionAttempt::new(question.get_id().to_string(), 6 - curr_question, 1, 0)
        } else if curr_question == 4 {
            QuestionAttempt::new(question.get_id().to_string(), 16, 2, 1)
        } else {
            QuestionAttempt::new(question.get_id().to_string(), 9, 1, 0)
        };

        lesson_attempted_questions.push(question_attempt);

        curr_question += 1;
    }

    // Example lesson result from a child
    let lesson_result: LessonResult = LessonResult::new(400, 5, lesson_attempted_questions);

    return lesson_result;
}
