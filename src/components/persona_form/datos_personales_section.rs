use yew::prelude::*;
use crate::state::persona_form_state::{PersonaFormAction, PersonaFormState};
use crate::components::inputs::{
    text_field::TextField,
    select_field::SelectField,
};
use crate::state::persona_form_model::{EstadoCivil, Sexo};

#[derive(Properties, PartialEq)]
pub struct DatosPersonalesSectionProps {
    pub state: UseReducerHandle<PersonaFormState>,
}

#[function_component(DatosPersonalesSection)]
pub fn datos_personales_section(props: &DatosPersonalesSectionProps) -> Html {
    let datos = &props.state.form.datos_personales;

    //
    // HANDLERS
    //

    let on_fecha_nacimiento_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
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
        Callback::from(move |value: String| {
            let v = if value.is_empty() { None } else { Some(value) };
            state.dispatch(PersonaFormAction::SetNacionalidad(v));
        })
    };

    //
    // RENDER
    //

    html! {
        <div class="section datos-personales-section">
            <h2>{ "Datos personales" }</h2>

            <TextField
                label="Fecha de nacimiento"
                value={datos.fecha_nacimiento.clone().unwrap_or_default()}
                on_input={on_fecha_nacimiento_change}
                warning={props.state.warning_menor_edad.clone()}
            />

            <SelectField
                label="Estado civil"
                value={datos.estado_civil.to_string()}
                options={EstadoCivil::all_as_pairs()}
                on_change={on_estado_civil_change}
            />

            <SelectField
                label="Sexo"
                value={datos.sexo.to_string()}
                options={Sexo::all_as_pairs()}
                on_change={on_sexo_change}
            />

            <TextField
                label="Nacionalidad (ISO3)"
                value={datos.nacionalidad_iso3.clone().unwrap_or_default()}
                on_input={on_nacionalidad_change}
            />

            {
                if let Some(edad) = datos.edad {
                    html! { <p class="edad-info">{ format!("Edad calculada: {} años", edad) }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
