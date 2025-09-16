//TODO  add a test to ask the server run the print function
//
use gloo_net::http::Request;
use leptos::*;

#[component]
pub fn Submit(cover_set: WriteSignal<bool>, ipaddr: ReadSignal<String>) -> impl IntoView {
    let ip = ipaddr.get();
    let request_url = format!("http://{}:3001/print", ip);
    let print = move |_| {
        let url = request_url.clone();
        spawn_local(async move {
            let _ = Request::get(url.as_str())
                .send()
                .await
                .expect("Request failed");
        });

        cover_set.set(true);
    };

    view! {
        <p class="p-2">"Submit the request to the server"</p>
        <button on:click={print} class="p-5 mt-5 font-bold bg-red-200 text-gray-700 rounded-2xl border-4 border-red-300">{"Submit"}</button>
    }
}

#[component]
pub fn Print(ipaddr: ReadSignal<String>) -> impl IntoView {
    let ip = ipaddr.get();
    let request_url = format!("http://{}:3001/invitation", ip);

    let print = move |_| {
        let url = request_url.clone();
        spawn_local(async move {
            Request::get(url.as_str())
                .send()
                .await
                .expect("Request failed");
        });
    };

    view! {
        <p class="p-2">"Submit the request to the server"</p>
        <button on:click={print} class="p-5 mt-5 font-bold bg-red-200 text-gray-700 rounded-2xl border-4 border-red-300">{"Invitation"}</button>
    }
}
