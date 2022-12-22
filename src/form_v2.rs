use crate::domain::{Date, PostalCode};
use crate::domain_form::{DomainForm, DomainFormOpt};
use crate::input_with_validation_v2::ValidationInputV2;

use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

fn check_and_send(domain_form_opt_handle: Rc<RefCell<DomainFormOpt>>) {
    let res: Result<DomainForm, _> = domain_form_opt_handle
        .replace_with(|opt| opt.clone())
        .try_into();
    if let Ok(domain_form) = res {
        // 送信の処理
        info!("form v2:{:?}", domain_form);
        *domain_form_opt_handle.borrow_mut() = DomainFormOpt::default();
    }
}

#[function_component(FormV2)]
pub fn form_v2() -> Html {
    let submit_ref = use_node_ref();
    let domain_form_opt_handle = use_mut_ref(DomainFormOpt::default);

    html! {
        <>
        <div>
            <label>{"郵便番号を入力"}
                <ValidationInputV2<PostalCode, _>
                    onsubmit={
                        let domain_form_opt_handle = domain_form_opt_handle.clone();
                        move |opt|{
                            domain_form_opt_handle.borrow_mut().postal_code = opt;
                            check_and_send(domain_form_opt_handle.clone());
                        }
                    }
                    submit_ref={submit_ref.clone()}
                    error_message={"有効な郵便番号ではありません"}
                    required={true}
                />
            </label>
        </div>
        <div>
            <label>{"年月日を入力"}
                <ValidationInputV2<Date, _>
                    onsubmit={
                        let domain_form_opt_handle = domain_form_opt_handle.clone();
                        move |opt|{
                            domain_form_opt_handle.borrow_mut().date = opt;
                            check_and_send(domain_form_opt_handle.clone());
                        }
                    }
                    submit_ref={submit_ref.clone()}
                    error_message={"有効な年月日ではありません"}
                    required={true}
                />
            </label>
        </div>
        <div>
            <button ref={submit_ref}>{"送信"}</button>
        </div>
        </>
    }
}
