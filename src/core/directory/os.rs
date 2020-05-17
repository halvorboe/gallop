use super::Directory;

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct OSDirectory {
    path: String,
}

impl OSDirectory {
    fn build_path_to_file(&self, file_name: String) -> String {
        let path = Path::new(&self.path).join(&file_name);
        path.to_string_lossy().to_owned().to_string()
    }

    fn create_if_not_exist(&self, path_to_file: String) {
        let _file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path_to_file);
    }
}

impl Directory for OSDirectory {
    fn new() -> Self {
        let id = uuid::Uuid::new_v4();
        let path = format!("/tmp/gallop-{}", id);
        fs::create_dir(path.clone()).unwrap();
        Self { path }
    }
    fn list(&self) -> Vec<String> {
        let mut items: Vec<String> = Vec::new();
        let paths = fs::read_dir(self.path.clone()).unwrap();
        for path in paths {
            items.push(path.unwrap().file_name().into_string().unwrap());
        }
        items
    }
    fn append(&mut self, name: String, line: String) {
        // println!("{} + {} = ?", name, line);
        let path_to_file = self.build_path_to_file(name);
        self.create_if_not_exist(path_to_file.clone());
        // println!("path: {}", path_to_file);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path_to_file)
            .unwrap();
        writeln!(file, "{}", line).unwrap();
    }
    fn read(&self, name: String) -> Option<Vec<String>> {
        let path_to_file = self.build_path_to_file(name);
        let file = fs::File::open(path_to_file);
        if file.is_err() {
            return None;
        }
        let mut lines: Vec<String> = Vec::new();
        for line in BufReader::new(file.unwrap()).lines() {
            if line.is_ok() {
                lines.push(line.unwrap());
            }
        }
        Some(lines)
    }

    fn read_size(&self, name: String) -> Option<u64> {
        let path_to_file = self.build_path_to_file(name);
        let metadata = fs::metadata(path_to_file);
        if metadata.is_err() {
            return None;
        }
        Some(metadata.unwrap().len())
    }
}

#[allow(unused_imports)]
mod tests {

    use super::*;

    #[test]
    fn test_directory_basic() {
        let name = String::from("test.txt");
        let mut directory: OSDirectory = OSDirectory::new();
        directory.append(name.clone(), name.clone());
        directory.append(name.clone(), name.clone());
        directory.append(name.clone(), name.clone());
        assert_eq!(
            directory.read(name.clone()).unwrap(),
            vec![name.clone(), name.clone(), name]
        )
    }

    #[test]
    fn test_directory_not_found() {
        let name = String::from("test.txt");
        let directory: OSDirectory = OSDirectory::new();
        assert_eq!(directory.read(name), None)
    }
}
