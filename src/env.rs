use std::collections::HashMap;

use crate::ast::Expr;

use crate::eval::{add, mul, sub};

pub type EnvType = HashMap::<String, Expr>;

pub fn standard_env() -> HashMap<String, Expr> {
    let mut env = EnvType::new();
    env.insert(
        "+".to_string(),
        Expr::Func(add),
    );
    env.insert(
        "*".to_string(),
        Expr::Func(mul),
    );
    env.insert(
        "-".to_string(),
        Expr::Func(sub),
    );
    return env;
}

