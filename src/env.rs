use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;

use slab::Slab;

use crate::ast::{Atom, Expr, ExprKind};

use crate::errors::{RuntimeError, SpressoError};
use crate::eval;

pub type EnvMapType = HashMap<String, Expr>;

pub struct Env {
    global_index: Rc<usize>,
    scopes: Vec<Rc<usize>>,
    scope_slab: Slab<EnvMapType>,
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
    }
}

impl Env {
    pub fn new() -> Self {
        let mut global = EnvMapType::new();
        // arithmetic ops
        global.insert("+".to_string(), ExprKind::Func(eval::add).into());
        global.insert("*".to_string(), ExprKind::Func(eval::mul).into());
        global.insert("-".to_string(), ExprKind::Func(eval::sub).into());
        global.insert("/".to_string(), ExprKind::Func(eval::div).into());

        // keywords
        global.insert("define".to_string(), ExprKind::Func(eval::define).into());
        global.insert("print".to_string(), ExprKind::Func(eval::print).into());
        global.insert("input".to_string(), ExprKind::Func(eval::input).into());
        global.insert("true".to_string(), ExprKind::Atom(Atom::Bool(true)).into());
        global.insert(
            "false".to_string(),
            ExprKind::Atom(Atom::Bool(false)).into(),
        );
        global.insert("if".to_string(), ExprKind::Func(eval::if_cond).into());
        global.insert("lambda".to_string(), ExprKind::Func(eval::lambda).into());
        global.insert("loop".to_string(), ExprKind::Func(eval::while_loop).into());

        // relational operators
        global.insert(">".to_string(), ExprKind::Func(eval::gt).into());
        global.insert("<".to_string(), ExprKind::Func(eval::lt).into());
        global.insert(">=".to_string(), ExprKind::Func(eval::gteq).into());
        global.insert("<=".to_string(), ExprKind::Func(eval::lteq).into());
        global.insert("==".to_string(), ExprKind::Func(eval::eq).into());
        global.insert("!=".to_string(), ExprKind::Func(eval::neq).into());

        // logical operators
        global.insert("not".to_string(), ExprKind::Func(eval::not).into());
        global.insert("and".to_string(), ExprKind::Func(eval::and).into());
        global.insert("or".to_string(), ExprKind::Func(eval::or).into());

        // lists and their functions
        global.insert("'".to_string(), ExprKind::Func(eval::list).into());
        global.insert("map".to_string(), ExprKind::Func(eval::map).into());
        global.insert("append".to_string(), ExprKind::Func(eval::append).into());

        let mut scope_slab = Slab::new();

        Env {
            global_index: scope_slab.insert(global),
            scopes: Vec::new(),
            scope_slab,
        }
    }

    fn scope(&self, index: Rc<usize>) -> &EnvMapType {
        self.scope_slab.get(index).unwrap().0
    }

    fn scope_mut(&mut self, index: Rc<usize>) -> &mut EnvMapType {
        self.scope_slab.get_mut(index).unwrap().0
    }

    fn global_scope(&self) -> &EnvMapType {
        self.scope(Rc::clone(&self.global_index))
    }

    fn global_scope_mut(&mut self) -> &mut EnvMapType {
        self.scope_mut(Rc::clone(&self.global_index))
    }

    pub fn insert(&mut self, key: &str, value: Expr) -> Option<Expr> {
        // TODO: just take a String lmao
        if let Some(last) = self.scopes.last() {
            self.scope_mut(Rc::clone(last))
                .insert(key.to_string(), value)
        } else {
            self.global_scope_mut().insert(key.to_string(), value)
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        if self
            .scopes
            .iter()
            .rev()
            .any(|map_index| self.scope(Rc::clone(map_index)).contains_key(key))
        {
            true
        } else {
            self.global_scope().contains_key(key)
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
        let scope_index = self.scope_slab.insert(EnvMapType::new());
        self.scopes.push(scope_index);
        let res = f(self);
        self.scopes.pop();
        res
    }

    pub fn display(&self) {
        for (key, value) in self.global_scope() {
            print!("{}\t:\t{}", key, value);
        }
    }

    pub fn get_current_scopes(&self) -> Vec<Rc<usize>> {
        self.scopes.iter().map(|s| Rc::clone(s)).collect()
    }
}

impl Index<&str> for Env {
    type Output = Expr;

    /// Ensure key exists or I panik!
    fn index(&self, key: &str) -> &Self::Output {
        if let Some(scope_index) = self
            .scopes
            .iter()
            .rev()
            .find(|map_index| self.scope(Rc::clone(map_index)).contains_key(key))
        {
            &self.scope(Rc::clone(scope_index))[key]
        } else {
            &self.global_scope()[key]
        }
    }
}
