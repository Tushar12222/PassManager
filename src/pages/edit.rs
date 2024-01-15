use yew::prelude::*;
use crate::components::form::Form;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub website: String,
    pub username: String,
    pub password: String,
}

#[function_component]
pub fn Edit(props: &Props) ->Html {
    html!{
        <Form website={props.website.clone()} username={props.username.clone()} password={props.password.clone()}/>
    }
}