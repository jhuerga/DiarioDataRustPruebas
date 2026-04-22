use yew::prelude::*;
use crate::state::persona_form_state::{PersonaFormAction, PersonaFormState};
use crate::components::inputs::text_field::TextField;

#[derive(Properties, PartialEq)]
pub struct ContactoSectionProps {
    pub state: UseReducerHandle<PersonaFormState>,
}

#[function_component(ContactoSection)]
pub fn contacto_section(props: &ContactoSectionProps) -> Html {
    let contacto = &props.state.form.contacto;

    //
    // HANDLERS
    //

    let on_email_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetEmail(value));
        })
    };

    let on_prefijo_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            let v = if value.is_empty() { None } else { Some(value) };
            state.dispatch(PersonaFormAction::SetPrefijoPais(v));
        })
    };

    let on_telefono_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetTelefono(value));
        })
    };

    //
    // RENDER
    //

    html! {
        <div class="section contacto-section">
            <h2>{ "Contacto" }</h2>

            <TextField
                label="Email"
                value={contacto.email.clone()}
                on_input={on_email_change}
            />

            <TextField
                label="Prefijo país"
                value={contacto.prefijo_pais.clone().unwrap_or_default()}
                on_input={on_prefijo_change}
            />

            <TextField
                label="Teléfono"
                value={contacto.telefono.clone()}
                on_input={on_telefono_change}
            />
        </div>
    }
}
