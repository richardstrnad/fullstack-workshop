use dioxus::prelude::*;

use crate::{
    components::{Blocks, Home, Layout, Lists, LoadOrCreateList, Profile},
    controllers::spawn_websocket,
};

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
    #[route("/lists")]
    Lists {},
    #[route("/blocks")]
    Blocks {},
}

fn main() {
    launch(App)
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let signal = use_signal_sync(|| String::from("starting..."));
    spawn(spawn_websocket(signal));
    use_context_provider(|| signal);
    rsx! {
        Router::<Route>{}
    }
}
