use types::{
    content::{
        Answer, ContentModule, DifficultyLevel, Lesson, Prompt, PromptType, Question,
        QuestionOption, QuestionOptionType, CIRCLE_IMAGE, HEPTAGON_IMAGE, HEXAGON_IMAGE,
        PENTAGON_IMAGE, SQUARE_IMAGE, TRIANGLE_IMAGE,
    },
    learner::{ASDTraits, Communicability, CommunicationLevel, MotorSkills},
};

/// Generates a question with provided image options where the first option is always the correct one.
fn generate_question(
    prompt: &str,
    correct_image: &str,
    distractors: Vec<&str>,
    asd_traits: Option<ASDTraits>,
) -> Question {
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
        asd_traits,
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
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    1, // Attention Span: 1 minute
                    vec![Communicability::NonVerbal],
                    CommunicationLevel::Low,
                    MotorSkills::Low,
                );

                if i < 3 || i == 5 {
                    generate_question(
                        "Select the circle!",
                        CIRCLE_IMAGE,
                        vec![],
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_question(
                        "Select the circle!",
                        CIRCLE_IMAGE,
                        vec![SQUARE_IMAGE],
                        Some(asd_traits),
                    )
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
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    2, // Attention Span: 2 minutes
                    vec![Communicability::NonVerbal],
                    CommunicationLevel::Medium,
                    MotorSkills::Low,
                );

                if i < 3 {
                    generate_question(
                        "Select the square!",
                        SQUARE_IMAGE,
                        vec![],
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_question(
                        "Select the square!",
                        SQUARE_IMAGE,
                        vec![CIRCLE_IMAGE],
                        Some(asd_traits),
                    )
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
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    3, // Attention Span: 3 minutes
                    vec![Communicability::NonVerbal],
                    CommunicationLevel::Medium,
                    MotorSkills::Medium,
                );

                if i < 3 {
                    generate_question(
                        "Select the triangle!",
                        TRIANGLE_IMAGE,
                        vec![],
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_question(
                        "Select the triangle!",
                        TRIANGLE_IMAGE,
                        vec![CIRCLE_IMAGE, SQUARE_IMAGE],
                        Some(asd_traits),
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
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    5, // Attention Span: 5 minutes
                    vec![Communicability::Verbal],
                    CommunicationLevel::High,
                    MotorSkills::Medium,
                );

                match i {
                    0..=3 | 11 => generate_question(
                        "Select the pentagon!",
                        PENTAGON_IMAGE,
                        vec![TRIANGLE_IMAGE, CIRCLE_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    4..=7 => generate_question(
                        "Select the hexagon!",
                        HEXAGON_IMAGE,
                        vec![SQUARE_IMAGE, SQUARE_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    _ => generate_question(
                        "Select the heptagon!",
                        HEPTAGON_IMAGE,
                        vec![PENTAGON_IMAGE, HEXAGON_IMAGE],
                        Some(asd_traits),
                    ),
                }
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
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    7, // Attention Span: 7 minutes
                    vec![Communicability::Verbal],
                    CommunicationLevel::High,
                    MotorSkills::High,
                );

                match i {
                    0..=3 | 11 => generate_question(
                        "Select the square!",
                        SQUARE_IMAGE,
                        vec![HEXAGON_IMAGE, TRIANGLE_IMAGE, PENTAGON_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    4..=7 => generate_question(
                        "Select the pentagon!",
                        PENTAGON_IMAGE,
                        vec![CIRCLE_IMAGE, HEXAGON_IMAGE, HEPTAGON_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    _ => generate_question(
                        "Select the hexagon!",
                        HEXAGON_IMAGE,
                        vec![CIRCLE_IMAGE, TRIANGLE_IMAGE, PENTAGON_IMAGE],
                        Some(asd_traits),
                    ),
                }
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
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    10, // Attention Span: 10 minutes
                    vec![Communicability::Verbal, Communicability::NonVerbal],
                    CommunicationLevel::High,
                    MotorSkills::High,
                );

                match i {
                    0..=3 | 11 => generate_question(
                        "Select the triangle!",
                        TRIANGLE_IMAGE,
                        vec![PENTAGON_IMAGE, HEXAGON_IMAGE, HEXAGON_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    4..=7 => generate_question(
                        "Select the square!",
                        SQUARE_IMAGE,
                        vec![CIRCLE_IMAGE, HEPTAGON_IMAGE, PENTAGON_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    _ => generate_question(
                        "Select the circle!",
                        CIRCLE_IMAGE,
                        vec![SQUARE_IMAGE, HEXAGON_IMAGE, TRIANGLE_IMAGE],
                        Some(asd_traits),
                    ),
                }
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
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    15, // Attention Span: 15 minutes
                    vec![Communicability::NonVerbal],
                    CommunicationLevel::High,
                    MotorSkills::VeryHigh,
                );

                match i {
                    0..=3 | 11 => generate_question(
                        "Select the heptagon!",
                        HEPTAGON_IMAGE,
                        vec![HEXAGON_IMAGE, PENTAGON_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    4..=7 => generate_question(
                        "Select the pentagon!",
                        PENTAGON_IMAGE,
                        vec![TRIANGLE_IMAGE, CIRCLE_IMAGE, SQUARE_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    _ => generate_question(
                        "Select the hexagon!",
                        HEXAGON_IMAGE,
                        vec![SQUARE_IMAGE, TRIANGLE_IMAGE],
                        Some(asd_traits),
                    ),
                }
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
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    20, // Attention Span: 20 minutes
                    vec![Communicability::NonVerbal],
                    CommunicationLevel::High,
                    MotorSkills::VeryHigh,
                );

                match i {
                    0..=2 => generate_question(
                        "Select the heptagon!",
                        HEPTAGON_IMAGE,
                        vec![HEXAGON_IMAGE, PENTAGON_IMAGE, SQUARE_IMAGE, CIRCLE_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    3..=5 => generate_question(
                        "Select the hexagon!",
                        HEXAGON_IMAGE,
                        vec![TRIANGLE_IMAGE, PENTAGON_IMAGE, SQUARE_IMAGE, CIRCLE_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    6..=8 => generate_question(
                        "Select the pentagon!",
                        PENTAGON_IMAGE,
                        vec![HEXAGON_IMAGE, TRIANGLE_IMAGE, SQUARE_IMAGE, CIRCLE_IMAGE],
                        Some(asd_traits.clone()),
                    ),
                    9..=10 => generate_question(
                        "Select the triangle!",
                        TRIANGLE_IMAGE,
                        vec![HEXAGON_IMAGE, PENTAGON_IMAGE, SQUARE_IMAGE, CIRCLE_IMAGE],
                        Some(asd_traits),
                    ),
                    11 => generate_question(
                        "Select the square!",
                        SQUARE_IMAGE,
                        vec![HEXAGON_IMAGE, PENTAGON_IMAGE, CIRCLE_IMAGE, TRIANGLE_IMAGE],
                        Some(asd_traits),
                    ),
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
                        Some(asd_traits),
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
