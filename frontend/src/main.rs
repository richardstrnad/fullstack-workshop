use components::Layout;
use dioxus::prelude::*;

use crate::components::{Home, Profile};

mod components;
mod controllers;
// Urls are relative to your Cargo.toml file
const _STYLE: &str = manganis::mg!(file("./public/tailwind.css"));

#[derive(Routable, Clone)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/profile")]
    Profile {},
}

fn main() {
    launch(App)
}

struct ListChanged;

#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! {
        Router::<Route>{}
    }
}
