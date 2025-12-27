#[macro_use]
extern crate assert_float_eq;

pub mod common;

use common::{check_list_expr_eq, eval_list_expr};
use spressolisp::env::Env;

#[test]
fn test_comment() {
    let mut env = Env::new();
    let res = eval_list_expr(
        r"'(
        1
        ;; hi
        2
    )",
        &mut env,
    );
    check_list_expr_eq(res, "'(1 2)")
}
