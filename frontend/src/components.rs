use dioxus::prelude::*;
use model::PostShopItem;

pub struct ListChanged;

use crate::{
    controllers::{create_list, delete_item, get_items, get_lists, post_item},
    Route,
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
                    class: "p-8 bg-base-700/50 rounded-md shadow-md backdrop-blur-sm w-200 rounded gap-1",
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
                let creator = author.read().to_string();

                let response = post_item(
                    list_uuid.read().as_str(),
                    PostShopItem {
                        title: item_name,
                        posted_by: creator,
                    },
                )
                .await;

                if response.is_ok() {
                    change_signal.write();
                    item.set("".to_string());
                    author.set("".to_string());
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
            let list_uuid = list_uuid.clone();
            let item_id = item_id.clone();

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

#[component]
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
            class: "flex flex-col w-screen h-screen from-purple-500 to-pink-500 bg-gradient-to-br text-base-100",
            div {
                class: "p-2 font-bold",
                Link { class: "p-4", to: Route::LoadOrCreateList{}, "Home" }
                Link { class: "p-4", to: Route::Profile{}, "Profile" }
                Link { class: "p-4", to: Route::Lists{}, "Lists" }
            }
            div {
                class: "m-auto m-4 p-8",
                Outlet::<Route>{}
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Home(list_uuid: String) -> Element {
    let list_uuid = use_signal(|| list_uuid);
    let change_signal = use_signal(|| ListChanged);
    rsx! {
        div {
            ShoppingList{list_uuid, change_signal}
            ItemInput{list_uuid, change_signal}
        }
    }
}

#[component]
pub fn LoadOrCreateList() -> Element {
    let nav = use_navigator();
    let mut list_uuid = use_signal(|| "".to_string());

    let onloadsubmit = move |_| {
        spawn({
            async move {
                let uuid_value = list_uuid.read().clone();
                if !uuid_value.is_empty() {
                    nav.push(Route::Home {
                        list_uuid: uuid_value,
                    });
                }
            }
        });
    };

    let on_create_list_click = move |_| {
        let nav = nav.clone();
        spawn({
            async move {
                let response = create_list().await;
                if let Ok(created_list) = response {
                    nav.push(Route::Home {
                        list_uuid: created_list.uuid,
                    });
                }
            }
        });
    };

    rsx! {
        div{
            class: "grid place-content-evently grid-cols-1 md:grid-cols-2 w-full gap-4",
            div {
                class: "card glass min-h-500 flex flex-col content-end gap-4 p-4",
                button{
                    class: "btn btn-primary",
                    onclick: on_create_list_click,
                    "Create new List"
                }
            }
            div { class: "card glass min-h-500",
                form {
                    onsubmit: onloadsubmit,
                    div {
                        class: "flex flex-col gap-4 p-4",
                        input{
                            class:"input input-bordered",
                            r#type:"text",
                            placeholder:"Enter UUID here...",
                            id: "uuid",
                            name: "uuid",
                            oninput: move |e| list_uuid.set(e.data.value())
                        }
                        button{
                            class: "btn btn-primary",
                            r#type: "submit",
                            "Load existing List"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Lists() -> Element {
    let lists_request = use_resource(move || async move { get_lists().await });

    match &*lists_request.read_unchecked() {
        Some(Ok(lists)) => rsx! {
            div { class: "grid bg-base-700/50 rounded-md shadow-md backdrop-blur-sm place-items-center min-h-500",
                ul {
                    class: "p-6 w-200 gap-1",
                    for list in lists {
                        li {
                            key: "{list}",
                            Link { class: "hover:text-sky-600", to: Route::Home{list_uuid: list.to_string()}, "Liste: {list.clone()}" }
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
