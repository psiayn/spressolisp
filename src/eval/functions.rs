use crate::{
    ast::{Atom, Expr, Lambda},
    env::Env,
    errors::{RuntimeError, SpressoError},
    eval::execute,
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
        Expr::Atom(Atom::Symbol(fn_param)) => Ok(Expr::Lambda(Lambda {
            params: vec![fn_param],
            body,
        })),
        Expr::List(fn_params) => {
            let params: Result<Vec<String>, SpressoError> = fn_params
                .into_iter()
                .map(|param| {
                    if let Expr::Atom(Atom::Symbol(param)) = param {
                        Ok(param)
                    } else {
                        Err(SpressoError::from(RuntimeError::from(
                            "lambda parameters must be a symbol",
                        )))
                    }
                })
                .collect();

            Ok(Expr::Lambda(Lambda {
                params: params?,
                body,
            }))
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
        .map(|arg| execute(&mut vec![arg], env))
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
            execute(&mut lambda.body.clone(), env)
        })
    }
}
