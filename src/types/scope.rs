use std::{collections::HashMap, ops::Index};

use super::{Cons, ConsValue};

#[derive(Debug)]
pub struct LexicalVarStorage {
    environ: HashMap<String, Cons>,
    local: HashMap<String, Cons>,
}

impl LexicalVarStorage {
    pub fn new() -> Self {
        LexicalVarStorage {
            environ: HashMap::new(),
            local: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Cons> {
        self.local.get(name).or_else(|| self.environ.get(name))
    }
    pub fn put(&mut self, name: &str, value: Cons) {
        self.local.insert(name.to_string(), value);
    }

    pub fn fork(&self) -> LexicalVarStorage {
        let mut environ_merge = HashMap::new();
        for (k, v) in &self.environ {
            environ_merge.insert(k.clone(), v.clone());
        }
        for (k, v) in &self.local {
            environ_merge.insert(k.clone(), v.clone());
        }

        LexicalVarStorage {
            environ: environ_merge,
            local: HashMap::new(),
        }
    }
}

impl Index<&str> for LexicalVarStorage {
    type Output = Cons;

    fn index(&self, name: &str) -> &Cons {
        self.get(name).unwrap_or(&Cons::Value(ConsValue::NIL))
    }
}
