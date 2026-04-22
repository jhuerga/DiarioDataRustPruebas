use yew::prelude::*;
use crate::state::persona_form_state::{PersonaFormAction, PersonaFormState};
use crate::components::inputs::{
    text_field::TextField,
    select_field::SelectField,
};
use crate::state::persona_form_model::{TipoDocumento};

#[derive(Properties, PartialEq)]
pub struct IdentificacionSectionProps {
    pub state: UseReducerHandle<PersonaFormState>,
}

#[function_component(IdentificacionSection)]
pub fn identificacion_section(props: &IdentificacionSectionProps) -> Html {
    let state = &props.state.form.identificacion;

    let on_dni_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetDni(value));
        })
    };

    let on_tipo_doc_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            if let Ok(tipo) = value.parse::<TipoDocumento>() {
                state.dispatch(PersonaFormAction::SetTipoDocumento(tipo));
            }
        })
    };
    let on_codigo_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetCodigo(value));
        })
    };

    let on_fecha_caducidad_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            let v = if value.is_empty() { None } else { Some(value) };
            state.dispatch(PersonaFormAction::SetFechaCaducidadDocumento(v));
        })
    };


    let on_naf_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetNaf(value));
        })
    };

    let on_apellidos_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetApellidos(value));
        })
    };

    let on_nombre_change = {
        let state = props.state.clone();
        Callback::from(move |value: String| {
            state.dispatch(PersonaFormAction::SetNombre(value));
        })
    };

    html! {
        <div class="section identificacion-section">
            <h2>{ "Identificaci?n" }</h2>

            <SelectField
                label="Tipo de documento"
                value={state.tipo_documento.to_string()}
                options={TipoDocumento::all_as_pairs()}
                on_change={on_tipo_doc_change}
            />

            <TextField
                label="DNI / NIE / Pasaporte"
                value={state.dni.clone()}
                on_input={on_dni_change}
                warning={props.state.warning_dni.clone()}
            />

            <TextField
                label="NAF"
                value={state.naf.clone()}
                on_input={on_naf_change}
                warning={props.state.warning_naf.clone()}
            />
            <TextField
                label="Código interno"
                value={state.codigo.clone()}
                on_input={on_codigo_change}
            />


            <TextField
                label="Fecha caducidad documento"
                value={state.fecha_caducidad_documento.clone().unwrap_or_default()}
                on_input={on_fecha_caducidad_change}
            />


            <TextField
                label="Apellidos"
                value={state.apellidos.clone()}
                on_input={on_apellidos_change}
            />

            <TextField
                label="Nombre"
                value={state.nombre.clone()}
                on_input={on_nombre_change}
            />
        </div>
    }
}
