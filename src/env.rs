use std::collections::HashMap;
use std::ops::Index;

use crate::ast::{Atom, Expr};

use crate::eval;

pub type EnvMapType = HashMap<String, Expr>;

pub struct Env {
    map: EnvMapType,
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

        // relational operators
        env.insert(">".to_string(), Expr::Func(eval::gt));
        env.insert("<".to_string(), Expr::Func(eval::lt));
        env.insert(">=".to_string(), Expr::Func(eval::gteq));
        env.insert("<=".to_string(), Expr::Func(eval::lteq));
        env.insert("==".to_string(), Expr::Func(eval::eq));
        env.insert("!=".to_string(), Expr::Func(eval::neq));

        return Env { map: env };
    }

    pub fn insert(&mut self, key: &str, value: Expr) -> Option<Expr> {
        self.map.insert(key.to_string(), value)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn display(&self) {
        for (key, value) in &self.map {
            println!("{}: {}", key, value);
        }
    }
}

impl Index<&str> for Env {
    type Output = Expr;

    fn index(&self, key: &str) -> &Self::Output {
        &self.map[key]
    }
}
