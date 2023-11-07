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

use rand::Rng;

use crate::{
    content::{ContentModule, DifficultyLevel, Lesson, LessonResult},
    learner::{ASDTraits, Learner},
};

// Define mastery thresholds as constants
const BASIC_MASTERY_THRESHOLD: f32 = 0.5;
const COMPETENT_MASTERY_THRESHOLD: f32 = 0.7;
const FULL_MASTERY_THRESHOLD: f32 = 0.8;

pub type QTable = HashMap<(Lesson, DifficultyLevel), f32>;

#[derive(Debug, Clone)]
pub enum Mastery {
    None,
    Basic,
    Competent,
    Full,
}

/// QTable Algorithm
/// As per comment blob at top of the file. This struct specifically deals with
/// a single q table associated to some module under some learner.
#[derive(Debug, Clone, PartialEq)]
pub struct QTableAlgorithm {
    id: String,
    /// The QTable is a mapping between a state and an action, and the value
    /// of that action.
    q_table: QTable,
    epsilon: f32,
    discount_factor: f32,
    learning_rate: f32,
}

impl QTableAlgorithm {
    pub fn new(q_table: Option<QTable>, epsilon: f32) -> QTableAlgorithm {
        QTableAlgorithm {
            id: uuid::Uuid::new_v4().to_string(),
            q_table: q_table.unwrap_or(HashMap::new()),
            discount_factor: 0.25,
            learning_rate: 0.75,
            epsilon,
        }
    }

    pub fn insert(&mut self, state: (Lesson, DifficultyLevel), value: f32) {
        self.q_table.insert(state, value);
    }

    /// Get the value of some state-action pair.
    pub fn get(&self, state: &(Lesson, DifficultyLevel)) -> Option<&f32> {
        self.q_table.get(state)
    }

    /// Get the id of the Q-Table.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    // Epsilon-greedy strategy to choose the next action
    pub fn epsilon_greedy_action(
        &self,
        state: &(Lesson, DifficultyLevel),
        mastery_level: Mastery,
    ) -> (Lesson, DifficultyLevel) {
        let rand_value = rand::thread_rng().gen::<f32>();
        if rand_value < self.epsilon {
            // Exploration: choose the next difficulty level.
            self.choose_next_difficulty(state, mastery_level)
        } else {
            // Exploitation: choose the best-known action.
            self.get_best_action(state)
                .clone()
                .unwrap_or_else(|| self.choose_next_difficulty(state, mastery_level))
        }
    }

    // Assuming we choose the next difficulty level.
    fn choose_next_difficulty(
        &self,
        state: &(Lesson, DifficultyLevel),
        mastery_level: Mastery,
    ) -> (Lesson, DifficultyLevel) {
        let difficulties = [
            DifficultyLevel::VeryEasy,
            DifficultyLevel::Easy,
            DifficultyLevel::Medium,
            DifficultyLevel::Hard,
            DifficultyLevel::VeryHard,
            DifficultyLevel::Expert,
            DifficultyLevel::Master,
            DifficultyLevel::Grandmaster,
        ];

        let current_index = difficulties
            .iter()
            .position(|d| d.clone() == state.1)
            .unwrap_or(0);

        let mut next_index = match mastery_level {
            Mastery::Full => current_index + 1, // Move up one level for full mastery
            // With a random probability of 0.6, move up one level for competent mastery
            Mastery::Competent => {
                if rand::thread_rng().gen::<f32>() < 0.6 {
                    current_index + 1
                } else {
                    current_index
                }
            }
            Mastery::Basic => current_index, // Stay at current level for basic mastery
            // Drop a level if below basic mastery, at probability of 0.4. That means there's a 0.6 chance you stay as is.
            _ => {
                if rand::thread_rng().gen::<f32>() < 0.4 {
                    if current_index == 0 {
                        0
                    } else {
                        current_index - 1
                    }
                } else {
                    current_index
                }
            }
        };

        next_index = next_index.min(difficulties.len() - 1); // Ensure index is within bounds
        next_index = next_index.max(0); // Ensure index is within bounds
        let next_difficulty = difficulties[next_index].clone();

        let lesson_with_difficulty = self.q_table.keys().find(|(_, d)| d == &next_difficulty);

        (
            lesson_with_difficulty.unwrap().0.clone(),
            difficulties[next_index].clone(),
        )
    }

    /// Update the value of some state-action pair, based on a lesson result
    /// from a learner.
    pub fn update(
        &mut self,
        state: (Lesson, DifficultyLevel),
        lesson_result: &LessonResult,
    ) -> Mastery {
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

        // Adjust the reward based on mastery thresholds
        let mastery_level = if reward >= FULL_MASTERY_THRESHOLD {
            Mastery::Full
        } else if reward >= COMPETENT_MASTERY_THRESHOLD {
            Mastery::Competent
        } else if reward >= BASIC_MASTERY_THRESHOLD {
            Mastery::Basic
        } else {
            Mastery::None
        };

        let new_reward = match mastery_level {
            Mastery::Full => 1.0,               // Give full reward for the complete mastery
            Mastery::Competent => reward + 0.1, // Give some additional reward for competent mastery
            Mastery::Basic => reward, // No additional reward, but no penalty either for basic mastery
            _ => reward - 0.1,        // Penalize for lack of basic mastery
        };

        let (next_state, _) = self.choose_next_difficulty(&state, mastery_level.clone());

        let next_max = self
            .q_table
            .iter()
            .filter(|((s, _), _)| s == &next_state)
            .map(|(_, &v)| v)
            .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let new_value = old_value
            + self.learning_rate * (new_reward + self.discount_factor * next_max - old_value);

        self.q_table.insert(state.clone(), new_value);

        mastery_level
    }

    /// Get the best action for some state.
    pub fn get_best_action(
        &self,
        state: &(Lesson, DifficultyLevel),
    ) -> Option<(Lesson, DifficultyLevel)> {
        let mut best_action = None;
        let mut best_value = f32::MIN;
        for ((l, difficulty_level), &value) in &self.q_table {
            if state.1 == *difficulty_level && value > best_value {
                best_action = Some((l.clone(), difficulty_level.clone()));
                best_value = value;
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
                    let lesson = learner.get_current_lesson();
                    let difficulty_level = lesson.clone().get_difficulty_level();
                    if let Some((_, recommended_level)) =
                        q_table.get_best_action(&(lesson.clone(), difficulty_level))
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
