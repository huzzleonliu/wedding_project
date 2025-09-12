use leptos::*;
use super::option_result::*;
use web_sys::HtmlInputElement;
use super::submit::*;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsValue;

#[component]
pub fn ShowText(#[prop(default = "hello world".to_string())] text: String) -> impl IntoView {
    let (text, set_text) = create_signal(text);
    //生成一个OptionResult对象
    let result_mod: OptionResult = OptionResult {
        name: "John Doe".to_string(),
        gender: "man".to_string(),
        q1: QuestionStatus::NotAnswered,
        q2: QuestionStatus::NotAnswered,
        q3: QuestionStatus::NotAnswered,
        q4: QuestionStatus::NotAnswered,
        q5: QuestionStatus::NotAnswered,
        q6: QuestionStatus::NotAnswered,
        q7: QuestionStatus::NotAnswered,
        q8: QuestionStatus::NotAnswered,
        q9: QuestionStatus::NotAnswered,
        q10: QuestionStatus::NotAnswered,
        q11: QuestionStatus::NotAnswered,
        q12: QuestionStatus::NotAnswered,
        q13: QuestionStatus::NotAnswered,
        q14: QuestionStatus::NotAnswered,
        q15: QuestionStatus::NotAnswered,
    };
    let (result, set_result) = create_signal(result_mod);
    let (name_input, set_name_input) = create_signal("".to_string());
    let (gender_input, set_gender_input) = create_signal("".to_string());


    view! {
        <div class="container text-center">
                //问卷部分
                <p>"1.What is the color of your spouse's toothbrush?"</p>
            <SelectButton number=1 answer=true set_result={set_result} />
            <SelectButton number=1 answer=false set_result={set_result} />
        <p>"2.When do you plan on having children?"</p>
            <SelectButton number=2 answer=true set_result={set_result} />
            <SelectButton number=2 answer=false set_result={set_result} />
        <p>"3.How many times per week do you have sexual intercourse?"</p>
            <SelectButton number=3 answer=true set_result={set_result} />
            <SelectButton number=3 answer=false set_result={set_result} />
        <p>"4.Who pays the bills?"</p>
            <SelectButton number=4 answer=true set_result={set_result} />
            <SelectButton number=4 answer=false set_result={set_result} />
        <p>"5.Who wakes up first?"</p>
            <SelectButton number=5 answer=true set_result={set_result} />
            <SelectButton number=5 answer=false set_result={set_result} />
        <p>"6.How much does your spouse earn each year?"</p>
            <SelectButton number=6 answer=true set_result={set_result} />
            <SelectButton number=6 answer=false set_result={set_result} />
        <p>"7.What side of the bed do you sleep on?"</p>
            <SelectButton number=7 answer=true set_result={set_result} />
            <SelectButton number=7 answer=false set_result={set_result} />
        <p>"8.Did your parents approve of your match?"</p>
            <SelectButton number=8 answer=true set_result={set_result} />
            <SelectButton number=8 answer=false set_result={set_result} />
        <p>"9.Do you think that your relationship will last?"</p>
            <SelectButton number=9 answer=true set_result={set_result} />
            <SelectButton number=9 answer=false set_result={set_result} />
        <p>"10.What is your landlord's name?"</p>
            <SelectButton number=10 answer=true set_result={set_result} />
            <SelectButton number=10 answer=false set_result={set_result} />
        <p>"11.What compass direction does your front door face?"</p>
            <SelectButton number=11 answer=true set_result={set_result} />
            <SelectButton number=11 answer=false set_result={set_result} />
        <p>"12.How many cars do you have?"</p>
            <SelectButton number=12 answer=true set_result={set_result} />
            <SelectButton number=12 answer=false set_result={set_result} />
        <p>"13.How many electric cars do you have?"</p>
            <SelectButton number=13 answer=true set_result={set_result} />
            <SelectButton number=13 answer=false set_result={set_result} />
        <p>"14.Does your refrigerator make ice, or do you use ice trays?"</p>
            <SelectButton number=14 answer=true set_result={set_result} />
            <SelectButton number=14 answer=false set_result={set_result} />
        <p>"15.Who does most of the cooking?"</p>
            <SelectButton number=15 answer=true set_result={set_result} />
            <SelectButton number=15 answer=false set_result={set_result} />

        <p>{move || format!("{:?}", result.get())}</p>
        </div>
    }

}

#[component]
pub fn SelectButton(
    number:usize, 
    answer: bool,
    set_result:WriteSignal<OptionResult>
) -> impl IntoView {
    let text: &str;
    match answer {
        true => text = "Yes",
        false => text = "No", 
    }
    
    view! {
        <button on:click=move |_| {
            match number{
                1 => set_result.update(|original| {
                    original.q1 = QuestionStatus::Answered(answer);
                }),
                2 => set_result.update(|original| {
                    original.q2 = QuestionStatus::Answered(answer);
                }),
                3 => set_result.update(|original| {
                    original.q3 = QuestionStatus::Answered(answer);
                }),
                4 => set_result.update(|original| {
                    original.q4 = QuestionStatus::Answered(answer);
                }),
                5 => set_result.update(|original| {
                    original.q5 = QuestionStatus::Answered(answer);
                }),
                6 => set_result.update(|original| {
                    original.q6 = QuestionStatus::Answered(answer);
                }),
                7 => set_result.update(|original| {
                    original.q7 = QuestionStatus::Answered(answer);
                }),
                8 => set_result.update(|original| {
                    original.q8 = QuestionStatus::Answered(answer);
                }),
                9 => set_result.update(|original| {
                    original.q9 = QuestionStatus::Answered(answer);
                }),
                10 => set_result.update(|original| {
                    original.q10 = QuestionStatus::Answered(answer);
                }),
                11 => set_result.update(|original| {
                    original.q11 = QuestionStatus::Answered(answer);
                }),
                12 => set_result.update(|original| {
                    original.q12 = QuestionStatus::Answered(answer);
                }),
                13 => set_result.update(|original| {
                    original.q13 = QuestionStatus::Answered(answer);
                }),
                14 => set_result.update(|original| {
                    original.q14 = QuestionStatus::Answered(answer);
                }),
                15 => set_result.update(|original| {
                    original.q15 = QuestionStatus::Answered(answer);
                }),
                _ => {}
            }
        }>{text}</button>
    }
    
    // view! {
    //     <button on:click=move |_| {
    //         set_result.update(|original| {
    //                 original.q1 = QuestionStatus::Answered(answer);
    //         });
    //     }>{text}</button>
    // }
}
