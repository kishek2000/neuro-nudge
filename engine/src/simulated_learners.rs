use std::collections::HashMap;
use types::content::Lesson;
use types::engine::QTableAlgorithm;
use types::learner::{ASDTraits, Communicability, CommunicationLevel, Learner};

fn generate_simulated_learner(
    name: &str,
    age: u8,
    asd_traits: ASDTraits,
    q_table: &mut QTableAlgorithm, // Use a mutable reference
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
) -> (Vec<&str>, HashMap<String, (Learner, QTableAlgorithm)>) {
    let mut simulated_learners_with_q_tables = HashMap::new();

    // Generate two learners with similar ASD traits (Verbal, Medium CommunicationLevel)
    let similar_asd_traits_1 = ASDTraits::new(
        "Learner 1".to_string(),
        5,
        vec![Communicability::Verbal],
        CommunicationLevel::Medium,
    );
    let similar_asd_traits_2 = ASDTraits::new(
        "Learner 2".to_string(),
        6,
        vec![Communicability::Verbal],
        CommunicationLevel::Medium,
    );

    // Generate two learners with different ASD traits (NonVerbal, Low CommunicationLevel; Verbal, NonVerbal, High CommunicationLevel)
    let different_asd_traits_1 = ASDTraits::new(
        "Learner 3".to_string(),
        7,
        vec![Communicability::NonVerbal],
        CommunicationLevel::Low,
    );
    let different_asd_traits_2 = ASDTraits::new(
        "Learner 4".to_string(),
        8,
        vec![Communicability::Verbal, Communicability::NonVerbal],
        CommunicationLevel::High,
    );

    // Generate two learners with random ASD traits
    let random_asd_traits_1 = ASDTraits::new(
        "Learner 5".to_string(),
        9,
        vec![Communicability::Verbal],
        CommunicationLevel::Medium,
    );
    let random_asd_traits_2 = ASDTraits::new(
        "Learner 6".to_string(),
        10,
        vec![Communicability::NonVerbal],
        CommunicationLevel::Low,
    );

    // Initialise a q table for all lessons and their difficulties, with a value of 0
    let mut q_table_1 = QTableAlgorithm::new(None, 0.2);
    let mut q_table_2 = QTableAlgorithm::new(None, 0.2);
    let mut q_table_3 = QTableAlgorithm::new(None, 0.2);
    let mut q_table_4 = QTableAlgorithm::new(None, 0.2);
    let mut q_table_5 = QTableAlgorithm::new(None, 0.2);
    let mut q_table_6 = QTableAlgorithm::new(None, 0.2);

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
        generate_simulated_learner("Learner 1", 7, similar_asd_traits_1, &mut q_table_1),
        generate_simulated_learner("Learner 2", 8, similar_asd_traits_2, &mut q_table_2),
        generate_simulated_learner("Learner 3", 9, different_asd_traits_1, &mut q_table_3),
        generate_simulated_learner("Learner 4", 10, different_asd_traits_2, &mut q_table_4),
        generate_simulated_learner("Learner 5", 11, random_asd_traits_1, &mut q_table_5),
        generate_simulated_learner("Learner 6", 12, random_asd_traits_2, &mut q_table_6),
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
                // Handle unknown learner
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
