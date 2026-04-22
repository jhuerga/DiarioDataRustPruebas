use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::persona_form_page::PersonaFormPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/persona")]
    PersonaForm,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <div>{ "Inicio" }</div> },
        Route::PersonaForm => html! { <PersonaFormPage /> },
    }
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

