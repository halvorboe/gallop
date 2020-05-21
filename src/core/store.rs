

pub trait Store {
    fn new() -> Self;
}

#[derive(Clone)]
pub struct FileStore {}

impl Store for FileStore {
    fn new() -> Self {
        Self {}
    }
}
