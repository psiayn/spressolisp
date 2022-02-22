#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_integer_expr_in_env, eval_expr_in_env};

use spressolisp::env::Env;

#[test]
fn test_lambda_basic() {
    let mut env = Env::new();
    eval_expr_in_env("(define mul_100 (lambda x (* x 100)))", &mut env);
    check_integer_expr_in_env("(mul_100 3)", 300, &mut env);
}

#[test]
fn test_lambda_two_arg() {
    let mut env = Env::new();
    eval_expr_in_env("(define mul (lambda (x y) (* x y)))", &mut env);
    check_integer_expr_in_env("(mul 3 4)", 12, &mut env);
}
