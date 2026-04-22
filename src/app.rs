use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::switch;
use crate::routes::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

