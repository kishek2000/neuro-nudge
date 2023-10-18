use uuid::Uuid;

/// Module
/// A module is a unit of study. It has a name and a list of lessons.
pub struct ContentModule {
    id: String,
    name: String,
    lessons: Vec<Lesson>,
}

impl ContentModule {
    pub fn new(name: String) -> ContentModule {
        let id = Uuid::new_v4().to_string();
        ContentModule {
            id,
            name,
            lessons: vec![],
        }
    }

    pub fn add_lesson(&mut self, lesson: Lesson) {
        self.lessons.push(lesson);
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_lessons(&self) -> &Vec<Lesson> {
        &self.lessons
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

/// LessonPlan
/// A lesson plan is a set of lessons that the learner is working on. It has a name
/// and a list of lessons.
pub struct LessonPlan {
    id: String,
    name: String,
    date: String,
    lessons: Vec<Lesson>,
}

impl LessonPlan {
    pub fn new(name: String) -> LessonPlan {
        let id = Uuid::new_v4().to_string();
        // Date time string in format YYYYMMDDHHMMSS
        let current_date_time_string = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            .to_string();

        LessonPlan {
            id,
            name,
            date: current_date_time_string,
            lessons: vec![],
        }
    }

    pub fn add_lesson(&mut self, lesson: Lesson) {
        self.lessons.push(lesson);
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_lessons(&self) -> &Vec<Lesson> {
        &self.lessons
    }

    pub fn get_date(&self) -> &String {
        &self.date
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

/// DifficultyLevel
/// The difficulty level is a qualitative measure of how difficult a lesson of
/// some module is.
pub enum DifficultyLevel {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    VeryHard,
    Expert,
    Master,
    Grandmaster,
}

/// Lesson
/// A lesson is a unit of a lesson plam. It has a name and a list of questions that
/// the learner requires to attempt.
pub struct Lesson {
    id: String,
    name: String,
    questions: Vec<Question>,
    difficulty_level: DifficultyLevel,
    module_id: String,
}

impl Lesson {
    pub fn new(
        name: String,
        questions: Vec<Question>,
        difficulty_level: DifficultyLevel,
        module_id: String,
    ) -> Lesson {
        let id = Uuid::new_v4().to_string();
        Lesson {
            id,
            name,
            questions,
            difficulty_level,
            module_id,
        }
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn get_questions(&self) -> &Vec<Question> {
        &self.questions
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_module_id(&self) -> &String {
        &self.module_id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_difficulty_level(&self) -> &DifficultyLevel {
        &self.difficulty_level
    }
}

/// QuestionOption
/// A question option is an option that the learner can select as an answer to a question.
/// This could be text or an image.
pub struct QuestionOption {
    id: String,
    option: String,
    /// If image, option is an image URL.
    is_image: bool,
}

impl QuestionOption {
    pub fn new(option: String, is_image: bool) -> QuestionOption {
        let id = Uuid::new_v4().to_string();
        QuestionOption {
            id,
            option,
            is_image,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_option(&self) -> &String {
        &self.option
    }

    pub fn get_is_image(&self) -> &bool {
        &self.is_image
    }
}

/// Question
/// A question is a unit of a lesson. A question could be:
/// - multiple choice selection between 4 answers for a particular prompt
///   - the prompt could be an image, a video, or simply text
/// - a fill in the blank question (learner says response to instructor nearby who will enter it)
/// - a question that requires the learner to imitate the prompt (such as an action) and the instructor
///   will determine if the learner has done it correctly
pub struct Question {
    id: String,
    prompt: Prompt,
    options: Option<Vec<QuestionOption>>,
    answer: Answer,
    hints: Option<Vec<String>>,
}

impl Question {
    pub fn new(
        prompt: Prompt,
        options: Option<Vec<QuestionOption>>,
        hints: Option<Vec<String>>,
        answer: Answer,
    ) -> Question {
        let id = Uuid::new_v4().to_string();

        Question {
            id,
            prompt,
            answer,
            hints,
            options,
        }
    }

    pub fn add_hint(&mut self, hint: String) {
        match &mut self.hints {
            Some(hints) => {
                hints.push(hint);
            }
            None => {
                self.hints = Some(vec![hint]);
            }
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_prompt(&self) -> &Prompt {
        &self.prompt
    }

    pub fn get_options(&self) -> &Option<Vec<QuestionOption>> {
        &self.options
    }

    pub fn get_answer(&self) -> &Answer {
        &self.answer
    }

    pub fn get_hints(&self) -> &Option<Vec<String>> {
        &self.hints
    }
}

/// Prompt
/// A prompt is the question that is asked of the learner. It could be an image, a video, or simply text.
pub struct Prompt {
    prompt_type: PromptType,
    prompt: String,
}

impl Prompt {
    pub fn new(prompt_type: PromptType, prompt: String) -> Prompt {
        Prompt {
            prompt_type,
            prompt,
        }
    }

    pub fn get_prompt_type(&self) -> &PromptType {
        &self.prompt_type
    }

    pub fn get_prompt(&self) -> &String {
        &self.prompt
    }
}

/// PromptType
/// The type of prompt that is being used. This could be an image, a video, or simply text.
pub enum PromptType {
    Image,
    Video,
    Text,
}

/// Answer
/// An answer is the response that the learner provides to the question. If the question requires
/// the instructor to confirm, then we expect a true or false response from the instructor.
/// Otherwise, we expect an integer response from the learner which is the index of the answer
/// that they have selected.
pub enum Answer {
    Integer(u8),
    Boolean(bool),
}

/// QuestionAttempt
/// This represents the attempt a learner makes at a question. This is where factors that are relevant to ASD
/// must be recorded. This includes:
/// - time taken to answer
/// - number of total attempts
/// - number of incorrect attempts
/// - number of hints requested (if relevant, might be irrelevant for a question)
/// Based on the above factors, the engine will determine the learner's progress and make recommendations.
pub struct QuestionAttempt {
    question_id: String,
    time_taken: u8,
    total_attempts: u8,
    incorrect_attempts: u8,
    hints_requested: Option<u8>,
}

impl QuestionAttempt {
    pub fn new(
        question_id: String,
        time_taken: u8,
        total_attempts: u8,
        incorrect_attempts: u8,
    ) -> QuestionAttempt {
        QuestionAttempt {
            question_id,
            time_taken,
            total_attempts,
            incorrect_attempts,
            hints_requested: None,
        }
    }

    pub fn increment_hints_requested(&mut self) {
        match &mut self.hints_requested {
            Some(hints_requested) => {
                *hints_requested += 1;
            }
            None => {
                self.hints_requested = Some(1);
            }
        }
    }

    pub fn get_question_id(&self) -> &String {
        &self.question_id
    }

    pub fn get_time_taken(&self) -> &u8 {
        &self.time_taken
    }

    pub fn get_total_attempts(&self) -> &u8 {
        &self.total_attempts
    }

    pub fn get_incorrect_attempts(&self) -> &u8 {
        &self.incorrect_attempts
    }

    pub fn get_hints_requested(&self) -> &Option<u8> {
        &self.hints_requested
    }
}

/// LessonResult
/// A lesson result is the result of a learner's attempt at a lesson. It has the following information:
/// - time taken to complete the lesson
/// - total number of questions in the lesson
/// - a list of question attempts
pub struct LessonResult {
    time_taken: u64,
    total_questions: u8,
    attempted_questions: Vec<QuestionAttempt>,
}

impl LessonResult {
    pub fn new(
        time_taken: u64,
        total_questions: u8,
        attempted_questions: Vec<QuestionAttempt>,
    ) -> LessonResult {
        LessonResult {
            time_taken,
            total_questions,
            attempted_questions,
        }
    }

    pub fn add_question_attempt(&mut self, question_attempt: QuestionAttempt) {
        self.attempted_questions.push(question_attempt);
    }

    pub fn get_time_taken(&self) -> &u64 {
        &self.time_taken
    }

    pub fn get_total_questions(&self) -> &u8 {
        &self.total_questions
    }

    pub fn get_attempted_questions(&self) -> &Vec<QuestionAttempt> {
        &self.attempted_questions
    }
}

pub const CIRCLE_IMAGE: &str =
    "https://i.pinimg.com/1200x/09/6b/9f/096b9f21d164aa34a980c85b8a5994b4.jpg";

pub const TRIANGLE_IMAGE: &str =
    "https://t4.ftcdn.net/jpg/01/77/67/85/360_F_177678515_ZCqLyYIR7OEzb0zy3Q8Tu0I9Af00j4Z9.jpg";

pub const SQUARE_IMAGE: &str = "https://previews.123rf.com/images/get4net/get4net1901/get4net190106174/126278452-rectangular-square-shape.jpg";