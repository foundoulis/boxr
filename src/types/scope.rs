use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use super::Value;

pub struct LexicalVarStorage {
    environ: HashMap<String, Value>,
    local: HashMap<String, Value>,
}

impl LexicalVarStorage {
    pub fn new() -> Self {
        LexicalVarStorage {
            environ: HashMap::new(),
            local: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.local.get(name).or_else(|| self.environ.get(name))
    }

    pub fn put(&mut self, name: &str, value: Value) {
        self.local.insert(name.to_string(), value);
    }

    pub fn fork(&self) -> LexicalVarStorage {
        let mut result = HashMap::new();
        for (k, v) in &self.environ {
            result.insert(k.clone(), v.clone());
        }
        for (k, v) in &self.local {
            result.insert(k.clone(), v.clone());
        }

        LexicalVarStorage {
            environ: result,
            local: HashMap::new(),
        }
    }
}

impl Index<&str> for LexicalVarStorage {
    type Output = Value;

    fn index(&self, name: &str) -> &Value {
        self.get(name).unwrap()
    }
}

impl IndexMut<&str> for LexicalVarStorage {
    fn index_mut(&mut self, name: &str) -> &mut Value {
        self.local.get_mut(name).unwrap()
    }
}
