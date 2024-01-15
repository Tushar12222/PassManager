use crate::components::details::Detail;
use crate::models::models::{PasswordDetails, Response};
use crate::router::Route;
use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::default;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Home() -> Html {
    let state = use_state(|| Vec::<PasswordDetails>::new());

    #[wasm_bindgen]
    extern "C" {
        fn send_message_to_background_script(
            message: JsValue,
            data: JsValue,
            callback: &Closure<dyn FnMut(JsValue)>,
        );
    }

    let rerender = use_state(|| false);
    let cloned_rerender = rerender.clone();
    let on_delete = Callback::from(move|_| {
        log!(*cloned_rerender);
        cloned_rerender.set(!*cloned_rerender)
    });

    let load_from_storage = {
        let state = state.clone();
        Callback::from(move |_| {
            let cloned_state = state.clone();
            let message = JsValue::from_str("Load");
            let callback = Closure::once(
                (move |response: JsValue| {
                    log!(response.clone());
                    let response: Response = from_value(response).unwrap();
                    if response.success {
                        cloned_state.set(response.message.unwrap());
                    } else {
                    }
                }),
            );
            unsafe {
                send_message_to_background_script(message, JsValue::UNDEFINED, &callback);
            }
            callback.forget();
        })
    };

    

    use_effect_with(*rerender, move |_| {
        load_from_storage.emit(());
        || {}
    });

    html!(
        <>
            <div class="m-3">
                <Link<Route> to={Route::Add}>
                    <button  class="bg-blue-500 w-full hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full focus:outline-none focus:shadow-outline" type="button">
                        {"Add"}
                      </button>
                </Link<Route>>
            </div>
            <div>
               {
                (&*state).into_iter().map(|details: &PasswordDetails| {
                    html!{<Detail website={details.website.clone()} username={details.username.clone()} password={details.password.clone()} on_change={on_delete.clone()}/>}
                }).collect::<Html>()
               }
            </div>
        </>
    )
}
