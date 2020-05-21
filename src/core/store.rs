

trait Store {
    fn new() -> Self;
}

struct FileStore {}

impl Store for FileStore {
    fn new() -> Self {
        Self {}
    }
}
