use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
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
        <div class="text-center w-full h-full p-20">
            <a href="/questions" class="p-10 m-3 bg-cyan-100 rounded-xl text-2xl font-black text-cyan-800 shadow-lg border-cyan-300 border-4 hover:bg-cyan-300 hover:underline hover:border-cyan-500 hover:shadow-2xl">
            {"Apply Here"}
            </a>
            </div>

        </ErrorBoundary>

    }
}
