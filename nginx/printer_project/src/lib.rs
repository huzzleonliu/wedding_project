use leptos::*;
use leptos::{create_signal, provide_context, ReadSignal, WriteSignal};
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use pages::home::Home;
use pages::ip::IpAddr;
use pages::not_found::NotFound;
use pages::questions::Questions;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (address, set_address): (ReadSignal<String>, WriteSignal<String>) =
        create_signal(String::new());
    provide_context(set_address);
    provide_context(address);

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Till UDI Do Us Part"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/*" view=NotFound/>
                <Route path="/questions" view=Questions/>
                <Route path="/ip" view=IpAddr/>
            </Routes>
        </Router>
    }
}
