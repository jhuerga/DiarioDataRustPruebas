use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::persona_form_page::PersonaFormPage;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/persona")]
    PersonaForm,
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <div>{ "Inicio" }</div> },
        Route::PersonaForm => html! { <PersonaFormPage /> },
    }
}