use std::collections::HashMap;

pub struct Environment {
    vars: HashMap<String, i64>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, var: &str) -> i64 {
        *self.vars.get(var).unwrap_or(&0)
    }

    pub fn set(&mut self, var: &str, value: i64) {
        self.vars.insert(var.to_string(), value);
    }
}
