mod app;
mod pages;
mod components;
mod state;
mod routes;

use yew::Renderer;
use crate::app::App;

fn main() {
    Renderer::<App>::new().render();
}

