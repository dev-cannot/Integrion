use std::collections::HashMap;
use crate::parser::Expr;

#[derive(Clone)]
pub struct Environment {
    variables: HashMap<String, i64>,
    functions: HashMap<String, (Vec<String>, Expr)>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get(&self, var: &str) -> i64 {
        *self.variables.get(var).unwrap_or(&0)
    }

    pub fn set(&mut self, var: String, value: i64) {
        self.variables.insert(var, value);
    }

    pub fn set_func(&mut self, name: String, params: Vec<String>, body: Expr) {
        self.functions.insert(name, (params, body));
    }

    pub fn get_func(&self, name: &str) -> (Vec<String>, Expr) {
        self.functions.get(name).expect("Function not found").clone()
    }
}
