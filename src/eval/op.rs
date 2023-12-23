use super::eval_value;
use crate::{env::*, parse::*};
use std::{cell::RefCell, rc::Rc};

pub fn eval_arithmetic_op(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let head = &list[0];
    let tail = &list[1..];

    match head {
        Value::Symbol(s) => match s.as_str() {
            "+" => {
                let mut r: f64 = 0.0;
                for v in tail {
                    match eval_value(v, env)? {
                        Value::Number(n) => r += n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "-" => {
                // return the negative if there's only 1 argument
                if tail.len() == 1 {
                    return match eval_value(&tail[0], env)? {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => return Err(String::from("All arguments must be numbers")),
                    };
                }

                let mut r: f64 = match eval_value(&tail[0], env)? {
                    Value::Number(n) => n,
                    _ => return Err(String::from("All arguments must be numbers")),
                };

                for v in &tail[1..] {
                    match eval_value(v, env)? {
                        Value::Number(n) => r -= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "*" => {
                let mut r: f64 = 1.0;
                for v in tail {
                    match eval_value(v, env)? {
                        Value::Number(n) => r *= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            "/" => {
                let mut r: f64 = match eval_value(&tail[0], env)? {
                    Value::Number(n) => n,
                    _ => return Err(String::from("All arguments must be numbers")),
                };
                for v in &tail[1..] {
                    match eval_value(v, env)? {
                        Value::Number(n) => r /= n,
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                Ok(Value::Number(r))
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

pub fn eval_comparison_op(list: &[Value], env: &mut Rc<RefCell<Env>>) -> Result<Value, String> {
    let head = &list[0];
    let tail = &list[1..];

    match head {
        Value::Symbol(s) => match s.as_str() {
            ">" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\">\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a > b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "<" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"<\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a < b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            ">=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\">=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a >= b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "<=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"<=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a <= b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a == b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            "!=" => {
                let mut r: bool = true;
                if tail.len() < 2 {
                    return Err(String::from("\"!=\" requires at least 2 arguments"));
                }

                for i in 1..tail.len() {
                    match (eval_value(&tail[i - 1], env)?, eval_value(&tail[i], env)?) {
                        (Value::Number(a), Value::Number(b)) => {
                            if !(a != b) {
                                r = false;
                                break;
                            }
                        }
                        _ => return Err(String::from("All arguments must be numbers")),
                    }
                }

                if r {
                    Ok(Value::T)
                } else {
                    Ok(Value::Nil)
                }
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}
