use yew::prelude::*;
use crate::router::AppRouter;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AppRouter />
    }
}