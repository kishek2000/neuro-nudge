//! This module defines the types used by the recommendation engine, NeuroNudge.
//!
//! The overall interaction between the domains is as follows - a learner attempts
//! and completes their lesson plan, with a certain level of success/progress. Based
//! on their performance in this lesson plan, their future lesson plans and scores for
//! different areas of study are determined. The goal is to master areas of study, from
//! basic to advanced, and to do so in a way that is sensitive to the learner's needs.
//!
//! For example, if in a lesson about Shapes a child is able to identify a square but
//! not a circle, then the next lesson for Shapes in the next lesson plan should focus
//! on circles but also ensure that a certain level of reinforcement is provided for
//! squares. This is a very simple example, but the idea is that the engine should be
//! able to determine the next lesson plan based on the learner's performance in the
//! current lesson plan.
//!
//! The way that the engine takes this information and curates a recommendation is not
//! the concern of this module.
//!
//! Note - difficulty level is a key concept. The way the content domain is setup is that
//! there are known modules such as Shapes. In a known module, there are known lessons.
//! These are pre-defined. Each lesson has a particular difficulty level. For now, the
//! concept of difficulty is split into 8 qualitative categories: Very Easy, Easy, Medium,
//! Hard, Very Hard, Expert, Master, and Grandmaster. The engine will determine the
//! difficulty level of a lesson plan based on the learner's performance in the previous
//! lesson plan.
//!

pub mod content;
pub mod engine;
pub mod learner;
