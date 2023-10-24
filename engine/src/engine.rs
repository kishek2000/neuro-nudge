//! The engine will be based on Q-Learning. The Q-Learning algorithm
//! is a model-free, reinforcement learning algorithm. It is a
//! technique to learn optimal values of an action given a state.
//!
//! In this case, the state will be the learner's current state of progress
//! in known modules, and the action will be their results from completing a
//! lesson plan. The algorithm will learn the optimal action to take given
//! a reward, which will be how well the learner is progressing in different modules.
//!
//! Simple example with a Shapes module:
//! - State: Learner has completed 3/5 lessons in the Shapes module.
//! - Action: Learner completes a lesson in the Shapes module.
//! - Reward: Learner's progress in the Shapes module increases.
//! - Outcome from this process - the algorithm learns that the learner
//!   should continue with the next level of lessons in the Shapes module.
//!
//! More complicated example with a Shapes module. In this, the learner has
//! completed 3/6 lessons but has struggled with the medium difficulty lesson 4,
//! and importantly, has made some mistakes that were not made in the easy lesson:
//! - State: Learner has completed 3/6 lessons in the Shapes module.
//! - Action: Learner attempts a medium difficulty lesson in the Shapes module.
//! - Reward: Learner's progress in the Shapes module decreases. They made mistakes
//!   from the previous lesson, and were unable to determine shapes in the current
//!   medium difficulty lesson.
//! - Outcome from this process - the algorithm learns that the learner
//!   should continue with a lower difficulty of lessons in the Shapes module.
//!
//! So, finally, here's how the algorithm will be structured in code:
//! - The algorithm will be a struct that contains a Q-Table.
//! - The Q-Table will be a HashMap of states to actions.
//! - The state will be a struct that contains the learner's progress in
//!   different modules. Note - each module will have its own notion of progress.
//! - The action will be a struct that contains the learner's results from
//!   completing a lesson plan. Note that a lesson plan can have lessons from different
//!   modules.
//! - The reward will be something that is computed from the learner's lesson plan result
//!   that is fed to the algorithm.

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use types::{
    content::{ContentModule, Lesson, LessonPlan, LessonResult},
    learner::Learner,
};

/// The Q-Learning algorithm.
pub struct QLearning<S, A, R> {
    /// The Q-Table.
    q_table: HashMap<S, HashMap<A, R>>,
}
