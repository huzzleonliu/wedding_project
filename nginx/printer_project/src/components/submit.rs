//TODO  add a test to ask the server run the print function
//
use gloo_net::http::Request;
use leptos::*;
use serde_json;
use web_sys::console;

#[component]
pub fn Submit(cover_set: WriteSignal<bool>) -> impl IntoView {
    let print = move |_| {
        spawn_local(async move {
            // match
            Request::get("http://10.193.105.135:8085/print")
                .send()
                .await
                .unwrap();
            // {
            //     Ok(response) => {
            //         let text = response.text().await.unwrap_or_else(|_| "Failed to get text".to_string());
            //         set_text.set(text);
            //     },
            //     Err(err) => {
            //         console::log_1(&format!("Error: {:?}", err).into());
            //     }
            // }
        });

        cover_set.set(true);
    };

    view! {
        <p class="p-2">"Submit the request to the server"</p>
        <button on:click={print} class="p-5 mt-5 font-bold bg-red-200 text-gray-700 rounded-2xl border-4 border-red-300">{"Submit"}</button>
    }
}

#[component]
pub fn Print() -> impl IntoView {
    let print = move |_| {
        spawn_local(async move {
            // match
            Request::get("http://10.193.105.135:8085/invitation")
                .send()
                .await
                .unwrap();
            // {
            //     Ok(response) => {
            //         let text = response.text().await.unwrap_or_else(|_| "Failed to get text".to_string());
            //         set_text.set(text);
            //     },
            //     Err(err) => {
            //         console::log_1(&format!("Error: {:?}", err).into());
            //     }
            // }
        });
    };

    view! {
        <p class="p-2">"Submit the request to the server"</p>
        <button on:click={print} class="p-5 mt-5 font-bold bg-red-200 text-gray-700 rounded-2xl border-4 border-red-300">{"Invitation"}</button>
    }
}
