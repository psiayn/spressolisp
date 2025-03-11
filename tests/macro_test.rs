#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_list_expr_eq, eval_list_expr, check_integer_expr_in_env, eval_expr_in_env};
use spressolisp::env::Env;

#[test]
fn test_simple_macro() {
    let mut env = Env::new();
    
    // Define a macro that doubles its argument
    eval_expr_in_env(
        "(defmacro double (x) (* x 2))",
        &mut env
    );
    
    // Test the macro
    check_integer_expr_in_env("(double 5)", 10, &mut env);
}

#[test]
fn test_list_macro() {
    let mut env = Env::new();
    
    // Define a macro that creates a list with duplicated elements
    eval_expr_in_env(
        "(defmacro duplicate-list (x) (append (list x) (list x)))",
        &mut env
    );
    
    // Test the macro with a list
    let res = eval_list_expr("(duplicate-list 1)", &mut env);
    check_list_expr_eq(res, "(list 1 1)");
}

#[test]
fn test_complex_macro() {
    let mut env = Env::new();
    
    // Define a macro for unless (opposite of if)
    eval_expr_in_env(
        "(defmacro unless (condition body) (if (not condition) body 0))",
        &mut env
    );
    
    // Test the unless macro
    check_integer_expr_in_env("(unless false 42)", 42, &mut env);
    check_integer_expr_in_env("(unless true 42)", 0, &mut env);
} 