use yew::prelude::*;
use crate::pages::persona_form_page::PersonaFormPage;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <PersonaFormPage />
    }
}