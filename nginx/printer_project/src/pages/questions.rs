use crate::components::cover::Cover;
use crate::components::question::QuestionPack;
use crate::components::submit::Print;
use crate::components::submit::Submit;
use leptos::*;
use rand::prelude::*;

/// 404 Not Found Page
#[component]
pub fn Questions() -> impl IntoView {
    let (cover, set_cover) = create_signal(false);
    let mut rng = rand::thread_rng();
    let q1 = rng.gen_range(0..15);
    let q2 = rng.gen_range(0..15);
    let address = use_context::<ReadSignal<String>>().expect("no address found");

    view! {
        <div class="w-full min-h-screen bg-red-100 text-center text-gray-700 ">

        <h1 class="p-7 text-xl font-normal font-bold text-gray-800">"Questions"</h1>

        <div class="p-7">
        <QuestionPack q1=Some(q1) q2=Some(q2)/>
        </div>
            // <div class="leading-lg">
            // <ShowText text="11111".to_string()/>
            // </div>

            <div>
            <Submit cover_set=set_cover ipaddr=address/>
            </div>
            <Print ipaddr=address/>

        </div>

        <Cover cover=cover cover_set=set_cover/>
    }
}
