use std::collections::HashMap;

pub struct KeyValeRepo {
    repo: HashMap<String, String>,
}

impl KeyValeRepo {
    pub fn new() -> KeyValeRepo {
        KeyValeRepo {
            repo: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: String) {
        self.repo.insert(key, value);
    }

    pub fn del(&mut self, key: String) {
        self.repo.remove(&key);
    }

    pub fn get(&self, key: String) -> String {
        format!("{}", self.repo.get(&key).unwrap())
    }
}
