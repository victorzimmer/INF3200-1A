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
        todo!();
    }

    pub fn retrieve(&self, key: &str) -> Option<&str> {
        todo!();
    }
}
