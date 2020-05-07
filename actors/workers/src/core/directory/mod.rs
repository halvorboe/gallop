pub mod memory;

pub trait Directory {
    fn new() -> Self;
    // Returns a list of file names.
    fn list(&self) -> Vec<String>;
    // Appends to the file named 'name'. Creates it if it does not exist.
    fn append(&mut self, name: String, line: String);
    // Returns all the lines in a file.
    fn read(&self, name: String) -> Option<Vec<String>>;
}