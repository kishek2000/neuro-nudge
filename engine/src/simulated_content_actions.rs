use types::{
    content::{
        Answer, ContentModule, DifficultyLevel, Lesson, Prompt, PromptType, Question,
        QuestionOption, QuestionOptionType,
    },
    learner::{ASDTraits, Communicability, CommunicationLevel, MotorSkills},
};

/// Generates a question for copying an action.
fn generate_copy_action_question(
    action_description: &str,
    action_media_url: &str,
    asd_traits_parameters: Option<ASDTraits>,
) -> Question {
    let prompt_text = format!("Copy this action: {}", action_description);
    Question::new(
        Prompt::new(PromptType::Video(prompt_text), action_media_url.to_string()), // Using video prompt
        None,
        None,
        Answer::Boolean(false), // Placeholder, actual answer to be provided by instructor
        asd_traits_parameters,
    )
}

/// Generates a question for recognizing an action.
fn generate_recognize_action_question(
    prompt: &str,
    correct_action_url: &str,
    distractors: Vec<&str>,
    asd_traits_parameters: Option<ASDTraits>,
) -> Question {
    let mut options = vec![correct_action_url];
    options.extend(distractors);

    let question_options = options
        .into_iter()
        .map(|action_url| QuestionOption::new(action_url.to_string(), QuestionOptionType::Video))
        .collect();

    Question::new(
        Prompt::new(PromptType::Text, prompt.to_string()),
        Some(question_options),
        None,
        Answer::Integer(0), // Assumes the correct action is always the first
        asd_traits_parameters,
    )
}

/// Generates lessons for different difficulty levels for the "Actions" module.
pub fn generate_actions_lessons() -> Vec<Lesson> {
    let mut lessons = Vec::new();

    // Very Easy lesson: Basic actions like clapping hands
    let very_easy_lesson = Lesson::new(
        "Basic Actions".to_string(),
        (0..6)
            .map(|i| {
                if i % 2 == 0 {
                    generate_copy_action_question(
                        "Clapping hands",
                        "https://example.com/clapping.gif",
                        Some(ASDTraits::new(
                            "".to_string(),
                            1,
                            vec![Communicability::NonVerbal],
                            CommunicationLevel::Low,
                            MotorSkills::Low,
                        )),
                    )
                } else {
                    generate_recognize_action_question(
                        "Which one is waving hello?",
                        "https://example.com/waving.gif",
                        vec!["https://example.com/nodding.gif"],
                        Some(ASDTraits::new(
                            "".to_string(),
                            1,
                            vec![Communicability::NonVerbal],
                            CommunicationLevel::Low,
                            MotorSkills::Low,
                        )),
                    )
                }
            })
            .collect(),
        DifficultyLevel::VeryEasy,
        "Actions".to_string(),
    );
    lessons.push(very_easy_lesson);

    // Easy lesson: Slightly more complex actions like jumping
    let easy_lesson = Lesson::new(
        "Intermediate Actions".to_string(),
        (0..8)
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    3, // Attention Span: 3 minutes
                    vec![Communicability::NonVerbal],
                    CommunicationLevel::Low,
                    MotorSkills::Medium,
                );

                if i < 4 {
                    generate_copy_action_question(
                        "Jumping",
                        "https://example.com/jumping.gif",
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_recognize_action_question(
                        "Which one is nodding?",
                        "https://example.com/nodding.gif",
                        vec!["https://example.com/waving.gif"],
                        Some(asd_traits),
                    )
                }
            })
            .collect(),
        DifficultyLevel::Easy,
        "Actions".to_string(),
    );
    lessons.push(easy_lesson);

    // Medium lesson: Actions that involve two steps
    let medium_lesson = Lesson::new(
        "Two-Step Actions".to_string(),
        (0..10)
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    5, // Attention Span: 5 minutes
                    vec![Communicability::NonVerbal],
                    CommunicationLevel::Medium,
                    MotorSkills::Medium,
                );

                if i % 3 == 0 {
                    generate_copy_action_question(
                        "Jump and Clap",
                        "https://example.com/jump_clap.gif",
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_recognize_action_question(
                        "Find the person doing a spin",
                        "https://example.com/spin.gif",
                        vec![
                            "https://example.com/jump.gif",
                            "https://example.com/clap.gif",
                        ],
                        Some(asd_traits),
                    )
                }
            })
            .collect(),
        DifficultyLevel::Medium,
        "Actions".to_string(),
    );
    lessons.push(medium_lesson);

    // Hard lesson: Multistep actions or actions requiring coordination
    let hard_lesson = Lesson::new(
        "Coordinated Actions".to_string(),
        (0..12)
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    7, // Attention Span: 7 minutes
                    vec![Communicability::NonVerbal],
                    CommunicationLevel::High,
                    MotorSkills::Medium,
                );

                if i % 3 == 0 {
                    generate_copy_action_question(
                        "Dance Move",
                        "https://example.com/dance_move.gif",
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_recognize_action_question(
                        "Which is a kickball change (dance step)?",
                        "https://example.com/kickball_change.gif",
                        vec![
                            "https://example.com/step_touch.gif",
                            "https://example.com/pivot_turn.gif",
                        ],
                        Some(asd_traits),
                    )
                }
            })
            .collect(),
        DifficultyLevel::Hard,
        "Actions".to_string(),
    );
    lessons.push(hard_lesson);

    // Very Hard lesson: More complex multi-step actions
    let very_hard_lesson = Lesson::new(
        "Complex Multi-Step Actions".to_string(),
        (0..14)
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    10, // Attention Span: 10 minutes
                    vec![Communicability::NonVerbal, Communicability::Verbal],
                    CommunicationLevel::High,
                    MotorSkills::Medium,
                );

                if i % 4 == 0 {
                    generate_copy_action_question(
                        "Yoga Pose Sequence",
                        "https://example.com/yoga_pose_sequence.gif",
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_recognize_action_question(
                        "Identify the cartwheel",
                        "https://example.com/cartwheel.gif",
                        vec![
                            "https://example.com/handstand.gif",
                            "https://example.com/forward_roll.gif",
                        ],
                        Some(asd_traits),
                    )
                }
            })
            .collect(),
        DifficultyLevel::VeryHard,
        "Actions".to_string(),
    );
    lessons.push(very_hard_lesson);

    // Expert lesson: Sequences of actions focusing on following instructions
    let expert_lesson = Lesson::new(
        "Action Sequences".to_string(),
        (0..16)
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    12, // Attention Span: 12 minutes
                    vec![Communicability::Verbal, Communicability::NonVerbal],
                    CommunicationLevel::High,
                    MotorSkills::High,
                );

                if i % 4 == 0 {
                    generate_copy_action_question(
                        "Miming an action without props",
                        "https://example.com/miming.gif",
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_recognize_action_question(
                        "Which action symbolizes 'thinking'?",
                        "https://example.com/thinking_pose.gif",
                        vec![
                            "https://example.com/looking_around.gif",
                            "https://example.com/shrugging.gif",
                        ],
                        Some(asd_traits),
                    )
                }
            })
            .collect(),
        DifficultyLevel::Expert,
        "Actions".to_string(),
    );
    lessons.push(expert_lesson);

    // Master lesson: Sequences of actions with emphasis on motor skills
    let master_lesson = Lesson::new(
        "Mastering Motor Skills".to_string(),
        (0..18)
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    15, // Attention Span: 15 minutes
                    vec![Communicability::NonVerbal, Communicability::Verbal],
                    CommunicationLevel::High,
                    MotorSkills::VeryHigh,
                );

                if i % 5 == 0 {
                    generate_copy_action_question(
                        "Complex Gymnastics Routine",
                        "https://example.com/gymnastics_routine.gif",
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_recognize_action_question(
                        "Select the correct sequence of actions",
                        "https://example.com/correct_sequence.gif",
                        vec![
                            "https://example.com/wrong_sequence_1.gif",
                            "https://example.com/wrong_sequence_2.gif",
                        ],
                        Some(asd_traits),
                    )
                }
            })
            .collect(),
        DifficultyLevel::Master,
        "Actions".to_string(),
    );
    lessons.push(master_lesson);

    // Grandmaster lesson: Advanced action sequences with focus on precision and coordination
    let grandmaster_lesson = Lesson::new(
        "Advanced Action Interpretation".to_string(),
        (0..20)
            .map(|i| {
                let asd_traits = ASDTraits::new(
                    "".to_string(),
                    20, // Attention Span: 20 minutes
                    vec![Communicability::NonVerbal, Communicability::Verbal],
                    CommunicationLevel::High,
                    MotorSkills::VeryHigh,
                );

                if i % 5 == 0 {
                    generate_copy_action_question(
                        "Intricate Dance Choreography",
                        "https://example.com/advanced_dance.gif",
                        Some(asd_traits.clone()),
                    )
                } else {
                    generate_recognize_action_question(
                        "Identify the most precise action",
                        "https://example.com/precise_action.gif",
                        vec![
                            "https://example.com/action_1.gif",
                            "https://example.com/action_2.gif",
                        ],
                        Some(asd_traits),
                    )
                }
            })
            .collect(),
        DifficultyLevel::Grandmaster,
        "Actions".to_string(),
    );
    lessons.push(grandmaster_lesson);

    // Return all the lessons
    lessons
}

pub fn generate_actions_module() -> ContentModule {
    ContentModule::new("Actions".to_string()).with_lessons(generate_actions_lessons())
}
