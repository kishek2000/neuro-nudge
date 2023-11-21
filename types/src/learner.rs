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
    VeryHigh,
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

pub trait ASDTraitComparison {
    fn calculate_alignment(&self, other: &Self) -> f32;
}

impl ASDTraitComparison for ASDTraits {
    fn calculate_alignment(&self, other: &ASDTraits) -> f32 {
        let attention_span_alignment =
            ((self.attention_span / other.attention_span) as f32).min(1.0) as f32;

        let equal_communicability = self
            .communicability
            .iter()
            .filter(|&c| other.communicability.contains(c))
            .count() as f32;

        let communicability_alignment = equal_communicability / other.communicability.len() as f32;
        let communication_level_alignment = match self.communication_level {
            CommunicationLevel::High => match other.communication_level {
                CommunicationLevel::High => 1.0,
                CommunicationLevel::Medium => 1.0,
                CommunicationLevel::Low => 1.0,
            },
            CommunicationLevel::Medium => match other.communication_level {
                CommunicationLevel::High => 0.5,
                CommunicationLevel::Medium => 1.0,
                CommunicationLevel::Low => 1.0,
            },
            CommunicationLevel::Low => match other.communication_level {
                CommunicationLevel::High => 0.0,
                CommunicationLevel::Medium => 0.5,
                CommunicationLevel::Low => 1.0,
            },
        };

        // Even if someone has the same communicability, the level of communication
        // is still important. For example, if someone is verbal, but has a low
        // communication level, they should not have a high alignment score if the question
        // does ask for verbal, but requires a high communication level = high verbal communication level.
        let communicability_alignment = communicability_alignment * communication_level_alignment;

        let motor_skills_alignment = match self.motor_skills {
            MotorSkills::Low => match other.motor_skills {
                MotorSkills::VeryHigh => 0.0,
                MotorSkills::High => 0.25,
                MotorSkills::Medium => 0.5,
                MotorSkills::Low => 1.0,
            },
            MotorSkills::Medium => match other.motor_skills {
                MotorSkills::VeryHigh => 0.25,
                MotorSkills::High => 0.5,
                MotorSkills::Medium => 1.0,
                MotorSkills::Low => 1.0,
            },
            MotorSkills::High => match other.motor_skills {
                MotorSkills::VeryHigh => 0.5,
                MotorSkills::High => 1.0,
                MotorSkills::Medium => 1.0,
                MotorSkills::Low => 1.0,
            },
            MotorSkills::VeryHigh => match other.motor_skills {
                MotorSkills::VeryHigh => 1.0,
                MotorSkills::High => 1.0,
                MotorSkills::Medium => 1.0,
                MotorSkills::Low => 1.0,
            },
        };

        // Weights for each trait (these should sum up to 1)
        let weight_attention_span = 0.4;
        let weight_communicability = 0.2;
        let weight_communication_level = 0.2;
        let weight_motor_skills = 0.2;

        // Calculate overall alignment score
        let overall_alignment = attention_span_alignment * weight_attention_span
            + communicability_alignment * weight_communicability
            + communication_level_alignment * weight_communication_level
            + motor_skills_alignment * weight_motor_skills;

        overall_alignment
    }
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
