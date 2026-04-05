use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::state::persona_form_state::PersonaFormState;

#[function_component(OtrosDatosSection)]
pub fn otros_datos_section() -> Html {
    let state = use_context::<PersonaFormState>()
        .expect("PersonaFormState no encontrado en el contexto");

    let form = (*state.form).clone();

    html! {
        <section class="seccion-otros-datos">
            <h2>{"Otros datos"}</h2>

            <label>{"¿Contratar?"}</label>
            <input
                type="checkbox"
                checked={form.otros.contratar}
                onclick={{
                    let state = state.clone();
                    move |e: MouseEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let checked = input.checked();
                        state.set_contratar(checked);
                    }
                }}
            />

            <label>{"Observaciones"}</label>
            <textarea
                value={form.otros.observaciones.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_observaciones(value);
                    }
                }}
            >
            </textarea>

            <label>{"Clasificación interna 1"}</label>
            <input
                value={form.otros.clas_info_1.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_clas_info_1(value);
                    }
                }}
            />

            <label>{"Clasificación interna 2"}</label>
            <input
                value={form.otros.clas_info_2.clone()}
                oninput={{
                    let state = state.clone();
                    move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        state.set_clas_info_2(value);
                    }
                }}
            />
        </section>
    }
}