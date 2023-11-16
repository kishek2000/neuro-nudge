//! This module defines the recommendation engine, NeuroNudge.
//! NeuroNudge is an unsupervised learning algorithm for young kids
//! with ASD.
//!
//! The goal of this algorithm is to develop a learning profile of
//! young learners who have ASD and accordingly provide lessons that
//! they can use to make progress. It aims to use reinforcement
//! learning and be sensitive to the different factors that exist for
//! a learner with ASD, such as their duration in answering a question,
//! the number of incorrect attempts, and so forth.
//!
//! This project simply explores the development of the algorithm and
//! tests it with data that simulates some potential young learners
//! (using GPT). It does NOT provide an application experience for the
//! learning.
//!
//! The engine will be based on Reinforcement Learning.
//!
//! A rough guide of how the engine might operate:
//! 1. The algorithm starts with a set of default recommendations for
//!    the child's initial state.
//! 2. The child interacts with the e-learning platform (e.g., by
//!    answering questions or completing activities).
//! 3. Based on the child's interactions and their current state, the
//!    algorithm suggests new modules or activities, adjusting
//!    difficulty levels, as needed.
//! 4. The child's responses and reactions are continuously monitored.
//!    If the child is engaged and achieving learning objectives, the
//!    algorithm continues with similar recommendations.
//!    If the child seems disengaged or frustrated, the algorithm may
//!    adjust the content, provide hints, or modify the difficulty level.
//! 5. Over time, the algorithm learns from the child's behavior and
//!    refines its recommendations to maximize the child's learning
//!    experience and satisfaction.
//!
//! Note that the state and types for stuff like learner, lesson etc are
//! defined in the `types` module.
//!

pub mod simulate;
pub mod simulated_content;
pub mod simulated_learners;

fn main() {
    println!(">> Welcome to NeuroNudge!");

    loop {
        // Ask which strategy you want to simulate
        println!(">> Which strategy do you want to simulate?");
        println!(">> 1. Simulate Q Learning without Mastery Thresholds");
        println!(">> 2. Simulate Q Learning with Mastery Thresholds");
        println!(">> 3. (todo - form a 3rd strategy)");
        println!(">> Q: Quit NeuroNudge");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "Q" || input == "q" {
            println!(">> Quitting NeuroNudge...");
            break;
        }

        if input != "1" && input != "2" && input != "3" {
            println!(">> Invalid input. Please try again.");
            continue;
        }

        let strategy = input.parse::<u8>().unwrap();

        if strategy == 1 {
            println!(
                ">> You have selected Strategy 1: Simulate Q Learning without Mastery Thresholds"
            );

            println!(">> Strategy 1: Running simulation now...");
            simulate::run_simulation_strategy_1();
            println!(">> Strategy 1: Simulation complete!");
        } else if strategy == 2 {
            println!(
                ">> You have selected Strategy 2: Simulate Q Learning with Mastery Thresholds"
            );

            println!(">> Strategy 2: Running simulation now...");
            simulate::run_simulation_strategy_2();
            println!(">> Strategy 2: Simulation complete!");
        } else if strategy == 3 {
            println!(">> You have selected Strategy 3: (todo - form a 3rd strategy)");
        }
    }
}
