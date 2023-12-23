use super::eval_value;
use crate::{env::*, parse::*};
use std::{cell::RefCell, rc::Rc};

pub fn eval_fun_definition(list: &[Value], _env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let params = match &list[1] {
        Value::List(list) => {
            let mut params = Vec::new();
            for param in list {
                match param {
                    Value::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid parameter name")),
                }
            }
            params
        }
        _ => {
            return Err(String::from(
                "First parameter to \"lambda\" must be a list of parameter names",
            ))
        }
    };

    let body = Rc::from(list[2].clone());

    Ok(Value::Lambda {
        params,
        body,
        is_macro: false,
    })
}

pub fn eval_fun_call(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let first = eval_value(&list[0], env)?;

    let fun = match first {
        Value::Symbol(s) => {
            let lamdba = env.borrow_mut().get(&s);
            if lamdba.is_none() {
                return Err(format!("Unbound symbol: {}", s));
            }

            lamdba.unwrap()
        }
        Value::Lambda { .. } => first.clone(),

        _ => return Err(String::from("First parameter is not a function")),
    };

    match fun {
        Value::Lambda {
            params,
            body,
            is_macro,
        } => {
            let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = eval_value(&list[i + 1], env)?;
                new_env.borrow_mut().set(param, val);
            }

            let value = eval_value(body.as_ref(), &mut new_env)?;

            if is_macro {
                // if it's a macro, evaluate the code it returns (with the calling code's environment)
                return eval_value(&value, env);
            } else {
                // if it's a function, just return its value
                return Ok(value);
            }
        }
        _ => unreachable!(),
    }
}

pub fn eval_macro_definition(list: &[Value], _env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let function = eval_fun_definition(list, _env);
    match function {
        Ok(Value::Lambda { params, body, .. }) => Ok(Value::Lambda {
            params,
            body,
            is_macro: true,
        }),

        _ => function,
    }
}

pub fn eval_macro_expand(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    if list.len() != 2 {
        return Err(String::from("\"macroexpand\" requires 1 argument"));
    }

    let value = list[1].clone();

    match eval_value(&value, env)? {
        Value::List(list) => {
            if list.len() > 1 {
                match eval_value(&list[0], env)? {
                    Value::Lambda { params, body, .. } => {
                        // macroexpand is really the same as calling the macro as a function (instead of a macro)

                        let mut new_value: Vec<Value> = vec![];
                        new_value.push(Value::Lambda {
                            params,
                            body,
                            is_macro: false,
                        });

                        for item in &list[1..] {
                            new_value.push(item.clone());
                        }

                        eval_value(&Value::List(new_value), env)
                    }

                    _ => Ok(value),
                }
            } else {
                Ok(value)
            }
        }

        _ => Ok(value),
    }
}
