use crate::content::{Lesson, LessonPlan};
use uuid::Uuid;

// ASD Traits
// The ASD traits are a set of measurements that are used to determine
// the similarity between learners.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Communicability {
    Verbal,
    NonVerbal,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommunicationLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MotorSkills {
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ASDTraits {
    learner_id: String,
    attention_span: i32,
    /// Assumed that if a learner does not have a particular communicability,
    /// it means they struggle to communicate in that manner (e.g. verbal)
    communicability: Vec<Communicability>,
    communication_level: CommunicationLevel,
    motor_skills: MotorSkills,
}

impl ASDTraits {
    pub fn new(
        learner_id: String,
        attention_span: i32,
        communicability: Vec<Communicability>,
        communication_level: CommunicationLevel,
        motor_skills: MotorSkills,
    ) -> ASDTraits {
        ASDTraits {
            learner_id,
            attention_span,
            communicability,
            communication_level,
            motor_skills,
        }
    }

    pub fn get_learner_id(&self) -> &String {
        &self.learner_id
    }

    pub fn get_attention_span(&self) -> &i32 {
        &self.attention_span
    }

    pub fn get_communicability(&self) -> &Vec<Communicability> {
        &self.communicability
    }

    pub fn get_communication_level(&self) -> &CommunicationLevel {
        &self.communication_level
    }

    pub fn get_motor_skills(&self) -> &MotorSkills {
        &self.motor_skills
    }
}

/// Learner
/// A learner is a person who is learning. They have a name, an age, and a set of
/// lesson plans that they are working on. They also have a unique set of measurements
/// for their ASD traits.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Learner {
    id: String,
    name: String,
    age: u8,
    lesson_plans: Vec<LessonPlan>,
    asd_traits: ASDTraits,
    q_table_id: String,
}

impl Learner {
    pub fn new(
        name: String,
        age: u8,
        asd_traits: ASDTraits,
        q_table_id: String,
        learner_id: Option<String>,
    ) -> Learner {
        let id = learner_id.unwrap_or(Uuid::new_v4().to_string());
        Learner {
            id,
            name,
            age,
            lesson_plans: vec![],
            asd_traits,
            q_table_id,
        }
    }

    pub fn add_lesson_plan(&mut self, lesson_plan: LessonPlan) {
        self.lesson_plans.push(lesson_plan);
    }

    pub fn set_current_lesson(&mut self, lesson: Lesson) {
        let mut new_lesson_plan = LessonPlan::new(lesson.get_name().clone());
        new_lesson_plan.add_lesson(lesson);
        self.add_lesson_plan(new_lesson_plan.clone());
    }

    pub fn get_lesson_plans(&self) -> &Vec<LessonPlan> {
        &self.lesson_plans
    }

    pub fn get_current_lesson(&self) -> &Lesson {
        // get the last (latest) lesson plan
        let latest_plan = self.lesson_plans.last().unwrap();
        // for now, all plans have 1 lesson in them. return it
        &latest_plan.get_lessons()[0]
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_age(&self) -> &u8 {
        &self.age
    }

    pub fn get_asd_traits(&self) -> &ASDTraits {
        &self.asd_traits
    }

    pub fn get_q_table_id(&self) -> &String {
        &self.q_table_id
    }
}
