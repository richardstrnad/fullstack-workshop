use dioxus::prelude::*;
use model::ShoppingListItem;

// Urls are relative to your Cargo.toml file
const _STYLE: &str = manganis::mg!(file("./public/tailwind.css"));

fn main() {
    launch(App)
}

async fn get_items() -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let url = "http://localhost:3001/items";

    reqwest::get(url).await?.json().await
}

#[component]
fn ShoppingListItemComponent(display_name: String, posted_by: String) -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow text-2xl",
                "{display_name} "
            }
            span {
                " posted by {posted_by}"
            }
        }
    }
}

#[component]
fn ShoppingList() -> Element {
    let items_request = use_resource(move || async move { get_items().await });

    match &*items_request.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div { class: "grid place-items-center min-h-500",
                ul {
                    class: "p-2 bg-base-700 w-200 rounded gap-1",
                    for i in list {
                        li {
                            key: "{i.uuid}",
                            ShoppingListItemComponent{
                                display_name: i.title.clone(),
                                posted_by: i.posted_by.clone()
                            },
                        }
                    }
                }
            }
        },
        Some(Err(err)) => {
            rsx! {
                p {
                    "Error: {err}"
                }
            }
        }
        None => {
            rsx! {
                p {
                    "Loading items..."
                }
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let rust_basel = "Rust Basel";
    rsx! {
        div {
            class: "flex w-screen h-screen bg-base-800 text-base-200",
            div {
                class: "m-auto m-4 p-8",
                h1 {
                    class: "font-bold font-mono text-xl",
                    "Welcome to {rust_basel}"
                }
                button {
                    class: "m-4 py-2 px-1 border border-2 border-base-500 rounded-md bg-base-700",
                    "My Button!"
                }
                ShoppingList{}
            }
        }
    }
}
