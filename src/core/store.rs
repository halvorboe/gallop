use std::collections::HashMap;

pub trait Store {
    fn new() -> Self;
    fn items(&self) -> Vec<(&String, &String)>;
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
    fn items(&self) -> Vec<(&String, &String)> {
        let mut result: Vec<(&String, &String)> = Vec::new();
        for (key, value) in self.map.iter() {
            result.push((key, value));
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

#[allow(unused_variables, unused_imports)]
mod tests {

    use super::{InMemoryStore, Store};

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

    // TODO: Test all
}
