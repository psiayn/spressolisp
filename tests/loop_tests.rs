#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_integer_expr_in_env, eval_expr_in_env};

use spressolisp::env::Env;

#[test]
fn test_loop_basic() {
    let mut env = Env::new();
    eval_expr_in_env("(define x 100)", &mut env);
    eval_expr_in_env("(loop (> x 50) (define x (- x 10)))", &mut env);
    check_integer_expr_in_env("x", 50, &mut env);
}
