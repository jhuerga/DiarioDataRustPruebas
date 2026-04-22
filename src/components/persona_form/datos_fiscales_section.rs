use yew::prelude::*;
use crate::state::persona_form_state::{PersonaFormAction, PersonaFormState};
use crate::components::inputs::{
    text_field::TextField,
    select_field::SelectField,
};
use crate::state::persona_form_model::Provincia;

#[derive(Properties, PartialEq)]
pub struct DatosFiscalesSectionProps {
    pub state: UseReducerHandle<PersonaFormState>,
}

#[function_component(DatosFiscalesSection)]
pub fn datos_fiscales_section(props: &DatosFiscalesSectionProps) -> Html {
    let datos = &props.state.form.datos_fiscales;

    //
    // HANDLERS
    //

    let on_iban_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetIban(value));
        })
    };

    let on_cp_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetCp(value));
        })
    };

    let on_domicilio_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetDomicilio(value));
        })
    };

    let on_poblacion_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetPoblacion(value));
        })
    };

    let on_provincia_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            let v = if value.is_empty() { None } else { Some(value) };
            state.dispatch(PersonaFormAction::SetProvincia(v));
        })
    };

    //
    // RENDER
    //

    html! {
        <div class="section datos-fiscales-section">
            <h2>{ "Datos fiscales" }</h2>

            <TextField
                label="IBAN"
                value={datos.iban.clone()}
                on_input={on_iban_change}
                warning={props.state.warning_iban.clone()}
            />

            <TextField
                label="Código Postal"
                value={datos.cp.clone()}
                on_input={on_cp_change}
            />

            <TextField
                label="Domicilio"
                value={datos.domicilio.clone()}
                on_input={on_domicilio_change}
            />

            <TextField
                label="Población"
                value={datos.poblacion.clone()}
                on_input={on_poblacion_change}
            />

            <SelectField
                label="Provincia"
                value={datos.provincia_cod.clone().unwrap_or_default()}
                options={Provincia::all_as_pairs()}
                on_change={on_provincia_change}
            />
        </div>
    }
}
