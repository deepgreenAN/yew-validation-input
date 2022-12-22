use crate::required::RequiredString;

use gloo_events::EventListener;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ValidationInputPropsV2<T, E>
where
    T: TryFrom<String, Error = E> + PartialEq + 'static,
    E: PartialEq,
{
    // 送信時に実行するコールバック
    pub onsubmit: Callback<Option<T>>,
    // 送信イベントを結びつけるボタンのNodeRef
    pub submit_ref: NodeRef,
    // バリデーションのエラーメッセージ
    pub error_message: String,
    // 必須のフィールドであるかどうか
    #[prop_or(false)]
    pub required: bool,
}

#[function_component(ValidationInputV2)]
pub fn validation_input_v2<T, E>(
    ValidationInputPropsV2 {
        onsubmit,
        submit_ref,
        error_message,
        required,
    }: &ValidationInputPropsV2<T, E>,
) -> Html
where
    T: TryFrom<String, Error = E> + PartialEq + 'static,
    E: PartialEq,
{
    let error_message = error_message.clone();
    let required = *required;

    let error_message_handle: UseStateHandle<Option<String>> = use_state(|| None);
    let text_ref = use_node_ref();

    use_effect_with_deps(
        {
            let parent_onsubmit = onsubmit.clone();
            let error_message_handle = error_message_handle.clone();
            let text_ref = text_ref.clone();
            let submit_ref = submit_ref.clone();

            let try_into_func = move |s: String| -> Result<T, String> {
                if required {
                    let _required_s: RequiredString = s
                        .clone()
                        .try_into()
                        .map_err(|_| "必須のフィールドです".to_string())?;
                }
                let domain_value: T = s.try_into().map_err(|_| error_message.clone())?;
                Ok(domain_value)
            };

            move |_| {
                let submit_node = submit_ref
                    .cast::<HtmlElement>()
                    .expect("This NodeRef should cast into HtmlElement");

                let listener = EventListener::new(&submit_node, "click", move |_| {
                    let input_text = text_ref
                        .cast::<HtmlInputElement>()
                        .expect("This NodeRef should cast into HtmlInputElement")
                        .value();

                    let try_into_res = try_into_func(input_text);
                    match try_into_res {
                        Ok(domain_value) => {
                            error_message_handle.set(None);
                            parent_onsubmit.emit(Some(domain_value));
                        }
                        Err(error_message) => {
                            error_message_handle.set(Some(error_message));
                            parent_onsubmit.emit(None);
                        }
                    }
                });
                move || drop(listener)
            }
        },
        (),
    );

    html! {
        <>
        <input type="text" ref={text_ref}/>
        if let Some(s) = (*error_message_handle).clone() {
            <span>{s}</span>
        }
        </>
    }
}
