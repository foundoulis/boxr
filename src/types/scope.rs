use std::{collections::HashMap, ops::Index};

use super::{function::UserFunction, Cons, ConsValue};

#[derive(Debug, Clone)]
pub struct LexicalVarStorage {
    environ: HashMap<String, Cons>,
    local: HashMap<String, Cons>,
    environ_func: HashMap<String, UserFunction>,
    local_func: HashMap<String, UserFunction>,
}

impl LexicalVarStorage {
    pub fn new() -> Self {
        LexicalVarStorage {
            environ: HashMap::new(),
            local: HashMap::new(),
            environ_func: HashMap::new(),
            local_func: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Cons> {
        self.local.get(name).or_else(|| self.environ.get(name))
    }
    pub fn put(&mut self, name: &str, value: Cons) {
        self.local.insert(name.to_string(), value);
    }

    pub fn get_func(&self, name: &str) -> Option<&UserFunction> {
        self.local_func
            .get(name)
            .or_else(|| self.environ_func.get(name))
    }
    pub fn put_func(&mut self, name: &str, value: UserFunction) {
        self.local_func.insert(name.to_string(), value);
    }

    pub fn fork(&self) -> LexicalVarStorage {
        let mut environ_merge = HashMap::new();
        for (k, v) in &self.environ {
            environ_merge.insert(k.clone(), v.clone());
        }
        for (k, v) in &self.local {
            environ_merge.insert(k.clone(), v.clone());
        }

        let mut environ_func_merge = HashMap::new();
        for (k, v) in &self.environ_func {
            environ_func_merge.insert(k.clone(), v.clone());
        }
        for (k, v) in &self.local_func {
            environ_func_merge.insert(k.clone(), v.clone());
        }

        LexicalVarStorage {
            environ: environ_merge,
            local: HashMap::new(),
            environ_func: environ_func_merge,
            local_func: HashMap::new(),
        }
    }
}

impl Index<&str> for LexicalVarStorage {
    type Output = Cons;

    fn index(&self, name: &str) -> &Cons {
        self.get(name).unwrap_or(&Cons::Value(ConsValue::NIL))
    }
}
