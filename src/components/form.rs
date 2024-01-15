
use serde_wasm_bindgen::{from_value, to_value};
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::models::models::{PasswordDetails, Response};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub website: String,
    pub username: String,
    pub password: String,
}

#[function_component]
pub fn Form(props: &Props) -> Html {
    let state = use_state(|| PasswordDetails{
        website: props.website.clone(),
        username: props.username.clone(),
        password: props.password.clone()
    });
    let cloned_state = state.clone();

    //handles input for website name
    let website_change = Callback::from(move |e: Event| {
        let input_event_target = e.target().unwrap();
        let website = input_event_target
            .unchecked_into::<HtmlInputElement>()
            .value();
        cloned_state.set(PasswordDetails {
            website,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    //handles input for website name
    let username_change = Callback::from(move |e: Event| {
        let input_event_target = e.target().unwrap();
        let username = input_event_target
            .unchecked_into::<HtmlInputElement>()
            .value();
        cloned_state.set(PasswordDetails {
            username,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    //handles input for website name
    let password_change = Callback::from(move |e: Event| {
        let input_event_target = e.target().unwrap();
        let password = input_event_target
            .unchecked_into::<HtmlInputElement>()
            .value();
        cloned_state.set(PasswordDetails {
            password,
            ..cloned_state.deref().clone()
        });
    });

    //sends data to background script to save
    #[wasm_bindgen]
    extern "C" {
        fn send_message_to_background_script(
            message: JsValue,
            data: JsValue,
            callback: &Closure<dyn FnMut(JsValue)>,
        );
    }

    let navigator = use_navigator().unwrap();

    let save_to_storage = {
        let state = state.clone();
        Callback::from(move |_| {
            let cloned_state = state.clone();
            let message = JsValue::from_str("Save");
            let data = to_value(&*cloned_state).unwrap();
            let callback = Closure::wrap(Box::new(move |response: JsValue| {
                let response: Response = from_value(response).unwrap();
                if response.success {
                    cloned_state.set(PasswordDetails::default());
                } else {
                }
            }) as Box<dyn FnMut(_)>);
            unsafe {
                send_message_to_background_script(message, data, &callback);
                navigator.push(&Route::Home);
            }
            callback.forget();
        })
    };

    html! {
           <>
            <div class="flex justify-center bg-black">
           <div class="w-full max-w-xs mt-10 mb-10 ">
      <form class="bg-white shadow-md rounded-lg px-8 pt-6 pb-8 mb-4">
        <div class="mb-4">
            <Link<Route> to={Route::Home}>
                    <button  class="bg-blue-500 w-full hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full focus:outline-none focus:shadow-outline" type="button">
                        {"Home"}
                      </button>
                </Link<Route>>
        </div>
        <div class="mb-4">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="username" >
            {"Website"}
          </label>
          <input value={((*state).website).clone()} onchange={website_change} class="shadow appearance-none border rounded-full w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text"   />
        </div>
        <div class="mb-4">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="username" >
            {"Username"}
          </label>
          <input value={((*state).username).clone()} onchange={username_change} class="shadow appearance-none border rounded-full w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" />
        </div>
        <div class="mb-6">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="password">
            {"Password"}
          </label>
          <input value={((*state).password).clone()} onchange={password_change} class="shadow appearance-none border rounded-full w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline" id="password" type="text" />
        </div>
          <button onclick={save_to_storage}  class="bg-blue-500 w-full hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full focus:outline-none focus:shadow-outline" type="button">
            {"Save"}
          </button>

      </form>
    </div>
</div>
           </>
        }
}
