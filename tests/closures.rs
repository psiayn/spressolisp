#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_integer_expr_in_env, eval_expr_in_env};
use spressolisp::env::Env;

#[test]
fn test_nested_add() {
    let mut env = Env::new();
    eval_expr_in_env("(define add (lambda x (lambda y (+ x y))))", &mut env);
    check_integer_expr_in_env("((add 10) 20)", 30, &mut env);
}
