#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_list_expr_eq, eval_list_expr};
use spressolisp::env::Env;

#[test]
fn test_creation_of_list() {
    let mut env = Env::new();
    let res = eval_list_expr("('(1 2 3))", &mut env);
    check_list_expr_eq(res, "('(1 2 3))")
}

#[test]
fn test_map_function_on_list() {
    let mut env = Env::new();
    let expr = "(map ('(1 2 3)) (lambda x (* x 10)))";
    let res = eval_list_expr(expr, &mut env);
    check_list_expr_eq(res, "('(10 20 30))")
}

#[test]
fn test_append_function_on_list() {
    let mut env = Env::new();
    let expr = "(append ('(1 2 3)) ('(4 5 6)) )";
    let res = eval_list_expr(expr, &mut env);
    check_list_expr_eq(res, "('(1 2 3 4 5 6))");
}
