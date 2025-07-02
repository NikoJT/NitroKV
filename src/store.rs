use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct LoaDb {
    store: HashMap<String, String>,
}

impl LoaDb {
    pub fn new() -> Self {
        LoaDb {
            store: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    pub fn keys(&mut self) -> impl Iterator<Item = &String> {
        self.store.keys()
    }
}
