use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;
use crate::pages::form::Form;

#[derive(Debug,  Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/add")]
    Add,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!(<Home/>),
        Route::Add => html!(<Form/>)
    }
}