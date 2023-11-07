use crate::simulated_learners::generate_simulated_learners_with_q_tables;
use std::cmp::max;
use std::fs::File;
use std::io::Write;
use types::content::{DifficultyLevel, Lesson, LessonPlan, LessonResult, QuestionAttempt};
use types::engine::QTableAlgorithm;

use crate::simulated_content;

use rand::Rng;

pub fn run_simulation() {
    // // Load the ContentModule (e.g., "Shapes") with lessons`` (use functions from simulated_content.rs).
    // // For now, we'll use a single module with lessons as an example.
    // let shapes_module = ContentModule::new("Shapes".to_string());
    // Load lessons for the "Shapes" module using functions from simulated_content.rs.
    let lessons = simulated_content::generate_shapes_lessons();

    // Generate simulated learners with Q-tables.
    let mut learners_with_q_tables = generate_simulated_learners_with_q_tables();

    // Define the number of iterations for the simulation.
    let num_iterations = 100000; // You can adjust this as needed.

    // Create a file to write simulation results (e.g., Q-tables).
    let mut output_file = File::create("simulation_results.txt").expect("Failed to create file");

    for (_, (learner, _)) in learners_with_q_tables.iter_mut() {
        // Initialise with first lesson in shapes.
        let mut lesson_plan = LessonPlan::new("Lesson 1".to_string());
        lesson_plan.add_lesson(lessons[0].clone());
        learner.add_lesson_plan(lesson_plan);
    }

    // Outer Iterations loop.
    for iteration in 0..num_iterations {
        println!("Iteration: {}", iteration);
        // Main simulation loop.
        for (learner_id, (learner, q_table)) in learners_with_q_tables.iter_mut() {
            let lesson = learner.get_current_lesson();
            // Get the lesson and difficulty level for the learner.
            let difficulty_level = lesson.clone().get_difficulty_level(); // Replace with your logic to get the difficulty level.

            // Simulate the learner attempting a lesson and get the lesson result.
            let lesson_result = simulate_lesson_attempt(&lesson);

            // Update learner's Q-table based on lesson result.
            update_q_table(q_table, lesson, difficulty_level, &lesson_result);

            // Write learner's Q-table to the output file.
            write_q_table_to_file(learner_id, q_table, &mut output_file, &lessons);

            // Choose the next lesson based on Q-table (you need to implement this logic).
            let next_lesson = choose_lesson_based_on_q_table(q_table, &lesson, lessons.clone());

            // Set the learner's next lesson.
            learner.set_current_lesson(next_lesson);
        }
    }
}

fn choose_lesson_based_on_q_table(
    q_table: &QTableAlgorithm,
    current_lesson: &Lesson,
    lessons: Vec<Lesson>,
) -> Lesson {
    let epsilon: f32 = 0.1; // You can adjust this as needed.
    let rand_value = rand::thread_rng().gen::<f32>();
    println!("{}", rand_value);
    if rand_value < epsilon {
        // Random exploration: choose a random lesson.
        let lesson = lessons[rand::thread_rng().gen_range(0..lessons.len())].clone();
        println!("{:?}", lesson.clone().get_difficulty_level());
        lesson
    } else {
        // Exploitation: choose the lesson with the highest Q-value.
        let possible_difficulty_levels = vec![
            DifficultyLevel::VeryEasy,
            DifficultyLevel::Easy,
            DifficultyLevel::Medium,
            DifficultyLevel::Hard,
            DifficultyLevel::VeryHard,
            DifficultyLevel::Expert,
            DifficultyLevel::Master,
            DifficultyLevel::Grandmaster,
        ];

        let mut best_lesson = current_lesson;
        let mut best_q_value = f32::NEG_INFINITY;

        for difficulty_level in possible_difficulty_levels {
            let state = (current_lesson.clone(), difficulty_level.clone());

            if let Some(q_value) = q_table.get(&state) {
                if *q_value > best_q_value {
                    best_lesson = current_lesson;
                    best_q_value = *q_value;
                }
            }
        }

        best_lesson.clone()
    }
}

fn simulate_lesson_attempt(current_lesson: &Lesson) -> LessonResult {
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
        let difficulty_factor = match current_lesson.clone().get_difficulty_level() {
            DifficultyLevel::VeryEasy => 0.9, // Easier lessons have a higher chance of correctness.
            DifficultyLevel::Easy => 0.8,
            DifficultyLevel::Medium => 0.7,
            DifficultyLevel::Hard => 0.6,
            DifficultyLevel::VeryHard => 0.5, // Harder lessons have a lower chance of correctness.
            DifficultyLevel::Expert => 0.4,
            DifficultyLevel::Master => 0.3,
            DifficultyLevel::Grandmaster => 0.2,
        };

        let mut attempts = 0;
        let mut is_correct = false;

        while !is_correct {
            // Simulate learner's answer attempt (random correctness).
            is_correct = rand::thread_rng().gen::<f64>() < difficulty_factor;

            // Increment the number of attempts.
            attempts += 1;
        }

        // Calculate the score (for simplicity, you can adjust scoring logic).
        let score = if is_correct { 10 } else { 0 };

        // Create a QuestionAttempt object.
        let question_attempt = QuestionAttempt::new(
            question.get_id().to_string(),
            score,
            attempts, // Total attempts it took to get it right.
            max(0, attempts - 1),
        );

        question_attempts.push(question_attempt);
    }

    // Calculate the time taken

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
) {
    // Update the learner's Q-table based on the lesson result.
    let state = (lesson.clone(), difficulty_level);
    q_table.update(state, lesson_result);
}

fn write_q_table_to_file(
    learner_id: &str,
    q_table: &QTableAlgorithm,
    file: &mut File,
    lessons: &Vec<Lesson>,
) {
    // Write learner's ID to the output file.
    writeln!(file, "Learner ID: {}\n", learner_id).expect("Failed to write to file");

    // Write header row with tab spacing.
    writeln!(
        file,
        "Lesson\tVeryEasy\tEasy\tMedium\tHard\tVeryHard\tExpert\tMaster\tGrandmaster"
    )
    .expect("Failed to write to file");

    // Iterate over lessons and difficulties and write Q-values with tab spacing.
    for lesson in lessons {
        write!(file, "{}\t", lesson.get_name()).expect("Failed to write to file");

        let difficulty_levels = vec![
            DifficultyLevel::VeryEasy,
            DifficultyLevel::Easy,
            DifficultyLevel::Medium,
            DifficultyLevel::Hard,
            DifficultyLevel::VeryHard,
            DifficultyLevel::Expert,
            DifficultyLevel::Master,
            DifficultyLevel::Grandmaster,
        ];

        for difficulty in difficulty_levels {
            let state = (lesson.clone(), difficulty);
            let q_value = q_table.get(&state).cloned().unwrap_or(0.0); // Get Q-value or default to 0.0.
            write!(file, "{:^7}\t", q_value).expect("Failed to write to file");
        }

        writeln!(file).expect("Failed to write to file");
    }

    writeln!(file, "\n").expect("Failed to write to file");
}
