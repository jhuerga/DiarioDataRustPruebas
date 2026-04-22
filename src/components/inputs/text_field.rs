use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct TextFieldProps {
    pub label: String,
    pub value: String,
    pub on_input: Callback<String>,
    #[prop_or_default]
    pub warning: Option<String>,
}

#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    let oninput = {
        let callback = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.value());
        })
    };

    html! {
        <div class="field-group">
            <label>{ &props.label }</label>
            <input
                type="text"
                value={props.value.clone()}
                oninput={oninput}
            />
            {
                if let Some(w) = &props.warning {
                    html! { <p class="warning">{ w }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
