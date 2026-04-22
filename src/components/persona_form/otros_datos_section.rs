use yew::prelude::*;
use crate::state::persona_form_state::{PersonaFormAction, PersonaFormState};
use crate::components::inputs::{
    text_field::TextField,
    textarea_field::TextareaField,
    checkbox_field::CheckboxField,
};

#[derive(Properties, PartialEq)]
pub struct OtrosDatosSectionProps {
    pub state: UseReducerHandle<PersonaFormState>,
}

#[function_component(OtrosDatosSection)]
pub fn otros_datos_section(props: &OtrosDatosSectionProps) -> Html {
    let otros = &props.state.form.otros;

    //
    // HANDLERS
    //

    let on_contratar_change = {
        let state = props.state.clone();
        Callback::from(move |value: bool| {
            state.dispatch(PersonaFormAction::SetContratar(value));
        })
    };

    let on_observaciones_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetObservaciones(value));
        })
    };

    let on_clas_info_1_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetClasInfo1(value));
        })
    };

    let on_clas_info_2_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetClasInfo2(value));
        })
    };

    //
    // RENDER
    //

    html! {
        <div class="section otros-datos-section">
            <h2>{ "Otros datos" }</h2>

            <CheckboxField
                label="¿Contratar?"
                checked={otros.contratar}
                on_change={on_contratar_change}
            />

            <TextareaField
                label="Observaciones"
                value={otros.observaciones.clone()}
                on_input={on_observaciones_change}
            />

            <TextField
                label="Clasificación interna 1"
                value={otros.clas_info_1.clone()}
                on_input={on_clas_info_1_change}
            />

            <TextField
                label="Clasificación interna 2"
                value={otros.clas_info_2.clone()}
                on_input={on_clas_info_2_change}
            />
        </div>
    }
}
