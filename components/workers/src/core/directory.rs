use std::mem;


pub trait Directory {
    // Returns a list of file names.
    fn list(&self) -> Vec<String>;
    // Appends to the file named 'name'. Creates it if it does not exist.
    fn append(&mut self, name: String, line: String);
    // Returns all the lines in a file.
    fn read(&self, name: String) -> Vec<String>;
}

#[derive(Debug)]
pub struct InMemoryFile {
    name: String,
    lines: Vec<String>,
}

#[derive(Debug)]
pub struct InMemoryDirectory {
    files: Vec<InMemoryFile>,
}

impl InMemoryFile {

    pub fn default() -> Self {
        Self { name: String::default(), lines: vec![] }
    }

    pub fn from(name: String) -> Self {
        Self { name: name, lines: vec![] }
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
    } 

    pub fn copy_of_lines(&self) -> Vec<String> {
        self.lines.iter().map(|it| it.clone()).collect()
    }
}


impl InMemoryDirectory {
    
    pub fn new() -> Self {
        Self { files: vec![] }
    }

    fn find(&self, name: String) -> Option<&InMemoryFile> {
        for file in &self.files {
            if file.name == name {
                return Some(file);
            }
        }
        None
    }


    fn find_mut(&mut self, name: String) -> Option<&mut InMemoryFile> {
        for file in &mut self.files {
            if file.name == name {
                return Some(file);
            }
        }
        None
    }

    fn create(&mut self, name: String) {
        self.files.push(InMemoryFile::from(name));
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
    fn read(&self, name: String) -> Vec<String> {
        self.find(name.clone()).unwrap().copy_of_lines()
    }
}

