//! This module defines the recommendation engine, NeuroNudge.
//! NeuroNudge is an unsupervised learning algorithm for young kids
//! with ASD.
//!
//! The goal of this algorithm is to develop a profile of learning progress
//! for young learners who have ASD and accordingly advise lessons that
//! they can do to make more progress. It aims to use reinforcement
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
//! Note that the state and types for stuff like learner, lesson etc are
//! defined in the `types` module.
//!

pub mod simulate;
pub mod simulated_content_actions;
pub mod simulated_content_shapes;
pub mod simulated_learners;

fn main() {
    println!(">> Welcome to NeuroNudge!");

    loop {
        // Ask which strategy you want to simulate
        println!(">> Which strategy do you want to simulate?");
        println!(">> 1. Simulate Q Learning without Mastery Thresholds");
        println!(">> 2. Simulate Q Learning with Mastery Thresholds");
        println!(">> 3. Simulate Q Learning with Mastery Thresholds and Decaying Q Values");
        println!(">> 4. Simulate Q Learning with Mastery Thresholds, Decaying Q Values and ASD Trait Sensitivity");
        println!(">> Q: Quit NeuroNudge");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "Q" || input == "q" {
            println!(">> Exiting...");
            break;
        }

        if input != "1" && input != "2" && input != "3" && input != "4" {
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
            println!(">> You have selected Strategy 3: Simulate Q Learning with Mastery Thresholds and Decaying Q Values");

            println!(">> Strategy 3: Running simulation now...");
            simulate::run_simulation_strategy_3();
            println!(">> Strategy 3: Simulation complete!");
        } else if strategy == 4 {
            println!(">> You have selected Strategy 4: Simulate Q Learning with Mastery Thresholds, Decaying Q Values and ASD Trait Sensitivity");

            println!(">> Strategy 4: Running simulation now...");
            simulate::run_simulation_strategy_4();
            println!(">> Strategy 4: Simulation complete!");
        }
    }
}
