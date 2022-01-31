use std::collections::HashMap;
use std::ops::Index;

use crate::ast::{Atom, Expr};

use crate::errors::{RuntimeError, SpressoError};
use crate::eval;

pub type EnvMapType = HashMap<String, Expr>;

pub struct Env {
    map: EnvMapType,
    scopes: Vec<EnvMapType>,
}

impl Env {
    pub fn new() -> Self {
        let mut env = EnvMapType::new();
        // arithmetic ops
        env.insert("+".to_string(), Expr::Func(eval::add));
        env.insert("*".to_string(), Expr::Func(eval::mul));
        env.insert("-".to_string(), Expr::Func(eval::sub));
        env.insert("/".to_string(), Expr::Func(eval::div));

        // keywords
        env.insert("define".to_string(), Expr::Func(eval::define));
        env.insert("print".to_string(), Expr::Func(eval::print));
        env.insert("true".to_string(), Expr::Atom(Atom::Bool(true)));
        env.insert("false".to_string(), Expr::Atom(Atom::Bool(false)));
        env.insert("if".to_string(), Expr::Func(eval::if_cond));
        env.insert("lambda".to_string(), Expr::Func(eval::lambda));
        env.insert("loop".to_string(), Expr::Func(eval::while_loop));

        // relational operators
        env.insert(">".to_string(), Expr::Func(eval::gt));
        env.insert("<".to_string(), Expr::Func(eval::lt));
        env.insert(">=".to_string(), Expr::Func(eval::gteq));
        env.insert("<=".to_string(), Expr::Func(eval::lteq));
        env.insert("==".to_string(), Expr::Func(eval::eq));
        env.insert("!=".to_string(), Expr::Func(eval::neq));

        // logical operators
        env.insert("not".to_string(), Expr::Func(eval::not));
        env.insert("and".to_string(), Expr::Func(eval::and));
        env.insert("or".to_string(), Expr::Func(eval::or));

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

    pub fn get_symbol(&self, key: &str) -> Result<Expr, SpressoError> {
        if self.contains_key(key) {
            Ok(self[key].clone())
        } else {
            Err(SpressoError::from(RuntimeError::from(format!(
                "Symbol not found: {}",
                key
            ))))
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

    pub fn display(&self) {
        for (key, value) in &self.map {
            print!("{}\t:\t{}", key, value);
        }
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
