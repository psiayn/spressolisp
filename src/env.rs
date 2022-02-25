use std::collections::HashMap;
use std::ops::Index;

use crate::ast::{Atom, Expr, ExprKind};

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
        env.insert("+".to_string(), ExprKind::Func(eval::add).into());
        env.insert("*".to_string(), ExprKind::Func(eval::mul).into());
        env.insert("-".to_string(), ExprKind::Func(eval::sub).into());
        env.insert("/".to_string(), ExprKind::Func(eval::div).into());

        // keywords
        env.insert("define".to_string(), ExprKind::Func(eval::define).into());
        env.insert("print".to_string(), ExprKind::Func(eval::print).into());
        env.insert("true".to_string(), ExprKind::Atom(Atom::Bool(true)).into());
        env.insert("false".to_string(), ExprKind::Atom(Atom::Bool(false)).into());
        env.insert("if".to_string(), ExprKind::Func(eval::if_cond).into());
        env.insert("lambda".to_string(), ExprKind::Func(eval::lambda).into());
        env.insert("loop".to_string(), ExprKind::Func(eval::while_loop).into());

        // relational operators
        env.insert(">".to_string(), ExprKind::Func(eval::gt).into());
        env.insert("<".to_string(), ExprKind::Func(eval::lt).into());
        env.insert(">=".to_string(), ExprKind::Func(eval::gteq).into());
        env.insert("<=".to_string(), ExprKind::Func(eval::lteq).into());
        env.insert("==".to_string(), ExprKind::Func(eval::eq).into());
        env.insert("!=".to_string(), ExprKind::Func(eval::neq).into());

        // logical operators
        env.insert("not".to_string(), ExprKind::Func(eval::not).into());
        env.insert("and".to_string(), ExprKind::Func(eval::and).into());
        env.insert("or".to_string(), ExprKind::Func(eval::or).into());

        // lists and their functions
        env.insert("'".to_string(), ExprKind::Func(eval::lists).into());
        env.insert("map".to_string(), ExprKind::Func(eval::map).into());
        env.insert("append".to_string(), ExprKind::Func(eval::append).into());
        
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
