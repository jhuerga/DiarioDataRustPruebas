use yew::prelude::*;
use crate::components::persona_form::persona_form_component::PersonaFormComponent;

#[function_component(PersonaFormPage)]
pub fn persona_form_page() -> Html {
    html! {
        <PersonaFormComponent />
    }
}