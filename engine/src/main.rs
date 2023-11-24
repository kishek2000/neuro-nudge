//! This module defines the recommendation engine, NeuroNudge.
//! NeuroNudge is an unsupervised learning algorithm for young kids
//! with ASD.
//!
//! The goal of this algorithm is to develop a profile of learning progress
//! for young learners who have ASD and accordingly advise lessons that
//! they can do to make more progress. It aims to use reinforcement
//! learning and be sensitive to the different factors that exist for
//! a learner with ASD.
//!
//! This project simply explores the development of the algorithm and
//! tests it with data that simulates some potential young learners
//! (using GPT). It does NOT provide an application experience for the
//! learning.
//!
//! Note that the state and types for stuff like learner, lesson etc are
//! defined in the `types` module.
//!

use std::fs::File;
use std::io::Write;

pub mod simulate;
pub mod simulated_content_actions;
pub mod simulated_content_shapes;
pub mod simulated_learners;

fn main() {
    println!(">> Welcome to NeuroNudge!");
    let mut all_time_statistics_file = File::create("all_time_statistics.txt").unwrap();

    loop {
        // Ask which strategy you want to simulate
        println!(">> Which strategy do you want to simulate?");
        println!(">> 1. Simulate Q Learning without Mastery Thresholds");
        println!(">> 2. Simulate Q Learning with Mastery Thresholds");
        println!(">> 3. Simulate Q Learning with Mastery Thresholds and Decaying Q Values");
        println!(">> 4. Simulate Q Learning with Mastery Thresholds, Decaying Q Values and ASD Trait Sensitivity");
        println!(">> 5. Run All");
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

        if input != "1" && input != "2" && input != "3" && input != "4" && input != "5" {
            println!(">> Invalid input. Please try again.");
            continue;
        }

        let strategy = input.parse::<u8>().unwrap();

        if strategy == 1 {
            println!(
                ">> You have selected Strategy 1: Simulate Q Learning without Mastery Thresholds"
            );

            println!(">> Strategy 1: Running simulation now...");
            let time = std::time::Instant::now();
            simulate::run_simulation_strategy_1(None);
            let elapsed = time.elapsed();

            write!(
                &mut all_time_statistics_file,
                "Strategy 1: {}\n",
                elapsed.as_millis()
            )
            .unwrap();

            println!(">> Strategy 1: Simulation complete!");
        } else if strategy == 2 {
            println!(
                ">> You have selected Strategy 2: Simulate Q Learning with Mastery Thresholds"
            );

            println!(">> Strategy 2: Running simulation now...");

            let time = std::time::Instant::now();
            simulate::run_simulation_strategy_2(None);
            let elapsed = time.elapsed();

            write!(
                &mut all_time_statistics_file,
                "Strategy 2: {}\n",
                elapsed.as_millis()
            )
            .unwrap();

            println!(">> Strategy 2: Simulation complete!");
        } else if strategy == 3 {
            println!(">> You have selected Strategy 3: Simulate Q Learning with Mastery Thresholds and Decaying Q Values");

            println!(">> Strategy 3: Running simulation now...");
            let time = std::time::Instant::now();
            simulate::run_simulation_strategy_3(None);
            let elapsed = time.elapsed();

            write!(
                &mut all_time_statistics_file,
                "Strategy 3: {}\n",
                elapsed.as_millis()
            )
            .unwrap();

            println!(">> Strategy 3: Simulation complete!");
        } else if strategy == 4 {
            println!(">> You have selected Strategy 4: Simulate Q Learning with Mastery Thresholds, Decaying Q Values and ASD Trait Sensitivity");

            println!(">> Strategy 4: Running simulation now...");
            let time = std::time::Instant::now();
            simulate::run_simulation_strategy_4(None);
            let elapsed = time.elapsed();

            write!(
                &mut all_time_statistics_file,
                "Strategy 4 Actions: {}\n",
                elapsed.as_millis()
            )
            .unwrap();

            println!(">> Strategy 4: Simulation complete!");
        } else if strategy == 5 {
            // No printing logs needed
            // 1000 Iterations, 5 times each

            println!("Running 1k iterations...");
            for _ in 0..5 {
                // 1
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_1(Some(1000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 1: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 2
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_2(Some(1000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 2: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 3
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_3(Some(1000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 3: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 4
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_4(Some(1000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 4: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();
            }

            println!("Running 5k iterations...");
            // 5000 Iterations, 5 times each
            for _ in 0..5 {
                // 1
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_1(Some(5000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 1: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 2
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_2(Some(5000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 2: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 3
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_3(Some(5000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 3: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 4
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_4(Some(5000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 4: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();
            }

            println!("Running 10k iterations...");
            // 10000 Iterations, 5 times each
            for _ in 0..5 {
                // 1
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_1(Some(10000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 1: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 2
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_2(Some(10000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 2: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 3
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_3(Some(10000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 3: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 4
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_4(Some(10000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 4: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();
            }

            println!("Running 20k iterations...");
            // 20000 Iterations, 5 times each
            for _ in 0..5 {
                // 1
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_1(Some(20000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 1: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 2
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_2(Some(20000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 2: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 3
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_3(Some(20000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 3: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();

                // 4
                let time = std::time::Instant::now();
                simulate::run_simulation_strategy_4(Some(20000));
                let elapsed = time.elapsed();

                write!(
                    &mut all_time_statistics_file,
                    "Strategy 4: {}\n",
                    elapsed.as_millis()
                )
                .unwrap();
            }
        }
    }
}
