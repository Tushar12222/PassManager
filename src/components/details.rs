use crate::router::Route;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use crate::models::models::Response;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub website: String,
    pub username: String,
    pub password: String,
    pub on_change: Callback<()>,
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    let clipboard = use_clipboard();
    let props = props.clone();let cloned_props = props.clone();
    let copy_to_clipboard = move |text: String| {
        let _ = clipboard.write_text(text);
    };
    let copy_to_clipboard1 = copy_to_clipboard.clone();
    let copy_to_clipboard2 = copy_to_clipboard.clone();
    let copy_to_clipboard3 = copy_to_clipboard.clone();



    #[wasm_bindgen]
    extern "C" {
        fn send_message_to_background_script(
            message: JsValue,
            data: JsValue,
            callback: &Closure<dyn FnMut(JsValue)>,
        );
    }
   
    let delete_from_storage = {
        let on_change = props.on_change.clone();
    Callback::from(move |_| {
        let clone_on_change = on_change.clone();
            let message = JsValue::from_str("Delete");
            let data = to_value(&props.website).unwrap();
            let callback = Closure::wrap(Box::new(move |response: JsValue| {
                let response: Response = from_value(response).unwrap();
                if response.success {
                    clone_on_change.emit(());
                } else {
                }
            }) as Box<dyn FnMut(_)>);
            unsafe {
                send_message_to_background_script(message, data, &callback);
            }
            callback.forget();
        })
    };

    html! {
           <div key={cloned_props.website.clone()} class="flex flex-col my-3 px-4 pb-2 bg-gray-100">
            <div class="flex justify-around">
                <Link<Route> to={Route::Edit{website: cloned_props.website.clone(), username: props.username.clone(), password: props.password.clone()}}>
                    <button  class="bg-blue-500  hover:bg-blue-700 text-white font-bold my-2 px-4 rounded-full focus:outline-none focus:shadow-outline" type="button">
                        {"Edit"}
                      </button>
                </Link<Route>>
                    <button onclick={delete_from_storage}  class="bg-blue-500 hover:bg-blue-700 text-white font-bold my-2 px-4 rounded-full focus:outline-none focus:shadow-outline" type="button">
                        {"Delete"}
                      </button>
            </div>
            <label class="font-bold text-black-700" for="website">{"Website"}</label>
        <div class="flex justify-between items-center">

            <p class="text-gray">{ cloned_props.website.clone() }</p>
            <button class="px-2 py-1 bg-blue-500 text-white rounded hover:bg-blue-600" onclick={Callback::from(move |_: MouseEvent| copy_to_clipboard1(cloned_props.website.clone()))}>
                {"Copy"}
            </button>
        </div>
        <label class="font-bold text-black" for="website">{"Username"}</label>
        <div class="flex justify-between items-center">

            <p class="text-gray-700">{ &props.username }</p>
            <button class="px-2 py-1 bg-blue-500 text-white rounded hover:bg-blue-600" onclick={Callback::from(move |_: MouseEvent| copy_to_clipboard2(props.username.clone()))}>
                {"Copy"}
            </button>
        </div>
        <label class="font-bold text-black" for="website">{"Password"}</label>
        <div class="flex justify-between items-center">

            <p class="text-gray-700">{ &props.password }</p>
            <button class="px-2 py-1 bg-blue-500 text-white rounded hover:bg-blue-600" onclick={Callback::from(move |_: MouseEvent| copy_to_clipboard3(props.password.clone()))}>
                {"Copy"}
            </button>
        </div>
    </div>
    }
}
