use crate::content::LessonPlan;
use uuid::Uuid;

/// Learner
/// A learner is a person who is learning. They have a name, an age, and a set of
/// lesson plans that they are working on.
pub struct Learner {
    id: String,
    name: String,
    age: u8,
    lesson_plans: Vec<LessonPlan>,
}

impl Learner {
    pub fn new(name: String, age: u8) -> Learner {
        let id = Uuid::new_v4().to_string();
        Learner {
            id,
            name,
            age,
            lesson_plans: vec![],
        }
    }

    pub fn add_lesson_plan(&mut self, lesson_plan: LessonPlan) {
        self.lesson_plans.push(lesson_plan);
    }

    pub fn get_lesson_plans(&self) -> &Vec<LessonPlan> {
        &self.lesson_plans
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
}
