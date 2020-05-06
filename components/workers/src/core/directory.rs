pub trait File {
    fn read();
    fn append();
}

pub trait Directory<F: File> {
    fn list(&self) -> Vec<F>;
    fn open_or_create(&mut self, name: String) -> &F;
}

#[derive(Debug, Clone)]
pub struct InMemoryDirectory<'a> {
    files: Vec<&'a InMemoryFile>
}

impl InMemoryDirectory<'_> {
    pub fn new() -> Self {
        return Self {
            files: vec![],
        };
    }
    fn create(&self, name: String) {
        self.files.push(&InMemoryFile::new())
    }
    fn find(&self, name: String) -> Option<&InMemoryFile> {
        self.files.into_iter().filter(|it| it.name == name).take(1).next()
    }
}
#[derive(Debug, Clone)]
pub struct InMemoryFile {
    name: String,
    contents: Vec<String>,
}

impl InMemoryFile {
    pub fn new() -> Self {
        Self {
            name: String::default(),
            contents: Vec::default(),
        }
    }
}

impl Directory<InMemoryFile> for InMemoryDirectory<'_> {
    fn list(&self) -> Vec<InMemoryFile> {
        vec![InMemoryFile::new()]
    }
    fn open_or_create(&mut self, name: String) -> &InMemoryFile  {
        let hit: Option<&InMemoryFile> = self.find(name.clone());
        if hit.is_some() {
            return &hit.unwrap();
        } else {
            self.create(name.clone());
            return self.find(name.clone()).unwrap();
        }
    }

}

impl File for InMemoryFile {
    fn read() {}
    fn append() {}
}

// struct TemporaryDirectory {

// }

// struct TemoraryFile {}

// impl File for TemoraryFile {
//     fn read() {

//     }
//     fn append() {

//     }
// }

// impl TemporaryDirectory {

//     fn new() -> Self {
//         Self{}
//     }

// }

// impl Directory for TemporaryDirectory {

//     pub fn list() -> Vec<TemoraryFile> {
//         return vec![]
//     }

// }
