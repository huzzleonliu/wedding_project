use leptos::*;
use super::option_result::*;

#[component]
pub fn QuestionPack(q1:Option<usize>, q2:Option<usize>) -> impl IntoView {
    let q1 = q1.unwrap_or(0);
    let q2 = q2.unwrap_or(1);
    let question_lib = question_lib();
view!{
    <div class="p-4">
    <p>{question_lib[q1].question.clone()}</p>
    <input type="radio" id="option1" name="question1" value="Option1" />
    <label for="option1">{question_lib[q1].answer1.clone()}</label>
    <br/>
    <input type="radio" id="option2" name="question1" value="Option2" />
    <label for="option2">{question_lib[q1].answer2.clone()}</label>
    </div>
    
    <div class="p-4">
    <p>{question_lib[q2].question.clone()}</p>
    <input type="radio" id="option3" name="question2" value="Option3" />
    <label for="option3">{question_lib[q2].answer1.clone()}</label>
    <br/>
    <input type="radio" id="option4" name="question2" value="Option4" />
    <label for="option4">{question_lib[q2].answer2.clone()}</label>
    </div>
}

}


fn question_lib() -> Vec<QuestionForm> {
    vec![
        QuestionForm {
            question: "What is the color of your spouse’s toothbrush? ".to_string(),
            answer1: "Blue".to_string(),
            answer2: "Red".to_string(),
        },
        QuestionForm {
            question: "When do you plan on having children?".to_string(),
            answer1: "This year".to_string(),
            answer2: "Next year".to_string(),
        },
        QuestionForm {
            question: "How many times per week do you have sexual intercourse?".to_string(),
            answer1: "3".to_string(),
            answer2: "√44".to_string(),
        },
        QuestionForm {
            question: "Who pays the bills?".to_string(),
            answer1: "I do".to_string(),
            answer2: "He does".to_string(),
        },
        QuestionForm {
            question: "Who wakes up first?".to_string(),
            answer1: "He does".to_string(),
            answer2: "I do".to_string(),
        },
        QuestionForm {
            question: "How much does your spouse earn each year?".to_string(),
            answer1: "More than 320,000".to_string(),
            answer2: "Less than 320,000".to_string(),
        },
        QuestionForm {
            question: "What side of the bed do you sleep on?".to_string(),
            answer1: "Left".to_string(),
            answer2: "Right".to_string(),
        },
        // TODO remove maybe
        QuestionForm {
            question: "Did your parents approve of your match?".to_string(),
            answer1: "Yes".to_string(),
            answer2: "No".to_string(),
        },
        QuestionForm {
            question: "Do you think that your relationship will last?".to_string(),
            answer1: "Yes".to_string(),
            answer2: "No".to_string(),
        },
        QuestionForm {
            question: "What is your landlord’s name?".to_string(),
            answer1: "Jan".to_string(),
            answer2: "Bjørn".to_string(),
        },
        QuestionForm {
            question: "What compass direction does your front door face?".to_string(),
            answer1: "North".to_string(),
            answer2: "West".to_string(),
        },
        // TODO remove 
        QuestionForm {
            question: "How many cars do you have?".to_string(),
            answer1: "1".to_string(),
            answer2: "More than 1".to_string(),
        },
        QuestionForm {
            question: "How many ELECTRIC cars do you have?".to_string(),
            answer1: "1".to_string(),
            answer2: "More than 1".to_string(),
        },
        QuestionForm {
            question: "Does your refrigerator make ice, or do you use ice trays?".to_string(),
            answer1: "Yes".to_string(),
            answer2: "We use ice trays".to_string(),
        },
        QuestionForm {
            question: "Who does most of the cooking?".to_string(),
            answer1: "I do".to_string(),
            answer2: "She does".to_string(),
        },
    ]
}
