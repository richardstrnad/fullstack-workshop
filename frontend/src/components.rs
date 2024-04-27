use dioxus::prelude::*;
use model::PostShopItem;

use crate::{
    controllers::{delete_item, get_items, post_item},
    ListChanged, Route,
};

#[component]
fn ShoppingListItemComponent(
    display_name: String,
    posted_by: String,
    list_uuid: String,
    item_id: String,
    change_signal: Signal<ListChanged>,
) -> Element {
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
            ItemDeleteButton{list_uuid, item_id, change_signal}
        }
    }
}

#[component]
pub fn ShoppingList(list_uuid: Signal<String>, change_signal: Signal<ListChanged>) -> Element {
    let items_request = use_resource(move || async move {
        change_signal.read();
        get_items(list_uuid.read().as_str()).await
    });

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
                                posted_by: i.posted_by.clone(),
                                list_uuid,
                                item_id: i.uuid.clone(),
                                change_signal
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

#[component]
pub fn ItemInput(list_uuid: Signal<String>, change_signal: Signal<ListChanged>) -> Element {
    let mut item = use_signal(|| "".to_string());
    let mut author = use_signal(|| "".to_string());

    // We implement this closure later
    let onsubmit = move |_| {
        spawn({
            async move {
                let item_name = item.read().to_string();
                let author = author.read().to_string();

                let response = post_item(
                    list_uuid.read().as_str(),
                    PostShopItem {
                        title: item_name,
                        posted_by: author,
                    },
                )
                .await;

                if response.is_ok() {
                    change_signal.write();
                }
            }
        });
    };

    rsx! {
        div {
            class: "w-300 m-4 mt-16 rounded",
            form { class: "grid grid-cols-3 gap-2",
                onsubmit: onsubmit,
                div {
                    input {
                        value: "{item}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "next item..",
                        r#type: "text",
                        id: "item_name",
                        name: "item_name",
                        oninput: move |e| item.set(e.data.value().clone())
                    }
                }
                div {
                    input {
                        value: "{author}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "wanted by..",
                        r#type: "text",
                        id: "author",
                        name: "author",
                        oninput: move |e| author.set(e.data.value().clone())
                    }
                }
                button {
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    "Commit"
                }
            }
        }
    }
}

#[component]
fn ItemDeleteButton(
    list_uuid: String,
    item_id: String,
    change_signal: Signal<ListChanged>,
) -> Element {
    let onclick = move |_| {
        spawn({
            let item_id = item_id.clone();
            let list_uuid = list_uuid.clone();

            async move {
                let response = delete_item(&list_uuid, &item_id).await;
                if response.is_ok() {
                    change_signal.write();
                }
            }
        });
    };

    rsx! {
        button {
            onclick:onclick,
            class: "rounded px-2 py-1",
            svg {
                class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                fill: "none",
                path {
                    d: "M6 18L18 6M6 6l12 12"
                }
            }
        }
    }
}

pub fn Profile() -> Element {
    rsx! {
        div {
            div {
                class: "flex flex-col gap-4 w-full",
                div {
                    class: "flex gap-4 items-center",
                    div {
                        class: "skeleton w-16 h-16 rounded-full shrink-0"
                    }
                    div {
                        class: "flex flex-col hap-4",
                        div {
                            class: "skeleton h-4 w-20"
                        }
                        div {
                            class: "skeleton h-4 w-28"
                        }
                    }
                }
                div {
                    class: "skeleton h-32 w-full"
                }
                div {
                    class: "skeleton h-32 w-full"
                }
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-screen h-screen bg-base-800 text-base-200",
            div {
                class: "bg-base-700",
                Link { class: "p-4", to: Route::Home{}, "Home" }
                Link { class: "p-4", to: Route::Profile{}, "Profile" }
            }
            Outlet::<Route>{}
        }
    }
}

#[allow(non_snake_case)]
pub fn Home() -> Element {
    let list_uuid = use_signal(|| "9e137e61-08ac-469d-be9d-6b3324dd20ad".to_string());
    let rust_basel = "Rust Basel";
    let change_signal = use_signal(|| ListChanged);
    rsx! {
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
            ShoppingList{list_uuid, change_signal}
            ItemInput{list_uuid, change_signal}
        }
    }
}