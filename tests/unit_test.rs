//! heh
//!
//! NOTE: NOT a unit test

#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_unit_expr_in_env, eval_expr_in_env};

use spressolisp::env::Env;

#[test]
fn unit_type_define() {
    let mut env = Env::new();
    eval_expr_in_env("(define x ())", &mut env);
    check_unit_expr_in_env("x", &mut env);
}

#[test]
fn empty_expr_is_unit() {
    let mut env = Env::new();
    check_unit_expr_in_env("", &mut env);
}

#[test]
fn loop_returns_unit() {
    let mut env = Env::new();
    eval_expr_in_env("(define x 10)", &mut env);
    check_unit_expr_in_env("(loop (> x 5) (define x (- x 1)))", &mut env);
}

#[test]
fn if_without_else_when_false_returns_unit() {
    let mut env = Env::new();
    check_unit_expr_in_env("(if (> 5 10) true)", &mut env);
}
