use crate::domain::{Date, PostalCode};
use crate::domain_form::{DomainForm, DomainFormOpt};
use crate::input_with_validation_v1::ValidationInputV1;
use log::info;

use yew::prelude::*;

#[function_component(FormV1)]
pub fn form_v1() -> Html {
    let domain_form_opt_handle = use_mut_ref(DomainFormOpt::default);
    let onclick = {
        let domain_form_opt_handle = domain_form_opt_handle.clone();
        Callback::from(move |_: MouseEvent| {
            let res: Result<DomainForm, _> = domain_form_opt_handle
                .replace_with(|domain_form_opt| domain_form_opt.clone())
                .try_into();
            if let Ok(domain_form) = res {
                // 送信の処理
                info!("form v1:{:?}", domain_form);
            }
        })
    };

    html! {
        <>
        <div>
            <label>
                {"郵便番号を入力"}
                <ValidationInputV1<PostalCode, _>
                    onchange={
                        let domain_form_opt_handle = domain_form_opt_handle.clone();
                        move |opt|{domain_form_opt_handle.borrow_mut().postal_code = opt;}
                    }
                    error_message={"有効な郵便番号ではありません"}
                    required={true}
                />
            </label>
        </div>
        <div>
            <label>
                {"年月日を入力"}
                <ValidationInputV1<Date, _>
                    onchange={
                        let domain_form_opt_handle = domain_form_opt_handle.clone();
                        move |opt|{domain_form_opt_handle.borrow_mut().date = opt;}
                    }
                    error_message={"有効な年月日ではありません"}
                    required={true}
                />
            </label>
        </div>
        <div>
            <button {onclick}>{"送信"}</button>
        </div>
        </>
    }
}
