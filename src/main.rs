mod domain;
mod domain_error;
mod domain_form;
mod form_v1;
mod form_v2;
mod input_with_validation_v1;
mod input_with_validation_v2;
mod required;

use form_v1::FormV1;
use form_v2::FormV2;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <FormV1/>
        <FormV2/>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
