#[macro_use]
extern crate assert_float_eq;

pub mod common;
use common::{check_float_expr, check_integer_expr, check_number_syntax_err};

#[test]
fn test_addition() {
    check_integer_expr("(+ 12 32)", 44);

    check_float_expr("(+ 12.3 43.2)", 55.5);
}

#[test]
fn test_subtraction() {
    check_integer_expr("(- 12 32)", -20);
    check_float_expr("(- 12.3 10.1)", 2.2);
}

#[test]
fn test_multiplication() {
    check_integer_expr("(* 2 3)", 6);
    check_float_expr("(* 12.3 10)", 123.0);
}

#[test]
fn test_division() {
    check_integer_expr("(/ 10 5)", 2);
    check_float_expr("(/ 12.3 10)", 1.23);
}

#[test]
fn test_remainder() {
    check_integer_expr("(% 10 3)", 1);
    check_float_expr("(% 10.2 3)", 1.2);
    check_float_expr("(% 10.2 3.1)", 0.899999999999999);
    check_float_expr("(% 10 3.1)", 0.6999999999999997);
}

#[test]
fn test_wrong_addition_syntax() {
    check_number_syntax_err("(+ 12 32", "'(' not closed");
}
