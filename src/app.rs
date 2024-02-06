use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let handle_consent = create_server_action::<HandleConsent>();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-playground.css"/>
        <Title text="Leptos Playground"/>

        {"Hello, World!"}
        <ActionForm action=handle_consent>
            <button type="submit" name="accept" value="true">Accept</button>
        </ActionForm>
    }
}

#[server(HandleConsent)]
async fn handle_consent(accept: Option<String>) -> Result<(), ServerFnError> {
    println!("{:?}", accept);
    Ok(())
}
