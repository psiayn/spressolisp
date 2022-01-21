use std::collections::HashMap;
use std::ops::Index;

use crate::ast::Expr;

use crate::errors::SpressoError;
use crate::eval::{add, define, div, lambda, mul, print, sub};

pub type EnvMapType = HashMap<String, Expr>;

pub struct Env {
    map: EnvMapType,
    scopes: Vec<EnvMapType>,
}

impl Env {
    pub fn new() -> Self {
        let mut env = EnvMapType::new();
        env.insert("+".to_string(), Expr::Func(add));
        env.insert("*".to_string(), Expr::Func(mul));
        env.insert("-".to_string(), Expr::Func(sub));
        env.insert("/".to_string(), Expr::Func(div));
        env.insert("define".to_string(), Expr::Func(define));
        env.insert("print".to_string(), Expr::Func(print));
        env.insert("lambda".to_string(), Expr::Func(lambda));

        Env {
            map: env,
            scopes: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: Expr) -> Option<Expr> {
        // TODO: just take a String lmao
        if let Some(last) = self.scopes.last_mut() {
            last.insert(key.to_string(), value)
        } else {
            self.map.insert(key.to_string(), value)
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        if self.scopes.iter().rev().any(|map| map.contains_key(key)) {
            true
        } else {
            self.map.contains_key(key)
        }
    }

    pub fn in_new_scope<F>(&mut self, f: F) -> Result<Expr, SpressoError>
    where
        F: FnOnce(&mut Self) -> Result<Expr, SpressoError>,
    {
        self.scopes.push(EnvMapType::new());
        let res = f(self);
        self.scopes.pop();
        res
    }
}

impl Index<&str> for Env {
    type Output = Expr;

    /// Ensure key exists or I panik!
    fn index(&self, key: &str) -> &Self::Output {
        if let Some(scope) = self.scopes.iter().rev().find(|map| map.contains_key(key)) {
            &scope[key]
        } else {
            &self.map[key]
        }
    }
}
