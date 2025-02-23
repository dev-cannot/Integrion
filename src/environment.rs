use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, i64>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<i64> {
        self.variables.get(name).copied()
    }

    pub fn set(&mut self, name: &str, value: i64) {
        self.variables.insert(name.to_string(), value);
    }
}
