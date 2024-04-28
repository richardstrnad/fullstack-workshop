use dioxus::prelude::*;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use model::{CreateListResponse, PostShopItem, ShoppingListItem};
use reqwest_websocket::{Message, RequestBuilderExt};
use serde_json::{json, Value};

pub async fn get_items(list_id: &str) -> Result<Vec<ShoppingListItem>, reqwest::Error> {
    let url = format!("http://localhost:3001/list/{}/items", list_id);
    let list = reqwest::get(&url)
        .await?
        .json::<Vec<ShoppingListItem>>()
        .await;

    list
}

pub async fn post_item(
    list_id: &str,
    item: PostShopItem,
) -> Result<ShoppingListItem, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(format!("http://localhost:3001/list/{}/items", list_id))
        .json(&item)
        .send()
        .await?
        .json::<ShoppingListItem>()
        .await?;

    Ok(response)
}

pub async fn delete_item(list_id: &str, item_id: &str) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(format!(
            "http://localhost:3001/list/{}/items/{}",
            list_id, item_id
        ))
        .send()
        .await?;

    Ok(())
}

pub async fn create_list() -> Result<CreateListResponse, reqwest::Error> {
    reqwest::Client::new()
        .get("http://localhost:3001/list")
        .send()
        .await?
        .json::<CreateListResponse>()
        .await
}

pub async fn get_lists() -> Result<Vec<String>, reqwest::Error> {
    reqwest::Client::new()
        .get("http://localhost:3001/lists")
        .send()
        .await?
        .json()
        .await
}

pub async fn spawn_websocket(mut signal: Signal<String, SyncStorage>) {
    let websocket = reqwest::Client::default()
        .get("wss://eth.merkle.io")
        .upgrade()
        .send()
        .await
        .unwrap()
        .into_websocket()
        .await
        .unwrap();

    let (mut tx, mut rx) = websocket.split();
    let j = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "eth_subscribe",
        "params": [
            "newHeads"
        ]
    });
    tx.send(Message::Text(j.to_string())).await.unwrap();

    while let Some(message) = rx.try_next().await.unwrap() {
        match message {
            Message::Text(message) => {
                let j: Value = serde_json::from_str(message.as_str()).unwrap();
                let number = j["params"]["result"]["number"].to_string();
                *signal.write() = number;
            }
            _ => (),
        }
    }
}
