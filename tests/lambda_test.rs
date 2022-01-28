use spressolisp::{ast::Number, env::Env, eval::extract_num, evaluate_expression};

#[test]
fn test_lambda_basic() {
    let mut env = Env::new();
    if let Ok(_) = evaluate_expression("(define mul_100 (lambda x (* x 100)))".to_string(), &mut env) {
        if let Ok(res) = evaluate_expression("(mul_100 3)".to_string(), &mut env) {
            if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
                assert_eq!(res, 300);
            } else {
                assert!(false, "Result is not an integer");
            }
        } else {
            assert!(false, "Error evaluating expression");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}

#[test]
fn test_lambda_two_arg() {
    let mut env = Env::new();
    if let Ok(_) = evaluate_expression("(define mul (lambda (x y) (* x y)))".to_string(), &mut env) {
        if let Ok(res) = evaluate_expression("(mul 3 4)".to_string(), &mut env) {
            if let Ok(Number::Int(res)) = extract_num(res, &mut env) {
                assert_eq!(res, 12);
            } else {
                assert!(false, "Result is not an integer");
            }
        } else {
            assert!(false, "Error evaluating expression");
        }
    } else {
        assert!(false, "Error evaluating expression");
    }
}
