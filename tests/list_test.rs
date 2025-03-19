#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_list_expr_eq, eval_list_expr, check_integer_expr_in_env, eval_expr_in_env};
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

#[test]
fn test_reduce_function_on_list() {
    let mut env = Env::new();
    // Test sum reduction
    eval_expr_in_env("(define sum (reduce ('(1 2 3 4 5)) 0 (lambda (acc x) (+ acc x))))", &mut env);
    check_integer_expr_in_env("sum", 15, &mut env);
    
    // Test product reduction
    eval_expr_in_env("(define product (reduce ('(1 2 3 4 5)) 1 (lambda (acc x) (* acc x))))", &mut env);
    check_integer_expr_in_env("product", 120, &mut env);
}

#[test]
fn test_filter_function_on_list() {
    let mut env = Env::new();
    // Filter even numbers (using remainder of division by 2)
    let expr = "(filter ('(1 2 3 4 5 6)) (lambda x (== 0 (- x (* 2 (/ x 2))))))";
    let res = eval_list_expr(expr, &mut env);
    check_list_expr_eq(res, "('(2 4 6))");

    // Filter numbers greater than 3
    let expr = "(filter ('(1 2 3 4 5 6)) (lambda x (> x 3)))";
    let res = eval_list_expr(expr, &mut env);
    check_list_expr_eq(res, "('(4 5 6))");
}
