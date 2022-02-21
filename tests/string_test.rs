#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_string_expr_in_env, eval_expr_in_env};

use spressolisp::env::Env;

#[test]
fn test_string_nospace() {
    let mut env = Env::new();
    eval_expr_in_env("(define x \"helloworld\")", &mut env);
    check_string_expr_in_env("(x)", "helloworld", &mut env);
}

#[test]
fn test_string_space() {
    let mut env = Env::new();
    eval_expr_in_env("(define x \"hello world\")", &mut env);
    check_string_expr_in_env("(x)", "hello world", &mut env);
}
