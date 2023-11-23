//! This defines the Q Learning algorithm.
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

use std::collections::HashMap;

use rand::Rng;

use crate::content::{DifficultyLevel, Lesson, LessonResult};

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
    decay_counters: HashMap<DifficultyLevel, f32>,
    /// Keeps track of how many attempts have passed since a particular difficulty
    /// level was attempted
    total_difficulty_non_attempts: HashMap<DifficultyLevel, f32>,
    has_attempted_difficulty: HashMap<DifficultyLevel, bool>,
    consecutive_attempts: HashMap<DifficultyLevel, f32>,
}

/// Strategy used by the engine
#[derive(Debug, Clone, PartialEq)]
pub enum Strategy {
    BaseQLearning,
    MasteryThresholds,
    DecayingQValues,
    TraitSensitivity,
}

impl QTableAlgorithm {
    pub fn new(q_table: Option<QTable>, epsilon: f32, strategy: Strategy) -> QTableAlgorithm {
        let mut decay_counters = HashMap::new();
        let mut total_difficulty_non_attempts = HashMap::new();
        let mut consecutive_attempts = HashMap::new();

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

        for difficulty in &difficulties {
            let total_decays_expected = match difficulty {
                DifficultyLevel::VeryEasy => 2.0,
                DifficultyLevel::Easy => 3.0,
                DifficultyLevel::Medium => 4.0,
                DifficultyLevel::Hard => 5.0,
                DifficultyLevel::VeryHard => 6.0,
                DifficultyLevel::Expert => 7.0,
                DifficultyLevel::Master => 8.0,
                DifficultyLevel::Grandmaster => 9.0,
            };

            decay_counters.insert(difficulty.clone(), total_decays_expected);
            total_difficulty_non_attempts.insert(difficulty.clone(), 0.0);
            consecutive_attempts.insert(difficulty.clone(), 0.0);
        }

        QTableAlgorithm {
            id: uuid::Uuid::new_v4().to_string(),
            q_table: q_table.unwrap_or(HashMap::new()),
            discount_factor: 0.25,
            learning_rate: 0.75,
            epsilon,
            strategy,
            decay_counters,
            total_difficulty_non_attempts,
            has_attempted_difficulty: HashMap::new(),
            consecutive_attempts,
        }
    }

    pub fn get_strategy(&self) -> &Strategy {
        &self.strategy
    }

    pub fn get_consecutive_attempts_for_difficulty(
        &self,
        difficulty_level: &DifficultyLevel,
    ) -> &f32 {
        self.consecutive_attempts.get(difficulty_level).unwrap()
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

    /// Determine if a particular difficulty level is weak in progress
    fn is_weak_level(&self, difficulty_level: &DifficultyLevel) -> bool {
        let current_value = self
            .q_table
            .iter()
            .filter(|((_, d), _)| d == difficulty_level)
            .map(|(_, &v)| v)
            .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        // Threshold for weakness, can be adjusted
        let threshold = 0.5;
        current_value <= threshold
    }

    /// Find the weakest difficulty level by q value, but ensure that the
    /// Level has actually been attempted before as well. The weakness of a level
    /// is the low q value, but if it's not true in the has_attempted_difficulty
    /// then it hasn't even been attempted.
    fn find_weaker_level(&self) -> Option<DifficultyLevel> {
        self.decay_counters
            .keys()
            // Find the weak levels
            .filter(|&level| self.is_weak_level(level))
            // Find the levels that have been attempted
            .find(|&level| {
                let has_attempted = self.has_attempted_difficulty.get(level).unwrap_or(&false);
                *has_attempted
            })
            // Return the weakest of the lot by q value
            .map(|level| {
                self.q_table
                    .iter()
                    .filter(|((_, d), _)| d == level)
                    .map(|(_, &v)| v)
                    .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(0.0);
                level.clone()
            })
    }

    // Epsilon-greedy strategy to choose the next action
    pub fn epsilon_greedy_action(
        &self,
        state: &(Lesson, DifficultyLevel),
        mastery_level: Option<Mastery>,
    ) -> (Lesson, DifficultyLevel) {
        let rand_value = rand::thread_rng().gen::<f32>();
        if rand_value < self.epsilon {
            if self.strategy == Strategy::DecayingQValues
                || self.strategy == Strategy::TraitSensitivity
            {
                // Exploration: Modified to prioritize weaker levels
                let weaker_level = self.find_weaker_level();
                if let Some(level) = weaker_level {
                    // Return an action for the weaker level
                    self.q_table
                        .keys()
                        .find(|(_, d)| d == &level)
                        .unwrap()
                        .clone()
                } else {
                    // If no weaker level, choose the next difficulty level
                    self.choose_next_difficulty(state, mastery_level)
                }
            } else {
                // Exploration: choose the next difficulty level.
                self.choose_next_difficulty(state, mastery_level)
            }
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

        // If we're in strategy 3 (decaying q values) then ensure that
        // decayed levels - i.e. potentially weak levels, are prioritized
        let current_difficulty = state.1.clone();
        let is_current_weak = self.is_weak_level(&current_difficulty);
        if (self.strategy == Strategy::DecayingQValues
            || self.strategy == Strategy::TraitSensitivity)
            && is_current_weak
        {
            // Balance between reinforcing a weak level and moving to a higher difficulty
            return self
                .q_table
                .keys()
                .find(|(_, d)| d == &current_difficulty)
                .unwrap()
                .clone();
        }

        if self.strategy == Strategy::BaseQLearning {
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

        self.has_attempted_difficulty.insert(state.1.clone(), true);

        // Update the consecutive attempts counter
        self.consecutive_attempts = self
            .consecutive_attempts
            .iter()
            .map(|(d, &v)| {
                if d != &state.1 {
                    (d.clone(), 0.0)
                } else {
                    (d.clone(), v + 1.0)
                }
            })
            .collect();

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
        // DifficultyLevel::VeryEasy => // Random time between 5 to 10 seconds.
        // DifficultyLevel::Easy => // Random time between 10 to 15 seconds.
        // DifficultyLevel::Medium => // Random time between 20 to 30 seconds.
        // DifficultyLevel::Hard => { // Random time between 30 to 40 seconds.
        // DifficultyLevel::VeryHard => { // Random time between 40 to 50 seconds.
        // DifficultyLevel::Expert => { // Random time between 50 to 60 seconds.
        // DifficultyLevel::Master => { // Random time between 60 to 70 seconds.
        // DifficultyLevel::Grandmaster => { // Random time between 70 to 80 seconds.

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

        // Adjust the reward based on mastery thresholds, if strategy isn't basic q learning
        let mut mastery_level: Option<Mastery> = None;
        if self.strategy != Strategy::BaseQLearning {
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

        self.q_table.insert(state.clone(), new_value.min(1.0)); // Ensure that the value is between 0 and 1

        self.update_difficulty_non_attempts(lesson_difficulty.clone());

        // If we're in strategy 3 (decaying q values) or 4 (trait sensitivty) then apply decay
        if self.strategy == Strategy::DecayingQValues || self.strategy == Strategy::TraitSensitivity
        {
            self.apply_decay();
        }

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

    fn update_difficulty_non_attempts(&mut self, attempted_difficulty_level: DifficultyLevel) {
        self.total_difficulty_non_attempts = self
            .total_difficulty_non_attempts
            .iter()
            .map(|(d, &v)| {
                let has_attempted = self.has_attempted_difficulty.get(d).unwrap_or(&false);
                if !has_attempted {
                    (d.clone(), v)
                } else if d == &attempted_difficulty_level {
                    (d.clone(), 0.0) // Reset the counter for the difficulty level that was attempted
                } else {
                    (d.clone(), v + 1.0)
                }
            })
            .collect();
    }

    pub fn apply_decay(&mut self) {
        // Apply decay to Q-values for difficulty levels based on decay_counters
        // Adjust the rate of decay or interval for decay events using an exponential backoff strategy
        self.q_table = self
            .q_table
            .iter()
            .map(|((l, d), &v)| {
                let non_attempts_counter =
                    self.total_difficulty_non_attempts.get(d).unwrap_or(&0.0);

                let required_non_attempts_to_apply_decay = match d {
                    DifficultyLevel::VeryEasy => 2000.0,
                    DifficultyLevel::Easy => 1750.0,
                    DifficultyLevel::Medium => 1600.0,
                    DifficultyLevel::Hard => 1400.0,
                    DifficultyLevel::VeryHard => 1200.0,
                    DifficultyLevel::Expert => 1050.0,
                    DifficultyLevel::Master => 900.0,
                    DifficultyLevel::Grandmaster => 750.0,
                };

                let decay_counter = self.decay_counters.get(d).unwrap_or(&0.0);
                let do_decay = non_attempts_counter >= &required_non_attempts_to_apply_decay
                    && decay_counter > &0.0;

                if do_decay {
                    let decay_rate = 1.0 / decay_counter;
                    self.decay_counters.insert(d.clone(), decay_counter - 1.0);
                    self.total_difficulty_non_attempts.insert(d.clone(), 0.0);
                    ((l.clone(), d.clone()), v * decay_rate)
                } else {
                    ((l.clone(), d.clone()), v)
                }
            })
            .collect();
    }
}
