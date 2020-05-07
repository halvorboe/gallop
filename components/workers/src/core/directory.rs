
pub trait Directory {
    // Returns a list of file names.
    fn list(&self) -> Vec<String>;
    // Appends to the file named 'name'. Creates it if it does not exist.
    fn append(&mut self, name: String, line: String);
    // Returns all the lines in a file.
    fn read(&self, name: String) -> Option<Vec<String>>;
}

#[derive(Debug, Clone)]
pub struct InMemoryFile {
    name: String,
    lines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct InMemoryDirectory {
    files:  Box<Vec<InMemoryFile>>,
}

impl InMemoryFile {
    pub fn default() -> Self {
        Self {
            name: String::default(),
            lines: vec![],
        }
    }

    pub fn from(name: String) -> Self {
        Self {
            name: name,
            lines: vec![],
        }
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn lines(&self) -> Vec<String> {
        self.lines.iter().map(|it| it.clone()).collect()
    }
}

impl InMemoryDirectory {
    pub fn new() -> Self {
        Self { files: Box::from(Vec::new())  }
    }

    fn find(&self, name: String) -> Option<&InMemoryFile> {
        for file in self.files.iter() {
            if file.name == name {
                return Some(file);
            }
        }
        None
    }

    fn find_mut(&mut self, name: String) -> Option<&mut InMemoryFile> {
        for file in self.files.iter_mut() {
            if file.name == name {
                return Some(file);
            }
        }
        None
    }

    fn create(&mut self, name: String) {
        debug!("Files before: [{}]", self.files.iter().map(|it| it.name.clone()).collect::<Vec<String>>().join(", "));
        let file = InMemoryFile::from(name);
        self.files.push(file);
        debug!("Files after: [{}]", self.files.iter().map(|it| it.name.clone()).collect::<Vec<String>>().join(", "));
    }
}

impl Directory for InMemoryDirectory {
    fn list(&self) -> Vec<String> {
        self.files.iter().map(|it| it.name.clone()).collect()
    }
    fn append(&mut self, name: String, line: String) {
        if self.find(name.clone()).is_none() {
            self.create(name.clone());
        }
        self.find_mut(name.clone()).unwrap().push(line);
    }
    fn read(&self, name: String) -> Option<Vec<String>> {
        Some(self.find(name.clone())?.lines())
    }
}

mod tests {

    use super::{InMemoryFile, InMemoryDirectory, Directory};

    #[test]
    fn test_file_basic() {
        let contents = vec!["Hello", "World", "!"];
        let file = generate_file("test".to_string(), contents);
        assert_eq!(vec!["Hello", "World", "!"], file.lines());
    }

    #[test]
    fn test_file_basic_empty() {
        let mut file = InMemoryFile::from("test".to_string());
        let empty: Vec<String> = Vec::new();
        assert_eq!(empty, file.lines());
    }

    #[test]
    fn test_directory_basic() {
        let name = String::from("test.txt");
        let mut directory: InMemoryDirectory = InMemoryDirectory::new();
        directory.append(name.clone(), name.clone());
        directory.append(name.clone(), name.clone());
        directory.append(name.clone(), name.clone());
        assert_eq!(directory.read(name.clone()).unwrap(), vec![name.clone(), name.clone(), name.clone()])
    }

    #[test]
    fn test_directory_not_found() {
        let name = String::from("test.txt");
        let mut directory: InMemoryDirectory = InMemoryDirectory::new();
        assert_eq!(directory.read(name.clone()), None)
    }


    fn generate_file(name: String, contents: Vec<&str>) -> InMemoryFile {
        let mut file = InMemoryFile::from(name);
        for line in contents.iter() {
            file.push(line.clone().to_string());
        }
        file
    }


}