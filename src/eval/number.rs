use crate::{
    ast::{Atom, Expr, ExprKind, Number},
    env::Env,
    errors::{NumericError, RuntimeError, SpressoError},
    eval::execute,
    TokenGiver, TokenHoarder,
};

fn number_op(
    args: Vec<Expr>,
    env: &mut Env,
    init: Number,
    op: fn(Number, Number) -> Result<Number, SpressoError>,
) -> Result<Expr, SpressoError> {
    Ok(
        ExprKind::Atom(Atom::Number(args.into_iter().try_fold::<_, _, Result<
            Number,
            SpressoError,
        >>(init, |x, y| {
            let num = extract_num(y, env)?;
            op(x, num)
        })?))
        .into(),
    )
}

pub fn extract_num(expr: Expr, env: &mut Env) -> Result<Number, SpressoError> {
    match expr.kind {
        ExprKind::Atom(Atom::Number(number)) => Ok(number),
        ExprKind::Atom(Atom::Symbol(ref symbol)) => {
            if env.contains_key(symbol.as_str()) {
                let sym = env[&symbol.as_str()].clone();
                match sym.kind {
                    ExprKind::Atom(Atom::Number(num)) => Ok(num),
                    _ => Err(SpressoError::from(NumericError {
                        err: "Tried to extract num from variable but failed".to_string(),
                    })
                    .maybe_with_tokens(sym.get_tokens())
                    .maybe_with_tokens(expr.get_tokens())),
                }
            } else {
                return Err(SpressoError::from(RuntimeError::from(format!(
                    "Symbol not found: {}",
                    symbol
                ))));
            }
        }
        ExprKind::List(mut exprs) => {
            let res = execute(&mut exprs, env)?;
            match res.kind {
                ExprKind::Atom(Atom::Number(num)) => Ok(num),
                _ => Err(SpressoError::from(NumericError::from(format!(
                    "trying to perform arithmetic on non-number: {}",
                    res
                )))),
            }
        }
        _ => Err(SpressoError::from(NumericError::from(format!(
            "trying to perform arithmetic on non-number: {}",
            expr
        )))),
    }
}

pub fn add(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    number_op(args, env, Number::Int(0), |x, y| x + y)
}

pub fn mul(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    number_op(args, env, Number::Int(1), |x, y| x * y)
}

pub fn sub(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut args = args.clone();
    let start = extract_num(args.remove(0), env)?;
    number_op(args, env, start, |x, y| x - y)
}

pub fn div(args: Vec<Expr>, env: &mut Env) -> Result<Expr, SpressoError> {
    let mut args = args.clone();
    let start = extract_num(args.remove(0), env)?;
    number_op(args, env, start, |x, y| x / y)
}
