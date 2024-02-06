use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-playground.css"/>
        <Title text="Leptos Playground"/>

        {"Hello, World!"}
    }
}
