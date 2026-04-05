use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use crate::state::persona_form_state::PersonaFormState;
use crate::state::persona_form_model::{Sexo, EstadoCivil};

#[function_component(DatosPersonalesSection)]
pub fn datos_personales_section() -> Html {
    // Obtenemos el estado desde el contexto
    let state = use_context::<PersonaFormState>()
        .expect("PersonaFormState no encontrado en el contexto");

    let form = (*state.form).clone();

    html! {
        <section class="seccion-datos-personales">
            <h2>{"Datos personales"}</h2>

            // FECHA DE NACIMIENTO
            <label>{"Fecha de nacimiento"}</label>
            <input
                type="date"
                value={form.datos_personales.fecha_nacimiento.clone().unwrap_or_default()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_fecha_nacimiento(Some(value));
                    }
                }}
            />

            {
                if let Some(w) = &*state.warning_menor_edad {
                    html! { <p class="warning">{w}</p> }
                } else {
                    html! {}
                }
            }

            // EDAD (SOLO LECTURA)
            <label>{"Edad"}</label>
            <input
                type="number"
                readonly=true
                value={form.datos_personales.edad.map(|e| e.to_string()).unwrap_or_default()}
            />

            // SEXO
            <label>{"Sexo"}</label>
            <select
                onchange={{
                    let state = state.clone();
                    move |e: Event| {
                        let select: HtmlSelectElement = e.target_unchecked_into();
                        let value = select.value();
                        let sexo = match value.as_str() {
                            "VARON" => Sexo::Varon,
                            "MUJER" => Sexo::Mujer,
                            _ => Sexo::NA,
                        };
                        state.set_sexo(sexo);
                    }
                }}
            >
                <option
                    value="VARON"
                    selected={matches!(form.datos_personales.sexo, Sexo::Varon)}
                >
                    {"Varón"}
                </option>
                <option
                    value="MUJER"
                    selected={matches!(form.datos_personales.sexo, Sexo::Mujer)}
                >
                    {"Mujer"}
                </option>
                <option
                    value="NA"
                    selected={matches!(form.datos_personales.sexo, Sexo::NA)}
                >
                    {"N/A"}
                </option>
            </select>

            // NACIONALIDAD ISO3
            <label>{"Nacionalidad (ISO3)"}</label>
            <input
                value={form.datos_personales.nacionalidad_iso3.clone().unwrap_or_default()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_nacionalidad(Some(value));
                    }
                }}
            />

            // ESTADO CIVIL
            <label>{"Estado civil"}</label>
            <select
                onchange={{
                    let state = state.clone();
                    move |e: Event| {
                        let select: HtmlSelectElement = e.target_unchecked_into();
                        let value = select.value();
                        let estado = match value.as_str() {
                            "SOLTERO" => EstadoCivil::Soltero,
                            "CASADO" => EstadoCivil::Casado,
                            "VIUDO" => EstadoCivil::Viudo,
                            "DIVORCIADO" => EstadoCivil::Divorciado,
                            "PAREJA" => EstadoCivil::ParejaDeHecho,
                            _ => EstadoCivil::Desconocido,
                        };
                        state.set_estado_civil(estado);
                    }
                }}
            >
                <option
                    value="SOLTERO"
                    selected={matches!(form.datos_personales.estado_civil, EstadoCivil::Soltero)}
                >
                    {"Soltero"}
                </option>
                <option
                    value="CASADO"
                    selected={matches!(form.datos_personales.estado_civil, EstadoCivil::Casado)}
                >
                    {"Casado"}
                </option>
                <option
                    value="VIUDO"
                    selected={matches!(form.datos_personales.estado_civil, EstadoCivil::Viudo)}
                >
                    {"Viudo"}
                </option>
                <option
                    value="DIVORCIADO"
                    selected={matches!(form.datos_personales.estado_civil, EstadoCivil::Divorciado)}
                >
                    {"Divorciado"}
                </option>
                <option
                    value="PAREJA"
                    selected={matches!(form.datos_personales.estado_civil, EstadoCivil::ParejaDeHecho)}
                >
                    {"Pareja de hecho"}
                </option>
                <option
                    value="DESCONOCIDO"
                    selected={matches!(form.datos_personales.estado_civil, EstadoCivil::Desconocido)}
                >
                    {"Desconocido"}
                </option>
            </select>
        </section>
    }
}