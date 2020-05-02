use uuid::Uuid;
use crate::util::random;
use tantivy::Index;

pub struct Table {
    index: Index,
}

pub struct Row {
    timestamp: u128,
    id: Uuid,
}

impl Table {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn insert(&self, row: Row) {
        println!("Indexing {}...", row.id);
    }

    pub fn select(&self) -> Vec<Row> {
        vec![]
    }

    pub fn count(&self) -> usize {
        3
    }
}

impl Row {
    pub fn new() -> Self {
        Self {
            timestamp: random::timestamp(),
            id: Uuid::new_v4(),
        }
    }
}
