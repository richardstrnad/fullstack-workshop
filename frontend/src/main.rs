use dioxus::prelude::*;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App)
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let rust_basel = "Rust Basel";
    rsx! {
        div {
            class: "flex w-screen h-screen bg-slate-800 text-slate-200",
            div {
                class: "m-auto m-4 p-8",
                h1 {
                    class: "font-bold font-mono text-xl",
                    "Welcome to {rust_basel}"
                }
                button {
                    class: "m-4 py-2 px-1 border border-2 border-slate-500 rounded-md bg-slate-700",
                    "My Button!"
                }
            }
        }
    }
}
