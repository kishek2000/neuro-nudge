//! The engine will be based on Q-Learning. The Q-Learning algorithm
//! is a model-free, reinforcement learning algorithm. It is a
//! technique to learn optimal values of an action given a state.
//!
//! In this context the following is the plan:
//! - a q table map is associated to each student for their own progression
//! - the q table map is a mapping between a module, and its own q table
//! - this means that a student has a q table for each module of content that they do
//! - recommendations by this algorithm for lesson plans are made based on the q table
//!   where state = lesson in module with a unique difficulty level, action = choosing
//!   to do a lesson until all are completed, and reward = lesson result of a student
//!   from their attempt of some lesson with some difficulty level in the module. So this
//!   means if the student did great in a lesson, then the q table will be updated to
//!   reflect that with a reward that is positive, vice versa.
//!
//!   However, when there is am ample network of learners in the system, then the
//!   recommendation will also use collaborative filtering to take into account
//!   the q table data of other learners.
//!
//!   The goal is thus to recommend lesson plans based on the progress of similar
//!   learners in a module, where similarity is measured by similar progress AND by
//!   similarity in ASD traits.

use std::collections::HashMap;

use crate::{
    content::{ContentModule, DifficultyLevel, Lesson, LessonResult},
    learner::{ASDTraits, Learner},
};

pub type QTable = HashMap<(Lesson, DifficultyLevel), f32>;

/// QTable Algorithm
/// As per comment blob at top of the file. This struct specifically deals with
/// a single q table associated to some module under some learner.
#[derive(Debug, Clone, PartialEq)]
pub struct QTableAlgorithm {
    id: String,
    /// The QTable is a mapping between a state and an action, and the value
    /// of that action.
    q_table: QTable,
    discount_factor: f32,
    learning_rate: f32,
}

impl QTableAlgorithm {
    pub fn new(q_table: Option<QTable>) -> QTableAlgorithm {
        QTableAlgorithm {
            id: uuid::Uuid::new_v4().to_string(),
            q_table: q_table.unwrap_or(HashMap::new()),
            discount_factor: 0.9,
            learning_rate: 0.1,
        }
    }

    /// Get the value of some state-action pair.
    pub fn get(&self, state: &(Lesson, DifficultyLevel)) -> Option<&f32> {
        self.q_table.get(state)
    }

    /// Get the id of the Q-Table.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Update the value of some state-action pair, based on a lesson result
    /// from a learner.
    pub fn update(&mut self, state: (Lesson, DifficultyLevel), lesson_result: &LessonResult) {
        let old_value = self.q_table.get(&state).unwrap_or(&0.0);

        let lesson_difficulty = lesson_result.get_difficulty_level();

        let difficulty_weight = match lesson_difficulty {
            DifficultyLevel::VeryEasy => 0.2,
            DifficultyLevel::Easy => 0.3,
            DifficultyLevel::Medium => 0.4,
            DifficultyLevel::Hard => 0.6,
            DifficultyLevel::VeryHard => 0.7,
            DifficultyLevel::Expert => 0.75,
            DifficultyLevel::Master => 0.775,
            DifficultyLevel::Grandmaster => 0.8,
        };

        let total_time_taken = lesson_result.get_time_taken() as f32;
        let total_attempts = lesson_result.get_attempted_questions().len() as f32;
        let total_incorrect_attempts = lesson_result.get_total_incorrect_attempts();
        let total_hints_requested = lesson_result.get_total_hints_requested();

        // Weights for each factor (I might adjust these further based on importance)
        let time_taken_weight = 0.1 * difficulty_weight;
        let incorrect_attempts_weight = 0.5 * difficulty_weight;
        let hints_requested_weight = 0.1 * difficulty_weight;

        // Normalized factors within the range (0 to 1)
        let time_taken_factor = total_time_taken / 100.0;
        let incorrect_attempts_factor = total_incorrect_attempts as f32 / total_attempts;
        let hints_requested_factor = total_hints_requested as f32 / total_attempts;

        // Exponential transformation constants - we want extreme values to have high impact.
        let time_taken_exp_factor = 0.03;
        let incorrect_attempts_exp_factor = 0.1;
        let hints_requested_exp_factor = 0.01;

        // AExponential transformations
        let time_taken_factor = (time_taken_factor * time_taken_weight).powf(time_taken_exp_factor);
        let incorrect_attempts_factor = (incorrect_attempts_factor * incorrect_attempts_weight)
            .powf(incorrect_attempts_exp_factor);
        let hints_requested_factor =
            (hints_requested_factor * hints_requested_weight).powf(hints_requested_exp_factor);

        // Overall reward calculation
        let reward = time_taken_factor - incorrect_attempts_factor + hints_requested_factor;
        let reward = reward.max(-1.0).min(1.0);
        let new_value = (1.0 - self.learning_rate) * old_value
            + self.learning_rate * (reward + self.discount_factor * old_value);

        self.q_table.insert(state.clone(), new_value);
    }

    /// Get the best action for some state.
    pub fn get_best_action(&self, state: &Lesson) -> Option<&DifficultyLevel> {
        let mut best_action = None;
        let mut best_value = f32::MIN;
        for ((lesson, difficulty_level), value) in self.q_table.iter() {
            if lesson == state && value > &best_value {
                best_action = Some(difficulty_level);
                best_value = *value;
            }
        }
        best_action
    }

    /// Get all state-action pairs in the Q-Table.
    pub fn get_lesson_difficulty_pairs(&self) -> Vec<(&(Lesson, DifficultyLevel), &f32)> {
        self.q_table.iter().collect()
    }

    /// Check if the Q-Table contains a specific state-action pair.
    pub fn has_lesson_difficulty_pair(
        &self,
        state_action_pair: &(Lesson, DifficultyLevel),
    ) -> bool {
        self.q_table.contains_key(state_action_pair)
    }
}

/// Collaborative Filtering Algorithm
/// This struct has a mapping between all learners and their q table map.
/// Then, it provides a method to recommend a lesson difficulty for a module just
/// like the QTableAlgorithm struct.
/// The recommendation is based on the q table map of other learners as well as
/// similarity based on ASD traits, all by using collaborative filtering.
pub struct CollaborativeFilteringAlgorithm {
    // The mapping between all learners and their q table map.
    learners_data: HashMap<Learner, HashMap<ContentModule, QTableAlgorithm>>,
}

impl CollaborativeFilteringAlgorithm {
    pub fn new() -> CollaborativeFilteringAlgorithm {
        CollaborativeFilteringAlgorithm {
            learners_data: HashMap::new(),
        }
    }

    pub fn get_total_learners(&self) -> usize {
        self.learners_data.len()
    }

    // Add a learner with their Q-Table data.
    pub fn add_learner(
        &mut self,
        learner: Learner,
        q_tables: HashMap<ContentModule, QTableAlgorithm>,
    ) {
        self.learners_data.insert(learner, q_tables);
    }

    // Get recommendations for a learner based on collaborative filtering.
    // This recommendation doesn't use a latest lesson result, these are assumed complete in the q table mappings.
    pub fn recommend_lesson_difficulty(
        &self,
        learner: &Learner,
        module: &ContentModule,
    ) -> Option<DifficultyLevel> {
        if let Some(q_tables) = self.learners_data.get(learner) {
            if let Some(q_table) = q_tables.get(module) {
                let mut most_similar_learner: Option<&Learner> = None;
                let mut most_similar_similarity = f32::NEG_INFINITY;

                // Compare the current learner's ASD traits with other learners in the system.
                for (other_learner, other_q_tables) in &self.learners_data {
                    if other_learner != learner {
                        // Similarity score based on ASD traits (weighed higher).
                        let asd_similarity = self.calculate_asd_similarity(
                            &learner.get_asd_traits(),
                            &other_learner.get_asd_traits(),
                        );

                        // Calculate a similarity score based on Q-Tables.
                        let q_table_similarity = self.calculate_q_table_similarity(
                            q_table,
                            other_q_tables.get(module).unwrap(),
                        );

                        // Calculate the combined similarity score with a higher weight for ASD traits.
                        let combined_similarity = 0.6 * asd_similarity + 0.4 * q_table_similarity;

                        if combined_similarity > most_similar_similarity {
                            most_similar_learner = Some(other_learner);
                            most_similar_similarity = combined_similarity;
                        }
                    }
                }

                // If a similar learner was found, we should recommend a lesson
                // difficulty level based on the similar learner's Q-Table.
                if most_similar_learner.is_some() {
                    if let Some(recommended_level) =
                        q_table.get_best_action(&module.get_lessons()[0])
                    {
                        return Some(recommended_level.clone());
                    }
                }
            }
        }

        None // Return None if no recommendation can be made.
    }

    fn calculate_asd_similarity(&self, asd_traits_1: &ASDTraits, asd_traits_2: &ASDTraits) -> f32 {
        // Calculate similarity based on ASD traits by a weighted sum.

        // Absolute difference in attention span.
        let attention_span_diff =
            (asd_traits_1.get_attention_span() - asd_traits_2.get_attention_span()).abs() as f32;
        let attention_span_similarity = 1.0 - attention_span_diff / 100.0; // Normalize to a 0-1 range.

        // Communicability similarity based on common communicability types.
        let common_communicability_count = asd_traits_1
            .get_communicability()
            .iter()
            .filter(|&comm| asd_traits_2.get_communicability().contains(comm))
            .count() as f32;
        let max_communicability_count = asd_traits_1
            .get_communicability()
            .len()
            .max(asd_traits_2.get_communicability().len())
            as f32;
        let communicability_similarity = common_communicability_count / max_communicability_count;

        // Communication level similarity (considered binary in this example).
        let communication_level_similarity =
            if asd_traits_1.get_communication_level() == asd_traits_2.get_communication_level() {
                1.0
            } else {
                0.0
            };

        // Combine individual similarities with appropriate weights - for sake of example right now,
        // we give most importance to attention span and communicability, and then communication level
        // comes last.
        let attention_span_weight = 0.4;
        let communicability_weight = 0.4;
        let communication_level_weight = 0.2;

        let similarity = (attention_span_similarity * attention_span_weight)
            + (communicability_similarity * communicability_weight)
            + (communication_level_similarity * communication_level_weight);

        similarity
    }

    fn calculate_q_table_similarity(
        &self,
        q_table_1: &QTableAlgorithm,
        q_table_2: &QTableAlgorithm,
    ) -> f32 {
        // Calculating the similarity based on Q-Tables using cosine similarity. This will be
        // explained, but basically it's better for this context where learning progress and
        // direction matters.

        // Collect the set of common state-action pairs between the two Q-Tables.
        let common_lesson_difficulty_pairs: Vec<(&(Lesson, DifficultyLevel), f32)> = q_table_1
            .get_lesson_difficulty_pairs()
            .into_iter()
            .filter(|(state, _)| q_table_2.has_lesson_difficulty_pair(state))
            .map(|(state, &value)| (state, value))
            .collect();

        if common_lesson_difficulty_pairs.is_empty() {
            return 0.0; // No common state-action pairs, similarity is zero.
        }

        // Calculate the dot product of the two vectors.
        let mut dot_product = 0.0;
        let mut magnitude_1 = 0.0;
        let mut magnitude_2 = 0.0;

        for (state, _) in common_lesson_difficulty_pairs {
            let value_1 = q_table_1.get(state).unwrap();
            let value_2 = q_table_2.get(state).unwrap();

            dot_product += value_1 * value_2;
            magnitude_1 += value_1 * value_1;
            magnitude_2 += value_2 * value_2;
        }

        magnitude_1 = magnitude_1.sqrt();
        magnitude_2 = magnitude_2.sqrt();

        // Calculate the cosine similarity.
        let similarity = dot_product / (magnitude_1 * magnitude_2);

        similarity
    }
}
