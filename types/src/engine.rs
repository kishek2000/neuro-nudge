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
    strategy: Strategy,
}

/// Strategy of engine
/// 1: Q learning without mastery
/// 2: Q learning with mastery
/// 3: Collaborative filtering with 2
#[derive(Debug, Clone, PartialEq)]
pub enum Strategy {
    Strategy1,
    Strategy2,
    Strategy3,
}

impl QTableAlgorithm {
    pub fn new(q_table: Option<QTable>, epsilon: f32, strategy: Strategy) -> QTableAlgorithm {
        QTableAlgorithm {
            id: uuid::Uuid::new_v4().to_string(),
            q_table: q_table.unwrap_or(HashMap::new()),
            discount_factor: 0.25,
            learning_rate: 0.75,
            epsilon,
            strategy,
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
        mastery_level: Option<Mastery>,
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
        mastery_level: Option<Mastery>,
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

        if self.strategy == Strategy::Strategy1 {
            // No mastery level considered, simply choose next difficulty
            let next_index = current_index + 1;
            let next_index = next_index.min(difficulties.len() - 1); // Ensure index is within bounds
            let next_index = next_index.max(0); // Ensure index is within bounds

            let next_difficulty = difficulties[next_index].clone();

            let lesson_with_difficulty = self.q_table.keys().find(|(_, d)| d == &next_difficulty);

            (
                lesson_with_difficulty.unwrap().0.clone(),
                difficulties[next_index].clone(),
            )
        } else {
            let mut next_index = match mastery_level.unwrap() {
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
                // Drop a level if below basic mastery
                _ => {
                    if current_index == 0 {
                        0
                    } else {
                        current_index - 1
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
    }

    /// Update the value of some state-action pair, based on a lesson result
    /// from a learner.
    pub fn update(
        &mut self,
        state: (Lesson, DifficultyLevel),
        lesson_result: &LessonResult,
    ) -> Option<Mastery> {
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
        let time_taken_weight = 0.3 * difficulty_weight;
        let incorrect_attempts_weight = 0.5 * difficulty_weight;
        let hints_requested_weight = 0.2 * difficulty_weight;

        // Overall reward calculation - the less time someone takes, the less incorrect they are, and less hints they request,
        // the higher the reward should be overall

        // The time taken is currently simulated as follows:
        // let time_taken = match current_lesson.clone().get_difficulty_level() {
        //     DifficultyLevel::VeryEasy => {
        //         // Simulate quicker time for very easy lessons.
        //         (rand::thread_rng().gen::<f64>() * 5.0) + 5.0 // Random time between 5 to 10 seconds.
        //     }
        //     DifficultyLevel::Easy => {
        //         (rand::thread_rng().gen::<f64>() * 5.0) + 10.0 // Random time between 10 to 15 seconds.
        //     }
        //     DifficultyLevel::Medium => {
        //         (rand::thread_rng().gen::<f64>() * 10.0) + 20.0 // Random time between 20 to 30 seconds.
        //     }
        //     DifficultyLevel::Hard => {
        //         (rand::thread_rng().gen::<f64>() * 10.0) + 30.0 // Random time between 30 to 40 seconds.
        //     }
        //     DifficultyLevel::VeryHard => {
        //         (rand::thread_rng().gen::<f64>() * 10.0) + 40.0 // Random time between 40 to 50 seconds.
        //     }
        //     DifficultyLevel::Expert => {
        //         (rand::thread_rng().gen::<f64>() * 10.0) + 50.0 // Random time between 50 to 60 seconds.
        //     }
        //     DifficultyLevel::Master => {
        //         (rand::thread_rng().gen::<f64>() * 10.0) + 60.0 // Random time between 60 to 70 seconds.
        //     }
        //     DifficultyLevel::Grandmaster => {
        //         (rand::thread_rng().gen::<f64>() * 10.0) + 70.0 // Random time between 70 to 80 seconds.
        //     }
        // } as i32;
        // So, this means when calculating the time taken reward, we need to normalize the time taken
        // so that the time taken for a particular difficulty level is relatively calculated. This means that
        // 6 seconds at VeryEasy is a high reward and positive outcome, or 75 seconds in Grandmaster is also
        // a high reward and positive outcome.

        // Hence, calculate the time taken reward as follows:
        let time_taken_range_for_difficulty = match state.1 {
            DifficultyLevel::VeryEasy => (5.0, 10.0),
            DifficultyLevel::Easy => (10.0, 15.0),
            DifficultyLevel::Medium => (20.0, 30.0),
            DifficultyLevel::Hard => (30.0, 40.0),
            DifficultyLevel::VeryHard => (40.0, 50.0),
            DifficultyLevel::Expert => (50.0, 60.0),
            DifficultyLevel::Master => (60.0, 70.0),
            DifficultyLevel::Grandmaster => (70.0, 80.0),
        };

        let time_taken_reward = if total_time_taken <= time_taken_range_for_difficulty.0 {
            1.0
        } else {
            // Here, suppose someone took 89 seconds in Grandmaster. They shouldn't be penalised harshly because 9 seconds over is
            // still not bad for grandmaster, where the total time expected is 70-80 seconds. So, it should be like a normal distrubtion
            // where the penalty is less harsh the closer you are to the expected time, but much harsher when you get quite far away.

            // So, we can calculate the penalty as follows:
            let time_taken_range =
                time_taken_range_for_difficulty.1 - time_taken_range_for_difficulty.0;
            // The total time taken is 89 seconds, and the expected time is 70-80 seconds. So, the time taken is 9 seconds over the expected time.
            // So, the penalty should be 9 seconds over the expected time, divided by the total time range, which is 10 seconds. So, the penalty
            // should be 0.9. So, the reward should be 1.0 - 0.9 = 0.1.
            // However, this is not what we want as the distance that the time taken is relative to the size of expected time is not accounted for.
            // So, we can do the following to account for the distance. We can calculate the distance as a percentage of the expected time, for now
            // this will be the middle of the expected time range. So, the distance is 9 seconds over 75 seconds, which is 12%. So, the penalty should
            // be 12% of the total time range, which is 1.2 seconds. So, the reward should be 1.0 - 1.2 / 10 = 0.88.
            let distance_from_expected_time =
                total_time_taken - (time_taken_range_for_difficulty.1 - time_taken_range / 2.0);
            let distance_from_expected_time_percentage = distance_from_expected_time
                / (time_taken_range_for_difficulty.1 - time_taken_range_for_difficulty.0);
            let penalty = distance_from_expected_time_percentage * time_taken_range;
            1.0 - (penalty / time_taken_range)
        };

        // Calculate the incorrect attempts and hints requested rewards as follows:
        let incorrect_attempts_reward = if total_incorrect_attempts == 0 {
            1.0
        } else {
            // Here, suppose someone had 3 incorrect attempts out of 10 total attempts. So, the reward should be 1.0 - 0.3 = 0.7.
            let incorrect_attempts_percentage = total_incorrect_attempts as f32 / total_attempts;
            1.0 - incorrect_attempts_percentage
        };

        let hints_requested_reward = if total_hints_requested == 0 {
            1.0
        } else {
            // Here, suppose someone requested 2 hints out of 10 total attempts. So, the reward should be 1.0 - 0.2 = 0.8.
            let hints_requested_percentage = total_hints_requested as f32 / total_attempts;
            1.0 - hints_requested_percentage
        };

        // Calculate the overall reward as follows:
        let mut reward = (time_taken_weight * time_taken_reward
            + incorrect_attempts_weight * incorrect_attempts_reward
            + hints_requested_weight * hints_requested_reward)
            / difficulty_weight;

        reward = reward.max(-1.0).min(1.0);

        // Adjust the reward based on mastery thresholds, if strategy 2
        let mut mastery_level: Option<Mastery> = None;
        if self.strategy == Strategy::Strategy2 {
            mastery_level = if reward >= FULL_MASTERY_THRESHOLD {
                Some(Mastery::Full)
            } else if reward >= COMPETENT_MASTERY_THRESHOLD {
                Some(Mastery::Competent)
            } else if reward >= BASIC_MASTERY_THRESHOLD {
                Some(Mastery::Basic)
            } else {
                Some(Mastery::None)
            };

            reward = match mastery_level {
                Some(Mastery::Full) => 1.0, // Give full reward for the complete mastery
                Some(Mastery::Competent) => reward + 0.1, // Give some additional reward for competent mastery
                Some(Mastery::Basic) => reward, // No additional reward, but no penalty either for basic mastery
                _ => reward - 0.1,              // Penalize for lack of basic mastery
            };
        }

        let (next_state, _) = self.choose_next_difficulty(&state, mastery_level.clone());

        let next_max = self
            .q_table
            .iter()
            .filter(|((s, _), _)| s == &next_state)
            .map(|(_, &v)| v)
            .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let new_value =
            old_value + self.learning_rate * (reward + self.discount_factor * next_max - old_value);

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
