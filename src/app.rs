use yew_router::prelude::*;
use yew::prelude::*;
use crate::router::{Route, switch};


#[function_component]
pub fn App() -> Html {
    html! {
       <>
       <HashRouter>
            <Switch<Route> render={switch}/>
       </HashRouter>
            
       </>
    }
}
