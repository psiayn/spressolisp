#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_conditional, check_integer_expr};

#[test]
fn test_relops() {
    check_conditional("(> 1 2)", false);
    check_conditional("(< 1 2)", true);
    check_conditional("(>= 1 2)", false);
    check_conditional("(<= 2 2)", true);
    check_conditional("(== 2 2)", true);
    check_conditional("(!= 1 2)", true);
}

#[test]
fn test_conditional_ops() {
    check_conditional("(not true)", false);
    check_conditional("(and (> 1 2) (> 3 2))", false);
    check_conditional("(or (> 1 2) (> 3 2))", true);
}

#[test]
fn test_conditional() {
    check_integer_expr("(if true (print 10) (print 11))", 10);
}
