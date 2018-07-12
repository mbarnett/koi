use std::collections::HashMap;
use std::fs;

pub struct Index {
    hashed_executables: HashMap<String, String>
}

impl Index {
    pub fn new(paths: &[&str]) -> Index {
        Index{hashed_executables: build_index(paths)}
    }

    pub fn lookup(&self, key: &str) -> Option<&String> {
        self.hashed_executables.get(key)
    }
}

fn build_index(paths: &[&str]) -> HashMap<String, String> {
    let mut hashed_executables = HashMap::new();

    for path in paths.iter() {
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => continue,
        };

        for entry in dir {
            let item = match entry {
                Ok(item) => item,
                Err(_) => continue,
            };

            let filename = match item.file_name().into_string() {
                Ok(name) => name,
                Err(_) => continue,
            };

            let filepath = match item.path().into_os_string().into_string() {
                Ok(path) => path,
                Err(_) => continue,
            };

            hashed_executables.insert(filename, filepath);
            debug!("Found: {:?}", item.path())
        }
    }

    hashed_executables
}
