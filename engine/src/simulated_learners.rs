//! Simulated learners for the simulation testing environment

use std::collections::HashMap;
use types::content::Lesson;
use types::engine::{QTableAlgorithm, Strategy};
use types::learner::{ASDTraits, Communicability, CommunicationLevel, Learner, MotorSkills};

fn generate_simulated_learner(
    name: &str,
    age: u8,
    asd_traits: ASDTraits,
    q_table: &mut QTableAlgorithm,
) -> Learner {
    let q_table_id = q_table.get_id();
    let learner = Learner::new(
        name.to_string(),
        age,
        asd_traits.clone(),
        q_table_id.to_string(),
        Some(asd_traits.get_learner_id().clone()),
    );

    learner
}

pub fn generate_simulated_learners_with_q_tables(
    lessons: &Vec<Lesson>,
    strategy: Strategy,
) -> (Vec<&str>, HashMap<String, (Learner, QTableAlgorithm)>) {
    let mut simulated_learners_with_q_tables = HashMap::new();

    let learner_1_traits = ASDTraits::new(
        "Learner 1".to_string(),
        5,
        vec![Communicability::NonVerbal],
        CommunicationLevel::Low,
        MotorSkills::Low,
    );

    let learner_2_traits = ASDTraits::new(
        "Learner 2".to_string(),
        6,
        vec![Communicability::NonVerbal],
        CommunicationLevel::Medium,
        MotorSkills::Low,
    );

    let learner_3_traits = ASDTraits::new(
        "Learner 3".to_string(),
        7,
        vec![Communicability::NonVerbal],
        CommunicationLevel::Medium,
        MotorSkills::Medium,
    );

    let learner_4_traits = ASDTraits::new(
        "Learner 4".to_string(),
        9,
        vec![Communicability::NonVerbal, Communicability::Verbal],
        CommunicationLevel::Medium,
        MotorSkills::High,
    );

    let learner_5_traits = ASDTraits::new(
        "Learner 5".to_string(),
        12,
        vec![Communicability::NonVerbal, Communicability::Verbal],
        CommunicationLevel::High,
        MotorSkills::High,
    );

    let learner_6_traits = ASDTraits::new(
        "Learner 6".to_string(),
        15,
        vec![Communicability::NonVerbal, Communicability::Verbal],
        CommunicationLevel::High,
        MotorSkills::VeryHigh,
    );

    // Initialise a q table for all lessons and their difficulties, with a value of 0
    let mut q_table_1 = QTableAlgorithm::new(None, 0.3, strategy.clone());
    let mut q_table_2 = QTableAlgorithm::new(None, 0.3, strategy.clone());
    let mut q_table_3 = QTableAlgorithm::new(None, 0.3, strategy.clone());
    let mut q_table_4 = QTableAlgorithm::new(None, 0.3, strategy.clone());
    let mut q_table_5 = QTableAlgorithm::new(None, 0.3, strategy.clone());
    let mut q_table_6 = QTableAlgorithm::new(None, 0.3, strategy.clone());

    let mut q_tables = vec![
        &mut q_table_1,
        &mut q_table_2,
        &mut q_table_3,
        &mut q_table_4,
        &mut q_table_5,
        &mut q_table_6,
    ];

    for lesson in lessons {
        let difficulty_level = &lesson.clone().get_difficulty_level();
        for q_table in &mut q_tables {
            q_table.insert((lesson.clone(), difficulty_level.clone()), 0.0);
        }
    }

    let learners = vec![
        generate_simulated_learner("Learner 1", 7, learner_1_traits, &mut q_table_1),
        generate_simulated_learner("Learner 2", 8, learner_2_traits, &mut q_table_2),
        generate_simulated_learner("Learner 3", 9, learner_3_traits, &mut q_table_3),
        generate_simulated_learner("Learner 4", 10, learner_4_traits, &mut q_table_4),
        generate_simulated_learner("Learner 5", 11, learner_5_traits, &mut q_table_5),
        generate_simulated_learner("Learner 6", 12, learner_6_traits, &mut q_table_6),
    ];

    for learner in learners {
        let learner_id = learner.get_id().clone();
        let q_table = match learner_id.as_str() {
            "Learner 1" => &q_table_1,
            "Learner 2" => &q_table_2,
            "Learner 3" => &q_table_3,
            "Learner 4" => &q_table_4,
            "Learner 5" => &q_table_5,
            "Learner 6" => &q_table_6,
            _ => {
                panic!("Unknown learner ID: {}", learner_id);
            }
        };
        simulated_learners_with_q_tables.insert(learner_id, (learner, q_table.clone()));
    }

    (
        vec![
            "Learner 1",
            "Learner 2",
            "Learner 3",
            "Learner 4",
            "Learner 5",
            "Learner 6",
        ],
        simulated_learners_with_q_tables,
    )
}
