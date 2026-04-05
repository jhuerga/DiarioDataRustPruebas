use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::state::persona_form_state::PersonaFormState;

#[function_component(ContactoSection)]
pub fn contacto_section() -> Html {
    let state = use_context::<PersonaFormState>()
        .expect("PersonaFormState no encontrado en el contexto");

    let form = (*state.form).clone();

    html! {
        <section class="seccion-contacto">
            <h2>{"Contacto"}</h2>

            <label>{"Email"}</label>
            <input
                value={form.contacto.email.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_email(value);
                    }
                }}
            />

            <label>{"Prefijo país"}</label>
            <input
                value={form.contacto.prefijo_pais.clone().unwrap_or_default()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_prefijo_pais(Some(value));
                    }
                }}
            />

            <label>{"Teléfono"}</label>
            <input
                value={form.contacto.telefono.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_telefono(value);
                    }
                }}
            />
        </section>
    }
}