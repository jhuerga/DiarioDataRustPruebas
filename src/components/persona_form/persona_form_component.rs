use yew::prelude::*;
use crate::state::persona_form_state::PersonaFormState;
use crate::components::persona_form::{
    datos_personales_section::DatosPersonalesSection,
    datos_fiscales_section::DatosFiscalesSection,
    contacto_section::ContactoSection,
    otros_datos_section::OtrosDatosSection,
    identificacion_section::IdentificacionSection,
};

#[derive(Properties, PartialEq)]
pub struct PersonaFormComponentProps {
    pub state: UseReducerHandle<PersonaFormState>,
}

#[function_component(PersonaFormComponent)]
pub fn persona_form_component(props: &PersonaFormComponentProps) -> Html {
    let state = props.state.clone();

    let on_guardar = {
        let state = state.clone();
        Callback::from(move |_| {
            // Log en consola del navegador
            web_sys::console::log_1(
                &format!("Guardando registro: {:?}", state.form).into()
            );
        })
    };

    html! {
        <div class="persona-form-container">

            <IdentificacionSection
                state={state.clone()}
            />

            <DatosPersonalesSection
                state={state.clone()}
            />

            <DatosFiscalesSection
                state={state.clone()}
            />

            <ContactoSection
                state={state.clone()}
            />

            <OtrosDatosSection
                state={state.clone()}
            />

            <div class="form-actions">
                <button onclick={on_guardar} class="btn">
                    { "Guardar" }
                </button>
            </div>
        </div>
    }
}
