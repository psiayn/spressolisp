#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_integer_expr, eval_expr_in_env};

use spressolisp::env::Env;

#[test]
fn test_check_define_in_scope() {
    let mut env = Env::new();
    eval_expr_in_env("(define x 100)", &mut env);
    if env.contains_key("x") {
        assert!(true, "Env contains x");
    } else {
        assert!(false, "Env does not contain x");
    }
}

#[test]
fn test_check_define_res() {
    check_integer_expr("(define x 100)", 100);
}

#[test]
fn test_print() {
    check_integer_expr("(print 100)", 100);
}
