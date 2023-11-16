use crate::simulated_learners::{
    generate_simulated_learners_for_strategy_3, generate_simulated_learners_with_q_tables,
};
use serde_json::{json, Value};
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::vec;
use types::content::{
    ContentModule, DifficultyLevel, Lesson, LessonPlan, LessonResult, QuestionAttempt,
};
use types::engine::{CollaborativeFilteringAlgorithm, Mastery, QTableAlgorithm, Strategy};
use types::learner::Learner;

use crate::simulated_content;

use rand::Rng;

// Strategy 1: Only Q Learning with no mastery thresholds.
pub fn run_simulation_strategy_1() {
    // Load lessons for the "Shapes" module using functions from simulated_content.rs.
    let lessons = simulated_content::generate_shapes_lessons();

    // Generate simulated learners with Q-tables.
    let (learner_ids, mut learners_with_q_tables) =
        generate_simulated_learners_with_q_tables(&lessons, Strategy::Strategy1);

    // Create a file to write simulation results (e.g., Q-tables).
    let output_file =
        File::create("strategy_1_simulation_results.json").expect("Failed to create file");

    for (_, (learner, _)) in learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in shapes.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);
    }

    // Run the simulation.
    run_q_learning_simulation(
        learner_ids,
        learners_with_q_tables,
        output_file,
        lessons.clone(),
    );
}

// Strategy 2: Only Q Learning with mastery thresholds.
pub fn run_simulation_strategy_2() {
    // Load lessons from the "Shapes" module using functions from simulated_content.rs.
    let lessons = simulated_content::generate_shapes_lessons();

    // Generate simulated learners with Q-tables.
    let (learner_ids, mut learners_with_q_tables) =
        generate_simulated_learners_with_q_tables(&lessons, Strategy::Strategy2);

    // Create a file to write simulation results (e.g., Q-tables).
    let output_file =
        File::create("strategy_2_simulation_results.json").expect("Failed to create file");

    for (_, (learner, _)) in learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in shapes.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);
    }

    // Run the simulation.
    run_q_learning_simulation(
        learner_ids,
        learners_with_q_tables,
        output_file,
        lessons.clone(),
    );
}

// Strategy 3: Collaborative Filtering combined with Q learning using mastery thresholds.
pub fn run_simulation_strategy_3() {
    let mut shapes = ContentModule::new("Shapes".to_string());

    // Load lessons from the "Shapes" module using functions from simulated_content.rs.
    let lessons = simulated_content::generate_shapes_lessons();

    for lesson in lessons.clone() {
        shapes.add_lesson(lesson);
    }

    // Generate simulated learners with Q-tables.
    let (
        (base_sim_learner_ids, mut base_sim_learners_with_q_tables),
        (collab_sim_learner_ids, mut collab_sim_learners_with_q_tables),
    ) = generate_simulated_learners_for_strategy_3(&lessons, Strategy::Strategy2);

    // Create a file to write simulation results (e.g., Q-tables).
    let output_file =
        File::create("strategy_3a_simulation_results.json").expect("Failed to create file");

    // Initialise the collaborative algorithm
    let mut collaborative_algorithm = CollaborativeFilteringAlgorithm::new();

    // From here, the steps are as follows:
    // 1 - run the base q learning simulation for the first 3 learners
    // 2 - run a new simulation with same number of iterations where CollaborativeFilteringAlgorithm is used instead
    //     to generate a recommendation for the next lesson.

    // Base simulation
    for (_, (learner, _)) in base_sim_learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in shapes.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);
    }

    base_sim_learners_with_q_tables = run_q_learning_simulation(
        base_sim_learner_ids,
        base_sim_learners_with_q_tables.clone(),
        output_file,
        lessons.clone(),
    );

    println!(
        "{:?}",
        &base_sim_learners_with_q_tables
            .get("Learner 1")
            .unwrap()
            .1
            .get_first_5()
    );

    for (_, (learner, q_table)) in base_sim_learners_with_q_tables.iter_mut() {
        let mut learner_q_tables = HashMap::new();
        learner_q_tables.insert(shapes.clone(), q_table.clone());
        collaborative_algorithm.add_learner(learner.clone(), learner_q_tables);
        println!("Simulation: Added learner {}", learner.get_id());
    }

    for (_, (learner, q_table)) in collab_sim_learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in shapes.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);

        let mut learner_q_tables = HashMap::new();
        learner_q_tables.insert(shapes.clone(), q_table.clone());
        collaborative_algorithm.add_learner(learner.clone(), learner_q_tables);
        println!("Simulation: Added learner {}", learner.get_id());
    }

    // New simulation
    let num_iterations = 250;

    // Create a file to write simulation results (e.g., Q-tables).
    let mut collab_output_file =
        File::create("strategy_3b_simulation_results.json").expect("Failed to create file");

    let mut iteration_jsons = vec![];

    // Outer Iterations loop.
    for iteration in 0..num_iterations {
        println!("Iteration: {}", iteration + 1);

        let mut values: Vec<Value> = vec![];

        // Main simulation loop.
        for learner_id in collab_sim_learner_ids.clone() {
            let (learner, q_table) = collab_sim_learners_with_q_tables
                .get_mut(learner_id)
                .unwrap();

            let lesson = learner.get_current_lesson();
            // Get the lesson and difficulty level for the learner.
            let difficulty_level = lesson.clone().get_difficulty_level(); // Replace with your logic to get the difficulty level.

            // Simulate the learner attempting a lesson and get the lesson result.
            let lesson_result = simulate_lesson_attempt(&lesson, q_table.clone());

            // Update learner's Q-table based on lesson result.
            println!(
                "Learner {} q table state before: {}",
                learner.get_id(),
                q_table
                    .get(&(lesson.clone(), lesson.clone().get_difficulty_level()))
                    .unwrap()
            );
            update_q_table(q_table, lesson, difficulty_level.clone(), &lesson_result);
            println!(
                "Learner {} q table state after: {}",
                learner.get_id(),
                q_table
                    .get(&(lesson.clone(), lesson.clone().get_difficulty_level()))
                    .unwrap()
            );

            // Write learner's Q-table to the output file.
            let value =
                write_q_table_to_file(learner_id, q_table, &lessons, difficulty_level.clone());
            values.push(value);

            // Choose the next lesson based on Q-table (you need to implement this logic).
            let next_lesson = choose_lesson_based_on_collaborative_filtering(
                learner.clone(),
                shapes.clone(),
                collaborative_algorithm.clone(),
                &lessons,
            );

            println!(
                "Recommended next lesson: {:?}",
                next_lesson.get_difficulty_level()
            );

            // Set the learner's next lesson.
            learner.set_current_lesson(next_lesson);
        }

        let iteration_json_obj = json!({
            "iteration": iteration + 1,
            "values": values
        });

        iteration_jsons.push(iteration_json_obj);
    }

    let simulation_results = json!({ "iterations": iteration_jsons });

    // Write the simulation results to a file.
    write!(
        collab_output_file,
        "{}",
        serde_json::to_string_pretty(&simulation_results).unwrap()
    )
    .expect("Failed to write to file");
}

fn run_q_learning_simulation(
    learner_ids: Vec<&str>,
    mut learners_with_q_tables: HashMap<String, (Learner, QTableAlgorithm)>,
    mut output_file: File,
    lessons: Vec<Lesson>,
) -> HashMap<String, (Learner, QTableAlgorithm)> {
    // Define the number of iterations for the simulation.
    let num_iterations = 250; // You can adjust this as needed.

    let mut iteration_jsons = vec![];

    // Outer Iterations loop.
    for iteration in 0..num_iterations {
        println!("Iteration: {}", iteration + 1);

        let mut values: Vec<Value> = vec![];

        // Main simulation loop.
        for learner_id in learner_ids.clone() {
            let (learner, q_table) = learners_with_q_tables.get_mut(learner_id).unwrap();

            let lesson = learner.get_current_lesson();
            // Get the lesson and difficulty level for the learner.
            let difficulty_level = lesson.clone().get_difficulty_level(); // Replace with your logic to get the difficulty level.

            // Simulate the learner attempting a lesson and get the lesson result.
            let lesson_result = simulate_lesson_attempt(&lesson, q_table.clone());

            // Update learner's Q-table based on lesson result.
            let mastery_level =
                update_q_table(q_table, lesson, difficulty_level.clone(), &lesson_result);

            // Write learner's Q-table to the output file.
            let value =
                write_q_table_to_file(learner_id, q_table, &lessons, difficulty_level.clone());
            values.push(value);

            // Choose the next lesson based on Q-table (you need to implement this logic).
            let next_lesson = choose_lesson_based_on_q_table(q_table, &lesson, mastery_level);

            // Set the learner's next lesson.
            learner.set_current_lesson(next_lesson);
        }

        let iteration_json_obj = json!({
            "iteration": iteration + 1,
            "values": values
        });

        iteration_jsons.push(iteration_json_obj);
    }

    let simulation_results = json!({ "iterations": iteration_jsons });

    // Write the simulation results to a file.
    write!(
        output_file,
        "{}",
        serde_json::to_string_pretty(&simulation_results).unwrap()
    )
    .expect("Failed to write to file");

    learners_with_q_tables
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

fn choose_lesson_based_on_collaborative_filtering(
    learner: Learner,
    module: ContentModule,
    collaborative_algorithm: CollaborativeFilteringAlgorithm,
    lessons: &Vec<Lesson>,
) -> Lesson {
    let recommended_difficulty = collaborative_algorithm
        .recommend_lesson_difficulty(&learner, &module)
        .unwrap();

    let lesson = lessons
        .iter()
        .find(|l| l.get_difficulty_level().eq(&recommended_difficulty.clone()));

    lesson.unwrap().clone()
}

fn simulate_lesson_attempt(
    current_lesson: &Lesson,
    current_learner_q_table: QTableAlgorithm,
) -> LessonResult {
    // Generate a simulated lesson result.
    let mut question_attempts = Vec::new();
    let num_attempts = current_lesson.get_questions().len(); // Number of questions in the lesson.
                                                             // Calculate the time taken based on lesson difficulty (in seconds).
    let time_taken = match current_lesson.clone().get_difficulty_level() {
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

    for question in current_lesson.get_questions() {
        // Calculate the probability of answering correctly based on lesson difficulty.
        let mut correctness_factor = match current_lesson.clone().get_difficulty_level() {
            DifficultyLevel::VeryEasy => 0.95, // Easier lessons have a higher chance of correctness.
            DifficultyLevel::Easy => 0.8,
            DifficultyLevel::Medium => 0.65,
            DifficultyLevel::Hard => 0.6,
            DifficultyLevel::VeryHard => 0.55,
            DifficultyLevel::Expert => 0.5,
            DifficultyLevel::Master => 0.45,
            DifficultyLevel::Grandmaster => 0.4,
        };

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
            time_taken / num_attempts as i32, // Time taken for each question on average.
            attempts,                         // Total attempts it took to get it right.
            max(0, attempts - 1),
        );

        question_attempts.push(question_attempt);
    }

    // Create a LessonResult.
    let lesson_result = LessonResult::new(
        current_lesson.clone().get_difficulty_level(),
        time_taken,          // Use the actual score or progress.
        num_attempts as i32, // Number of questions attempted.
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
