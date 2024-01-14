use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub website: String,
    pub username: String,
    pub password: String,
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    let clipboard = use_clipboard();
    let props = props.clone();
    let copy_to_clipboard = move |text: String| {
        let _ = clipboard.write_text(text);
    };
    let copy_to_clipboard1 = copy_to_clipboard.clone();
    let copy_to_clipboard2 = copy_to_clipboard.clone();
    let copy_to_clipboard3 = copy_to_clipboard.clone();

    html! {
           <div key={props.website.clone()} class="flex flex-col my-3 px-4 pb-2 bg-gray-100">
            <label class="font-bold text-black-700" for="website">{"Website"}</label>
        <div class="flex justify-between items-center">

            <p class="text-gray">{ props.website.clone() }</p>
            <button class="px-2 py-1 bg-blue-500 text-white rounded hover:bg-blue-600" onclick={Callback::from(move |_: MouseEvent| copy_to_clipboard1(props.website.clone()))}>
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
