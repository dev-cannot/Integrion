// src/environment.rs

use std::collections::HashMap;

pub struct Environment {
    pub vars: HashMap<String, i64>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: HashMap::new(),
        }
    }
    
    pub fn get(&self, name: &str) -> Option<i64> {
        self.vars.get(name).cloned()
    }
    
    pub fn set(&mut self, name: String, value: i64) {
        self.vars.insert(name, value);
    }
}
