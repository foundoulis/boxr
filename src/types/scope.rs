use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use super::{userfunctions::UserDefinedFunction, Expr};

#[derive(Debug)]
pub struct LexicalVarStorage {
    environ: HashMap<String, Expr>,
    local: HashMap<String, Expr>,
    functions_env: HashMap<String, UserDefinedFunction>,
    functions_local: HashMap<String, UserDefinedFunction>,
}

impl LexicalVarStorage {
    pub fn new() -> Self {
        LexicalVarStorage {
            environ: HashMap::new(),
            local: HashMap::new(),
            functions_env: HashMap::new(),
            functions_local: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.local.get(name).or_else(|| self.environ.get(name))
    }
    pub fn put(&mut self, name: &str, value: Expr) {
        self.local.insert(name.to_string(), value);
    }

    pub fn put_func(&mut self, name: &str, value: UserDefinedFunction) {
        self.functions_local.insert(name.to_string(), value);
    }
    pub fn get_func(&self, name: &str) -> Option<&UserDefinedFunction> {
        self.functions_local
            .get(name)
            .or_else(|| self.functions_env.get(name))
    }

    pub fn fork(&self) -> LexicalVarStorage {
        let mut environ_merge = HashMap::new();
        for (k, v) in &self.environ {
            environ_merge.insert(k.clone(), v.clone());
        }
        for (k, v) in &self.local {
            environ_merge.insert(k.clone(), v.clone());
        }

        let mut function_merge = HashMap::new();
        for (k, v) in &self.functions_env {
            function_merge.insert(k.clone(), v.clone());
        }
        for (k, v) in &self.functions_local {
            function_merge.insert(k.clone(), v.clone());
        }

        LexicalVarStorage {
            environ: environ_merge,
            local: HashMap::new(),
            functions_env: function_merge,
            functions_local: HashMap::new(),
        }
    }
}

impl Index<&str> for LexicalVarStorage {
    type Output = Expr;

    fn index(&self, name: &str) -> &Expr {
        self.get(name).unwrap()
    }
}

impl IndexMut<&str> for LexicalVarStorage {
    fn index_mut(&mut self, name: &str) -> &mut Expr {
        self.local.get_mut(name).unwrap()
    }
}
