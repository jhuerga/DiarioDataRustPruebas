use yew::prelude::*;
use web_sys::HtmlSelectElement;

#[derive(Properties, PartialEq)]
pub struct SelectFieldProps {
    pub label: String,
    pub value: String,
    pub options: Vec<(String, String)>, // (value, label)
    pub on_change: Callback<String>,
}

#[function_component(SelectField)]
pub fn select_field(props: &SelectFieldProps) -> Html {
    let onchange = {
        let callback = props.on_change.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            callback.emit(select.value());
        })
    };

    html! {
        <div class="field-group">
            <label>{ &props.label }</label>
            <select value={props.value.clone()} {onchange}>
                {
                    for props.options.iter().map(|(val, label)| {
                        html! {
                            <option value={val.clone()}>
                                { label }
                            </option>
                        }
                    })
                }
            </select>
        </div>
    }
}
