use leptos::ev::Event;
use leptos::event_target_value;
use leptos::*;

/// Default Home Page
#[component]
pub fn IpAddr() -> impl IntoView {
    let set_ip = use_context::<WriteSignal<String>>().expect("no set_address found");

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                {move || {
                    errors
                        .get()
                        .into_iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                        .collect_view()
                }}

                </ul>
            }
        }>
        <input type="text" placeholder="Enter your IP address" class="p-3 m-3 rounded-lg border-2 border-cyan-300" on:input=move |ev:Event| {
            set_ip(event_target_value(&ev));
        }/>
        <div class="text-center w-full h-full p-20">
            <a href="/" class="p-10 m-3 bg-cyan-100 rounded-xl text-2xl font-black text-cyan-800 shadow-lg border-cyan-300 border-4 hover:bg-cyan-300 hover:underline hover:border-cyan-500 hover:shadow-2xl">
            {"Back to Home"}
            </a>
            </div>

        </ErrorBoundary>

    }
}
