use dioxus::prelude::*;

use crate::components::{Home, Layout, LoadOrCreateList, Profile};

mod components;
mod controllers;
// Urls are relative to your Cargo.toml file
const _STYLE: &str = manganis::mg!(file("./public/tailwind.css"));

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    LoadOrCreateList {},
    #[route("/list/:list_uuid")]
    Home { list_uuid: String },
    #[route("/profile")]
    Profile {},
}

fn main() {
    launch(App)
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! {
        Router::<Route>{}
    }
}
