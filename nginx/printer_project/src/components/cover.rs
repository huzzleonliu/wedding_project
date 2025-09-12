
use leptos::*;

#[component]
pub fn Cover(cover:ReadSignal<bool>, cover_set:WriteSignal<bool>) -> impl IntoView {



    view!{
        {move || if cover.get(){
            view!{
                <div class="fixed inset-0 bg-gray-900 bg-opacity-50 z-50 flex items-center justify-center">
                <div class="bg-red-100 z-60 w-1/2 h-auto p-7 m-auto p-5 rounded-lg border-4 border-red-200">
                <p class="text-center text-lg font-bold text-gray-800">"The form has been submited"</p>
                <a href="/" class="text-center text-gray-900 font-bold bg-red-200 rounded-lg border-4 border-red-300 m-3 p-3 block">"Return"</a>
                </div>
                </div>
            }.into_view()
        }else{
                view!{
                <div></div>
            }.into_view()
        
        }
    }
}
}
