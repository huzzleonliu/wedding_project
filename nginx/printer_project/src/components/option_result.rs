#[derive(Clone, Debug)]
pub enum QuestionStatus  {
    NotAnswered,
    Answered(Answer),
}

#[derive(Clone, Debug)]
pub enum Answer  {
    A,
    B,
}

#[derive(Clone, Debug)]
pub struct QuestionForm  {
    pub question: String,
    pub answer1: String,
    pub answer2: String,
}



#[derive(Clone, Debug)]
pub struct OptionResult {
    pub name: String,
    pub gender: String,
    pub q1: QuestionStatus ,
    pub q2: QuestionStatus ,
    pub q3: QuestionStatus ,
    pub q4: QuestionStatus ,
    pub q5: QuestionStatus ,
    pub q6: QuestionStatus ,
    pub q7: QuestionStatus ,
    pub q8: QuestionStatus ,
    pub q9: QuestionStatus ,
    pub q10: QuestionStatus ,
    pub q11: QuestionStatus ,
    pub q12: QuestionStatus ,
    pub q13: QuestionStatus ,
    pub q14: QuestionStatus ,
    pub q15: QuestionStatus ,
}

