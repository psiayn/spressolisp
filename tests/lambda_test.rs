#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_integer_expr_in_env, eval_expr_in_env, check_list_expr_eq};

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

#[test]
fn test_recursion() {
    let mut env = Env::new();
    eval_expr_in_env("(define factorial (lambda n (if (== n 0) 1 (* n (factorial (- n 1))))))", &mut env);
    check_integer_expr_in_env("(factorial 5)", 120, &mut env);
}

#[test]
fn test_tail_recursion() {
    let mut env = Env::new();
    // Tail recursive factorial with accumulator
    eval_expr_in_env("(define fact_tail (lambda (n acc) (if (== n 0) acc (fact_tail (- n 1) (* n acc)))))", &mut env);
    eval_expr_in_env("(define factorial_tail (lambda n (fact_tail n 1)))", &mut env);
    check_integer_expr_in_env("(factorial_tail 5)", 120, &mut env);
}

#[test]
fn test_mutual_recursion() {
    let mut env = Env::new();
    // even and odd functions that call each other
    eval_expr_in_env("(define is_even (lambda n (if (== n 0) true (is_odd (- n 1)))))", &mut env);
    eval_expr_in_env("(define is_odd (lambda n (if (== n 0) false (is_even (- n 1)))))", &mut env);
    eval_expr_in_env("(define test_even (is_even 4))", &mut env);
    eval_expr_in_env("(define test_odd (is_odd 3))", &mut env);
    check_integer_expr_in_env("(if test_even 1 0)", 1, &mut env);
    check_integer_expr_in_env("(if test_odd 1 0)", 1, &mut env);
}

#[test]
fn test_recursive_list_processing() {
    let mut env = Env::new();
    // Sum all elements in a list recursively
    eval_expr_in_env("(define sum_list (lambda lst (if (empty? lst) 0 (+ (nth lst 0) (sum_list (rest lst))))))", &mut env);
    check_integer_expr_in_env("(sum_list ('(1 2 3 4 5)))", 15, &mut env);
}

#[test]
fn test_multiple_recursion() {
    let mut env = Env::new();
    // Fibonacci with multiple recursive calls
    eval_expr_in_env("(define fib (lambda n (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))", &mut env);
    check_integer_expr_in_env("(fib 6)", 8, &mut env); // 0,1,1,2,3,5,8
}
