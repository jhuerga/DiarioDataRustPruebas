use yew::prelude::*;
use crate::components::layout::{Sidebar, Topbar};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="layout">
            <Topbar />
            <div class="layout-body">
                <Sidebar />
                <main>
                    { for props.children.iter() }
                </main>
            </div>
        </div>
    }
}