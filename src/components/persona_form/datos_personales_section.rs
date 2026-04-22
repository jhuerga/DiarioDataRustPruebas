use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::state::persona_form_state::{PersonaFormAction, PersonaFormState};
use crate::components::inputs::select_field::SelectField;
use crate::state::persona_form_model::{EstadoCivil, Sexo};

#[derive(Properties, PartialEq)]
pub struct DatosPersonalesSectionProps {
    pub state: UseReducerHandle<PersonaFormState>,
}

#[function_component(DatosPersonalesSection)]
pub fn datos_personales_section(props: &DatosPersonalesSectionProps) -> Html {
    let state = &props.state.form.datos_personales;

    let on_fecha_nacimiento_change = {
        let state = props.state.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            let v = if value.is_empty() { None } else { Some(value) };
            state.dispatch(PersonaFormAction::SetFechaNacimiento(v));
        })
    };

    let on_estado_civil_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            if let Ok(ec) = value.parse::<EstadoCivil>() {
                state.dispatch(PersonaFormAction::SetEstadoCivil(ec));
            }
        })
    };

    let on_sexo_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            if let Ok(sexo) = value.parse::<Sexo>() {
                state.dispatch(PersonaFormAction::SetSexo(sexo));
            }
        })
    };

    let on_nacionalidad_change = {
        let state = props.state.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            let v = if value.is_empty() { None } else { Some(value) };
            state.dispatch(PersonaFormAction::SetNacionalidad(v));
        })
    };

    html! {
        <div class="section datos-personales-section">
            <h2>{ "Datos Personales" }</h2>

            <div class="field-group">
                <label>{ "Fecha de nacimiento" }</label>
                <input
                    type="date"
                    value={state.fecha_nacimiento.clone().unwrap_or_default()}
                    onchange={on_fecha_nacimiento_change}
                />
                if let Some(edad) = state.edad {
                    <span class="edad-display">{ format!("Edad: {} años", edad) }</span>
                }
                if let Some(warning) = &props.state.warning_menor_edad {
                    <div class="warning">{ warning }</div>
                }
            </div>

            <SelectField
                label="Estado civil"
                value={state.estado_civil.to_string()}
                options={EstadoCivil::all_as_pairs()}
                on_change={on_estado_civil_change}
            />

            <SelectField
                label="Sexo"
                value={state.sexo.to_string()}
                options={Sexo::all_as_pairs()}
                on_change={on_sexo_change}
            />

            <div class="field-group">
                <label>{ "Nacionalidad (ISO3)" }</label>
                <input
                    type="text"
                    value={state.nacionalidad_iso3.clone().unwrap_or_default()}
                    placeholder="ESP"
                    onchange={on_nacionalidad_change}
                />
            </div>
        </div>
    }
}
