use std::collections::HashMap;

use model::ShoppingListItem;
use uuid::Uuid;

pub struct InMemoryDatabase {
    inner: HashMap<String, ShoppingList>,
}

#[derive(Clone)]
pub struct ShoppingItem {
    pub title: String,
    pub creator: String,
}

struct ShoppingList {
    list: HashMap<String, ShoppingItem>,
}

impl Default for ShoppingList {
    fn default() -> Self {
        Self {
            list: [
                (
                    "6855cfc9-78fd-4b66-8671-f3c90ac2abd8".to_string(),
                    ShoppingItem {
                        title: "Coffee".to_string(),
                        creator: "Roland".to_string(),
                    },
                ),
                (
                    "3d778d1c-5a4e-400f-885d-10212027382d".to_string(),
                    ShoppingItem {
                        title: "Tomato Seeds".to_string(),
                        creator: "Tania".to_string(),
                    },
                ),
            ]
            .into(),
        }
    }
}

impl InMemoryDatabase {
    pub fn insert_item(&mut self, list_uuid: &str, item_uuid: &str, shopping_item: ShoppingItem) {
        self.inner
            .get_mut(list_uuid)
            .and_then(|list| list.list.insert(item_uuid.to_string(), shopping_item));
    }

    pub fn delete_item(&mut self, list_uuid: &str, item_uuid: &str) {
        self.inner
            .get_mut(list_uuid)
            .and_then(|list| list.list.remove(item_uuid));
    }

    pub fn create_list(&mut self, list_uuid: &str) {
        self.inner
            .insert(list_uuid.to_string(), ShoppingList::default());
    }

    fn get_list(&self, list_uuid: &str) -> Option<&ShoppingList> {
        self.inner.get(list_uuid)
    }

    pub fn get_lists(&self) -> Vec<String> {
        self.inner.keys().map(|s| s.clone()).collect()
    }

    pub fn as_vec(&self, list_uuid: &str) -> Vec<ShoppingListItem> {
        let list = self.get_list(list_uuid);
        match list {
            Some(list) => list
                .list
                .iter()
                .map(|(key, item)| ShoppingListItem {
                    title: item.title.clone(),
                    posted_by: item.creator.clone(),
                    uuid: key.clone(),
                })
                .collect(),
            None => Vec::default(),
        }
    }
}

impl Default for InMemoryDatabase {
    fn default() -> Self {
        let mut inner = HashMap::new();
        inner.insert(
            "9e137e61-08ac-469d-be9d-6b3324dd20ad".to_string(),
            ShoppingList::default(),
        );

        InMemoryDatabase { inner }
    }
}
