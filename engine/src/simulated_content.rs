use types::content::{
    Answer, ContentModule, DifficultyLevel, Lesson, Prompt, PromptType, Question, QuestionOption,
    QuestionOptionType, CIRCLE_IMAGE, HEPTAGON_IMAGE, HEXAGON_IMAGE, PENTAGON_IMAGE, SQUARE_IMAGE,
    TRIANGLE_IMAGE,
};

/// Generates a question with provided image options where the first option is always the correct one.
fn generate_question(prompt: &str, correct_image: &str, distractors: Vec<&str>) -> Question {
    let mut images = vec![correct_image];
    images.extend(distractors);

    let options = images
        .into_iter()
        .map(|img| QuestionOption::new(img.to_string(), QuestionOptionType::Image))
        .collect();

    Question::new(
        Prompt::new(PromptType::Text, prompt.to_string()),
        Some(options),
        None,
        Answer::Integer(0), // Assumes the correct image is always the first
    )
}

/// Generates lessons for different difficulty levels for the "Shapes" module.
pub fn generate_shapes_lessons() -> Vec<Lesson> {
    let mut lessons = Vec::new();

    // Very Easy lesson: "Recognising Circles"
    let very_easy_lesson = Lesson::new(
        "Recognising Circles".to_string(),
        (0..6)
            .map(|i| {
                if i < 3 || i == 5 {
                    generate_question("Select the circle!", CIRCLE_IMAGE, vec![])
                } else {
                    generate_question("Select the circle!", CIRCLE_IMAGE, vec![SQUARE_IMAGE])
                }
            })
            .collect(),
        DifficultyLevel::VeryEasy,
        "Shapes".to_string(),
    );
    lessons.push(very_easy_lesson);

    // Easy lesson: "Introducing Rectangles and Squares"
    let easy_lesson = Lesson::new(
        "Introducing Squares".to_string(),
        (0..8)
            .map(|i| {
                if i < 3 {
                    generate_question("Select the square!", SQUARE_IMAGE, vec![])
                } else {
                    generate_question("Select the square!", SQUARE_IMAGE, vec![CIRCLE_IMAGE])
                }
            })
            .collect(),
        DifficultyLevel::Easy,
        "Shapes".to_string(),
    );
    lessons.push(easy_lesson);

    // Medium lesson: "Getting Comfortable with Triangles"
    let medium_lesson = Lesson::new(
        "Getting Comfortable with Triangles".to_string(),
        (0..6)
            .map(|i| {
                if i < 3 {
                    generate_question("Select the triangle!", TRIANGLE_IMAGE, vec![])
                } else {
                    generate_question(
                        "Select the triangle!",
                        TRIANGLE_IMAGE,
                        vec![CIRCLE_IMAGE, SQUARE_IMAGE],
                    )
                }
            })
            .collect(),
        DifficultyLevel::Medium,
        "Shapes".to_string(),
    );
    lessons.push(medium_lesson);

    // Hard lesson: "Identifying Complex Shapes"
    let hard_lesson = Lesson::new(
        "Identifying Complex Shapes".to_string(),
        (0..12)
            .map(|i| match i {
                0..=3 | 11 => generate_question(
                    "Select the pentagon!",
                    PENTAGON_IMAGE,
                    vec![TRIANGLE_IMAGE, CIRCLE_IMAGE],
                ),
                4..=7 => generate_question(
                    "Select the hexagon!",
                    HEXAGON_IMAGE,
                    vec![SQUARE_IMAGE, SQUARE_IMAGE],
                ),
                _ => generate_question(
                    "Select the heptagon!",
                    HEPTAGON_IMAGE,
                    vec![PENTAGON_IMAGE, HEXAGON_IMAGE],
                ),
            })
            .collect(),
        DifficultyLevel::Hard,
        "Shapes".to_string(),
    );
    lessons.push(hard_lesson);

    // Very Hard lesson: "Shape Differentiation"
    let very_hard_lesson = Lesson::new(
        "Shape Differentiation".to_string(),
        (0..12)
            .map(|i| match i {
                0..=3 | 11 => generate_question(
                    "Select the square!",
                    SQUARE_IMAGE,
                    vec![HEXAGON_IMAGE, TRIANGLE_IMAGE, PENTAGON_IMAGE],
                ),
                4..=7 => generate_question(
                    "Select the pentagon!",
                    PENTAGON_IMAGE,
                    vec![CIRCLE_IMAGE, HEXAGON_IMAGE, HEPTAGON_IMAGE],
                ),
                _ => generate_question(
                    "Select the hexagon!",
                    HEXAGON_IMAGE,
                    vec![CIRCLE_IMAGE, TRIANGLE_IMAGE, PENTAGON_IMAGE],
                ),
            })
            .collect(),
        DifficultyLevel::VeryHard,
        "Shapes".to_string(),
    );
    lessons.push(very_hard_lesson);

    // Expert lesson: "Advanced Shape Identification"
    let expert_lesson = Lesson::new(
        "Advanced Shape Identification".to_string(),
        (0..12)
            .map(|i| match i {
                0..=3 | 11 => generate_question(
                    "Select the triangle!",
                    TRIANGLE_IMAGE,
                    vec![PENTAGON_IMAGE, HEXAGON_IMAGE, HEXAGON_IMAGE],
                ),
                4..=7 => generate_question(
                    "Select the square!",
                    SQUARE_IMAGE,
                    vec![CIRCLE_IMAGE, HEPTAGON_IMAGE, PENTAGON_IMAGE],
                ),
                _ => generate_question(
                    "Select the circle!",
                    CIRCLE_IMAGE,
                    vec![SQUARE_IMAGE, HEXAGON_IMAGE, TRIANGLE_IMAGE],
                ),
            })
            .collect(),
        DifficultyLevel::Expert,
        "Shapes".to_string(),
    );
    lessons.push(expert_lesson);

    // Master lesson: "Mastering Shape Recognition"
    let master_lesson = Lesson::new(
        "Mastering Shape Recognition".to_string(),
        (0..12)
            .map(|i| match i {
                0..=3 | 11 => generate_question(
                    "Select the heptagon!",
                    HEPTAGON_IMAGE,
                    vec![HEXAGON_IMAGE, PENTAGON_IMAGE],
                ),
                4..=7 => generate_question(
                    "Select the pentagon!",
                    PENTAGON_IMAGE,
                    vec![TRIANGLE_IMAGE, CIRCLE_IMAGE, SQUARE_IMAGE],
                ),
                _ => generate_question(
                    "Select the hexagon!",
                    HEXAGON_IMAGE,
                    vec![SQUARE_IMAGE, TRIANGLE_IMAGE],
                ),
            })
            .collect(),
        DifficultyLevel::Master,
        "Shapes".to_string(),
    );
    lessons.push(master_lesson);

    // Grandmaster lesson: "The Ultimate Shape Challenge"
    let grandmaster_lesson = Lesson::new(
        "The Ultimate Shape Challenge".to_string(),
        (0..12)
            .map(|i| {
                match i {
                    0..=2 => generate_question(
                        "Select the heptagon!",
                        HEPTAGON_IMAGE,
                        vec![HEXAGON_IMAGE, PENTAGON_IMAGE, SQUARE_IMAGE, CIRCLE_IMAGE],
                    ),
                    3..=5 => generate_question(
                        "Select the hexagon!",
                        HEXAGON_IMAGE,
                        vec![TRIANGLE_IMAGE, PENTAGON_IMAGE, SQUARE_IMAGE, CIRCLE_IMAGE],
                    ),
                    6..=8 => generate_question(
                        "Select the pentagon!",
                        PENTAGON_IMAGE,
                        vec![HEXAGON_IMAGE, TRIANGLE_IMAGE, SQUARE_IMAGE, CIRCLE_IMAGE],
                    ),
                    9..=10 => generate_question(
                        "Select the triangle!",
                        TRIANGLE_IMAGE,
                        vec![HEXAGON_IMAGE, PENTAGON_IMAGE, SQUARE_IMAGE, CIRCLE_IMAGE],
                    ),
                    11 => generate_question(
                        "Select the square!",
                        SQUARE_IMAGE,
                        vec![HEXAGON_IMAGE, PENTAGON_IMAGE, CIRCLE_IMAGE, TRIANGLE_IMAGE],
                    ),
                    // This last question can include a mix of all shapes
                    _ => generate_question(
                        "Select the circle!",
                        CIRCLE_IMAGE,
                        vec![
                            SQUARE_IMAGE,
                            TRIANGLE_IMAGE,
                            PENTAGON_IMAGE,
                            HEXAGON_IMAGE,
                            HEPTAGON_IMAGE,
                        ],
                    ),
                }
            })
            .collect(),
        DifficultyLevel::Grandmaster,
        "Shapes".to_string(),
    );
    lessons.push(grandmaster_lesson);

    // Return all the lessons
    lessons
}

pub fn generate_shapes_module() -> ContentModule {
    ContentModule::new("Shapes".to_string()).with_lessons(generate_shapes_lessons())
}
