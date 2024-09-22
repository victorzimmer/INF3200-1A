use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Storage {
    storage: Arc<RwLock<HashMap<String, String>>>,
}

impl Storage {
    pub fn new() -> Self {
        println!("Initialized HashMap storage!");
        Storage {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn store(&self, key: &str, value: &str) {
        let mut storage = self.storage.write().unwrap();
        storage.insert(key.to_string(), value.to_string());
    }

    pub fn retrieve(&self, key: &str) -> Option<String> {
        let storage = self.storage.read().unwrap();
        let value = storage.get(key).map_or(None, |v| Some(v.as_str()));
        return value.map_or(None, |s| Some(s.to_string()));
    }
}
