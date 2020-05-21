

use std::collections::HashMap;

pub trait Store {
    fn new() -> Self;
    fn keys(&self) -> Vec<&String>;
    fn get(&self, key: String) -> Option<&String>;
    fn set(&mut self, key: String, value: String);
}

#[derive(Clone)]
pub struct InMemoryStore {
    map: HashMap<String, String>,
}

impl Store for InMemoryStore {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn keys(&self) -> Vec<&String> {
        let mut result:Vec<&String> = Vec::new();
        for key in self.map.keys() {
            result.push(key);
        }
        result
    }
    fn get(&self, key: String) -> Option<&String> {
        self.map.get(&key)
    }
    fn set(&mut self, key: String, value: String) { 
        self.map.insert(key.clone(), value.clone());
    }

}

mod tests {

    use super::{Store, InMemoryStore};

    #[test]
    fn test_basic() {
        let mut store = InMemoryStore::new();
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();
        let d = "d".to_string();
        store.set(a.clone(), b.clone());
        store.set(b.clone(), c.clone());
        store.set(c.clone(), a.clone());
        assert_eq!(store.get(a.clone()).unwrap(), &b.clone());
        assert_eq!(store.get(b.clone()).unwrap(), &c.clone());
        assert_eq!(store.get(c.clone()).unwrap(), &a.clone());
        assert_eq!(store.get(d.clone()), None);
        // assert_eq!(store.keys(), vec![&a.clone(), &b.clone(), &c.clone()])
    }

}