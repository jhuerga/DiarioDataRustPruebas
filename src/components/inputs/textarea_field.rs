use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextareaFieldProps {
    pub label: String,
    pub value: String,
    pub on_input: Callback<String>,
}

#[function_component(TextareaField)]
pub fn textarea_field(props: &TextareaFieldProps) -> Html {
    let on_input = {
        let on_input = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<web_sys::HtmlTextAreaElement>();
            if let Some(textarea) = target {
                on_input.emit(textarea.value());
            }
        })
    };

    html! {
        <div class="field-group">
            <label>{ &props.label }</label>
            <textarea value={props.value.clone()} oninput={on_input}></textarea>
        </div>
    }
}