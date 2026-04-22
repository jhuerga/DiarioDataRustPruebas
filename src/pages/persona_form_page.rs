use yew::prelude::*;
use crate::components::persona_form::persona_form_component::PersonaFormComponent;

#[function_component(PersonaFormPage)]
pub fn persona_form_page() -> Html {
    let state = use_reducer(|| crate::state::persona_form_state::PersonaFormState::default());

    html! {
        <div class="page-container">
            <h1>{ "Formulario de Persona" }</h1>
            <PersonaFormComponent state={state} />
        </div>
    }
}


