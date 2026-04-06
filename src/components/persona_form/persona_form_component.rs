use yew::prelude::*;
use crate::state::persona_form_state::{PersonaFormState, PersonaFormAction};

use super::{
    identificacion_section::IdentificacionSection,
    datos_personales_section::DatosPersonalesSection,
    datos_fiscales_section::DatosFiscalesSection,
    contacto_section::ContactoSection,
    otros_datos_section::OtrosDatosSection,
};

#[function_component(PersonaFormComponent)]
pub fn persona_form_component() -> Html {
    //
    // ============================================================
    //  ESTADO GLOBAL DEL FORMULARIO (REDUCER)
    // ============================================================
    //
    let state = use_reducer(PersonaFormState::default);

    //
    // ============================================================
    //  HANDLERS GLOBALES (si necesitas alguno aquí)
    // ============================================================
    //

    let on_guardar = {
        let state = state.clone();
        Callback::from(move |_| {
            web_sys::console::log_1(&format!("Guardando registro: {:?}", state.form).into());
        })
    };

    //
    // ============================================================
    //  RENDER
    // ============================================================
    //
    html! {
        <ContextProvider<UseReducerHandle<PersonaFormState>> context={state}>
            <div class="persona-form">

                <IdentificacionSection />
                <DatosPersonalesSection />
                <DatosFiscalesSection />
                <ContactoSection />
                <OtrosDatosSection />

                <div class="acciones-formulario">
                    <button class="btn-guardar" onclick={on_guardar}>
                        {"Guardar registro"}
                    </button>
                </div>

            </div>
        </ContextProvider<UseReducerHandle<PersonaFormState>>>
    }
}