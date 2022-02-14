use crate::{
    ast::{Atom, AtomKind, Expr, Lambda},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute_single,
    TokenGiver, TokenHoarder,
};

pub fn lambda(args: Vec<Expr>, _env: &mut Env) -> Result<Expr, SpressoError> {
    if args.len() < 2 {
        return Err(SpressoError::from(RuntimeError::from(
            "A lambda definition must have a param list and a body (any number of lists)",
        )));
    }

    let fn_params = args[0].clone();
    let body = args[1..].to_vec();

    match fn_params {
        Expr::Atom(Atom {
            kind: AtomKind::Symbol(fn_param),
            ..
        }) => Ok(Expr::Lambda(lambda_with_tokens(vec![fn_param], body, args))),
        Expr::List(fn_params) => {
            let params: Result<Vec<String>, SpressoError> = fn_params
                .into_iter()
                .map(|param| {
                    if let Expr::Atom(Atom {
                        kind: AtomKind::Symbol(param),
                        ..
                    }) = param
                    {
                        Ok(param)
                    } else {
                        Err(SpressoError::from(RuntimeError::from(
                            "lambda parameters must be a symbol",
                        )))
                    }
                })
                .collect();

            Ok(Expr::Lambda(lambda_with_tokens(params?, body, args)))
        }
        _ => Err(SpressoError::from(RuntimeError::from(
            "lambda parameters must be a symbol",
        ))),
    }
}

pub fn execute_lambda(
    lambda: Lambda,
    args: Vec<Expr>,
    env: &mut Env,
) -> Result<Expr, SpressoError> {
    let args: Result<Vec<Expr>, SpressoError> = args
        .into_iter()
        .map(|arg| execute_single(arg, env))
        .collect();
    let args = args?;

    if args.len() != lambda.params.len() {
        Err(SpressoError::from(RuntimeError::from(format!(
            "Expected {} arguments, got {}",
            lambda.params.len(),
            args.len()
        ))))
    } else {
        env.in_new_scope(|env| {
            args.into_iter().enumerate().for_each(|(i, arg)| {
                env.insert(lambda.params[i].as_str(), arg);
            });

            lambda
                .body
                .clone()
                .into_iter()
                .map(|expr| execute_single(expr, env))
                .take_while(Result::is_ok)
                .last()
                // TODO: replace this with empty value (unit?)
                .unwrap_or(Ok(Expr::Atom(Atom::new_bool(false))))
        })
    }
}

fn lambda_with_tokens(fn_params: Vec<String>, body: Vec<Expr>, exprs: Vec<Expr>) -> Lambda {
    let mut new_lambda = Lambda::new(fn_params, body);
    for expr in exprs {
        new_lambda = new_lambda.with_tokens(expr.get_tokens());
    }
    new_lambda
}
