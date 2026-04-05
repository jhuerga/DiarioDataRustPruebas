use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use crate::state::persona_form_state::PersonaFormState;

#[function_component(DatosFiscalesSection)]
pub fn datos_fiscales_section() -> Html {
    let state = use_context::<PersonaFormState>()
        .expect("PersonaFormState no encontrado en el contexto");

    let form = (*state.form).clone();

    html! {
        <section class="seccion-datos-fiscales">
            <h2>{"Datos fiscales"}</h2>

            <label>{"Código postal"}</label>
            <input
                value={form.datos_fiscales.cp.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_cp(value);
                    }
                }}
            />

            <label>{"Domicilio fiscal"}</label>
            <input
                value={form.datos_fiscales.domicilio.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_domicilio(value);
                    }
                }}
            />

            <label>{"Población"}</label>
            <input
                value={form.datos_fiscales.poblacion.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_poblacion(value);
                    }
                }}
            />

            <label>{"Provincia (código)"}</label>
            <input
                value={form.datos_fiscales.provincia_cod.clone().unwrap_or_default()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_provincia(Some(value));
                    }
                }}
            />

            <label>{"IBAN"}</label>
            <input
                value={form.datos_fiscales.iban.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_iban(value);
                    }
                }}
            />

            {
                if let Some(w) = &*state.warning_iban {
                    html! { <p class="warning">{w}</p> }
                } else {
                    html! {}
                }
            }
        </section>
    }
}