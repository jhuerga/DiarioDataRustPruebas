use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CheckboxFieldProps {
    pub label: String,
    pub checked: bool,
    pub on_change: Callback<bool>,
}

#[function_component(CheckboxField)]
pub fn checkbox_field(props: &CheckboxFieldProps) -> Html {
    let on_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = target {
                on_change.emit(input.checked());
            }
        })
    };

    html! {
        <div class="field-group checkbox-container">
            <label>
                <input type="checkbox" checked={props.checked} onchange={on_change} />
                { &props.label }
            </label>
        </div>
    }
}