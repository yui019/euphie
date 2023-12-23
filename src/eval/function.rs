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

    let mut markers: Vec<usize> = vec![];
    for i in 0..params.len() {
        let param = params[i].clone();
        if param == "&optional" || param == "&key" || param == "&rest" {
            markers.push(i);
        }
    }
    markers.push(params.len()); // add a placeholder marker at the end, this is used later on
    markers.sort();

    let mut required: Vec<String> = vec![];
    let mut optional: Vec<String> = vec![];
    let mut keyword: Vec<String> = vec![];
    let mut rest: Option<String> = None;

    println!("{:?}", markers);
    if markers.len() > 0 {
        let mut last_marker_index: i32 = -1;
        for marker in &markers {
            for i in last_marker_index..(*marker as i32) {
                if last_marker_index == -1 {
                    if i == -1 {
                        continue;
                    }
                    required.push(params[i as usize].clone());
                } else {
                    // skip if the current index is a marker
                    if markers.contains(&(i as usize)) {
                        continue;
                    }

                    match params[last_marker_index as usize].as_str() {
                        "&optional" => optional.push(params[i as usize].clone()),
                        "&key" => keyword.push(params[i as usize].clone()),
                        "&rest" => {
                            if rest.is_some() {
                                return Err(String::from("There can be only 1 rest parameter"));
                            } else {
                                rest = Some(params[i as usize].clone());
                            }
                        }

                        _ => unreachable!(),
                    }
                }
            }

            last_marker_index = *marker as i32;
        }
    } else {
        required = params;
    }

    Ok(Value::Lambda {
        params: LambdaParams {
            required,
            optional,
            keyword,
            rest,
        },
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
            // first set all parameters to nil (or empty list in the case of the rest parameter)
            for name in params.required.iter() {
                new_env.borrow_mut().set(name, Value::Nil);
            }
            for name in params.optional.iter() {
                new_env.borrow_mut().set(name, Value::Nil);
            }
            for name in params.keyword.iter() {
                new_env.borrow_mut().set(name, Value::Nil);
            }
            if params.rest.is_some() {
                new_env
                    .borrow_mut()
                    .set(&params.rest.clone().unwrap(), Value::List(vec![]));
            }

            // set required and optional parameter values
            let mut last_index = 1;
            let mut last_required = 0;
            for (i, param_val) in list[1..].iter().enumerate() {
                let val = eval_value(&param_val, env)?;

                if i < params.required.len() {
                    let name = &params.required[i];
                    new_env.borrow_mut().set(name, val);
                    last_required = i;
                } else if i < params.required.len() + params.optional.len() {
                    let name = &params.optional[i - last_required - 1];
                    new_env.borrow_mut().set(name, val);
                } else {
                    break;
                }

                last_index = i;
            }

            // if there's more parameters after required and optional ones, set the rest parameter to a list of them
            if list.len() > last_index + 2 {
                if params.rest.is_some() {
                    let mut values: Vec<Value> = vec![];
                    for value in list[(last_index + 2)..].iter() {
                        values.push(eval_value(value, env)?);
                    }
                    new_env
                        .borrow_mut()
                        .set(&params.rest.clone().unwrap(), Value::List(values));
                }
            }

            for (i, param_val) in list[(last_index + 2)..].iter().enumerate() {
                match eval_value(&param_val, env)? {
                    Value::Symbol(s) => {
                        // if it starts with a :, it's a keyword
                        if s.starts_with(":") {
                            let name = &String::from(&s[1..]);
                            // if there's a parameter after the keyword and there's a keyword parameter with that name
                            if last_index + 2 + i + 1 < list.len() && params.keyword.contains(name)
                            {
                                let value = list[last_index + 2 + i + 1].clone();
                                new_env.borrow_mut().set(name, eval_value(&value, env)?);
                            }
                        }
                    }

                    _ => {}
                }
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
