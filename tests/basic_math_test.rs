use spressolisp::{
    ast::Number,
    env::Env,
    errors::{SpressoErrorType, SyntaxError},
    eval::extract_num,
    evaluate_expression,
};

#[macro_use]
extern crate assert_float_eq;

#[test]
fn test_addition() {
    let mut env = Env::new();
    // integer
    if let Ok(res) = evaluate_expression("(+ 12 32)".to_string(), &mut env) {
        if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
            assert_eq!(res, 44);
        } else {
            assert!(false, "Result was not an integer");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
    // floating point
    if let Ok(res) = evaluate_expression("(+ 12.3 43.2)".to_string(), &mut env) {
        if let Ok(Number::Float(res)) = extract_num(res, &mut env) {
            assert_f64_near!(res, 55.5);
        } else {
            assert!(false, "Result was not a float");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_subtraction() {
    let mut env = Env::new();
    // integer
    if let Ok(res) = evaluate_expression("(- 12 32)".to_string(), &mut env) {
        if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
            assert_eq!(res, -20);
        } else {
            assert!(false, "Result was not an integer");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
    // floating point
    if let Ok(res) = evaluate_expression("(- 12.3 10.1)".to_string(), &mut env) {
        if let Ok(Number::Float(res)) = extract_num(res, &mut env) {
            assert_f64_near!(res, 2.2);
        } else {
            assert!(false, "Result was not a float");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_multiplication() {
    let mut env = Env::new();
    // integer
    if let Ok(res) = evaluate_expression("(* 2 3)".to_string(), &mut env) {
        if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
            assert_eq!(res, 6);
        } else {
            assert!(false, "Result was not an integer");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
    // floating point
    if let Ok(res) = evaluate_expression("(* 12.3 10)".to_string(), &mut env) {
        if let Ok(Number::Float(res)) = extract_num(res, &mut env) {
            assert_f64_near!(res, 123.0);
        } else {
            assert!(false, "Result was not a float");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_division() {
    let mut env = Env::new();
    // integer
    if let Ok(res) = evaluate_expression("(/ 10 5)".to_string(), &mut env) {
        if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
            assert_eq!(res, 2);
        } else {
            assert!(false, "Result was not an integer");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
    // floating point
    if let Ok(res) = evaluate_expression("(/ 12.3 10)".to_string(), &mut env) {
        if let Ok(Number::Float(res)) = extract_num(res, &mut env) {
            assert_f64_near!(res, 1.23);
        } else {
            assert!(false, "Result was not a float");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_wrong_addition_syntax() {
    let mut env = Env::new();
    let inp = evaluate_expression("(+ 12 32".to_string(), &mut env);

    if let Err(err) = inp {
        if let SpressoErrorType::Syntax(SyntaxError { err: err_str }) = err.detail {
            assert_eq!(err_str, "'(' not closed");
        } else {
            assert!(false, "Wrong type of error. Hmm.");
        }
    } else {
        assert!(false, "Invalid Expression successfully evaluated");
    }
}
