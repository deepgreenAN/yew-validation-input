use crate::required::RequiredString;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ValidationInputPropsV1<T, E>
where
    T: TryFrom<String, Error = E> + PartialEq + 'static,
    E: PartialEq,
{
    // 値の更新時に実行するコールバック
    pub onchange: Callback<Option<T>>,
    // バリデーションのエラーメッセージ
    pub error_message: String,
    // 必須のフィールドであるかどうか
    #[prop_or(false)]
    pub required: bool,
}

#[function_component(ValidationInputV1)]
pub fn validation_input_v1<T, E>(
    ValidationInputPropsV1 {
        onchange,
        error_message,
        required,
    }: &ValidationInputPropsV1<T, E>,
) -> Html
where
    T: TryFrom<String, Error = E> + PartialEq + 'static,
    E: PartialEq,
{
    let error_message = error_message.clone();
    let required = *required;
    let error_message_handle: UseStateHandle<Option<String>> =
        use_state(|| Some("必須のフィールドです".to_string()));

    let text_onchange = {
        let parent_onchange = onchange.clone();
        let error_message_handle = error_message_handle.clone();

        let try_into_func = move |s: String| -> Result<T, String> {
            // required
            if required {
                let _required_s: RequiredString = s
                    .clone()
                    .try_into()
                    .map_err(|_| "必須のフィールドです".to_string())?;
            }
            let domain_value: T = s.try_into().map_err(|_| error_message.clone())?;
            Ok(domain_value)
        };

        Callback::from(move |e: Event| {
            let input_element = e
                .target()
                .expect("Event should have a target when dispatched")
                .dyn_into::<HtmlInputElement>()
                .expect("This EventTarget should cast into HtmlInputElement");

            let input_text = input_element.value();

            let try_into_res = try_into_func(input_text);
            match try_into_res {
                Ok(domain_value) => {
                    error_message_handle.set(None);
                    parent_onchange.emit(Some(domain_value));
                }
                Err(error_message) => {
                    error_message_handle.set(Some(error_message));
                    parent_onchange.emit(None);
                }
            }
        })
    };

    html! {
        <>
        <input type="text" onchange={text_onchange}/>
        if let Some(s) = (*error_message_handle).clone() {
            <span>{s}</span>
        }
        </>
    }
}
