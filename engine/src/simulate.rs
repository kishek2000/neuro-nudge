use crate::simulated_learners::generate_simulated_learners_with_q_tables;
use serde_json::{json, Value};
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::vec;
use types::content::{DifficultyLevel, Lesson, LessonPlan, LessonResult, QuestionAttempt};
use types::engine::{Mastery, QTableAlgorithm, Strategy};
use types::learner::{ASDTraitComparison, ASDTraits, Learner};

use crate::{simulated_content_actions, simulated_content_shapes};

use rand::Rng;

// Strategy 1: Only Q Learning with no mastery thresholds.
pub fn run_simulation_strategy_1(iterations: Option<u32>) {
    // Load lessons for the "Shapes" module using functions from simulated_content.rs.
    let lessons = simulated_content_shapes::generate_shapes_lessons();

    // Generate simulated learners with Q-tables.
    let (learner_ids, mut learners_with_q_tables) =
        generate_simulated_learners_with_q_tables(&lessons, Strategy::BaseQLearning);

    // Create a file to write simulation results (e.g., Q-tables).
    let output_file = File::create(format!(
        "strategy_1_simulation_results_i{}.json",
        iterations.unwrap_or(5000)
    ))
    .expect("Failed to create file");

    for (_, (learner, _)) in learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in shapes.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);
    }

    // Run the simulation.
    run_simulation(
        learner_ids,
        learners_with_q_tables,
        output_file,
        lessons.clone(),
        iterations,
    );
}

// Strategy 2: Only Q Learning with mastery thresholds.
pub fn run_simulation_strategy_2(iterations: Option<u32>) {
    // Load lessons from the "Shapes" module using functions from simulated_content.rs.
    let lessons = simulated_content_shapes::generate_shapes_lessons();

    // Generate simulated learners with Q-tables.
    let (learner_ids, mut learners_with_q_tables) =
        generate_simulated_learners_with_q_tables(&lessons, Strategy::MasteryThresholds);

    // Create a file to write simulation results (e.g., Q-tables).
    let output_file = File::create(format!(
        "strategy_2_simulation_results_i{}.json",
        iterations.unwrap_or(5000)
    ))
    .expect("Failed to create file");

    for (_, (learner, _)) in learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in shapes.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);
    }

    // Run the simulation.
    run_simulation(
        learner_ids,
        learners_with_q_tables,
        output_file,
        lessons.clone(),
        iterations,
    );
}

// Strategy 3: Q Learning with decaying q values for reinforced learning.
pub fn run_simulation_strategy_3(iterations: Option<u32>) {
    // Load lessons from the "Actions" module using functions from simulated_content.rs.
    let lessons = simulated_content_actions::generate_actions_lessons();

    // Generate simulated learners with Q-tables.
    let (learner_ids, mut learners_with_q_tables) =
        generate_simulated_learners_with_q_tables(&lessons, Strategy::DecayingQValues);

    // Create a file to write simulation results (e.g., Q-tables).
    let output_file = File::create(format!(
        "strategy_3_simulation_results_i{}.json",
        iterations.unwrap_or(5000)
    ))
    .expect("Failed to create file");

    for (_, (learner, _)) in learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in actions.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);
    }

    // Run the simulation.
    run_simulation(
        learner_ids,
        learners_with_q_tables,
        output_file,
        lessons.clone(),
        iterations,
    );
}

// Strategy 4: Q Learning with decaying q values for reinforced learning, alongside ASD Trait sentivity
pub fn run_simulation_strategy_4(iterations: Option<u32>) {
    // Load lessons from the "Actions" module using functions from simulated_content.rs.
    let lessons = simulated_content_actions::generate_actions_lessons();

    // Generate simulated learners with Q-tables.
    let (learner_ids, mut learners_with_q_tables) =
        generate_simulated_learners_with_q_tables(&lessons, Strategy::TraitSensitivity);

    // Create a file to write simulation results (e.g., Q-tables).
    let output_file = File::create(format!(
        "strategy_4_simulation_results_i{}.json",
        iterations.unwrap_or(5000)
    ))
    .expect("Failed to create file");

    for (_, (learner, _)) in learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in actions.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);
    }

    // Run the simulation.
    run_simulation(
        learner_ids,
        learners_with_q_tables,
        output_file,
        lessons.clone(),
        iterations,
    );
}

fn run_simulation(
    learner_ids: Vec<&str>,
    mut learners_with_q_tables: HashMap<String, (Learner, QTableAlgorithm)>,
    mut output_file: File,
    lessons: Vec<Lesson>,
    iterations: Option<u32>,
) {
    // Define the number of iterations for the simulation.
    let num_iterations = iterations.unwrap_or(5000);

    let mut iteration_jsons = vec![];

    // Outer Iterations loop.
    for iteration in 0..num_iterations {
        let mut values: Vec<Value> = vec![];

        // Main simulation loop.
        for learner_id in learner_ids.clone() {
            let (learner, q_table) = learners_with_q_tables.get_mut(learner_id).unwrap();

            let lesson = learner.get_current_lesson();
            // Get the lesson and difficulty level for the learner.
            let difficulty_level = lesson.clone().get_difficulty_level();

            // Simulate the learner attempting a lesson and get the lesson result.
            let lesson_result =
                simulate_lesson_attempt(&lesson, q_table.clone(), learner.get_asd_traits());

            // Update learner's Q-table based on lesson result.
            let mastery_level =
                update_q_table(q_table, lesson, difficulty_level.clone(), &lesson_result);

            // Write learner's Q-table to the output file.
            let value =
                write_q_table_to_file(learner_id, q_table, &lessons, difficulty_level.clone());
            values.push(value);

            // Choose the next lesson based on Q-table.
            let next_lesson = choose_lesson_based_on_q_table(q_table, &lesson, mastery_level);

            // Set the learner's next lesson.
            learner.set_current_lesson(next_lesson);
        }

        let iteration_json_obj = json!({
            "iteration": iteration + 1,
            "values": values
        });

        iteration_jsons.push(iteration_json_obj);
        // println!("Iteration {} completed...", iteration + 1);
    }

    let simulation_results = json!({ "iterations": iteration_jsons });

    // Write the simulation results to a file.
    write!(
        output_file,
        "{}",
        serde_json::to_string_pretty(&simulation_results).unwrap()
    )
    .expect("Failed to write to file");
}

fn choose_lesson_based_on_q_table(
    q_table: &QTableAlgorithm,
    current_lesson: &Lesson,
    mastery_level: Option<Mastery>,
) -> Lesson {
    q_table
        .epsilon_greedy_action(
            &(
                current_lesson.clone(),
                current_lesson.clone().get_difficulty_level(),
            ),
            mastery_level,
        )
        .0
}

fn simulate_lesson_attempt(
    current_lesson: &Lesson,
    current_learner_q_table: QTableAlgorithm,
    learner_asd_traits: &ASDTraits,
) -> LessonResult {
    // Generate a simulated lesson result.
    let mut question_attempts = Vec::new();
    let total_questions = current_lesson.get_questions().len();

    // Time taken to complete the lesson //
    let learner_attention_span = learner_asd_traits.get_attention_span();

    // Calculate the time taken based on lesson difficulty (in seconds).
    // However, it should also be influenced by the learner's attention span.
    // For example, if the learner has a low attention span, they will take longer to complete
    // the lesson.
    let generated_time_taken_by_difficulty = match current_lesson.clone().get_difficulty_level() {
        DifficultyLevel::VeryEasy => {
            // Simulate quicker time for very easy lessons.
            (rand::thread_rng().gen::<f64>() * 5.0) + 5.0 // Random time between 5 to 10 seconds.
        }
        DifficultyLevel::Easy => {
            (rand::thread_rng().gen::<f64>() * 5.0) + 10.0 // Random time between 10 to 15 seconds.
        }
        DifficultyLevel::Medium => {
            (rand::thread_rng().gen::<f64>() * 10.0) + 20.0 // Random time between 20 to 30 seconds.
        }
        DifficultyLevel::Hard => {
            (rand::thread_rng().gen::<f64>() * 10.0) + 30.0 // Random time between 30 to 40 seconds.
        }
        DifficultyLevel::VeryHard => {
            (rand::thread_rng().gen::<f64>() * 10.0) + 40.0 // Random time between 40 to 50 seconds.
        }
        DifficultyLevel::Expert => {
            (rand::thread_rng().gen::<f64>() * 10.0) + 50.0 // Random time between 50 to 60 seconds.
        }
        DifficultyLevel::Master => {
            (rand::thread_rng().gen::<f64>() * 10.0) + 60.0 // Random time between 60 to 70 seconds.
        }
        DifficultyLevel::Grandmaster => {
            (rand::thread_rng().gen::<f64>() * 10.0) + 70.0 // Random time between 70 to 80 seconds.
        }
    } as i32;

    let mut total_time_taken = generated_time_taken_by_difficulty as f64;

    if current_learner_q_table.get_strategy() == &Strategy::TraitSensitivity {
        // Attention span is given in minutes, so convert it to seconds for comparison
        let attention_span_seconds = learner_attention_span * 60;

        // Calculate a factor representing the extent to which the generated time exceeds the attention span
        // This factor exponentially increases the time taken based on how much the generated time exceeds the attention span
        let time_excess_factor = if generated_time_taken_by_difficulty > attention_span_seconds {
            let excess_time = generated_time_taken_by_difficulty - attention_span_seconds;
            // The exponential factor could be adjusted as needed for realism
            let exponential_factor = 1.2;
            // Apply the exponential increase
            excess_time as f64 * exponential_factor
        } else {
            0.0 // No increase if within attention span
        };

        // Total time taken is the sum of generated time and the additional time due to attention span
        total_time_taken = generated_time_taken_by_difficulty as f64 + time_excess_factor;

        // Ensure total time taken is at least the generated time
        total_time_taken = total_time_taken.max(generated_time_taken_by_difficulty as f64);
    }

    // Each lesson has identical ASD trait parameters set
    let lesson_asd_traits = current_lesson.get_asd_traits_parameters();
    // Calculate the probability of answering correctly based on lesson difficulty.
    let mut correctness_factor: f32 = match current_lesson.clone().get_difficulty_level() {
        DifficultyLevel::VeryEasy => 0.95, // Easier lessons have a higher chance of correctness.
        DifficultyLevel::Easy => 0.85,
        DifficultyLevel::Medium => 0.7,
        DifficultyLevel::Hard => 0.6,
        DifficultyLevel::VeryHard => 0.55,
        DifficultyLevel::Expert => 0.5,
        DifficultyLevel::Master => 0.45,
        DifficultyLevel::Grandmaster => 0.4,
    };
    // ASD trait parameters - if the learner's ASD trait qualities are comparably lower
    // than the question's ASD trait parameters, the probability of success should decrease
    // accordingly, based on how much lower/different the learner's traits are.
    // This is the final strategy, strategy 4
    if current_learner_q_table.get_strategy() == &Strategy::TraitSensitivity {
        let alignment_score = learner_asd_traits.calculate_alignment(&lesson_asd_traits);

        let consecutive_attempts = current_learner_q_table
            .get_consecutive_attempts_for_difficulty(&current_lesson.clone().get_difficulty_level())
            .clone();

        // Although the alignment of traits should affect the probability of success,
        // it should not be the only factor. The learner should still have a chance of
        // success even if their traits are not aligned with the question's traits - especially
        // if they have consecutively made a large number of attempts.
        // Therefore, the alignment score is multiplied by a factor that is inversely proportional
        // to the number of consecutive attempts.

        // Using 0 as min and 4000 as max due to 5000 iterations being run and unlikely we exceed 4000
        let normalised_consecutive_attempts =
            (consecutive_attempts - 0.0) as f32 / (5000 - 0) as f32;

        correctness_factor = correctness_factor
            * (alignment_score + (normalised_consecutive_attempts * 20.0).min(1.0));
    }

    // Within the context of what we are solving, as a learner becomes more accustomed
    // to a particular difficulty or makes progress, their chances of success should increase.
    // While this doesn't mean mastery, it means it should at least increase, meaning the
    // correctness_factor variable above in turn should increase, **depending on if the learner
    // has made progress in that difficulty level**. We should still not make it too easy as
    // reinforcement is very important for ASD learners even on something they have learnt well
    // already, but we should make it easier than it was before.
    let current_q_value = current_learner_q_table
        .get(&(
            current_lesson.clone(),
            current_lesson.clone().get_difficulty_level(),
        ))
        .unwrap_or(&0.0);

    // If the learner has made progress in the current difficulty level, decrease the difficulty factor
    // by a factor that is relative to the progress.
    if current_q_value > &0.0 {
        correctness_factor += current_q_value * 0.1;
    }

    let mut attempts = 0;
    let mut is_correct = false;

    // Ultimately, if there is a very low chance, we still don't want the
    // correctness_factor to go any lower than 5%
    correctness_factor = correctness_factor.max(0.05);

    for question in current_lesson.get_questions() {
        while !is_correct {
            let rand_value = rand::thread_rng().gen::<f64>();
            // Simulate learner's answer attempt (random correctness).
            is_correct = rand_value < correctness_factor.into();

            // Increment the number of attempts.
            attempts += 1;
        }

        // Create a QuestionAttempt object.
        let question_attempt = QuestionAttempt::new(
            question.get_id().to_string(),
            (total_time_taken / total_questions as f64) as i32, // Time taken for each question on average.
            attempts, // Total attempts it took to get it right.
            max(0, attempts - 1),
        );

        question_attempts.push(question_attempt);
    }

    // Create a LessonResult.
    let lesson_result = LessonResult::new(
        current_lesson.clone().get_difficulty_level(),
        total_time_taken as i32, // Use the actual score or progress.
        total_questions as i32,  // Number of questions attempted.
        question_attempts,
    );

    lesson_result
}

fn update_q_table(
    q_table: &mut QTableAlgorithm,
    lesson: &Lesson,
    difficulty_level: DifficultyLevel,
    lesson_result: &LessonResult,
) -> Option<Mastery> {
    // Update the learner's Q-table based on the lesson result.
    let state = (lesson.clone(), difficulty_level);
    q_table.update(state, lesson_result)
}

fn write_q_table_to_file(
    learner_id: &str,
    q_table: &QTableAlgorithm,
    lessons: &Vec<Lesson>,
    difficulty_level: DifficultyLevel,
) -> Value {
    let very_easy = q_table
        .get(&(lessons[0].clone(), DifficultyLevel::VeryEasy))
        .unwrap_or(&0.0);
    let easy = q_table
        .get(&(lessons[1].clone(), DifficultyLevel::Easy))
        .unwrap_or(&0.0);
    let medium = q_table
        .get(&(lessons[2].clone(), DifficultyLevel::Medium))
        .unwrap_or(&0.0);
    let hard = q_table
        .get(&(lessons[3].clone(), DifficultyLevel::Hard))
        .unwrap_or(&0.0);
    let very_hard = q_table
        .get(&(lessons[4].clone(), DifficultyLevel::VeryHard))
        .unwrap_or(&0.0);
    let expert = q_table
        .get(&(lessons[5].clone(), DifficultyLevel::Expert))
        .unwrap_or(&0.0);
    let master = q_table
        .get(&(lessons[6].clone(), DifficultyLevel::Master))
        .unwrap_or(&0.0);
    let grandmaster = q_table
        .get(&(lessons[7].clone(), DifficultyLevel::Grandmaster))
        .unwrap_or(&0.0);

    let difficulty_str: &str = difficulty_level.clone().into();

    json!({
        "learner_id": learner_id,
        "values": {
            "VeryEasy": very_easy,
            "Easy": easy,
            "Medium": medium,
            "Hard": hard,
            "VeryHard": very_hard,
            "Expert": expert,
            "Master": master,
            "Grandmaster": grandmaster
        },
        "difficulty_level": difficulty_str
    })
}
