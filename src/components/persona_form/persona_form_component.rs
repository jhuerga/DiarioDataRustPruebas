use yew::prelude::*;
use crate::state::persona_form_state::PersonaFormState;

use super::{
    identificacion_section::IdentificacionSection,
    datos_personales_section::DatosPersonalesSection,
    datos_fiscales_section::DatosFiscalesSection,
    contacto_section::ContactoSection,
    otros_datos_section::OtrosDatosSection,
};

#[function_component(PersonaFormComponent)]
pub fn persona_form_component() -> Html {
    // Creamos el estado del formulario
    let state = {
        // Necesitamos un componente "falso" para inicializar el estado
        // Yew requiere un contexto de componente para crear UseStateHandle
        struct Dummy;
        impl Component for Dummy {
            type Message = ();
            type Properties = ();

            fn create(_ctx: &Context<Self>) -> Self { Self }
            fn view(&self, _ctx: &Context<Self>) -> Html { html! {} }
        }

        // Creamos un nodo temporal para obtener un contexto válido
        let node = yew::virtual_dom::VNode::from(html! { <Dummy /> });
        let ctx = node
            .as_component()
            .expect("No se pudo obtener contexto para PersonaFormState");

        PersonaFormState::new(ctx)
    };

    html! {
        <ContextProvider<PersonaFormState> context={state.clone()}>
            <div class="persona-form">

                <IdentificacionSection />
                <DatosPersonalesSection />
                <DatosFiscalesSection />
                <ContactoSection />
                <OtrosDatosSection />

                <div class="acciones-formulario">
                    <button class="btn-guardar">
                        {"Guardar registro"}
                    </button>
                </div>

            </div>
        </ContextProvider<PersonaFormState>>
    }
}