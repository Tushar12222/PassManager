pub mod app;
mod components;
mod pages;
mod models;
pub mod router;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}