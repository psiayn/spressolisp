#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_expr_error_in_env, check_integer_expr_in_env, eval_expr_in_env};

use spressolisp::env::Env;

#[test]
// see https://github.com/psiayn/spressolisp/issues/32
fn noncallable_as_first_arg_is_disallowed() {
    let mut env = Env::new();
    check_expr_error_in_env("(1)", "this is not something I can execute: 1 ", &mut env);
    check_expr_error_in_env(
        "(1 2 3 4)",
        "this is not something I can execute: 1 ",
        &mut env,
    );

    eval_expr_in_env("(define x 10)", &mut env);
    check_expr_error_in_env("(x)", "this is not something I can execute: 10 ", &mut env);

    eval_expr_in_env("(define x (lambda y (+ y 10)))", &mut env);
    check_integer_expr_in_env("(x 10)", 20, &mut env);
}
