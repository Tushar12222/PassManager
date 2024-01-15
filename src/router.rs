use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;
use crate::pages::add::Add;
use crate::pages::edit::Edit;

#[derive(Debug,  Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/add")]
    Add,
    #[at("/edit/:website/:username/:password")]
    Edit{ website: String, username: String, password: String },
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!(<Home/>),
        Route::Add => html!(<Add/>),
        Route::Edit {website, username, password} => html!(<Edit website={website} username={username} password={password}/>)
    }
}