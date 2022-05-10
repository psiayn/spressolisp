use spressolisp::{
    ast::{Atom, Expr, ExprKind, Number},
    env::Env,
    errors::{SpressoErrorType, SyntaxError},
    eval::extract_num,
    evaluate_expression,
};

pub fn eval_expr_in_env(expr: &str, env: &mut Env) -> Expr {
    match evaluate_expression("test".to_string(), expr.to_string(), env) {
        Ok(res) => res,
        Err(err) => panic!("Error evaluating '{}':\n{}", expr, err),
    }
}

fn eval_number_expr_in_env<T>(expr: &str, env: &mut Env, cb: T)
where
    T: Fn(Number),
{
    let res = eval_expr_in_env(expr, env);
    if let Ok(num) = extract_num(res, env) {
        cb(num)
    } else {
        panic!("Result of '{}' was not a number.", expr);
    }
}

fn eval_number_expr<T>(expr: &str, cb: T)
where
    T: Fn(Number),
{
    let mut env = Env::new();
    eval_number_expr_in_env(expr, &mut env, cb);
}

pub fn check_integer_expr_in_env(expr: &str, expected: i64, env: &mut Env) {
    eval_number_expr_in_env(expr, env, |num| {
        if let Number::Int(res) = num {
            assert_eq!(res, expected);
        } else {
            panic!("Result of '{}' was not an integer", expr);
        }
    });
}

pub fn check_integer_expr(expr: &str, expected: i64) {
    let mut env = Env::new();
    check_integer_expr_in_env(expr, expected, &mut env);
}

pub fn check_float_expr(expr: &str, expected: f64) {
    eval_number_expr(expr, |num| {
        if let Number::Float(res) = num {
            assert_f64_near!(res, expected);
        } else {
            panic!("Result of '{}' was not an integer", expr);
        }
    });
}

pub fn check_string_expr_in_env(expr: &str, expected: &str, env: &mut Env) {
    let res = eval_expr_in_env(expr, env);
    if let ExprKind::Atom(Atom::String(s)) = res.kind {
        assert_eq!(s, expected);
    } else {
        panic!("Result of '{}' was not a string.", expr);
    }
}

pub fn check_number_syntax_err(expr: &str, expected: &str) {
    let mut env = Env::new();

    if let Err(err) = evaluate_expression("test".to_string(), expr.to_string(), &mut env) {
        if let SpressoErrorType::Syntax(SyntaxError { err: err_str }) = err.detail {
            assert_eq!(err_str, expected);
        } else {
            panic!(
                "Expected syntax error for '{}', but got something else",
                expr
            );
        }
    } else {
        panic!("Invalid expression '{}' evaluated successfully", expr);
    }
}

pub fn check_conditional(expr: &str, expected: bool) {
    let mut env = Env::new();

    if let Ok(Expr {
        kind: ExprKind::Atom(Atom::Bool(res)),
        ..
    }) = evaluate_expression("test".to_string(), expr.to_string(), &mut env)
    {
        assert_eq!(expected, res, "Expected {} but got {}", expected, res);
    } else {
        panic!("Error evaluating expression: '{}'", expr);
    }
}

pub fn eval_list_expr(expr: &str, env: &mut Env) -> Vec<Expr> {
    if let Expr {
        kind: ExprKind::List(res),
        ..
    } = eval_expr_in_env(expr, env)
    {
        res
    } else {
        panic!("Error evaluating '{}' to a List", expr);
    }
}

pub fn check_list_expr_eq(list: Vec<Expr>, expected: &str) {
    let mut env = Env::new();
    let expected = eval_list_expr(expected, &mut env);
    assert_eq!(list, expected)
}

pub fn check_unit_expr_in_env(expr: &str, env: &mut Env) {
    let res = eval_expr_in_env(expr, env);
    if let ExprKind::Atom(Atom::Unit) = res.kind {
    } else {
        panic!("'{}' was expected to be a unit, but was not a unit.", expr);
    }
}
