use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>{ "Diario Data — Proyecto limpio" }</div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
