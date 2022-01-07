use std::collections::HashMap;
use std::ops::Index;

use crate::ast::Expr;

use crate::eval::{add, mul, sub, div};

pub type EnvMapType = HashMap<String, Expr>;

pub struct Env {
    map: EnvMapType,
}

impl Env {
    pub fn new() -> Self {
        let mut env = EnvMapType::new();
        env.insert("+".to_string(), Expr::Func(add));
        env.insert("*".to_string(), Expr::Func(mul));
        env.insert("-".to_string(), Expr::Func(sub));
        env.insert("/".to_string(), Expr::Func(div));
        return Env { map: env };
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}

impl Index<&str> for Env {
    type Output = Expr;

    fn index(&self, key: &str) -> &Self::Output {
        &self.map[key]
    }
}
